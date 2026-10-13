//! From the shapes, the layers, the bits and the device to the moves of the machine, and from the moves to G-code.
//!
//! The moves are in the coordinates of the work area: X to the right, Y away from you (the drawing is turned over, as
//! the scene counts Y downwards), Z = 0 at the top of the material and negative below it.
use crate::device::{Device, Origin};
use crate::layers::{MillLayer, Operation};
use crate::materials::Stock;
use crate::tools::{Library, Tool};
use base_core::geom::{Polyline, Pt};
use base_core::ops;
use base_core::scene::{Kind, Scene};

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
    pub warnings: Vec<String>,
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
struct Builder<'a> {
    moves: Vec<Move>,
    at: P3,
    safe: f64,
    layer: usize,
    feed: f64,
    plunge: f64,
    dev: &'a Device,
}

impl Builder<'_> {
    fn push(&mut self, kind: MoveKind, to: P3, feed: f64) {
        if self.at == to {
            return;
        }
        self.moves.push(Move { kind, from: self.at, to, layer: self.layer, feed });
        self.at = to;
    }
    /// Up to the safe height.
    fn retract(&mut self) {
        if self.at[2] < self.safe {
            let to = [self.at[0], self.at[1], self.safe];
            let kind = if self.at[2] < 0.0 { MoveKind::Plunge } else { MoveKind::Rapid };
            self.push(kind, to, self.feed);
        }
    }
    /// To a point above the material and down to `z` (negative).
    fn dive(&mut self, x: f64, y: f64, z: f64) {
        self.retract();
        self.push(MoveKind::Rapid, [x, y, self.safe], self.dev.rapid);
        // Fast to just above the surface, then at the plunge feed.
        if self.safe > 1.0 {
            self.push(MoveKind::Rapid, [x, y, 1.0], self.dev.rapid);
        }
        self.push(MoveKind::Plunge, [x, y, z], self.plunge);
    }
    fn cut_to(&mut self, x: f64, y: f64, z: f64) {
        self.push(MoveKind::Cut, [x, y, z], self.feed);
    }
    fn down_to(&mut self, z: f64) {
        let [x, y, _] = self.at;
        self.push(MoveKind::Plunge, [x, y, z], self.plunge);
    }
}

/// A closed shape of the layer with holes, as rings: is the ring inside an odd number of the others?
fn is_hole(rings: &[Polyline], i: usize) -> bool {
    let Some(p) = rings[i].pts.first().copied() else { return false };
    rings.iter().enumerate().filter(|(j, r)| *j != i && r.closed && r.contains(p)).count() % 2 == 1
}

