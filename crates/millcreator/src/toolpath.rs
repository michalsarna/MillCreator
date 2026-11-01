//! From the shapes, the layers, the bits and the device to the moves of the machine, and from the moves to G-code.
//!
//! The moves are in the coordinates of the work area: X to the right, Y away from you (the drawing is turned over, as
//! the scene counts Y downwards), Z = 0 at the top of the material and negative below it.
use crate::device::{Device, Origin};
use crate::layers::{MillLayer, Operation, PictureMethod};
use crate::materials::Stock;
use crate::tools::{Library, Tool};
use crate::warn::Warn;
use base_core::geom::{Polyline, Pt};
use base_core::image::ImageData;
use base_core::ops;
use base_core::raster;
use base_core::scene::{Kind, Scene, Shape};

pub type P3 = [f64; 3];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveKind {
    /// A fast move above the material.
    Rapid,
    /// Straight down or up in the material.
    Plunge,
    /// A cut.
    Cut,
}

#[derive(Clone, Debug)]
pub struct Move {
    pub kind: MoveKind,
    pub from: P3,
    pub to: P3,
    pub layer: usize,
    /// Feed (mm/min) for plunges and cuts.
    pub feed: f64,
}

impl Move {
    pub fn length(&self) -> f64 {
        let d = |i: usize| self.to[i] - self.from[i];
        (d(0) * d(0) + d(1) * d(1) + d(2) * d(2)).sqrt()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Program {
    pub moves: Vec<Move>,
    /// What could not be done, or looks wrong, in words for the user.
    pub warnings: Vec<Warn>,
    /// The box that holds the cuts: the lowest and the highest corner (X, Y, Z).
    pub bounds: Option<(P3, P3)>,
}

impl Program {
    /// The length of the cuts and the plunges, in mm.
    pub fn cut_length(&self) -> f64 {
        self.moves.iter().filter(|m| m.kind != MoveKind::Rapid).map(|m| m.length()).sum()
    }

    /// The time the machine needs, in seconds, from the feeds and the speed of the moves above the material.
    pub fn estimate_seconds(&self, rapid: f64) -> f64 {
        self.moves
            .iter()
            .map(|m| {
                let speed = if m.kind == MoveKind::Rapid { rapid } else { m.feed };
                if speed > 0.0 {
                    m.length() / speed * 60.0
                } else {
                    0.0
                }
            })
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }
}

/// The part of the program that is being built: it knows where the spindle is.
pub(crate) struct Builder<'a> {
    pub(crate) moves: Vec<Move>,
    pub(crate) at: P3,
    pub(crate) safe: f64,
    pub(crate) layer: usize,
    pub(crate) feed: f64,
    pub(crate) plunge: f64,
    pub(crate) dev: &'a Device,
}

impl Builder<'_> {
    pub(crate) fn push(&mut self, kind: MoveKind, to: P3, feed: f64) {
        if self.at == to {
            return;
        }
        self.moves.push(Move { kind, from: self.at, to, layer: self.layer, feed });
        self.at = to;
    }
    /// Up to the safe height.
    pub(crate) fn retract(&mut self) {
        if self.at[2] < self.safe {
            let to = [self.at[0], self.at[1], self.safe];
            let kind = if self.at[2] < 0.0 { MoveKind::Plunge } else { MoveKind::Rapid };
            self.push(kind, to, self.feed);
        }
    }
    /// To a point above the material and down to `z` (negative).
    pub(crate) fn dive(&mut self, x: f64, y: f64, z: f64) {
        self.retract();
        self.push(MoveKind::Rapid, [x, y, self.safe], self.dev.rapid);
        // Fast to just above the surface, then at the plunge feed.
        if self.safe > 1.0 {
            self.push(MoveKind::Rapid, [x, y, 1.0], self.dev.rapid);
        }
        self.push(MoveKind::Plunge, [x, y, z], self.plunge);
    }
    pub(crate) fn cut_to(&mut self, x: f64, y: f64, z: f64) {
        self.push(MoveKind::Cut, [x, y, z], self.feed);
    }
    pub(crate) fn down_to(&mut self, z: f64) {
        let [x, y, _] = self.at;
        self.push(MoveKind::Plunge, [x, y, z], self.plunge);
    }
}

/// A closed shape of the layer with holes, as rings: is the ring inside an odd number of the others?
pub(crate) fn is_hole(rings: &[Polyline], i: usize) -> bool {
    let Some(p) = rings[i].pts.first().copied() else { return false };
    rings.iter().enumerate().filter(|(j, r)| *j != i && r.closed && r.contains(p)).count() % 2 == 1
}

/// The outlines of the shapes of a layer in the coordinates of the work area. Pictures are not outlines; they are counted
/// when the layer does not engrave them (`count_pictures`).
fn layer_outlines(scene: &Scene, layer: usize, area_y: f64, images: &mut usize, count_pictures: bool) -> Vec<Polyline> {
    let mut out = vec![];
    for s in scene.shapes.iter().filter(|s| s.layer == layer) {
        if matches!(s.kind, Kind::Image(_)) {
            if count_pictures {
                *images += 1;
            }
            continue;
        }
        for p in s.polys() {
            if p.pts.len() < 2 {
                continue;
            }
            let pts = p.pts.iter().map(|q| Pt::new(q.x, area_y - q.y)).collect();
            out.push(Polyline::new(pts, p.closed));
        }
    }
    out
}

/// The pictures on a layer.
fn layer_pictures(scene: &Scene, layer: usize) -> Vec<(&Shape, &ImageData)> {
    scene.shapes.iter().filter(|s| s.layer == layer).filter_map(|s| if let Kind::Image(im) = &s.kind { Some((s, im)) } else { None }).collect()
}

/// The most moves a program may have: a picture with a very fine step makes millions.
const MAX_MOVES: usize = 3_000_000;

/// Engrave a picture: dots, rows, or rows whose depth follows the tone. The distance of the rows is the step over of the bit.
/// Returns false when the program got too long and the rest was left out.
fn engrave_picture(b: &mut Builder, layer: &MillLayer, tool: &Tool, shape: &Shape, im: &ImageData, area_y: f64, depth: f64) -> bool {
    let step = (tool.dims.diameter * layer.step_over_pct / 100.0).max(0.05);
    if layer.picture == PictureMethod::Relief {
        return relief_passes(b, layer, tool, shape, im, area_y, depth, step);
    }
    let mode = if layer.picture == PictureMethod::Depth { raster::Dither::Grayscale } else { layer.dither };
    let Some(g) = raster::sample(shape, im, step, layer.scan_angle, mode) else { return true };
    let at = |p: Pt| -> (f64, f64) {
        let q = g.rot.apply(p);
        (q.x, area_y - q.y)
    };
    let mut k = 0usize;
    for r in 0..g.rows {
        // Every second row is cut the other way round, so the bit does not travel back with nothing to do.
        let cols: Vec<usize> = if k % 2 == 0 { (0..g.cols).collect() } else { (0..g.cols).rev().collect() };
        let mut started = false;
        let mut cut_any = false;
        let y = g.min.y + (r as f64 + 0.5) * g.step;
        match layer.picture {
            PictureMethod::Dots => {
                for &c in &cols {
                    if g.at(c, r) >= 0.5 {
                        let (x, yy) = at(g.centre(c, r));
                        b.dive(x, yy, -depth);
                        cut_any = true;
                    }
                }
            }
            PictureMethod::Rows => {
                // Runs of dark cells, in the order of the row.
                let mut c = 0;
                let mut runs: Vec<(usize, usize)> = vec![];
                while c < g.cols {
                    if g.at(c, r) < 0.5 {
                        c += 1;
                        continue;
                    }
                    let s = c;
                    while c < g.cols && g.at(c, r) >= 0.5 {
                        c += 1;
                    }
                    runs.push((s, c));
                }
                if k % 2 == 1 {
                    runs.reverse();
                }
                for (s, e) in runs {
                    let (a, z) = (Pt::new(g.min.x + s as f64 * g.step, y), Pt::new(g.min.x + e as f64 * g.step, y));
                    let (a, z) = if k % 2 == 1 { (z, a) } else { (a, z) };
                    let (x0, y0) = at(a);
                    let (x1, y1) = at(z);
                    b.dive(x0, y0, -depth);
                    b.cut_to(x1, y1, -depth);
                    cut_any = true;
                }
            }
            PictureMethod::Relief => {}
            PictureMethod::Depth => {
                for &c in &cols {
                    let level = g.at(c, r) as f64;
                    if level < 0.02 {
                        if started {
                            b.retract();
                            started = false;
                        }
                        continue;
                    }
                    let z = -(depth * level.min(1.0)).max(0.01);
                    let (x, yy) = at(g.centre(c, r));
                    if started {
                        b.cut_to(x, yy, z);
                    } else {
                        b.dive(x, yy, z);
                        started = true;
                    }
                    cut_any = true;
                }
                if started {
                    b.retract();
                }
            }
        }
        if cut_any {
            k += 1;
        }
        if b.moves.len() > MAX_MOVES {
            return false;
        }
    }
    true
}

/// A picture as a relief: passes down to the full depth (the last one is the relief itself), rows `step` apart, the bit kept out
/// of the relief everywhere under its body. Returns false when the program got too long.
fn relief_passes(b: &mut Builder, layer: &MillLayer, tool: &Tool, shape: &Shape, im: &ImageData, area_y: f64, depth: f64, step: f64) -> bool {
    let Some(map) = crate::relief::HeightMap::of(shape, im, step, layer.scan_angle, depth) else { return true };
    let mut tips = crate::relief::tip_heights(&map, tool);
    // Roughing leaves the stock to leave on the relief; finishing is one go over the relief itself.
    use crate::layers::ReliefPart;
    if layer.relief_part == ReliefPart::Rough && layer.leave > 0.0 {
        for t in tips.iter_mut() {
            *t = (*t + layer.leave).min(0.0);
        }
    }
    let limits = if layer.relief_part == ReliefPart::Finish { vec![depth] } else { crate::relief::pass_limits(depth, layer.step_down) };
    for limit in limits {
        let mut k = 0usize;
        for r in 0..map.grid.rows {
            let runs = crate::relief::row_runs(&map, &tips, r, limit, k % 2 == 1, area_y);
            if runs.is_empty() {
                continue;
            }
            k += 1;
            for run in runs {
                let Some(first) = run.first() else { continue };
                b.dive(first[0], first[1], first[2]);
                for q in run.iter().skip(1) {
                    b.cut_to(q[0], q[1], q[2]);
                }
            }
            if b.moves.len() > MAX_MOVES {
                return false;
            }
        }
    }
    true
}

fn closed_only(polys: &[Polyline]) -> Vec<Polyline> {
    polys.iter().filter(|p| p.closed && p.pts.len() >= 3).cloned().collect()
}

/// Go round a ring at every depth of the layer; the ring is closed, so the next pass follows without a retract.
fn ring_passes(b: &mut Builder, ring: &Polyline, depths: &[f64]) {
    let Some(first) = ring.pts.first() else { return };
    b.dive(first.x, first.y, -depths[0]);
    for (i, d) in depths.iter().enumerate() {
        if i > 0 {
            b.down_to(-d);
        }
        for q in ring.pts.iter().skip(1) {
            b.cut_to(q.x, q.y, -d);
        }
        b.cut_to(first.x, first.y, -d);
    }
}

fn open_passes(b: &mut Builder, line: &Polyline, depths: &[f64]) {
    let Some(first) = line.pts.first() else { return };
    b.dive(first.x, first.y, -depths[0]);
    for (i, d) in depths.iter().enumerate() {
        if i > 0 {
            b.down_to(-d);
        }
        let forward = i % 2 == 0;
        let pts: Vec<&Pt> = if forward { line.pts.iter().collect() } else { line.pts.iter().rev().collect() };
        for q in pts.into_iter().skip(1) {
            b.cut_to(q.x, q.y, -d);
        }
    }
}

fn centre(p: &Polyline) -> Option<Pt> {
    p.bounds().map(|r| r.center())
}

/// What the program asks of a job beyond the project (an option of the Control tab, never saved).
#[derive(Clone, Copy, Debug, Default)]
pub struct JobOptions {
    /// Cut the elements one after the other: all the layers (and their passes) of an element, a group or a single shape that
    /// is in none, before the next one, the nearest element first. Off: a layer after a layer. The layers of an element are
    /// meant to use the same bit; with different bits the bit is changed (M0) at every switch.
    pub one_by_one: bool,
}

/// The toolpaths of every enabled layer, in the order the layers are done in (their rank, then their number).
#[cfg(test)]
pub fn generate(scene: &Scene, layers: &[MillLayer], tools: &Library, dev: &Device, stock: &Stock) -> Program {
    generate_with(scene, layers, tools, dev, stock, JobOptions::default())
}

/// The elements of the scene in the order they are cut one by one: groups (even over several layers) and single shapes, each next
/// one the nearest to where the head was left (it starts at the machine's zero). Each element is a scene of its own shapes.
fn elements_by_distance(scene: &Scene, dev: &Device) -> Vec<Scene> {
    let mut elements: Vec<(Option<u64>, Vec<Shape>)> = vec![];
    for s in &scene.shapes {
        match s.group.and_then(|g| elements.iter().position(|(k, _)| *k == Some(g))) {
            Some(i) => elements[i].1.push(s.clone()),
            None => elements.push((s.group, vec![s.clone()])),
        }
    }
    let (next_id, next_group) = scene.counters();
    let center = |e: &[Shape]| -> Pt {
        match e.iter().filter_map(|s| s.bounds()).reduce(base_core::geom::Rect::union) {
            Some(b) => Pt::new((b.min.x + b.max.x) / 2.0, (b.min.y + b.max.y) / 2.0),
            None => Pt::new(0.0, 0.0),
        }
    };
    let mut left: Vec<(Pt, Vec<Shape>)> = elements.into_iter().map(|(_, e)| (center(&e), e)).collect();
    // The machine's zero is the bottom left of the work area; the scene counts from the top.
    let (mut at, mut out) = (Pt::new(0.0, dev.area.y), vec![]);
    while !left.is_empty() {
        let next = (0..left.len()).min_by(|&a, &b| {
            let d = |i: usize| (left[i].0.x - at.x).powi(2) + (left[i].0.y - at.y).powi(2);
            d(a).total_cmp(&d(b))
        });
        let (c, e) = left.remove(next.unwrap_or(0));
        at = c;
        out.push(Scene::from_parts(e, next_id, next_group));
    }
    out
}