/// The outlines of the shapes of a layer in the coordinates of the work area.
fn layer_outlines(scene: &Scene, layer: usize, area_y: f64, images: &mut usize) -> Vec<Polyline> {
    let mut out = vec![];
    for s in scene.shapes.iter().filter(|s| s.layer == layer) {
        if matches!(s.kind, Kind::Image(_)) {
            *images += 1;
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

/// The toolpaths of every enabled layer, in the order of the layers.
pub fn generate(scene: &Scene, layers: &[MillLayer], tools: &Library, dev: &Device, stock: &Stock) -> Program {
    let mut prog = Program::default();
    let mut b = Builder { moves: vec![], at: [0.0, 0.0, dev.safe_z], safe: dev.safe_z, layer: 0, feed: 0.0, plunge: 0.0, dev };
    for (li, layer) in layers.iter().enumerate() {
        let mut images = 0;
        let outlines = layer_outlines(scene, li, dev.area.y, &mut images);
        if images > 0 && layer.enabled {
            prog.warnings.push(format!("{}: pictures are not milled", layer.name));
        }
        if !layer.enabled || outlines.is_empty() {
            continue;
        }
        if let Err(e) = layer.check() {
            prog.warnings.push(e);
            continue;
        }
        let Some(tool) = layer.tool.as_deref().and_then(|id| tools.get(id)) else {
            prog.warnings.push(format!("{}: no bit chosen", layer.name));
            continue;
        };
        let mut layer = layer.clone();
        if layer.depth > stock.thickness + 1e-9 {
            prog.warnings.push(format!("{}: the depth of {} mm is more than the thickness of the material ({} mm)", layer.name, layer.depth, stock.thickness));
        }
        if layer.depth > dev.area.z {
            prog.warnings.push(format!("{}: the depth is limited to the travel of the machine ({} mm)", layer.name, dev.area.z));
            layer.depth = dev.area.z;
        }
        if layer.rpm > dev.spindle.max_rpm || layer.rpm < dev.spindle.min_rpm {
            prog.warnings.push(format!("{}: {} rpm is outside the spindle range {}…{} rpm", layer.name, layer.rpm, dev.spindle.min_rpm, dev.spindle.max_rpm));
        }
        let depths = layer.passes();
        if depths.is_empty() {
            continue;
        }
        b.layer = li;
        b.feed = layer.feed;
        b.plunge = layer.plunge;
        let start = b.moves.len();
        run_layer(&mut b, &layer, tool, &outlines, &depths, &mut prog.warnings);
        b.retract();
        if b.moves.len() == start {
            prog.warnings.push(format!("{}: nothing to cut (the line may be too small for the bit)", layer.name));
        }
    }
    prog.moves = b.moves;
    prog.bounds = bounds(&prog.moves);
    if let Some((lo, hi)) = prog.bounds {
        if lo[0] < -1e-6 || lo[1] < -1e-6 || hi[0] > dev.area.x + 1e-6 || hi[1] > dev.area.y + 1e-6 {
            prog.warnings.push("The cuts go outside of the work area of the device".to_string());
        }
    }
    prog
}

fn run_layer(b: &mut Builder, layer: &MillLayer, tool: &Tool, outlines: &[Polyline], depths: &[f64], warnings: &mut Vec<String>) {
    let r = tool.radius();
    match layer.op {
        Operation::ProfileOutside | Operation::ProfileInside => {
            let closed = closed_only(outlines);
            if closed.len() < outlines.len() {
                warnings.push(format!("{}: open lines are left out of a profile (use \"Follow the line\")", layer.name));
            }
            let d = if layer.op == Operation::ProfileOutside { r } else { -r };
            for ring in ops::offset(&closed, d) {
                ring_passes(b, &ring, depths);
            }
        }
        Operation::Pocket => {
            let closed = closed_only(outlines);
            if closed.len() < outlines.len() {
                warnings.push(format!("{}: open lines are left out of a pocket", layer.name));
            }
            let inner = ops::offset(&closed, -r);
            let step = (tool.dims.diameter * layer.step_over_pct / 100.0).max(0.05);
            for ring in ops::inset_rings(&inner, step, 500) {
                ring_passes(b, &ring, depths);
            }
        }
        Operation::OnPath => {
            for line in outlines {
                if line.closed {
                    ring_passes(b, line, depths);
                } else {
                    open_passes(b, line, depths);
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

/// The machine zero of the device: the coordinates of a point of the work area as the machine counts them.
pub fn machine_xy(dev: &Device, x: f64, y: f64) -> (f64, f64) {
    match dev.origin {
        Origin::FrontLeft => (x, y),
        Origin::FrontRight => (x - dev.area.x, y),
        Origin::BackLeft => (x, y - dev.area.y),
        Origin::BackRight => (x - dev.area.x, y - dev.area.y),
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
    let mut g = String::new();
    let clean = |s: &str| s.chars().filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_' || *c == '.').collect::<String>();
    g.push_str(&format!("(MillCreator: {})\n(Device: {})\n", clean(program_name), clean(&dev.name)));
    g.push_str("G21 G90 G17\n");
    let mut feed = f64::NAN;
    let mut layer = usize::MAX;
    let mut tool_id: Option<String> = None;
    let mut spindle_on = false;
    let xyz = |p: P3| {
        let (x, y) = machine_xy(dev, p[0], p[1]);
        (x, y, p[2])
    };
    for m in &prog.moves {
        if m.layer != layer {
            layer = m.layer;
            if let Some(l) = layers.get(layer) {
                g.push_str(&format!("(Layer: {})\n", clean(&l.name)));
                if tool_id != l.tool {
                    if spindle_on {
                        g.push_str(&format!("G0 Z{}\nM5\n", num(dev.safe_z)));
                    }
                    let name = l.tool.as_deref().and_then(|id| tools.get(id)).map(|t| clean(&t.name)).unwrap_or_default();
                    if tool_id.is_some() {
                        g.push_str(&format!("M0 (Change the bit to: {name})\n"));
                    } else {
                        g.push_str(&format!("(Bit: {name})\n"));
                    }
                    tool_id = l.tool.clone();
                    spindle_on = false;
                }
                let rpm = l.rpm.clamp(dev.spindle.min_rpm, dev.spindle.max_rpm);
                if rpm > 0.0 {
                    g.push_str(&format!("S{} M3\n", num(rpm)));
                    if dev.spindle.spin_up > 0.0 && !spindle_on {
                        g.push_str(&format!("G4 P{}\n", num(dev.spindle.spin_up)));
                    }
                    spindle_on = true;
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
    g.push_str(&format!("G0 Z{}\nM5\n", num(dev.safe_z)));
    let (hx, hy) = machine_xy(dev, 0.0, 0.0);
    g.push_str(&format!("G0 X{} Y{}\nM30\n", num(hx), num(hy)));
    g
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
        assert!(prog.warnings.iter().any(|w| w.contains("nothing to cut")), "{:?}", prog.warnings);
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
        assert!(prog.is_empty() && prog.warnings.iter().any(|w| w.contains("no bit")));
        layers[0].tool = Some("std-flat-3".into());
        layers[0].depth = 25.0;
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.iter().any(|w| w.contains("more than the thickness")), "{:?}", prog.warnings);
        // Outside the work area.
        let mut far = Scene::default();
        square(&mut far, 0, 280.0, 20.0, 40.0);
        layers[0].depth = 5.0;
        let prog = generate(&far, &layers, &lib, &dev, &stock);
        assert!(prog.warnings.iter().any(|w| w.contains("outside of the work area")), "{:?}", prog.warnings);
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
        assert!(g.contains("S16000 M3\nG4 P2\n"), "{g}");
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
        assert_eq!(g.matches(" M3\n").count(), 2);
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
}