/// The toolpaths as `generate` makes them, with what the program asks of the job (`JobOptions`).
pub fn generate_with(scene: &Scene, layers: &[MillLayer], tools: &Library, dev: &Device, stock: &Stock, opts: JobOptions) -> Program {
    let mut prog = Program::default();
    let mut b = Builder { moves: vec![], at: [0.0, 0.0, dev.safe_z], safe: dev.safe_z, layer: 0, feed: 0.0, plunge: 0.0, dev };
    if opts.one_by_one {
        for element in elements_by_distance(scene, dev) {
            run_layers(&mut b, &mut prog, &element, layers, tools, dev, stock);
        }
        // Every element repeats the warnings of its layers: say each one once.
        let mut seen = std::collections::HashSet::new();
        prog.warnings.retain(|w| seen.insert(w.to_string()));
    } else {
        run_layers(&mut b, &mut prog, scene, layers, tools, dev, stock);
    }
    prog.moves = b.moves;
    // The machine is already above the first cut when the program starts: no move comes from the corner of the table.
    if let Some(first) = prog.moves.first_mut() {
        if first.kind == MoveKind::Rapid && first.from == [0.0, 0.0, dev.safe_z] {
            first.from = first.to;
        }
    }
    prog.bounds = bounds(&prog.moves);
    if dev.rotary.enabled {
        if let Some((lo, hi)) = prog.bounds {
            let round = dev.rotary.circumference();
            if hi[1] - lo[1] > round + 1e-6 {
                prog.warnings.push(Warn::new("The drawing is higher than the way round the workpiece ({} mm)", &[&format!("{round:.1}")]));
            }
        }
    }
    if let Some((lo, hi)) = prog.bounds {
        if lo[0] < -1e-6 || lo[1] < -1e-6 || hi[0] > dev.area.x + 1e-6 || hi[1] > dev.area.y + 1e-6 {
            prog.warnings.push(Warn::new("The cuts go outside of the work area of the device", &[]));
        }
    }
    prog
}

/// The layers of `scene` one after the other, in the order of their rank, into `b`.
fn run_layers(b: &mut Builder, prog: &mut Program, scene: &Scene, layers: &[MillLayer], tools: &Library, dev: &Device, stock: &Stock) {
    for li in crate::layers::layer_order(layers) {
        let layer = &layers[li];
        let mut images = 0;
        let picture_layer = layer.op == Operation::Picture;
        let outlines = layer_outlines(scene, li, dev.area.y, &mut images, !picture_layer);
        if images > 0 && layer.enabled {
            prog.warnings.push(Warn::new("{}: pictures are not milled", &[&layer.name]));
        }
        let pictures = if picture_layer { layer_pictures(scene, li) } else { vec![] };
        if !layer.enabled || (outlines.is_empty() && pictures.is_empty()) {
            continue;
        }
        if let Err(e) = layer.check() {
            prog.warnings.push(Warn::new("{}", &[&e]));
            continue;
        }
        let Some(tool) = layer.tool.as_deref().and_then(|id| tools.get(id)) else {
            prog.warnings.push(Warn::new("{}: no bit chosen", &[&layer.name]));
            continue;
        };
        // A pass that goes deeper in one go than the cutting part of the bit is long.
        let one_pass = if layer.step_down <= 0.0 { layer.depth } else { layer.step_down.min(layer.depth) };
        if tool.dims.cut_length > 0.0 && one_pass > tool.dims.cut_length + 1e-9 && !matches!(layer.op, Operation::Drill) {
            prog.warnings.push(Warn::new("{}: one pass of {} mm is deeper than the cutting part of the bit ({} mm)", &[&layer.name, &format!("{one_pass:.1}"), &tool.dims.cut_length]));
        }
        if dev.changer && tool.number == 0 {
            prog.warnings.push(Warn::new("{}: the bit has no changer number: the machine is asked to stop for it instead", &[&layer.name]));
        }
        if layer.dust && dev.gcode.dust_on.trim().is_empty() {
            prog.warnings.push(Warn::new("{}: the layer asks for the dust collector, but the device has no lines that switch it on", &[&layer.name]));
        }
        let mut layer = layer.clone();
        if layer.depth > stock.thickness + 1e-9 {
            prog.warnings.push(Warn::new("{}: the depth of {} mm is more than the thickness of the material ({} mm)", &[&layer.name, &layer.depth, &stock.thickness]));
        }
        if layer.depth > dev.area.z {
            prog.warnings.push(Warn::new("{}: the depth is limited to the travel of the machine ({} mm)", &[&layer.name, &dev.area.z]));
            layer.depth = dev.area.z;
        }
        if layer.rpm > dev.spindle.max_rpm || layer.rpm < dev.spindle.min_rpm {
            prog.warnings.push(Warn::new("{}: {} rpm is outside the spindle range {}…{} rpm", &[&layer.name, &layer.rpm, &dev.spindle.min_rpm, &dev.spindle.max_rpm]));
        }
        // A chamfer is as deep as its width asks for with this V-bit.
        if layer.op == Operation::Chamfer {
            match crate::clear::chamfer_depth(tool, layer.chamfer) {
                Some(d) => layer.depth = d,
                None => prog.warnings.push(Warn::new("{}: a chamfer needs a V-bit or an engraving bit", &[&layer.name])),
            }
        }
        let depths = layer.passes();
        if depths.is_empty() {
            continue;
        }
        // A rest pocket: the radius of the bit of the layer whose leftovers it clears.
        let rest_r = layer.rest_of.filter(|_| layer.op == Operation::Pocket).and_then(|i| layers.get(i)).and_then(|l| l.tool.as_deref()).and_then(|id| tools.get(id)).map(|t| t.radius());
        if layer.op == Operation::Pocket && layer.rest_of.is_some() && rest_r.is_none() {
            prog.warnings.push(Warn::new("{}: the layer of the rest pocket has no bit", &[&layer.name]));
        } else if rest_r.is_some_and(|rb| rb <= tool.radius()) {
            prog.warnings.push(Warn::new("{}: the bit of a rest pocket has to be smaller than the bit of the layer it takes over from", &[&layer.name]));
        }
        b.layer = li;
        b.feed = layer.feed;
        b.plunge = layer.plunge;
        let start = b.moves.len();
        if stock.first_pass && stock.first_pass_depth > 0.0 && stock.first_pass_depth < layer.depth - 1e-9 {
            first_pass(b, &layer, tool, &outlines, stock.first_pass_depth);
        }
        // The tabs placed by hand on this layer, where they lie on the toolpath's side of the page (y turned over).
        let anchors: Vec<Pt> = layer.tabs.manual.iter().filter_map(|t| scene.shape(t.shape).filter(|s| s.layer == li).and_then(|s| crate::handtabs::point_on(s, t.at))).map(|p| Pt::new(p.x, dev.area.y - p.y)).collect();
        run_layer(b, &layer, tool, &outlines, &depths, anchors, rest_r, &mut prog.warnings);
        // A picture is engraved at the depth of the layer in one go (a shallow depth: the dots and the rows are many).
        let full = depths.last().copied().unwrap_or(layer.depth);
        for (shape, im) in &pictures {
            if !engrave_picture(b, &layer, tool, shape, im, dev.area.y, full) {
                prog.warnings.push(Warn::new("{}: the program of the picture is too long: the rest was left out (use a bigger step over)", &[&layer.name]));
                break;
            }
        }
        b.retract();
        if b.moves.len() == start {
            prog.warnings.push(Warn::new("{}: nothing to cut (the line may be too small for the bit)", &[&layer.name]));
        }
    }
}


/// One shallow pass along the edges the layer will cut, before its real passes: the line of a profile, the line itself for
/// "Follow the line", the outermost ring of a pocket (not its inside), nothing for drilling.
fn first_pass(b: &mut Builder, layer: &MillLayer, tool: &Tool, outlines: &[Polyline], depth: f64) {
    let r = tool.radius();
    let depths = [depth];
    match layer.op {
        Operation::ProfileOutside | Operation::ProfileInside | Operation::Bore | Operation::DogboneSlot => {
            let d = if layer.op == Operation::ProfileOutside { r } else { -r };
            for ring in ops::offset(&closed_only(outlines), d) {
                ring_passes(b, &ring, &depths);
            }
        }
        Operation::Pocket => {
            for ring in ops::offset(&closed_only(outlines), -r) {
                ring_passes(b, &ring, &depths);
            }
        }
        Operation::OnPath | Operation::Chamfer => {
            for line in outlines {
                if line.closed {
                    ring_passes(b, line, &depths);
                } else {
                    open_passes(b, line, &depths);
                }
            }
        }
        Operation::Drill | Operation::Picture | Operation::Face | Operation::VCarve => {}
    }
}

fn run_layer(b: &mut Builder, layer: &MillLayer, tool: &Tool, outlines: &[Polyline], depths: &[f64], anchors: Vec<Pt>, rest_r: Option<f64>, warnings: &mut Vec<Warn>) {
    use crate::layers::{OutlinePass, PocketPattern};
    use crate::passes::{self, Notes, Plan};
    let r = tool.radius();
    // A bore and a dogbone slot are inside profiles; a bore goes down in a spiral along the hole.
    let inner_profile = matches!(layer.op, Operation::ProfileInside | Operation::Bore | Operation::DogboneSlot);
    let mut plan = Plan::of(layer, tool.dims.diameter, tool.cutting.ramp);
    if layer.op == Operation::Bore && layer.entry == crate::layers::Entry::Plunge {
        plan.entry = crate::layers::Entry::RampContour;
    }
    plan.anchors = anchors;
    plan.reach = if matches!(layer.op, Operation::OnPath | Operation::Chamfer) { 0.0 } else { r + layer.leave.max(0.0) };
    let mut notes = Notes::default();
    let full = depths.last().copied().unwrap_or(layer.depth);
    // The role of the rings for the direction: a bore and a dogbone slot are inside profiles.
    let role_op = if inner_profile { Operation::ProfileInside } else { layer.op };
    // The rings of a profile or a pocket, turned the way the layer asks for, each with its role (a hole of a part or not).
    let go_rings = |b: &mut Builder, rings: Vec<Polyline>, depths: &[f64], notes: &mut Notes| {
        let mut rings = passes::oriented(rings, role_op, layer.direction);
        let hole = passes::roles(&rings);
        if layer.op == Operation::DogboneSlot {
            rings = rings.iter().zip(&hole).map(|(ring, hole)| crate::clear::dogbone(ring, r, *hole)).collect();
        }
        let holes: Vec<Polyline> = rings.iter().zip(&hole).filter(|(_, h)| **h).map(|(r, _)| r.clone()).collect();
        for (i, (ring, hole)) in rings.iter().zip(&hole).enumerate() {
            passes::ring_passes(b, ring, depths, &plan, *hole, &holes, i > 0, notes);
        }
    };
    // Clear a region (where the centre of the bit may go) by rings or by a raster, every depth in turn.
    let step = (tool.dims.diameter * layer.step_over_pct / 100.0).max(0.05);
    let clear_region = |b: &mut Builder, region: Vec<Polyline>, notes: &mut Notes| match layer.pattern {
        PocketPattern::Rings => go_rings(b, crate::clear::rings_with_middle(&region, step), depths, notes),
        PocketPattern::Raster => {
            for d in depths {
                for path in crate::clear::raster_paths(&region, layer.raster_angle, step) {
                    passes::open_passes(b, &path, &[*d], &plan);
                }
            }
        }
    };
    match layer.op {
        Operation::ProfileOutside | Operation::ProfileInside | Operation::Bore | Operation::DogboneSlot => {
            let closed = closed_only(outlines);
            if closed.len() < outlines.len() {
                warnings.push(Warn::new("{}: open lines are left out of a profile (use \"Follow the line\")", &[&layer.name]));
            }
            let d = if layer.op == Operation::ProfileOutside { r } else { -r };
            let left = layer.leave.max(0.0);
            go_rings(b, ops::offset(&closed, d + d.signum() * left), depths, &mut notes);
            // The material that was left on the wall comes off in one more pass at the full depth.
            if left > 0.0 && layer.finish {
                go_rings(b, ops::offset(&closed, d), &[full], &mut notes);
            }
        }
        Operation::Pocket => {
            let closed = closed_only(outlines);
            if closed.len() < outlines.len() {
                warnings.push(Warn::new("{}: open lines are left out of a pocket", &[&layer.name]));
            }
            let left = layer.leave.max(0.0);
            let walls = ops::offset(&closed, -r);
            if layer.outline == OutlinePass::Before {
                go_rings(b, walls.clone(), depths, &mut notes);
            }
            // The region the centre of the bit moves in; for a rest pocket only what the bigger bit left.
            let region = match rest_r {
                Some(rb) if rb > r => crate::clear::rest_region(&closed, rb, r),
                _ => ops::offset(&closed, -(r + left)),
            };
            clear_region(b, region, &mut notes);
            if layer.outline == OutlinePass::After {
                go_rings(b, walls.clone(), depths, &mut notes);
            }
            if left > 0.0 && layer.finish {
                go_rings(b, walls, &[full], &mut notes);
            }
        }
        Operation::Face => {
            // The box of everything on the layer, planed in one go.
            let mut lo = Pt::new(f64::INFINITY, f64::INFINITY);
            let mut hi = Pt::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
            for p in outlines.iter().flat_map(|o| &o.pts) {
                lo = Pt::new(lo.x.min(p.x), lo.y.min(p.y));
                hi = Pt::new(hi.x.max(p.x), hi.y.max(p.y));
            }
            if lo.x.is_finite() && hi.x - lo.x > 1e-6 && hi.y - lo.y > 1e-6 {
                let region = crate::clear::face_region(base_core::geom::Rect { min: lo, max: hi }, layer.overhang, r);
                clear_region(b, region, &mut notes);
            } else {
                warnings.push(Warn::new("{}: nothing to plane: draw a shape that gives the size of the surface", &[&layer.name]));
            }
        }
        Operation::OnPath | Operation::Chamfer | Operation::Picture => {
            for line in outlines {
                // Trochoids: the bit loops along the line, the slot is the loops plus the bit wide.
                let looped;
                let (line, ring) = if layer.trochoid.enabled && layer.op == Operation::OnPath {
                    looped = crate::clear::trochoid(line, layer.trochoid.diameter, layer.trochoid.step, layer.trochoid.half);
                    (&looped, false)
                } else {
                    (line, line.closed)
                };
                if ring {
                    passes::ring_passes(b, line, depths, &plan, false, &[], false, &mut notes);
                } else {
                    passes::open_passes(b, line, depths, &plan);
                }
            }
        }
        Operation::VCarve => {
            use crate::layers::Inlay;
            let closed = closed_only(outlines);
            if closed.len() < outlines.len() {
                warnings.push(Warn::new("{}: open lines are left out of a V-carve", &[&layer.name]));
            }
            // The plug of an inlay is the shape made smaller by the gap, with the walls carved round it.
            let outside = layer.inlay == Inlay::Plug;
            let region = if outside && layer.inlay_gap > 0.0 { ops::offset(&closed, -layer.inlay_gap) } else { closed.clone() };
            // The socket is a little bigger than the shape when there is a gap to keep (the plug stays the shape's size).
            let region = if layer.inlay == Inlay::Socket && layer.inlay_gap > 0.0 { ops::offset(&closed, layer.inlay_gap) } else { region };
            let wall_step = step.max(0.05);
            match crate::clear::vcarve_rings(&region, tool, full, layer.vc_start, wall_step, tool.dims.tip.max(0.1).max(step * 0.5), outside) {
                None => warnings.push(Warn::new("{}: a V-carve needs a V-bit or an engraving bit", &[&layer.name])),
                Some(rings) => {
                    let floor = rings.iter().filter(|(z, _)| (*z - full).abs() < 1e-9).count();
                    if floor > 2 && !outside {
                        warnings.push(Warn::new("{}: the flat bottom is cut with the V-bit: it leaves ridges and takes long (a flat bit in a Pocket layer is better for it)", &[&layer.name]));
                    }
                    // All the rings are turned together, so that the holes of the shapes get the other way round.
                    let turned = passes::oriented(rings.iter().map(|(_, r)| r.clone()).collect(), Operation::Pocket, layer.direction);
                    for (i, (ring, (z, _))) in turned.iter().zip(&rings).enumerate() {
                        passes::ring_passes(b, ring, &[*z], &plan, false, &[], i > 0, &mut notes);
                        if b.moves.len() > MAX_MOVES {
                            warnings.push(Warn::new("{}: the program of the V-carve is too long: the rest was left out", &[&layer.name]));
                            break;
                        }
                    }
                }
            }
        }
        Operation::Drill => {
            let rings = closed_only(outlines);
            let holes: Vec<Pt> = rings.iter().enumerate().filter(|(i, _)| !is_hole(&rings, *i)).filter_map(|(_, p)| centre(p)).collect();
            for c in holes {
                // Peck: down in steps, out to the surface between them.
                b.dive(c.x, c.y, -depths[0]);
                for d in depths.iter().skip(1) {
                    b.retract();
                    b.dive(c.x, c.y, -d);
                }
            }
        }
    }
    if notes.helix_fallback {
        warnings.push(Warn::new("{}: a helix did not fit somewhere: the ramp along the contour was used there", &[&layer.name]));
    }
    if notes.tabs_fewer {
        warnings.push(Warn::new("{}: some rings are too short for all their tabs: they have fewer", &[&layer.name]));
    }
    if layer.entry != crate::layers::Entry::Plunge && layer.ramp_angle > 45.0 {
        warnings.push(Warn::new("{}: the ramp angle of {}° is steep (more than 45°)", &[&layer.name, &layer.ramp_angle]));
    }
    if layer.tabs.enabled && layer.tabs.height >= layer.depth - 1e-9 && layer.tabs.kind != crate::layers::TabKind::Full {
        warnings.push(Warn::new("{}: the tabs are as high as the cut: the part is not cut free", &[&layer.name]));
    }
}

fn bounds(moves: &[Move]) -> Option<(P3, P3)> {
    let mut it = moves.iter().filter(|m| m.kind != MoveKind::Rapid).flat_map(|m| [m.from, m.to]);
    let first = it.next()?;
    let (mut lo, mut hi) = (first, first);
    for p in it {
        for i in 0..3 {
            lo[i] = lo[i].min(p[i]);
            hi[i] = hi[i].max(p[i]);
        }
    }
    Some((lo, hi))
}

/// G-code that takes the tool once round the box `b` (min x, min y, max x, max y, in the coordinates of the work area) at
/// the safe height, with the spindle off.
pub fn frame_gcode(b: [f64; 4], dev: &Device) -> String {
    let feed = dev.rapid.clamp(100.0, 6000.0);
    let corner = |x: f64, y: f64| machine_xy(dev, x, y);
    let (x0, y0) = corner(b[0], b[1]);
    let (x1, y1) = corner(b[2], b[3]);
    let mut g = String::from("G21 G90\n");
    if let Some(w) = dev.wcs.word() {
        g.push_str(&w);
        g.push_str("\n");
    }
    g.push_str("M5\n");
    g.push_str(&format!("G0 Z{}\nG0 X{} Y{}\n", num(dev.safe_z + dev.z_offset), num(x0), num(y0)));
    for (x, y) in [(x1, y0), (x1, y1), (x0, y1), (x0, y0)] {
        g.push_str(&format!("G1 X{} Y{} F{}\n", num(x), num(y), num(feed)));
    }
    g
}

/// The convex hull of the points (counter-clockwise, no point twice).
pub fn convex_hull(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut p: Vec<(f64, f64)> = points.iter().copied().filter(|q| q.0.is_finite() && q.1.is_finite()).collect();
    p.sort_by(|a, b| a.partial_cmp(b).unwrap());
    p.dedup();
    if p.len() < 3 {
        return p;
    }
    let cross = |o: (f64, f64), a: (f64, f64), b: (f64, f64)| (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0);
    let mut hull: Vec<(f64, f64)> = vec![];
    for pass in 0..2 {
        let start = hull.len();
        let it: Box<dyn Iterator<Item = &(f64, f64)>> = if pass == 0 { Box::new(p.iter()) } else { Box::new(p.iter().rev()) };
        for &q in it {
            while hull.len() >= start + 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], q) <= 0.0 {
                hull.pop();
            }
            hull.push(q);
        }
        hull.pop();
    }
    hull
}

/// G-code that takes the tool once round the outline of the cuts (a rubber band round them, the convex hull) at the safe height,
/// with the spindle off. The points are in the coordinates of the work area.
pub fn frame_hull_gcode(points: &[(f64, f64)], dev: &Device) -> String {
    let hull = convex_hull(points);
    let feed = dev.rapid.clamp(100.0, 6000.0);
    let Some(first) = hull.first() else { return String::new() };
    let (x0, y0) = machine_xy(dev, first.0, first.1);
    let mut g = String::from("G21 G90\n");
    if let Some(w) = dev.wcs.word() {
        g.push_str(&w);
        g.push('\n');
    }
    g.push_str("M5\n");
    g.push_str(&format!("G0 Z{}\nG0 X{} Y{}\n", num(dev.safe_z + dev.z_offset), num(x0), num(y0)));
    for q in hull.iter().skip(1).chain(std::iter::once(first)) {
        let (x, y) = machine_xy(dev, q.0, q.1);
        g.push_str(&format!("G1 X{} Y{} F{}\n", num(x), num(y), num(feed)));
    }
    g
}

/// The machine zero of the device: the coordinates of a point of the work area as the machine counts them.
pub fn machine_xy(dev: &Device, x: f64, y: f64) -> (f64, f64) {
    let (x, y) = match dev.origin {
        Origin::FrontLeft => (x, y),
        Origin::FrontRight => (x - dev.area.x, y),
        Origin::BackLeft => (x, y - dev.area.y),
        Origin::BackRight => (x - dev.area.x, y - dev.area.y),
    };
    // The zero of the program (a work coordinate system with its own zero) moves everything the other way.
    (x - dev.wcs.x, y - dev.wcs.y)
}

/// The lines of a block of own G-code, each with a newline.
/// The lines of an own G-code text with the variables put in (an unknown one gives nothing: the device is checked before a file
/// is made, see `CustomGcode::check_variables`).
fn custom_lines(text: &str, value: &dyn Fn(&str) -> Option<String>) -> Vec<String> {
    crate::device::CustomGcode::lines(text).into_iter().map(|l| base_core::gcode_out::substitute(l, value)).collect()
}

fn custom(g: &mut String, text: &str, value: &dyn Fn(&str) -> Option<String>) {
    for l in custom_lines(text, value) {
        g.push_str(&l);
        g.push('\n');
    }
}

fn num(v: f64) -> String {
    let s = format!("{v:.3}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" || s.is_empty() {
        "0".to_string()
    } else {
        s.to_string()
    }
}

/// G-code (GRBL flavour) for the program. The bit is changed with a pause (M0) when the layer asks for another one.
pub fn to_gcode(prog: &Program, dev: &Device, layers: &[MillLayer], tools: &Library, program_name: &str) -> String {
    let info = base_core::gcode_out::Info { project_name: program_name.to_string(), ..base_core::gcode_out::Info::now() };
    to_gcode_info(prog, dev, layers, tools, &info)
}

/// The place in the program that the variables of the own G-code tell about.
struct Here {
    layer_name: String,
    layer_index: usize,
    total_layers: usize,
}

/// The value of a variable of the own G-code.
fn value_of(info: &base_core::gcode_out::Info, here: &Here, name: &str) -> Option<String> {
    match name {
        "layer_name" => Some(here.layer_name.clone()),
        "layer_index" => Some(here.layer_index.to_string()),
        "total_layers" => Some(here.total_layers.to_string()),
        _ => info.value(name),
    }
}

/// The program as G-code text. `info` tells the variables of the own G-code (the name and the path of the project, the moment);
/// the box of the paths, the machine and the layer are filled in here. The very start and end of the file and the flavour of
/// the device are applied at the end.
pub fn to_gcode_info(prog: &Program, dev: &Device, layers: &[MillLayer], tools: &Library, info: &base_core::gcode_out::Info) -> String {
    let program_name = info.project_name.as_str();
    let mut info = info.clone();
    info.machine_name = dev.name.clone();
    // The box of everything that is cut, in the coordinates of the machine.
    {
        let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for m in prog.moves.iter().filter(|m| m.kind != MoveKind::Rapid) {
            let (x, y) = machine_xy(dev, m.to[0], m.to[1]);
            for (i, v) in [x, y, m.to[2] + dev.z_offset].into_iter().enumerate() {
                lo[i] = lo[i].min(v);
                hi[i] = hi[i].max(v);
            }
        }
        info.bounds = lo[0].is_finite().then_some((lo, hi));
    }
    let mut here = Here { layer_name: String::new(), layer_index: 0, total_layers: layers.len() };
    let mut g = String::new();
    let clean = |s: &str| s.chars().filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_' || *c == '.').collect::<String>();
    g.push_str(&format!("(MillCreator: {})\n(Device: {})\n", clean(program_name), clean(&dev.name)));
    g.push_str("G21 G90 G17\n");
    if let Some(w) = dev.wcs.word() {
        g.push_str(&w);
        g.push_str("\n");
    }
    custom(&mut g, &dev.gcode.start, &|n| value_of(&info, &here, n));
    let mut feed = f64::NAN;
    let mut layer = usize::MAX;
    let mut tool_id: Option<String> = None;
    let mut spindle_on = false;
    let mut coolant = crate::layers::Coolant::Off;
    let mut dust = false;
    let xyz = |p: P3| {
        let (x, y) = machine_xy(dev, p[0], p[1]);
        (x, y, p[2] + dev.z_offset)
    };
    for m in &prog.moves {
        if m.layer != layer {
            layer = m.layer;
            if let Some(l) = layers.get(layer) {
                g.push_str(&format!("(Layer: {})\n", clean(&l.name)));
                here = Here { layer_name: l.name.clone(), layer_index: layer, total_layers: layers.len() };
                custom(&mut g, &dev.gcode.operation_change, &|n| value_of(&info, &here, n));
                if tool_id != l.tool {
                    if spindle_on {
                        g.push_str(&format!("G0 Z{}\nM5\n", num(dev.safe_z + dev.z_offset)));
                    }
                    // The coolant and the dust collector stop while the bit is changed.
                    if coolant != crate::layers::Coolant::Off {
                        g.push_str("M9\n");
                        coolant = crate::layers::Coolant::Off;
                    }
                    if dust {
                        custom(&mut g, &dev.gcode.dust_off, &|n| value_of(&info, &here, n));
                        dust = false;
                    }
                    let tool = l.tool.as_deref().and_then(|id| tools.get(id));
                    let name = tool.map(|t| clean(&t.name)).unwrap_or_default();
                    let number = tool.map_or(0, |t| t.number);
                    if dev.changer && number > 0 {
                        // A machine with a tool changer takes the bit by itself: no pause.
                        g.push_str(&format!("(Bit: {name})\nT{number} M6\n"));
                    } else if tool_id.is_some() {
                        g.push_str(&format!("M0 (Change the bit to: {name})\n"));
                    } else {
                        g.push_str(&format!("(Bit: {name})\n"));
                    }
                    tool_id = l.tool.clone();
                    spindle_on = false;
                    custom(&mut g, &dev.gcode.tool_change, &|n| value_of(&info, &here, n));
                }
                let rpm = l.rpm.clamp(dev.spindle.min_rpm, dev.spindle.max_rpm);
                if rpm > 0.0 {
                    // `M3 S…` is the way every sender writes it, and what the controllers are tried with.
                    g.push_str(&format!("M3 S{}\n", num(rpm)));
                    if dev.spindle.spin_up > 0.0 && !spindle_on {
                        g.push_str(&format!("G4 P{}\n", num(dev.spindle.spin_up)));
                    }
                    spindle_on = true;
                }
                // The coolant and the dust collector of the layer.
                if l.coolant != coolant {
                    if coolant != crate::layers::Coolant::Off {
                        g.push_str("M9\n");
                    }
                    if let Some(w) = l.coolant.word() {
                        g.push_str(w);
                        g.push('\n');
                    }
                    coolant = l.coolant;
                }
                if l.dust != dust {
                    custom(&mut g, if l.dust { &dev.gcode.dust_on } else { &dev.gcode.dust_off }, &|n| value_of(&info, &here, n));
                    dust = l.dust;
                }
            }
        }
        let (x, y, z) = xyz(m.to);
        match m.kind {
            MoveKind::Rapid => g.push_str(&format!("G0 X{} Y{} Z{}\n", num(x), num(y), num(z))),
            MoveKind::Plunge | MoveKind::Cut => {
                let f = if feed == m.feed { String::new() } else { format!(" F{}", num(m.feed)) };
                feed = m.feed;
                if m.kind == MoveKind::Plunge && m.from[0] == m.to[0] && m.from[1] == m.to[1] {
                    g.push_str(&format!("G1 Z{}{f}\n", num(z)));
                } else {
                    g.push_str(&format!("G1 X{} Y{} Z{}{f}\n", num(x), num(y), num(z)));
                }
            }
        }
    }
    g.push_str(&format!("G0 Z{}\nM5\n", num(dev.safe_z + dev.z_offset)));
    if coolant != crate::layers::Coolant::Off {
        g.push_str("M9\n");
    }
    if dust {
        custom(&mut g, &dev.gcode.dust_off, &|n| value_of(&info, &here, n));
    }
    let (hx, hy) = machine_xy(dev, 0.0, 0.0);
    g.push_str(&format!("G0 X{} Y{}\n", num(hx), num(hy)));
    custom(&mut g, &dev.gcode.end, &|n| value_of(&info, &here, n));
    g.push_str("M30\n");
    let lines = |text: &str| custom_lines(text, &|n| value_of(&info, &here, n));
    base_core::gcode_out::finish(&g, &lines(&dev.gcode.very_start), &lines(&dev.gcode.very_end), &dev.gcode.flavour)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layers::default_layers;
    use base_core::geom::Xf;

    fn setup() -> (Library, Device, Stock, Vec<MillLayer>) {
        let lib = Library::new();
        let dev = Device::dummy();
        let layers = default_layers(&lib);
        (lib, dev, Stock::default(), layers)
    }

    /// A square of `s` mm with the corner at (x, y) on the layer, in scene coordinates.
    fn square(scene: &mut Scene, layer: usize, x: f64, y: f64, s: f64) {
        scene.add(layer, Kind::Rect { w: s, h: s }, Xf::translate(x, y));
    }

    fn xy_bounds(prog: &Program) -> (f64, f64, f64, f64) {
        let cuts = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut);
        let pts: Vec<P3> = cuts.flat_map(|m| [m.from, m.to]).collect();
        let f = |i: usize, min: bool| pts.iter().map(|p| p[i]).fold(if min { f64::INFINITY } else { f64::NEG_INFINITY }, |a, b| if min { a.min(b) } else { a.max(b) });
        (f(0, true), f(1, true), f(0, false), f(1, false))
    }

    #[test]
    fn layers_are_milled_in_the_order_they_were_moved_to() {
        let (lib, dev, stock, mut layers) = setup();
        let mut scene = Scene::default();
        square(&mut scene, 0, 10.0, 10.0, 40.0);
        square(&mut scene, 2, 100.0, 10.0, 40.0);
        let first_layer = |prog: &Program| prog.moves.iter().find(|m| m.kind != MoveKind::Rapid).map(|m| m.layer);
        assert_eq!(first_layer(&generate(&scene, &layers, &lib, &dev, &stock)), Some(0));
        crate::layers::move_layer(&mut layers, 2, true);
        crate::layers::move_layer(&mut layers, 2, true);
        assert_eq!(first_layer(&generate(&scene, &layers, &lib, &dev, &stock)), Some(2), "the pocket goes first now");
    }

    fn cuts_at(prog: &Program, z: f64) -> Vec<&Move> {
        prog.moves.iter().filter(|m| m.kind == MoveKind::Cut && (m.to[2] + z).abs() < 1e-9).collect()
    }

    #[test]
    fn the_first_pass_goes_along_the_edge_only_and_before_the_deeper_passes() {
        let (lib, dev, mut stock, mut layers) = setup();
        let mut scene = Scene::default();
        square(&mut scene, 2, 20.0, 20.0, 40.0);
        layers[2].depth = 3.0;
        // Without the first pass nothing is cut at 0.1 mm.
        let plain = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(cuts_at(&plain, 0.1).is_empty());
        stock.first_pass = true;
        stock.first_pass_depth = 0.1;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let shallow = cuts_at(&prog, 0.1);
        assert!(!shallow.is_empty(), "a first pass is made");
        // It is the first thing cut.
        assert_eq!(prog.moves.iter().find(|m| m.kind == MoveKind::Cut).map(|m| m.to[2]), Some(-0.1));
        // A pocket gets it along the edge only: one ring, the outermost (the same outline as the outer ring of the pocket).
        let deeper: Vec<&Move> = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut && m.to[2] < -0.1 - 1e-9).collect();
        assert!(shallow.len() < deeper.len() / 2, "{} shallow, {} deeper", shallow.len(), deeper.len());
        let xs = |v: &[&Move]| (v.iter().map(|m| m.to[0]).fold(f64::INFINITY, f64::min), v.iter().map(|m| m.to[0]).fold(f64::NEG_INFINITY, f64::max));
        assert_eq!(xs(&shallow), xs(&deeper), "as wide as the whole pocket");
        // Nothing changes for the cuts of the layer itself.
        assert_eq!(deeper.len(), plain.moves.iter().filter(|m| m.kind == MoveKind::Cut).count());
    }

    #[test]
    fn the_first_pass_follows_a_profile_and_a_line_and_skips_drilling_and_a_layer_that_is_not_deeper() {
        let (lib, dev, mut stock, mut layers) = setup();
        stock.first_pass = true;
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 40.0);
        layers[0].depth = 4.0;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let ring = cuts_at(&prog, 0.1).len();
        let full = cuts_at(&prog, layers[0].passes()[0]).len();
        assert!(ring > 0 && ring == full, "the same ring as a deeper pass: {ring} and {full}");
        // A layer that is not deeper than the first pass gets no extra pass.
        layers[0].depth = 0.1;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let once = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut).count();
        stock.first_pass = false;
        assert_eq!(once, generate(&scene, &layers, &lib, &dev, &stock).moves.iter().filter(|m| m.kind == MoveKind::Cut).count());
        // Drilling has no first pass.
        stock.first_pass = true;
        layers[0].op = Operation::Drill;
        layers[0].depth = 4.0;
        let with = generate(&scene, &layers, &lib, &dev, &stock);
        stock.first_pass = false;
        assert_eq!(with.moves.len(), generate(&scene, &layers, &lib, &dev, &stock).moves.len());
    }

    #[test]
    fn an_old_project_without_the_first_pass_reads_with_it_off() {
        let old = r#"{"kind":"WoodLight","color":[1,2,3],"thickness":12.0}"#;
        let s: Stock = serde_json::from_str(old).unwrap();
        assert!(!s.first_pass);
        assert_eq!(s.first_pass_depth, 0.1);
        let mut bad = s.clone();
        bad.first_pass_depth = -3.0;
        assert_eq!(bad.sane().first_pass_depth, 0.1);
    }

    #[test]
    fn the_frame_goes_round_the_box_at_the_safe_height_and_a_rotary_limits_the_height() {
        let (lib, mut dev, stock, layers) = setup();
        let g = frame_gcode([10.0, 20.0, 60.0, 50.0], &dev);
        assert!(g.contains("G0 Z5") && g.contains("M5") && g.contains("G1 X60 Y20") && g.contains("G1 X60 Y50"), "{g}");
        assert!(g.trim_end().ends_with("G1 X10 Y20 F3000"), "{g}");
        dev.origin = Origin::BackLeft;
        assert!(frame_gcode([10.0, 20.0, 60.0, 50.0], &dev).contains("Y-180"), "machine zero moves it");
        // A rotary with a small workpiece: the drawing is higher than the way round it.
        let mut scene = Scene::default();
        square(&mut scene, 0, 10.0, 10.0, 40.0);
        let mut layers = layers;
        layers[0].depth = 2.0;
        dev.origin = Origin::FrontLeft;
        dev.rotary.enabled = true;
        dev.rotary.diameter = 10.0;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.iter().any(|w| w.to_string().contains("way round")), "{:?}", prog.warnings);
        dev.rotary.diameter = 100.0;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(!prog.warnings.iter().any(|w| w.to_string().contains("way round")));
    }

    #[test]
    fn an_outside_profile_goes_round_the_outside_by_the_radius_of_the_bit() {
        let (lib, dev, stock, mut layers) = setup();
        layers[0].depth = 6.0;
        layers[0].step_down = 3.0;
        let mut scene = Scene::default();
        square(&mut scene, 0, 50.0, 50.0, 40.0);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let r = lib.get("std-flat-3").unwrap().radius();
        let (x0, y0, x1, y1) = xy_bounds(&prog);
        assert!((x0 - (50.0 - r)).abs() < 0.01 && (x1 - (90.0 + r)).abs() < 0.01, "{x0} {x1}");
        // The scene counts Y down: the square from 50 to 90 is at 200-90 .. 200-50 in the work area.
        assert!((y0 - (110.0 - r)).abs() < 0.01 && (y1 - (150.0 + r)).abs() < 0.01, "{y0} {y1}");
        let (lo, hi) = prog.bounds.unwrap();
        assert!((lo[2] + 6.0).abs() < 1e-9 && hi[2] <= dev.safe_z, "the deepest pass at -6: {lo:?} {hi:?}");
        let levels: std::collections::BTreeSet<i64> = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut).map(|m| (m.to[2] * 1000.0).round() as i64).collect();
        assert_eq!(levels.into_iter().collect::<Vec<_>>(), vec![-6000, -3000], "two passes");
        assert!(prog.warnings.is_empty(), "{:?}", prog.warnings);
    }

    #[test]
    fn a_pass_depth_of_zero_cuts_the_full_depth_in_one_pass_and_a_short_bit_gives_a_warning() {
        let (lib, dev, stock, mut layers) = setup();
        layers[0].depth = 6.0;
        layers[0].step_down = 0.0;
        let mut scene = Scene::default();
        square(&mut scene, 0, 50.0, 50.0, 40.0);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let levels: std::collections::BTreeSet<i64> = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut).map(|m| (m.to[2] * 1000.0).round() as i64).collect();
        assert_eq!(levels.into_iter().collect::<Vec<_>>(), vec![-6000], "one pass");
        let cut = lib.get("std-flat-3").unwrap().dims.cut_length;
        assert!(cut >= 6.0, "the bit of the test cuts 6 mm deep: {cut}");
        assert!(prog.warnings.is_empty(), "{:?}", prog.warnings);
        // A bit whose cutting part is shorter than the pass: a warning, the file is made all the same.
        layers[0].depth = cut + 4.0;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.iter().any(|w| w.text().contains("deeper than the cutting part")), "{:?}", prog.warnings.iter().map(|w| w.text()).collect::<Vec<_>>());
        assert!(!prog.moves.is_empty());
        // The same depth in passes the bit can take: no warning.
        layers[0].step_down = cut;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(!prog.warnings.iter().any(|w| w.text().contains("deeper than the cutting part")));
    }

    #[test]
    fn the_file_has_a_very_start_and_end_variables_and_a_flavour_and_the_default_changes_nothing() {
        use base_core::gcode_out::{DwellWord, Flavour};
        let (lib, mut dev, stock, mut layers) = setup();
        layers[0].depth = 2.0;
        layers[0].step_down = 2.0;
        let mut scene = Scene::default();
        square(&mut scene, 0, 50.0, 50.0, 40.0);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let plain = to_gcode(&prog, &dev, &layers, &lib, "job");
        // The default device: the file is what it always was (the header, the start, the end, M30).
        assert!(plain.starts_with("(MillCreator: job)\n(Device: ") && plain.trim_end().ends_with("M30") && !plain.contains('%'));
        // The very start and the very end, with variables.
        dev.gcode.very_start = "%\nO{project_name}".into();
        dev.gcode.very_end = "(done {machine_name})".into();
        dev.gcode.start = "(box {min_x} {max_x} {width}x{height}x{depth})".into();
        dev.gcode.operation_change = "(layer {layer_index}/{total_layers} {layer_name})".into();
        let info = base_core::gcode_out::Info { project_name: "part".into(), project_path: "/w/part.cre".into(), date: "2026-11-01".into(), time: "12:00".into(), ..Default::default() };
        let text = to_gcode_info(&prog, &dev, &layers, &lib, &info);
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(&lines[..3], &["%", "Opart", "(MillCreator: part)"]);
        assert!(lines.iter().any(|l| l.starts_with("(box ") && !l.contains('{')), "{text}");
        assert!(lines.iter().any(|l| l.starts_with("(layer 0/") && !l.contains('{')), "{text}");
        assert_eq!(lines.last().copied(), Some(format!("(done {})", dev.name).as_str()));
        assert_eq!(lines[lines.len() - 2], "M30", "the very end is after M30");
        // The box is the one of the cuts: the square 40 mm wide, with the radius of the bit.
        let r = lib.get("std-flat-3").unwrap().radius();
        let w: f64 = lines.iter().find(|l| l.starts_with("(box ")).unwrap().split(['(', ' ', 'x']).filter_map(|p| p.parse::<f64>().ok()).nth(2).unwrap();
        assert!((w - (40.0 + 2.0 * r)).abs() < 0.01, "{w}");
        // The variables are checked before a file is made.
        assert!(dev.gcode.check_variables().is_ok());
        dev.gcode.end = "(oops {tool_number})".into();
        assert!(dev.gcode.check_variables().unwrap_err().contains("At the end of the program"));
        dev.gcode.end.clear();
        // The flavour: a point after whole numbers, the dwell in milliseconds, line numbers, % lines.
        dev.gcode = Default::default();
        dev.spindle.spin_up = 2.0;
        dev.gcode.flavour = Flavour { decimal_point: true, percent: true, dwell: DwellWord::PMilliseconds, line_numbers: true, number_step: 10, remove_comments: true };
        let g = to_gcode(&prog, &dev, &layers, &lib, "job");
        let l: Vec<&str> = g.lines().collect();
        assert_eq!((l[0], l[l.len() - 1]), ("%", "%"));
        assert!(l[1].starts_with("N10 G21 G90 G17"), "{}", l[1]);
        assert!(l.iter().any(|x| x.contains("G4 P2000.")), "dwell in milliseconds with a point: {g}");
        assert!(l.iter().any(|x| x.contains(" X") && x.contains("Y") && !x.contains("(")) && !g.contains("(MillCreator"));
        assert!(!g.contains("M3 S12000.") && g.contains("M3 S"), "the spindle speed gets no point");
    }

    #[test]
    fn an_inside_profile_stays_inside_by_the_radius_and_a_small_one_gives_nothing() {
        let (lib, dev, stock, layers) = setup();
        let r = lib.get("std-flat-3").unwrap().radius();
        let mut scene = Scene::default();
        square(&mut scene, 1, 50.0, 50.0, 40.0);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let (x0, _, x1, _) = xy_bounds(&prog);
        assert!((x0 - (50.0 + r)).abs() < 0.01 && (x1 - (90.0 - r)).abs() < 0.01);
        // A square smaller than the bit cannot be cut from inside.
        let mut tiny = Scene::default();
        square(&mut tiny, 1, 10.0, 10.0, 2.0);
        let prog = generate(&tiny, &layers, &lib, &dev, &stock);
        assert!(prog.is_empty());
        assert!(prog.warnings.iter().any(|w| w.to_string().contains("nothing to cut")), "{:?}", prog.warnings);
    }

    #[test]
    fn a_pocket_is_cleared_in_rings_that_stay_inside_the_line() {
        let (lib, dev, stock, layers) = setup();
        let t = lib.get("std-flat-3").unwrap();
        let mut scene = Scene::default();
        square(&mut scene, 2, 20.0, 20.0, 30.0);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let (x0, _, x1, _) = xy_bounds(&prog);
        assert!(x0 >= 20.0 + t.radius() - 0.01 && x1 <= 50.0 - t.radius() + 0.01, "{x0} {x1}");
        // More than one ring: the cuts at the first depth reach in towards the middle.
        let step = t.dims.diameter * layers[2].step_over_pct / 100.0;
        let near_middle = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut).any(|m| (m.to[0] - 35.0).abs() < step * 2.0 + 0.1);
        assert!(near_middle);
        assert!(prog.cut_length() > 4.0 * 20.0, "several rings");
    }

    #[test]
    fn a_line_is_followed_there_and_back_at_the_deeper_passes() {
        let (lib, dev, stock, mut layers) = setup();
        layers[3].depth = 2.0;
        layers[3].step_down = 1.0;
        let mut scene = Scene::default();
        scene.add(3, Kind::Path(vec![Polyline::new(vec![Pt::new(10.0, 10.0), Pt::new(60.0, 10.0)], false)]), Xf::IDENTITY);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let cuts: Vec<&Move> = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut).collect();
        assert_eq!(cuts.len(), 2);
        assert_eq!((cuts[0].from[0], cuts[0].to[0]), (10.0, 60.0));
        assert_eq!((cuts[1].from[0], cuts[1].to[0]), (60.0, 10.0));
        assert_eq!((cuts[0].to[2], cuts[1].to[2]), (-1.0, -2.0));
    }

    #[test]
    fn a_drill_makes_a_hole_at_the_centre_in_pecks() {
        let (lib, dev, stock, mut layers) = setup();
        layers[4].depth = 9.0;
        layers[4].step_down = 3.0;
        let mut scene = Scene::default();
        scene.add(4, Kind::Ellipse { w: 4.0, h: 4.0 }, Xf::translate(30.0, 40.0));
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let plunges: Vec<&Move> = prog.moves.iter().filter(|m| m.kind == MoveKind::Plunge && m.to[2] < 0.0).collect();
        let depths: Vec<f64> = plunges.iter().map(|m| m.to[2]).collect();
        assert_eq!(depths, vec![-3.0, -6.0, -9.0]);
        assert!(plunges.iter().all(|m| (m.to[0] - 32.0).abs() < 0.05 && (m.to[1] - (200.0 - 42.0)).abs() < 0.05));
        // Between the pecks the bit comes out to the safe height.
        assert!(prog.moves.iter().any(|m| m.kind == MoveKind::Plunge && m.to[2] >= dev.safe_z - 1e-9));
    }

    #[test]
    fn the_spindle_moves_between_cuts_at_the_safe_height() {
        let (lib, dev, stock, layers) = setup();
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 20.0);
        square(&mut scene, 0, 100.0, 100.0, 20.0);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        for m in &prog.moves {
            if m.kind == MoveKind::Rapid {
                assert!(m.from[2] >= 1.0 - 1e-9 && m.to[2] >= 1.0 - 1e-9, "a rapid move in the material: {m:?}");
            }
            if m.kind == MoveKind::Cut {
                assert!(m.to[2] < 0.0);
            }
        }
        assert!(prog.moves.iter().any(|m| m.kind == MoveKind::Rapid && (m.to[2] - dev.safe_z).abs() < 1e-9));
        // The last move leaves the spindle at the safe height.
        assert!((prog.moves.last().unwrap().to[2] - dev.safe_z).abs() < 1e-9);
    }

    #[test]
    fn what_cannot_be_done_is_said() {
        let (lib, dev, stock, mut layers) = setup();
        let mut scene = Scene::default();
        square(&mut scene, 0, 20.0, 20.0, 20.0);
        layers[0].tool = None;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(prog.is_empty() && prog.warnings.iter().any(|w| w.to_string().contains("no bit")));
        layers[0].tool = Some("std-flat-3".into());
        layers[0].depth = 25.0;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.iter().any(|w| w.to_string().contains("more than the thickness")), "{:?}", prog.warnings);
        // Outside the work area.
        let mut far = Scene::default();
        square(&mut far, 0, 280.0, 20.0, 40.0);
        layers[0].depth = 5.0;
        let prog = generate(&far, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.iter().any(|w| w.to_string().contains("outside of the work area")), "{:?}", prog.warnings);
        // A switched-off layer gives nothing.
        layers[0].enabled = false;
        assert!(generate(&scene, &layers, &lib, &dev, &stock).is_empty());
    }

    #[test]
    fn the_gcode_is_in_order_and_uses_the_machine_zero() {
        let (lib, mut dev, stock, mut layers) = setup();
        layers[0].depth = 2.0;
        layers[0].step_down = 2.0;
        let mut scene = Scene::default();
        square(&mut scene, 0, 50.0, 50.0, 40.0);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let g = to_gcode(&prog, &dev, &layers, &lib, "job");
        assert!(g.starts_with("(MillCreator: job)\n(Device: Dummy)\nG21 G90 G17\n"), "{g}");
        assert!(g.contains("M3 S16000\nG4 P2\n"), "{g}");
        assert!(g.contains(" F900") && g.contains("G1 Z-2 F300"), "{g}");
        assert!(g.trim_end().ends_with("M5\nG0 X0 Y0\nM30"), "{g}");
        let first_cut = g.lines().find(|l| l.starts_with("G1 X")).unwrap();
        assert!(first_cut.contains("X"), "{first_cut}");
        // With the zero at the back right corner the numbers are negative.
        dev.origin = Origin::BackRight;
        let g = to_gcode(&prog, &dev, &layers, &lib, "job");
        let x: f64 = g.lines().find(|l| l.starts_with("G1 X")).unwrap().split('X').nth(1).unwrap().split(' ').next().unwrap().parse().unwrap();
        assert!(x < 0.0, "{x}");
        assert!(g.trim_end().ends_with("G0 X-300 Y-200\nM30"), "{g}");
    }

    #[test]
    fn own_gcode_and_the_work_coordinate_system_are_written_where_they_belong() {
        let (lib, mut dev, stock, mut layers) = setup();
        layers[0].depth = 1.0;
        layers[0].step_down = 1.0;
        let mut scene = Scene::default();
        square(&mut scene, 0, 50.0, 50.0, 40.0);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let plain = to_gcode(&prog, &dev, &layers, &lib, "job");
        assert!(!plain.contains("G54") && !plain.contains("M8"), "nothing is added by default: {plain}");
        dev.gcode = crate::device::CustomGcode { start: "M8\n\n  G4 P1 ".into(), end: "M9".into(), tool_change: "T1 M6".into(), operation_change: "(next)".into(), ..Default::default() };
        dev.wcs = crate::device::Wcs { code: 55, x: 10.0, y: 20.0 };
        let g = to_gcode(&prog, &dev, &layers, &lib, "job");
        assert!(g.contains("G21 G90 G17\nG55\nM8\nG4 P1\n"), "the word, then the own start lines: {g}");
        let layer = g.find("(Layer:").unwrap();
        assert!(g[layer..].contains("(next)\n") && g.find("(next)").unwrap() < g.find("T1 M6").unwrap() && g.find("T1 M6").unwrap() < g.find("M3 S").unwrap(), "operation, bit, then the spindle: {g}");
        assert!(g.trim_end().ends_with("G0 X-10 Y-20\nM9\nM30"), "the end lines come before M30, the head returns to the zero of the program: {g}");
        // The zero of the program moves every number, in the frame as well.
        let x = |text: &str| -> f64 { text.lines().find(|l| l.starts_with("G1 X")).unwrap().split('X').nth(1).unwrap().split(' ').next().unwrap().parse().unwrap() };
        assert!((x(&plain) - x(&g) - 10.0).abs() < 1e-9);
        let f = frame_gcode([0.0, 0.0, 10.0, 10.0], &dev);
        assert!(f.starts_with("G21 G90\nG55\nM5\n") && f.contains("G0 X-10 Y-20"), "{f}");
        // Checked: too long, too many lines, a control character, a wrong word.
        dev.gcode.start = "x".repeat(101);
        assert!(dev.check().is_err());
        dev.gcode.start = (0..21).map(|i| format!("G4 P{i}")).collect::<Vec<_>>().join("\n");
        assert!(dev.check().is_err());
        dev.gcode.start = "M8\u{7}".into();
        assert!(dev.check().is_err());
        dev.gcode.start.clear();
        dev.wcs.code = 53;
        assert!(dev.check().is_err());
    }

    use crate::layers::{CutDir, Entry, TabKind};

    /// The path the centre of the bit takes at the deepest pass of layer 0, as (x, y, z) of the cut moves.
    fn lowest_cuts(prog: &Program, depth: f64) -> Vec<P3> {
        prog.moves.iter().filter(|m| m.kind == MoveKind::Cut && (m.to[2] + depth).abs() < 1e-9).map(|m| m.to).collect()
    }

    /// Twice the signed area of the polyline of the points (positive: counter-clockwise).
    fn turn(pts: &[P3]) -> f64 {
        (0..pts.len()).map(|i| (pts[i][0] * pts[(i + 1) % pts.len()][1]) - (pts[(i + 1) % pts.len()][0] * pts[i][1])).sum()
    }

    fn square_job(layers: &mut [MillLayer], depth: f64, step: f64, size: f64) -> Scene {
        layers[0].depth = depth;
        layers[0].step_down = step;
        let mut scene = Scene::default();
        square(&mut scene, 0, 50.0, 50.0, size);
        scene
    }

    #[test]
    fn climb_and_conventional_turn_a_profile_the_other_way_and_a_hole_against_the_part() {
        let (lib, dev, stock, mut layers) = setup();
        let scene = square_job(&mut layers, 2.0, 2.0, 40.0);
        let mut ccw = |dir: CutDir, layers: &mut [MillLayer]| {
            layers[0].direction = dir;
            let prog = generate(&scene, layers, &lib, &dev, &stock);
            turn(&lowest_cuts(&prog, 2.0)) > 0.0
        };
        // A part cut out, climb, clockwise spindle: the bit goes counter-clockwise round it.
        assert!(ccw(CutDir::Climb, &mut layers));
        assert!(!ccw(CutDir::Conventional, &mut layers));
        // The same square as a pocket (the material is outside): the other way round.
        layers[0].op = Operation::Pocket;
        layers[0].step_over_pct = 100.0;
        let prog = {
            layers[0].direction = CutDir::Climb;
            generate(&scene, &layers, &lib, &dev, &stock)
        };
        let first_ring: Vec<P3> = lowest_cuts(&prog, 2.0).into_iter().take(5).collect();
        assert!(turn(&first_ring[..4]) < 0.0, "climbing a pocket goes clockwise");
        // As drawn is what it always was.
        layers[0].op = Operation::ProfileOutside;
        layers[0].direction = CutDir::AsDrawn;
        let plain = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(plain.moves.len() > 4);
    }

    #[test]
    fn stock_to_leave_is_taken_off_in_a_finishing_pass_at_the_full_depth() {
        let (lib, dev, stock, mut layers) = setup();
        let scene = square_job(&mut layers, 4.0, 2.0, 40.0);
        let r = lib.get(layers[0].tool.as_deref().unwrap()).unwrap().radius();
        let rough = generate(&scene, &layers, &lib, &dev, &stock);
        let (x0, ..) = xy_bounds(&rough);
        layers[0].leave = 0.5;
        let with = generate(&scene, &layers, &lib, &dev, &stock);
        let cuts: Vec<&Move> = with.moves.iter().filter(|m| m.kind == MoveKind::Cut).collect();
        // The passes down to the depth stay 0.5 mm further from the part ...
        let rough_x = cuts.iter().filter(|m| m.to[2] > -4.0 + 1e-9).map(|m| m.to[0]).fold(f64::INFINITY, f64::min);
        assert!((rough_x - (x0 - 0.5)).abs() < 0.05, "{rough_x} vs {x0}");
        // ... and the last pass is at the full depth on the real line.
        let last: Vec<&&Move> = cuts.iter().rev().take(5).collect();
        assert!(last.iter().all(|m| (m.to[2] + 4.0).abs() < 1e-9));
        let fin_x = cuts.iter().rev().take(5).map(|m| m.to[0]).fold(f64::INFINITY, f64::min);
        assert!((fin_x - (50.0 - r)).abs() < 0.05, "{fin_x}");
        // Without the finishing pass there is none.
        layers[0].finish = false;
        let without = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(without.moves.len() < with.moves.len());
    }

    #[test]
    fn a_ramp_goes_down_at_its_angle_and_ends_at_the_full_depth_without_cutting_deeper() {
        let (lib, dev, stock, mut layers) = setup();
        let scene = square_job(&mut layers, 3.0, 1.5, 40.0);
        for entry in [Entry::RampContour, Entry::RampZigzag, Entry::Helix] {
            layers[0].entry = entry;
            layers[0].ramp_angle = 5.0;
            layers[0].ramp_feed = 123.0;
            let prog = generate(&scene, &layers, &lib, &dev, &stock);
            let cuts: Vec<&Move> = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut).collect();
            assert!(cuts.iter().all(|m| m.to[2] >= -3.0 - 1e-9), "{entry:?}: never deeper than the layer");
            // Every sloping cut has the slope that was asked for (the helix has one all the way).
            let sloped: Vec<&&Move> = cuts.iter().filter(|m| (m.to[2] - m.from[2]).abs() > 1e-6).collect();
            assert!(!sloped.is_empty(), "{entry:?}");
            for m in sloped.iter().filter(|m| m.feed == 123.0) {
                let run = ((m.to[0] - m.from[0]).powi(2) + (m.to[1] - m.from[1]).powi(2)).sqrt();
                if run > 1e-6 {
                    let slope = ((m.to[2] - m.from[2]).abs() / run).atan().to_degrees();
                    assert!(slope < 5.0 + 0.6, "{entry:?}: {slope}° is steeper than the ramp");
                }
            }
            assert!(cuts.iter().any(|m| (m.to[2] + 3.0).abs() < 1e-9), "{entry:?}: reaches the depth");
            assert!(prog.moves.iter().filter(|m| m.kind == MoveKind::Plunge && m.to[2] < 0.0).count() == 0, "{entry:?}: no plunge into the material");
        }
    }

    #[test]
    fn a_helix_that_does_not_fit_falls_back_to_the_ramp_with_a_warning() {
        let (lib, dev, stock, mut layers) = setup();
        layers[2].depth = 2.0;
        layers[2].step_down = 2.0;
        layers[2].entry = Entry::Helix;
        layers[2].ramp_angle = 10.0;
        let mut scene = Scene::default();
        // A pocket 1.2 mm wide with a 3 mm bit in it cannot hold a helix; a big pocket can.
        scene.add(2, Kind::Rect { w: 4.0, h: 30.0 }, Xf::translate(10.0, 10.0));
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.iter().any(|w| w.to_string().contains("helix did not fit")), "{:?}", prog.warnings.iter().map(|w| w.to_string()).collect::<Vec<_>>());
        let mut big = Scene::default();
        big.add(2, Kind::Rect { w: 40.0, h: 40.0 }, Xf::translate(10.0, 10.0));
        let prog = generate(&big, &layers, &lib, &dev, &stock);
        assert!(!prog.warnings.iter().any(|w| w.to_string().contains("helix did not fit")));
    }

    #[test]
    fn tabs_hold_the_part_and_only_the_passes_below_their_top_are_raised() {
        let (lib, dev, stock, mut layers) = setup();
        let scene = square_job(&mut layers, 6.0, 2.0, 100.0);
        layers[0].tabs = crate::layers::Tabs { enabled: true, kind: TabKind::Partial, length: 6.0, height: 2.0, spacing: 100.0, min: 2, max: 8, skip_inner: true, ..Default::default() };
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let cuts: Vec<&Move> = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut).collect();
        assert!(cuts.iter().all(|m| m.to[2] >= -6.0 - 1e-9));
        // The pass at 4 mm and above never touch the tabs (their top is at 4 mm); the last pass is raised to it in the tabs.
        let at_tab_top = cuts.iter().filter(|m| (m.to[2] + 4.0).abs() < 1e-9 && m.from[2] < -4.0 + 1e-6).count();
        assert!(at_tab_top >= 2, "the bit rises to the top of the tab at least twice (two tabs on 400 mm): {at_tab_top}");
        let length_at = |prog: &Program| -> f64 { prog.moves.iter().filter(|m| m.kind == MoveKind::Cut && (m.to[2] + 6.0).abs() < 1e-9 && (m.from[2] + 6.0).abs() < 1e-9).map(|m| m.length()).sum() };
        let with_tabs = length_at(&prog);
        let without = {
            layers[0].tabs.enabled = false;
            let p = generate(&scene, &layers, &lib, &dev, &stock);
            length_at(&p)
        };
        // Two tabs, each as wide as 6 mm and the bit: that much of the deepest pass is raised.
        assert!(without - with_tabs > 2.0 * 6.0, "{without} - {with_tabs}");
        // A full tab goes above the material; a ramp tab rises on a slope.
        layers[0].tabs.enabled = true;
        layers[0].tabs.kind = TabKind::Full;
        let full = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(full.moves.iter().any(|m| m.kind == MoveKind::Cut && m.to[2] > 0.0));
        layers[0].tabs.kind = TabKind::Ramp;
        let ramp = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(ramp.moves.iter().any(|m| m.kind == MoveKind::Cut && m.to[2] < m.from[2] - 1e-6 && (m.to[2] - m.from[2]).abs() > 0.3 && m.from[2] > m.to[2]), "slopes down");
        assert!(ramp.moves.iter().all(|m| m.to[2] >= -6.0 - 1e-9));
        // Tabs as high as the cut: a warning.
        layers[0].tabs.kind = TabKind::Partial;
        layers[0].tabs.height = 6.0;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.iter().any(|w| w.to_string().contains("tabs are as high as the cut")));
    }

    fn cut_points(prog: &Program) -> Vec<P3> {
        prog.moves.iter().filter(|m| m.kind == MoveKind::Cut).flat_map(|m| [m.from, m.to]).collect()
    }

    fn with_tool(layers: &mut [MillLayer], li: usize, lib: &Library, id: &str) {
        let t = lib.get(id).unwrap().clone();
        layers[li].use_tool(&t);
    }

    #[test]
    fn a_pocket_by_raster_runs_lines_at_the_angle_inside_the_walls_and_the_outline_pass_comes_where_asked() {
        use crate::layers::{OutlinePass, PocketPattern};
        let (lib, dev, stock, mut layers) = setup();
        layers[2].depth = 2.0;
        layers[2].step_down = 2.0;
        layers[2].pattern = PocketPattern::Raster;
        layers[2].raster_angle = 90.0;
        let mut scene = Scene::default();
        square(&mut scene, 2, 50.0, 50.0, 40.0);
        let r = lib.get(layers[2].tool.as_deref().unwrap()).unwrap().radius();
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let pts = cut_points(&prog);
        assert!(!pts.is_empty());
        // The bit never goes nearer to the wall than its radius, and the lines are along y (the angle 90°).
        let (x0, y0, x1, y1) = xy_bounds(&prog);
        assert!(x0 >= 50.0 - 0.0 + r - 1e-6 && x1 <= 90.0 - r + 1e-6, "{x0} {x1}");
        let (top, bottom) = (dev.area.y - 90.0 + r, dev.area.y - 50.0 - r);
        assert!(y0 >= top - 1e-6 && y1 <= bottom + 1e-6 && y1 - y0 > 30.0, "{y0} {y1}");
        let along_y = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut && (m.to[0] - m.from[0]).abs() < 1e-9 && (m.to[1] - m.from[1]).abs() > 5.0).count();
        assert!(along_y >= 5, "{along_y}");
        // The raster is cheaper than going round and round, and the outline pass adds the wall.
        let plain = generate(&scene, &layers, &lib, &dev, &stock).moves.len();
        layers[2].outline = OutlinePass::Before;
        let with = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(with.moves.len() > plain);
        // Before: the first cut is on the wall (at the radius from it).
        let first = with.moves.iter().find(|m| m.kind == MoveKind::Cut).unwrap();
        let on_wall = |p: P3| (p[0] - (50.0 + r)).abs() < 1e-6 || (p[0] - (90.0 - r)).abs() < 1e-6 || (p[1] - (dev.area.y - 90.0 + r)).abs() < 1e-6 || (p[1] - (dev.area.y - 50.0 - r)).abs() < 1e-6;
        assert!(on_wall(first.to), "{:?}", first.to);
    }

    #[test]
    fn a_rest_pocket_clears_only_what_the_bigger_bit_left() {
        let (lib, dev, stock, mut layers) = setup();
        for li in [2, 3] {
            layers[li].op = Operation::Pocket;
            layers[li].depth = 2.0;
            layers[li].step_down = 2.0;
        }
        with_tool(&mut layers, 2, &lib, "std-flat-6");
        with_tool(&mut layers, 3, &lib, "std-flat-3");
        layers[3].rest_of = Some(2);
        let mut scene = Scene::default();
        square(&mut scene, 2, 50.0, 50.0, 40.0);
        square(&mut scene, 3, 50.0, 50.0, 40.0);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let rest: Vec<P3> = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut && m.layer == 3).flat_map(|m| [m.from, m.to]).collect();
        assert!(!rest.is_empty(), "{:?}", prog.warnings.iter().map(|w| w.to_string()).collect::<Vec<_>>());
        // The middle of the square (x 70, y 70 in the page; the toolpath has y turned over) was cleared by the big bit.
        let mid = |p: &P3| (p[0] - 70.0).abs() < 6.0 && (p[1] - (dev.area.y - 70.0)).abs() < 6.0;
        assert!(!rest.iter().any(mid), "the small bit stays at the corners");
        // Without the rest it clears everything.
        layers[3].rest_of = None;
        let all = generate(&scene, &layers, &lib, &dev, &stock);
        let whole: Vec<P3> = all.moves.iter().filter(|m| m.kind == MoveKind::Cut && m.layer == 3).flat_map(|m| [m.from, m.to]).collect();
        assert!(whole.iter().any(mid) && whole.len() > rest.len());
        // A rest pocket that takes over from a smaller bit is said to be wrong.
        layers[3].rest_of = Some(3);
        with_tool(&mut layers, 3, &lib, "std-flat-3");
        let wrong = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(wrong.warnings.iter().any(|w| w.to_string().contains("has to be smaller")));
    }

    #[test]
    fn a_face_planes_the_box_of_the_shapes_and_goes_over_the_edge() {
        let (lib, dev, stock, mut layers) = setup();
        layers[2].op = Operation::Face;
        layers[2].depth = 0.5;
        layers[2].step_down = 0.5;
        layers[2].overhang = 4.0;
        let r = lib.get(layers[2].tool.as_deref().unwrap()).unwrap().radius();
        let mut scene = Scene::default();
        square(&mut scene, 2, 50.0, 50.0, 40.0);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let (x0, _, x1, _) = xy_bounds(&prog);
        // The edge of the bit goes 4 mm over: the centre is 4 - r over the edge of the box.
        assert!((x0 - (50.0 - (4.0 - r))).abs() < 1e-6 && (x1 - (90.0 + (4.0 - r))).abs() < 1e-6, "{x0} {x1}");
        assert!(prog.moves.iter().filter(|m| m.kind == MoveKind::Cut).all(|m| (m.to[2] + 0.5).abs() < 1e-9));
        // A face without shapes has nothing to plane: said so.
        let empty = Scene::default();
        let none = generate(&empty, &layers, &lib, &dev, &stock);
        assert!(none.is_empty());
    }

    #[test]
    fn a_bore_goes_down_round_the_hole_in_a_spiral_and_never_plunges_into_the_material() {
        let (lib, dev, stock, mut layers) = setup();
        layers[1].op = Operation::Bore;
        layers[1].depth = 6.0;
        layers[1].step_down = 6.0;
        layers[1].ramp_angle = 4.0;
        let r = lib.get(layers[1].tool.as_deref().unwrap()).unwrap().radius();
        let mut scene = Scene::default();
        scene.add(1, Kind::Ellipse { w: 20.0, h: 20.0 }, Xf::translate(100.0, 100.0));
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let cuts: Vec<&Move> = prog.moves.iter().filter(|m| m.kind == MoveKind::Cut).collect();
        assert!(cuts.iter().any(|m| m.to[2] < m.from[2] - 1e-6), "sloping cuts: the spiral");
        assert!(prog.moves.iter().filter(|m| m.kind == MoveKind::Plunge && m.to[2] < 0.0).count() == 0);
        // Every point is at the radius of the hole less the radius of the bit from its middle.
        let cy = dev.area.y - 110.0;
        for m in &cuts {
            let d = ((m.to[0] - 110.0).powi(2) + (m.to[1] - cy).powi(2)).sqrt();
            assert!((d - (10.0 - r)).abs() < 0.05, "{d}");
        }
        assert!(cuts.iter().any(|m| (m.to[2] + 6.0).abs() < 1e-9));
    }

    #[test]
    fn a_chamfer_follows_the_edge_at_the_depth_of_its_width_with_a_v_bit_and_complains_about_others() {
        let (lib, dev, stock, mut layers) = setup();
        layers[3].op = Operation::Chamfer;
        layers[3].chamfer = 1.5;
        layers[3].step_down = 5.0;
        // The V-bit of the first layers is 60°; the depth of a 1.5 mm chamfer is 1.5 / tan 30°.
        let tool = lib.get(layers[3].tool.as_deref().unwrap()).unwrap().clone();
        let want = crate::clear::chamfer_depth(&tool, 1.5).unwrap();
        let mut scene = Scene::default();
        square(&mut scene, 3, 50.0, 50.0, 40.0);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let deepest = prog.moves.iter().map(|m| m.to[2]).fold(0.0, f64::min);
        assert!((deepest + want).abs() < 1e-6, "{deepest} {want}");
        // The centre of the bit follows the line itself.
        let (x0, _, x1, _) = xy_bounds(&prog);
        assert!((x0 - 50.0).abs() < 1e-6 && (x1 - 90.0).abs() < 1e-6);
        with_tool(&mut layers, 3, &lib, "std-flat-3");
        let flat = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(flat.warnings.iter().any(|w| w.to_string().contains("needs a V-bit")));
    }

    #[test]
    fn a_dogbone_slot_goes_into_every_corner_so_that_the_edge_of_the_bit_reaches_it() {
        let (lib, dev, stock, mut layers) = setup();
        layers[1].op = Operation::DogboneSlot;
        layers[1].depth = 2.0;
        layers[1].step_down = 2.0;
        let r = lib.get(layers[1].tool.as_deref().unwrap()).unwrap().radius();
        let mut scene = Scene::default();
        scene.add(1, Kind::Rect { w: 30.0, h: 10.0 }, Xf::translate(50.0, 50.0));
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let cy = |y: f64| dev.area.y - y;
        for (cx, y) in [(50.0, 50.0), (80.0, 50.0), (50.0, 60.0), (80.0, 60.0)] {
            let near = cut_points(&prog).iter().map(|p| ((p[0] - cx).powi(2) + (p[1] - cy(y)).powi(2)).sqrt()).fold(f64::INFINITY, f64::min);
            assert!((near - r).abs() < 1e-6, "the centre comes to a radius from the corner ({cx}, {y}): {near}");
        }
        // Without the notches (the plain inside profile) it stays a radius times the square root of two away.
        layers[1].op = Operation::ProfileInside;
        let plain = generate(&scene, &layers, &lib, &dev, &stock);
        let near = cut_points(&plain).iter().map(|p| ((p[0] - 50.0).powi(2) + (p[1] - cy(50.0)).powi(2)).sqrt()).fold(f64::INFINITY, f64::min);
        assert!((near - r * 2f64.sqrt()).abs() < 1e-6, "{near}");
    }

    #[test]
    fn a_trochoid_slot_is_as_wide_as_its_loops_and_the_bit_and_the_pocket_leaves_no_strip() {
        let (lib, dev, stock, mut layers) = setup();
        layers[3].op = Operation::OnPath;
        with_tool(&mut layers, 3, &lib, "std-flat-3");
        layers[3].depth = 2.0;
        layers[3].step_down = 2.0;
        layers[3].trochoid = crate::layers::Trochoid { enabled: true, diameter: 4.0, step: 1.5, half: false };
        let mut scene = Scene::default();
        scene.add(3, Kind::Path(vec![Polyline::new(vec![Pt::new(20.0, 100.0), Pt::new(80.0, 100.0)], false)]), Xf::IDENTITY);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let (x0, y0, x1, y1) = xy_bounds(&prog);
        let cy = dev.area.y - 100.0;
        assert!(y0 >= cy - 2.0 - 1e-6 && y1 <= cy + 2.0 + 1e-6 && y1 - y0 > 3.5, "{y0} {y1}");
        assert!(x0 >= 18.0 && x1 <= 82.0 && x1 > 78.0);
        assert!(prog.moves.len() > 500, "many small moves: {}", prog.moves.len());
        // A pocket whose width is not a multiple of the step over: the middle is cleared (a ring runs along it).
        layers[2].depth = 2.0;
        layers[2].step_down = 2.0;
        let mut pocket = Scene::default();
        pocket.add(2, Kind::Rect { w: 11.0, h: 40.0 }, Xf::translate(50.0, 50.0));
        let p = generate(&pocket, &layers, &lib, &dev, &stock);
        let mid_x = 50.0 + 11.0 / 2.0;
        assert!(cut_points(&p).iter().any(|q| (q[0] - mid_x).abs() < 0.8), "something runs near the middle line x = {mid_x}");
    }

    #[test]
    fn no_operation_cuts_deeper_than_its_layer_or_moves_fast_below_the_safe_height() {
        use crate::layers::{PocketPattern, Entry};
        let (lib, dev, stock, base) = setup();
        let mut scene = Scene::default();
        square(&mut scene, 0, 50.0, 50.0, 40.0);
        scene.add(0, Kind::Ellipse { w: 12.0, h: 12.0 }, Xf::translate(60.0, 60.0));
        for op in Operation::ALL {
            for pattern in PocketPattern::ALL {
                let mut layers = base.clone();
                layers[0].op = op;
                layers[0].depth = 3.0;
                layers[0].step_down = 1.0;
                layers[0].pattern = pattern;
                layers[0].entry = Entry::RampContour;
                layers[0].chamfer = 1.0;
                layers[0].trochoid = crate::layers::Trochoid { enabled: true, ..Default::default() };
                if op == Operation::Chamfer {
                    with_tool(&mut layers, 0, &lib, "std-v-60");
                }
                let prog = generate(&scene, &layers, &lib, &dev, &stock);
                let depth = if op == Operation::Chamfer { crate::clear::chamfer_depth(lib.get("std-v-60").unwrap(), 1.0).unwrap() } else { 3.0 };
                for m in &prog.moves {
                    assert!(m.to[2] >= -depth - 1e-9 && m.from[2] >= -depth - 1e-9, "{op:?}: {m:?}");
                    if m.kind == MoveKind::Rapid {
                        assert!(m.from[2] >= 1.0 - 1e-9 && m.to[2] >= 1.0 - 1e-9, "{op:?}: a fast move in the material: {m:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn an_old_layer_without_the_new_settings_loads_with_the_old_behaviour() {
        let lib = Library::new();
        let l = default_layers(&lib).remove(0);
        let mut v = serde_json::to_value(&l).unwrap();
        for k in ["direction", "leave", "finish", "entry", "ramp_angle", "ramp_feed", "tabs"] {
            v.as_object_mut().unwrap().remove(k);
        }
        let back: MillLayer = serde_json::from_value(v).unwrap();
        assert_eq!((back.direction, back.leave, back.finish, back.entry, back.tabs.enabled), (CutDir::AsDrawn, 0.0, true, Entry::Plunge, false));
        assert_eq!(back, l);
    }

    /// The cuts of a program as a list of (square, layer): the squares are 10 to 40 wide and 200 apart (x = 0, 200 ...), one entry
    /// for each stretch of cuts on the same square and layer.
    fn cut_order(prog: &Program) -> Vec<(usize, usize)> {
        let mut out: Vec<(usize, usize)> = vec![];
        for m in prog.moves.iter().filter(|m| m.kind == MoveKind::Cut) {
            let at = ((m.from[0] + m.to[0]) / 2.0 / 200.0).round() as usize * 2;
            if out.last() != Some(&(at, m.layer)) {
                out.push((at, m.layer));
            }
        }
        out
    }

    #[test]
    fn one_by_one_cuts_all_layers_of_an_element_before_the_next_and_the_nearest_element_first() {
        let (lib, dev, stock, mut layers) = setup();
        layers[0].depth = 2.0;
        layers[0].step_down = 1.0;
        layers[1].depth = 2.0;
        layers[1].step_down = 2.0;
        // Two groups, each a square on layer 0 (cut out) and a smaller one on layer 1 (inside); the far one is drawn first.
        let mut scene = Scene::default();
        for x in [200.0, 0.0] {
            let outer = scene.add(0, Kind::Rect { w: 40.0, h: 40.0 }, Xf::translate(x, 10.0));
            let inner = scene.add(1, Kind::Rect { w: 20.0, h: 20.0 }, Xf::translate(x + 10.0, 20.0));
            let g = scene.new_group_id();
            for s in scene.shapes.iter_mut().filter(|s| s.id == outer || s.id == inner) {
                s.group = Some(g);
            }
        }
        // A layer after a layer, as before.
        let by_layer = cut_order(&generate(&scene, &layers, &lib, &dev, &stock));
        assert_eq!(by_layer.iter().map(|c| c.1).collect::<Vec<_>>(), vec![0, 0, 1, 1], "{by_layer:?}");
        // One by one: the group at x = 0 (nearest to the machine's zero) completely, then the other.
        let one = generate_with(&scene, &layers, &lib, &dev, &stock, JobOptions { one_by_one: true });
        assert_eq!(cut_order(&one), vec![(0, 0), (0, 1), (2, 0), (2, 1)]);
        // The same work in another order, and the warnings of the layers are said once.
        let plain = generate(&scene, &layers, &lib, &dev, &stock);
        assert_eq!(one.moves.iter().filter(|m| m.kind == MoveKind::Cut).count(), plain.moves.iter().filter(|m| m.kind == MoveKind::Cut).count());
        assert_eq!(one.warnings.len(), plain.warnings.len());
    }

    #[test]
    fn single_shapes_that_are_in_no_group_are_elements_of_their_own() {
        let (lib, dev, stock, mut layers) = setup();
        layers[0].depth = 2.0;
        layers[0].step_down = 2.0;
        let mut scene = Scene::default();
        for x in [400.0, 0.0, 200.0] {
            scene.add(0, Kind::Rect { w: 40.0, h: 40.0 }, Xf::translate(x, 10.0));
        }
        let one = generate_with(&scene, &layers, &lib, &dev, &stock, JobOptions { one_by_one: true });
        assert_eq!(cut_order(&one).iter().map(|c| c.0).collect::<Vec<_>>(), vec![0, 2, 4], "nearest first, not as they were drawn");
    }

    #[test]
    fn the_bit_is_changed_with_a_pause_when_the_next_layer_needs_another() {
        let (lib, dev, stock, mut layers) = setup();
        layers[0].depth = 1.0;
        layers[3].depth = 1.0;
        let mut scene = Scene::default();
        square(&mut scene, 0, 50.0, 50.0, 40.0);
        scene.add(3, Kind::Path(vec![Polyline::new(vec![Pt::new(10.0, 10.0), Pt::new(30.0, 10.0)], false)]), Xf::IDENTITY);
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let g = to_gcode(&prog, &dev, &layers, &lib, "job");
        assert_eq!(g.matches("M0 (Change the bit to: V-bit 60").count(), 1, "{g}");
        assert_eq!(g.matches("M3 S").count(), 2);
    }

    #[test]
    fn the_time_estimate_grows_with_the_length_of_the_cuts() {
        let (lib, dev, stock, layers) = setup();
        let mut a = Scene::default();
        square(&mut a, 0, 20.0, 20.0, 20.0);
        let mut b = Scene::default();
        square(&mut b, 0, 20.0, 20.0, 80.0);
        let ta = generate(&a, &layers, &lib, &dev, &stock).estimate_seconds(dev.rapid);
        let tb = generate(&b, &layers, &lib, &dev, &stock).estimate_seconds(dev.rapid);
        assert!(ta > 0.0 && tb > ta);
    }

    /// A picture of `w` x `h` mm at (x, y): black everywhere, or a gradient from black (left) to white (right).
    fn picture(scene: &mut Scene, layer: usize, x: f64, y: f64, w: f64, h: f64, gradient: bool) {
        let gray: Vec<u8> = (0..64).map(|i| if gradient { ((i % 8) * 255 / 7) as u8 } else { 0 }).collect();
        let im = base_core::image::ImageData { name: "t".into(), px_w: 8, px_h: 8, gray, w, h, invert: false };
        scene.add(layer, Kind::Image(im), Xf::translate(x, y));
    }

    fn engraving(method: PictureMethod) -> Vec<MillLayer> {
        let (_, _, _, mut layers) = setup();
        layers[3].op = Operation::Picture;
        layers[3].picture = method;
        layers[3].dither = base_core::raster::Dither::Threshold;
        layers[3].depth = 0.5;
        layers[3].step_over_pct = 10.0;
        layers[3].scan_angle = 0.0;
        layers
    }

    #[test]
    fn a_picture_is_engraved_as_rows_dots_or_rows_with_a_depth_that_follows_the_tone() {
        let (lib, dev, stock, _) = setup();
        let mut scene = Scene::default();
        picture(&mut scene, 3, 20.0, 20.0, 20.0, 10.0, false);
        // The rows are a tenth of the diameter of the bit apart.
        let tool_d = lib.get(engraving(PictureMethod::Rows)[3].tool.as_deref().unwrap()).unwrap().dims.diameter * 0.1;
        let cuts = |p: &Program| p.moves.iter().filter(|m| m.kind == MoveKind::Cut).count();
        // Rows: a cut for every row of the picture, all at the depth of the layer.
        let rows = generate(&scene, &engraving(PictureMethod::Rows), &lib, &dev, &stock);
        assert!(rows.warnings.is_empty(), "{:?}", rows.warnings);
        let n = (10.0 / tool_d).round() as usize;
        assert_eq!(cuts(&rows), n, "a row for every {tool_d} mm of 10");
        assert!(rows.moves.iter().filter(|m| m.kind == MoveKind::Cut).all(|m| (m.to[2] + 0.5).abs() < 1e-9));
        // Dots: a plunge to the depth for every dark cell and no cutting along.
        let dots = generate(&scene, &engraving(PictureMethod::Dots), &lib, &dev, &stock);
        assert_eq!(cuts(&dots), 0);
        let plunges = dots.moves.iter().filter(|m| m.kind == MoveKind::Plunge && (m.to[2] + 0.5).abs() < 1e-9).count();
        assert_eq!(plunges, n * (20.0 / tool_d).round() as usize);
        // Depth by tone: the gradient is deep where it is dark (left) and shallow where it is light.
        let mut g = Scene::default();
        picture(&mut g, 3, 20.0, 20.0, 20.0, 10.0, true);
        let depth = generate(&g, &engraving(PictureMethod::Depth), &lib, &dev, &stock);
        let zs: Vec<f64> = depth.moves.iter().filter(|m| m.kind == MoveKind::Cut).map(|m| m.to[2]).collect();
        assert!(zs.iter().any(|z| *z < -0.4) && zs.iter().any(|z| *z > -0.15), "deep and shallow: {zs:?}");
        assert!(zs.iter().all(|z| *z <= 0.0 && *z >= -0.5 - 1e-9));
        // A white (light) picture gives nothing, and says so.
        let mut w = Scene::default();
        let im = base_core::image::ImageData { name: "w".into(), px_w: 2, px_h: 2, gray: vec![255; 4], w: 10.0, h: 10.0, invert: false };
        w.add(3, Kind::Image(im), Xf::translate(10.0, 10.0));
        let none = generate(&w, &engraving(PictureMethod::Rows), &lib, &dev, &stock);
        assert!(none.moves.is_empty() && none.warnings.iter().any(|x| x.to_string().contains("nothing to cut")), "{:?}", none.warnings);
    }

    #[test]
    fn a_picture_on_another_operation_is_not_milled_and_a_picture_layer_follows_the_lines_on_it_too() {
        let (lib, dev, stock, layers) = setup();
        let mut scene = Scene::default();
        picture(&mut scene, 3, 20.0, 20.0, 20.0, 10.0, false);
        let p = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(p.warnings.iter().any(|w| w.to_string().contains("pictures are not milled")), "{:?}", p.warnings);
        // A line and a picture on one picture layer: both are cut.
        let mut layers = engraving(PictureMethod::Rows);
        layers[3].depth = 1.0;
        scene.add(3, Kind::Rect { w: 5.0, h: 5.0 }, Xf::translate(100.0, 100.0));
        layers[3].step_down = 1.0;
        let both = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(both.bounds.unwrap().1[0] > 100.0, "the rectangle is followed too");
        assert!(!both.warnings.iter().any(|w| w.to_string().contains("pictures are not milled")));
    }

    #[test]
    fn a_relief_is_cut_in_passes_down_to_the_full_depth_and_never_deeper() {
        let (lib, dev, stock, _) = setup();
        let mut g = Scene::default();
        // A picture of 40 x 20 mm, dark on the left and light on the right.
        picture(&mut g, 3, 20.0, 20.0, 40.0, 20.0, true);
        let mut layers = engraving(PictureMethod::Relief);
        layers[3].tool = Some("std-ball-3".into());
        layers[3].depth = 1.0;
        layers[3].step_down = 1.0;
        let one = generate(&g, &layers, &lib, &dev, &stock);
        assert!(one.warnings.is_empty(), "{:?}", one.warnings);
        let cuts = |p: &Program| p.moves.iter().filter(|m| m.kind == MoveKind::Cut).count();
        assert!(cuts(&one) > 0);
        let zs: Vec<f64> = one.moves.iter().filter(|m| m.kind != MoveKind::Rapid).flat_map(|m| [m.from[2], m.to[2]]).filter(|z| *z < 1.0).collect();
        assert!(zs.iter().all(|z| *z >= -1.0 - 1e-9), "not deeper than the layer");
        assert!(zs.iter().any(|z| *z < -0.5), "the dark side is deep");
        // With a pass depth of 0.25 there are four passes: more cuts, and the first pass is shallow.
        layers[3].step_down = 0.25;
        let four = generate(&g, &layers, &lib, &dev, &stock);
        assert!(cuts(&four) > cuts(&one) * 2, "{} against {}", cuts(&four), cuts(&one));
        let first_cuts: Vec<f64> = four.moves.iter().filter(|m| m.kind == MoveKind::Cut).take(5).map(|m| m.to[2]).collect();
        assert!(first_cuts.iter().all(|z| *z >= -0.25 - 1e-9), "the first pass is at most 0.25 deep: {first_cuts:?}");
    }

    fn vcarve_layers(lib: &Library, depth: f64) -> Vec<MillLayer> {
        let (_, _, _, mut layers) = setup();
        layers[0].op = Operation::VCarve;
        layers[0].depth = depth;
        with_tool(&mut layers, 0, lib, "std-v-60");
        layers[0].depth = depth;
        layers
    }

    #[test]
    fn a_v_carve_goes_as_deep_as_the_distance_from_the_edge_asks_and_never_deeper_than_the_layer() {
        let (lib, dev, stock, _) = setup();
        let mut g = Scene::default();
        // A strip 3 mm wide: a 60° bit reaches the middle at 1.5 / tan 30° = 2.6 mm.
        g.add(0, Kind::Rect { w: 3.0, h: 40.0 }, Xf::translate(50.0, 50.0));
        let layers = vcarve_layers(&lib, 5.0);
        let prog = generate(&g, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.is_empty(), "{:?}", prog.warnings);
        let deepest = cut_points(&prog).iter().map(|p| p[2]).fold(0.0f64, f64::min);
        let want = 1.5 / 30f64.to_radians().tan() - lib.get("std-v-60").unwrap().dims.tip / 2.0 / 30f64.to_radians().tan();
        assert!((deepest + want).abs() < 0.15, "the crest is {want:.2} deep, the deepest cut is {}", -deepest);
        // A shallower layer stops the bit at its depth and cuts a flat bottom.
        let shallow = vcarve_layers(&lib, 1.0);
        let prog = generate(&g, &shallow, &lib, &dev, &stock);
        let deepest = cut_points(&prog).iter().map(|p| p[2]).fold(0.0f64, f64::min);
        assert!((deepest + 1.0).abs() < 1e-6, "{deepest}");
    }

    #[test]
    fn the_rings_of_a_v_carve_are_deeper_the_farther_they_are_from_the_edge() {
        let (lib, _, _, _) = setup();
        let strip = Polyline::new(vec![Pt::new(0.0, 0.0), Pt::new(20.0, 0.0), Pt::new(20.0, 6.0), Pt::new(0.0, 6.0)], true);
        let rings = crate::clear::vcarve_rings(&[strip.clone()], lib.get("std-v-60").unwrap(), 10.0, 0.0, 0.5, 0.5, false).unwrap();
        assert!(rings.len() >= 4, "{}", rings.len());
        assert!(rings.windows(2).all(|w| w[1].0 >= w[0].0 - 1e-9), "deeper and deeper");
        // The crest of the strip is the deepest: 3 mm from the edge.
        let tan = 30f64.to_radians().tan();
        let tip = lib.get("std-v-60").unwrap().dims.tip / 2.0;
        let last = rings.last().unwrap().0;
        assert!((last - (3.0 - tip) / tan).abs() < 0.1, "{last}");
        // A bit that is not a V-bit gives nothing.
        assert!(crate::clear::vcarve_rings(&[strip], lib.get("std-flat-3").unwrap(), 2.0, 0.0, 0.5, 0.5, false).is_none());
    }

    #[test]
    fn a_start_depth_leaves_the_shallow_part_to_the_deeper_rings_and_a_flat_bit_gives_a_warning() {
        let (lib, dev, stock, _) = setup();
        let mut g = Scene::default();
        square(&mut g, 0, 50.0, 50.0, 10.0);
        let mut layers = vcarve_layers(&lib, 2.0);
        layers[0].step_over_pct = 5.0;
        let all = generate(&g, &layers, &lib, &dev, &stock);
        layers[0].vc_start = 1.0;
        let late = generate(&g, &layers, &lib, &dev, &stock);
        let shallowest = |p: &Program| cut_points(p).iter().map(|q| q[2]).fold(f64::NEG_INFINITY, f64::max);
        assert!(shallowest(&late) <= -1.0 + 1e-6, "nothing shallower than the start depth: {}", shallowest(&late));
        assert!(shallowest(&all) > shallowest(&late));
        // A flat bit cannot V-carve.
        with_tool(&mut layers, 0, &lib, "std-ball-3");
        let bad = generate(&g, &layers, &lib, &dev, &stock);
        assert!(bad.warnings.iter().any(|w| w.text().contains("V-bit")), "{:?}", bad.warnings);
    }

    #[test]
    fn the_plug_of_an_inlay_is_carved_outside_the_line_and_the_socket_inside() {
        use crate::layers::Inlay;
        let (lib, dev, stock, _) = setup();
        let mut g = Scene::default();
        square(&mut g, 0, 50.0, 50.0, 10.0);
        let mut layers = vcarve_layers(&lib, 2.0);
        layers[0].inlay = Inlay::Plug;
        layers[0].inlay_gap = 0.1;
        let plug = generate(&g, &layers, &lib, &dev, &stock);
        let pts = cut_points(&plug);
        assert!(!pts.is_empty());
        // The scene is y down: in the toolpath the square is where the scene's page flip puts it; look at the distance from its box.
        let (x0, y0, x1, y1) = xy_bounds(&plug);
        assert!(x1 - x0 > 10.0 + 0.2, "the plug rings go round the shape: {}", x1 - x0);
        assert!(y1 - y0 > 10.0 + 0.2);
        assert!(pts.iter().all(|p| p[2] >= -2.0 - 1e-9));
        layers[0].inlay = Inlay::Socket;
        let socket = generate(&g, &layers, &lib, &dev, &stock);
        let (a0, _, a1, _) = xy_bounds(&socket);
        assert!(a1 - a0 < 10.0 + 0.2 + 1e-6, "the socket stays inside the line and the gap: {}", a1 - a0);
    }

    #[test]
    fn a_rough_relief_leaves_stock_and_a_finish_relief_is_one_go_over_the_relief() {
        use crate::layers::ReliefPart;
        let (lib, dev, stock, _) = setup();
        let mut g = Scene::default();
        picture(&mut g, 3, 20.0, 20.0, 40.0, 20.0, true);
        let mut layers = engraving(PictureMethod::Relief);
        layers[3].tool = Some("std-ball-3".into());
        layers[3].depth = 2.0;
        layers[3].step_down = 0.5;
        let both = generate(&g, &layers, &lib, &dev, &stock);
        layers[3].relief_part = ReliefPart::Finish;
        let finish = generate(&g, &layers, &lib, &dev, &stock);
        layers[3].relief_part = ReliefPart::Rough;
        layers[3].leave = 0.3;
        let rough = generate(&g, &layers, &lib, &dev, &stock);
        let cuts = |p: &Program| p.moves.iter().filter(|m| m.kind == MoveKind::Cut).count();
        assert!(cuts(&finish) > 0 && cuts(&finish) < cuts(&both), "{} < {}", cuts(&finish), cuts(&both));
        let deepest = |p: &Program| cut_points(p).iter().map(|q| q[2]).fold(0.0f64, f64::min);
        assert!((deepest(&finish) + 2.0).abs() < 1e-6, "the finish reaches the relief: {}", deepest(&finish));
        assert!(deepest(&rough) > -2.0 + 0.25, "the rough stays above the relief: {}", deepest(&rough));
    }

    fn two_layer_job(lib: &Library) -> (Scene, Vec<MillLayer>) {
        let (_, _, _, mut layers) = setup();
        let mut scene = Scene::default();
        square(&mut scene, 0, 10.0, 10.0, 20.0);
        square(&mut scene, 2, 60.0, 10.0, 20.0);
        for i in [0, 2] {
            with_tool(&mut layers, i, lib, "std-flat-3");
            layers[i].depth = 1.0;
            layers[i].step_down = 1.0;
        }
        (scene, layers)
    }

    #[test]
    fn coolant_and_the_dust_collector_go_on_with_the_layer_and_off_before_a_bit_change_and_at_the_end() {
        use crate::layers::Coolant;
        let (lib, mut dev, stock, _) = setup();
        let (scene, mut layers) = two_layer_job(&lib);
        dev.gcode.dust_on = "M10".into();
        dev.gcode.dust_off = "M11".into();
        layers[0].coolant = Coolant::Flood;
        layers[0].dust = true;
        layers[2].coolant = Coolant::Mist;
        layers[2].dust = true;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.is_empty(), "{:?}", prog.warnings);
        let g = to_gcode(&prog, &dev, &layers, &lib, "t");
        let lines: Vec<&str> = g.lines().collect();
        let at = |w: &str| lines.iter().position(|l| *l == w).unwrap_or_else(|| panic!("no {w} in {g}"));
        assert!(at("M8") < at("M10") || at("M10") < at("M8"), "both are on");
        let m9 = lines.iter().position(|l| *l == "M9").unwrap();
        assert!(at("M8") < m9 && m9 < at("M7"), "flood, then off, then mist (same bit: no change in between)");
        assert_eq!(lines.iter().filter(|l| **l == "M10").count(), 1, "the collector stays on between the layers");
        assert_eq!(lines.iter().filter(|l| **l == "M9").count(), 2, "off between the two kinds and at the end");
        assert!(lines.iter().rposition(|l| *l == "M11").unwrap() > at("M7"), "the collector goes off at the end");
        let end = lines.iter().rposition(|l| *l == "M9").unwrap();
        assert!(end > at("M7") && end < lines.len() - 3, "M9 before the machine goes home");
        // A layer that wants the collector with a device that has no lines for it is reported.
        dev.gcode.dust_on.clear();
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.iter().any(|w| w.text().contains("dust collector")), "{:?}", prog.warnings);
        // With neither, the program has none of it.
        layers[0].coolant = Coolant::Off;
        layers[0].dust = false;
        layers[2].coolant = Coolant::Off;
        layers[2].dust = false;
        let g = to_gcode(&generate(&scene, &layers, &lib, &dev, &stock), &dev, &layers, &lib, "t");
        assert!(!g.contains("M7") && !g.contains("M8") && !g.contains("M9") && !g.contains("M10"), "{g}");
    }

    #[test]
    fn a_tool_changer_takes_the_bit_without_a_pause_and_switches_the_coolant_off_first() {
        use crate::layers::Coolant;
        let (mut lib, mut dev, stock, _) = setup();
        let (scene, mut layers) = two_layer_job(&lib);
        with_tool(&mut layers, 2, &lib, "std-flat-6");
        layers[0].coolant = Coolant::Mist;
        // Without a changer there is a pause for the second bit.
        let g = to_gcode(&generate(&scene, &layers, &lib, &dev, &stock), &dev, &layers, &lib, "t");
        assert!(g.contains("M0 (Change the bit to") && !g.contains(" M6"), "{g}");
        // With one and numbers in the library: T… M6 for both, no pause.
        dev.changer = true;
        for (id, own, n) in [("std-flat-3", "my-1", 1), ("std-flat-6", "my-2", 2)] {
            let t = Tool { id: own.into(), builtin: false, number: n, ..lib.get(id).unwrap().clone() };
            lib.user.push(t);
        }
        with_tool(&mut layers, 0, &lib, "my-1");
        with_tool(&mut layers, 2, &lib, "my-2");
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.is_empty(), "{:?}", prog.warnings);
        let g = to_gcode(&prog, &dev, &layers, &lib, "t");
        assert!(!g.contains("M0"), "{g}");
        let (a, b) = (g.find("T1 M6").expect("the first bit"), g.find("T2 M6").expect("the second bit"));
        assert!(a < b);
        assert!(g[a..b].contains("M9"), "the coolant goes off before the change: {g}");
        // A bit without a number is reported and the machine stops for it.
        layers[2].tool = Some("std-flat-6".into());
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.iter().any(|w| w.text().contains("changer number")), "{:?}", prog.warnings);
    }

    #[test]
    fn the_rubber_band_frame_goes_round_the_hull_and_not_into_a_notch() {
        let (_, dev, _, _) = setup();
        // An L: the notch at (20, 20) is inside the box and not on the hull.
        let l = [(0.0, 0.0), (40.0, 0.0), (40.0, 20.0), (20.0, 20.0), (20.0, 40.0), (0.0, 40.0)];
        let hull = convex_hull(&l);
        assert_eq!(hull.len(), 5, "{hull:?}");
        assert!(!hull.contains(&(20.0, 20.0)));
        let area: f64 = hull.iter().zip(hull.iter().cycle().skip(1)).map(|(a, b)| a.0 * b.1 - b.0 * a.1).sum::<f64>() / 2.0;
        assert!(area > 0.0 && (area - 1400.0).abs() < 1e-9, "counter-clockwise, 1400 mm2 ({area})");
        let g = frame_hull_gcode(&l, &dev);
        assert!(g.contains("M5") && g.contains("G0 Z"), "{g}");
        assert!(!g.contains("X20 Y20"), "{g}");
        assert_eq!(g.matches("G1 ").count(), 5, "one move to every corner and back: {g}");
        assert_eq!(convex_hull(&[(1.0, 1.0)]).len(), 1);
        assert!(frame_hull_gcode(&[], &dev).is_empty());
    }
}
