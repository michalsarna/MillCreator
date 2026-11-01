//! How the bit goes round a ring or along a line: which way (climb or conventional), how it goes down into the material
//! (plunge, ramps, a helix) and where it leaves tabs (bridges) standing.
use crate::layers::{CutDir, Entry, MillLayer, Operation, TabKind, Tabs};
use crate::toolpath::{Builder, MoveKind};
use base_core::geom::{Polyline, Pt};
use base_core::ops;

/// The height the bit rises to over a full tab: just above the material.
const OVER_MATERIAL: f64 = 0.2;
/// The step of a tab with straight edges: so short that it is a vertical move.
const EDGE: f64 = 0.002;
/// Points of a helix per turn.
const HELIX_STEPS: usize = 24;

/// What went differently from what was asked for, found while the passes were made.
#[derive(Default)]
pub(crate) struct Notes {
    /// A helix did not fit: the ramp along the contour was used.
    pub helix_fallback: bool,
    /// A ring was too short for a tab, or too small for its tabs: it has fewer, or none.
    pub tabs_fewer: bool,
}

/// The way down and the tabs of a layer, for the passes of one ring.
pub(crate) struct Plan {
    pub entry: Entry,
    /// The slope of the ramp or the helix, degrees from the horizontal.
    pub angle: f64,
    pub ramp_feed: f64,
    pub tabs: Option<Tabs>,
    /// Where tabs were placed by hand (in the coordinates of the toolpaths) and how far from the line of the shape the bit goes: a ring
    /// with an anchor within that distance (and a little more) gets a tab at the nearest point of the ring, and no others.
    pub anchors: Vec<Pt>,
    pub reach: f64,
    /// A pocket (the only place for a helix: the bit goes down in the area that is cleared anyway).
    pub pocket: bool,
    /// The depth of the layer and the diameter of the bit (a tab is as wide as the part and the bit).
    pub depth: f64,
    pub diameter: f64,
}

impl Plan {
    pub fn of(layer: &MillLayer, diameter: f64, ramp_default: f64) -> Plan {
        let tabs = (layer.tabs.enabled && matches!(layer.op, Operation::ProfileOutside | Operation::ProfileInside | Operation::OnPath)).then(|| layer.tabs.clone());
        Plan { entry: layer.entry, angle: layer.ramp_angle.clamp(0.1, 89.0), ramp_feed: if layer.ramp_feed > 0.0 { layer.ramp_feed } else if ramp_default > 0.0 { ramp_default } else { layer.plunge }, tabs, anchors: vec![], reach: 0.0, pocket: layer.op == Operation::Pocket, depth: layer.depth, diameter }
    }
}

/// Is the ring the other way round than a ring that surrounds the biggest one's area? (Offsetting gives outer rings one way
/// and holes the other.)
pub(crate) fn roles(rings: &[Polyline]) -> Vec<bool> {
    let outer = rings.iter().map(|r| r.area()).fold(0.0f64, |a, b| if b.abs() > a.abs() { b } else { a });
    rings.iter().map(|r| r.area() * outer < 0.0).collect()
}

/// The rings turned the way `dir` asks for the operation: with a clockwise spindle a climb cut has the material on the left of
/// the bit. (The toolpaths are in the coordinates of the work area, y up: a positive area is counter-clockwise.)
pub(crate) fn oriented(mut rings: Vec<Polyline>, op: Operation, dir: CutDir) -> Vec<Polyline> {
    if dir == CutDir::AsDrawn {
        return rings;
    }
    let hole = roles(&rings);
    for (r, hole) in rings.iter_mut().zip(hole) {
        // The material that is kept: inside an outside profile, outside an inside profile or a pocket; the other way round for a hole.
        let keep_inside = match op {
            Operation::ProfileOutside => !hole,
            _ => hole,
        };
        let mut want_ccw = keep_inside;
        if dir == CutDir::Conventional {
            want_ccw = !want_ccw;
        }
        if (r.area() > 0.0) != want_ccw {
            r.pts.reverse();
        }
    }
    rings
}

/// A closed line with the length along it at every point.
struct Path {
    pts: Vec<Pt>,
    cum: Vec<f64>,
    len: f64,
}

impl Path {
    fn of(ring: &Polyline) -> Path {
        let mut cum = vec![0.0];
        for w in ring.pts.windows(2) {
            let l = cum[cum.len() - 1] + w[0].dist(w[1]);
            cum.push(l);
        }
        let n = ring.pts.len();
        let len = cum[n - 1] + ring.pts[n - 1].dist(ring.pts[0]);
        Path { pts: ring.pts.clone(), cum, len }
    }

    /// The point `s` mm along the ring (it goes round and round).
    fn at(&self, s: f64) -> Pt {
        let s = s.rem_euclid(self.len);
        let i = self.cum.partition_point(|c| *c <= s).saturating_sub(1);
        let (a, b) = (self.pts[i], self.pts[(i + 1) % self.pts.len()]);
        let seg = (if i + 1 < self.cum.len() { self.cum[i + 1] } else { self.len }) - self.cum[i];
        let t = if seg > 1e-12 { (s - self.cum[i]) / seg } else { 0.0 };
        Pt::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t)
    }

    /// The point of the ring nearest to `p`: how far along the ring it is, and how far from `p`.
    fn nearest(&self, p: Pt) -> Option<(f64, f64)> {
        let n = self.pts.len();
        let mut best: Option<(f64, f64)> = None;
        for i in 0..n {
            let (a, b) = (self.pts[i], self.pts[(i + 1) % n]);
            let (dx, dy) = (b.x - a.x, b.y - a.y);
            let l2 = dx * dx + dy * dy;
            let t = if l2 > 1e-12 { (((p.x - a.x) * dx + (p.y - a.y) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
            let q = Pt::new(a.x + dx * t, a.y + dy * t);
            let d = q.dist(p);
            if best.is_none_or(|(_, bd)| d < bd) {
                best = Some((self.cum[i] + t * l2.sqrt(), d));
            }
        }
        best
    }

    /// Where the vertices lie between `from` and `to`, going round as often as needed.
    fn vertices_between(&self, from: f64, to: f64) -> Vec<f64> {
        let mut out = vec![];
        let laps = (from / self.len).floor() as i64;
        let mut k = laps;
        while (k as f64) * self.len < to {
            for c in &self.cum {
                let s = k as f64 * self.len + c;
                if s > from + 1e-9 && s < to - 1e-9 {
                    out.push(s);
                }
            }
            k += 1;
        }
        out
    }
}

/// A tab of a ring: where its middle is along the ring (mm), and how far from the middle the top reaches.
struct Tab {
    centre: f64,
    half: f64,
}

/// The tabs of a ring of length `len`: evenly spread, between `min` and `max`, none for a hole when they are skipped, fewer when
/// the ring is too short to keep half of it uncut.
fn tab_places(t: &Tabs, path: &Path, hole: bool, diameter: f64, anchors: &[Pt], reach: f64, notes: &mut Notes) -> Vec<Tab> {
    let len = path.len;
    let width = t.length + diameter;
    // Tabs placed by hand near this ring are the ones it gets.
    let by_hand: Vec<Tab> = anchors.iter().filter_map(|a| path.nearest(*a)).filter(|(_, d)| *d <= reach + 0.5).map(|(s, _)| Tab { centre: s, half: width / 2.0 }).collect();
    if !by_hand.is_empty() {
        return by_hand;
    }
    if hole && t.skip_inner {
        return vec![];
    }
    let mut n = ((len / t.spacing).round() as u32).clamp(t.min, t.max.max(t.min));
    let room = (len * 0.5 / width).floor() as u32;
    if n > room {
        n = room;
        notes.tabs_fewer = true;
    }
    (0..n).map(|i| Tab { centre: (i as f64 + 0.5) * len / n as f64, half: width / 2.0 }).collect()
}

/// The height of the bit over the tabs at `s` along a ring of length `len`: `None` where there is no tab.
fn tab_top(tabs: &[Tab], t: &Tabs, depth: f64, len: f64, s: f64) -> Option<f64> {
    let top = match t.kind {
        TabKind::Full => OVER_MATERIAL,
        _ => -(depth - t.height).max(0.0),
    };
    let mut best: Option<f64> = None;
    for tab in tabs {
        let mut d = (s - tab.centre).rem_euclid(len);
        d = d.min(len - d);
        let z = if d <= tab.half + 1e-9 {
            Some(top)
        } else if t.kind == TabKind::Ramp && d <= tab.half + (top + depth) / t.slope.to_radians().tan() {
            Some(top - (d - tab.half) * t.slope.to_radians().tan())
        } else {
            None
        };
        if let Some(z) = z {
            best = Some(best.map_or(z, |b: f64| b.max(z)));
        }
    }
    best
}

/// The places along `[from, to]` where the height of a tab changes (its edges and the feet of its slopes), and just before the edges of
/// a tab that has straight sides, so that the bit rises in a vertical move.
fn tab_knots(tabs: &[Tab], t: &Tabs, depth: f64, len: f64, from: f64, to: f64) -> Vec<f64> {
    let mut out = vec![];
    let top = match t.kind {
        TabKind::Full => OVER_MATERIAL,
        _ => -(depth - t.height).max(0.0),
    };
    let run = if t.kind == TabKind::Ramp { (top + depth).max(0.0) / t.slope.to_radians().tan() } else { 0.0 };
    let laps = (from / len).floor() as i64 - 1;
    let mut k = laps;
    while (k as f64) * len < to + len {
        for tab in tabs {
            let c = k as f64 * len + tab.centre;
            let mut marks = vec![c - tab.half, c + tab.half];
            if t.kind == TabKind::Ramp {
                marks.push(c - tab.half - run);
                marks.push(c + tab.half + run);
            } else {
                marks.push(c - tab.half - EDGE);
                marks.push(c + tab.half + EDGE);
            }
            for m in marks {
                if m > from + 1e-9 && m < to - 1e-9 {
                    out.push(m);
                }
            }
        }
        k += 1;
    }
    out
}

/// What the bit follows along a ring at one depth pass.
struct Ride<'a> {
    path: &'a Path,
    tabs: &'a [Tab],
    plan: &'a Plan,
}

impl Ride<'_> {
    /// Cut along the ring from `from` to `to` (mm along it), the height following `z_path`, raised over the tabs. `feed` is the feed of the
    /// cuts; the bit stays where it is when the way is empty.
    fn follow(&self, b: &mut Builder, from: f64, to: f64, feed: f64, z_path: &dyn Fn(f64) -> f64, breaks: &[f64]) {
        let mut s: Vec<f64> = self.path.vertices_between(from, to);
        s.extend(breaks.iter().copied().filter(|x| *x > from + 1e-9 && *x < to - 1e-9));
        if let Some(t) = &self.plan.tabs {
            if !self.tabs.is_empty() {
                s.extend(tab_knots(self.tabs, t, self.plan.depth, self.path.len, from, to));
            }
        }
        s.push(to);
        s.sort_by(|a, c| a.total_cmp(c));
        s.dedup_by(|a, c| (*a - *c).abs() < 1e-9);
        for s in s {
            let p = self.path.at(s);
            let mut z = z_path(s);
            if let Some(t) = &self.plan.tabs {
                if let Some(top) = tab_top(self.tabs, t, self.plan.depth, self.path.len, s) {
                    z = z.max(top);
                }
            }
            b.push(MoveKind::Cut, [p.x, p.y, z], feed);
        }
    }
}

/// A helix that fits inside `ring` (and outside the `holes`): the centre and the radius of the circle the centre of the bit follows. The
/// circle is put on the way from `start` to the middle of the ring, as near the start as it fits.
fn helix_place(ring: &Polyline, holes: &[Polyline], start: Pt, radius: f64) -> Option<(Pt, f64)> {
    let middle = ring.bounds()?.center();
    for k in [1.0, 0.7, 0.5, 0.35] {
        let rh = radius * k;
        for step in 1..=10 {
            let t = step as f64 / 10.0;
            let c = Pt::new(start.x + (middle.x - start.x) * t, start.y + (middle.y - start.y) * t);
            let fits = (0..HELIX_STEPS).all(|i| {
                let a = i as f64 / HELIX_STEPS as f64 * std::f64::consts::TAU;
                let p = Pt::new(c.x + rh * a.cos(), c.y + rh * a.sin());
                ring.contains(p) && !holes.iter().any(|h| h.contains(p))
            });
            if fits && rh >= 0.15 {
                return Some((c, rh));
            }
        }
    }
    None
}

/// Go round `ring` at every depth of `depths` (positive, deepest last). The first pass starts at the first point; the next ones
/// follow without a retract (so that is where the bit goes down again). `quiet`: a helix that does not fit is no news. `hole` says that the ring is a hole of a part (no 
/// when the layer skips them, no helix); `holes` are the other holes, which a helix keeps out of.
pub(crate) fn ring_passes(b: &mut Builder, ring: &Polyline, depths: &[f64], plan: &Plan, hole: bool, holes: &[Polyline], quiet: bool, notes: &mut Notes) {
    let Some(mut rp) = RingPass::new(ring, plan, hole, holes, quiet, notes) else { return };
    for (i, d) in depths.iter().enumerate() {
        rp.pass(b, *d, i == 0, notes);
    }
}

/// Is the straight way from `a` to `b` inside the region (the parts of it that are in an odd number of the rings)? Looked at every
/// quarter of a millimetre; `region` should be a little wider than the real one, so that its edge counts as inside.
pub(crate) fn way_inside(region: &[Polyline], a: Pt, b: Pt) -> bool {
    let inside = |p: Pt| region.iter().filter(|r| r.contains(p)).count() % 2 == 1;
    let len = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
    let n = (len / 0.25).ceil().max(1.0) as usize;
    (0..=n).all(|i| {
        let t = i as f64 / n as f64;
        inside(Pt::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t))
    })
}

/// The rings of a pocket with the bit staying down: every depth goes round all the rings in turn, and the way to the next ring is a
/// cut at that depth when it lies inside `region` (where the centre of the bit may go); otherwise the bit rises and dives again as
/// it always did. The first ring of every depth starts from above: it goes down from the depth of the pass before.
pub(crate) fn rings_staying_down(b: &mut Builder, rings: &[Polyline], hole: &[bool], holes: &[Polyline], depths: &[f64], plan: &Plan, region: &[Polyline], notes: &mut Notes) {
    let wide = ops::offset(region, 0.02);
    let mut rp: Vec<RingPass> = rings.iter().zip(hole).enumerate().filter_map(|(i, (r, h))| RingPass::new(r, plan, *h, holes, i > 0, notes)).collect();
    for (k, d) in depths.iter().enumerate() {
        let mut at: Option<Pt> = None;
        for p in rp.iter_mut() {
            let start = p.start();
            if at.is_some_and(|e| way_inside(&wide, e, start)) {
                // Already at the depth: a cut to the next ring, then one lap.
                b.cut_to(start.x, start.y, -d);
                p.lap_at_depth(b, *d, notes);
            } else if k == 0 {
                p.pass(b, *d, true, notes);
            } else {
                // From above, down to the depth of the pass before (the way is open there), then the usual way down.
                b.dive(start.x, start.y, -depths[k - 1]);
                p.pass(b, *d, false, notes);
            }
            at = Some(p.start());
        }
    }
}

/// A ring that is gone round one depth at a time (`ring_passes` does all the depths of a ring in a row; a pocket that stays down
/// between its rings does one depth of every ring before it goes deeper). It remembers where on the ring the next pass starts and how
/// deep the last one was.
pub(crate) struct RingPass<'a> {
    ring: &'a Polyline,
    path: Path,
    tabs: Vec<Tab>,
    plan: &'a Plan,
    hole: bool,
    holes: &'a [Polyline],
    quiet: bool,
    first: Pt,
    /// Where on the ring (mm along it) the next pass starts.
    s: f64,
    /// The depth of the last pass (negative; 0 before the first).
    pub(crate) z_from: f64,
}

impl<'a> RingPass<'a> {
    /// `None` when the ring is too short to be gone round.
    pub(crate) fn new(ring: &'a Polyline, plan: &'a Plan, hole: bool, holes: &'a [Polyline], quiet: bool, notes: &mut Notes) -> Option<RingPass<'a>> {
        if ring.pts.len() < 2 {
            return None;
        }
        let path = Path::of(ring);
        if path.len < 1e-6 {
            return None;
        }
        let tabs = match &plan.tabs {
            Some(t) => tab_places(t, &path, hole, plan.diameter, &plan.anchors, plan.reach, notes),
            None => vec![],
        };
        Some(RingPass { ring, path, tabs, plan, hole, holes, quiet, first: ring.pts[0], s: 0.0, z_from: 0.0 })
    }

    /// Where the next pass starts.
    pub(crate) fn start(&self) -> Pt {
        self.path.at(self.s)
    }

    /// The bit is at the depth `d` already (it came there by a cut): one lap at that depth from where the pass starts.
    pub(crate) fn lap_at_depth(&mut self, b: &mut Builder, d: f64, notes: &mut Notes) {
        self.z_from = -d;
        self.pass(b, d, false, notes);
    }

    /// One pass to the depth `d` (positive). `first` is the first pass of the ring: the bit dives to the start from above.
    pub(crate) fn pass(&mut self, b: &mut Builder, d: f64, first: bool, notes: &mut Notes) {
        let (plan, hole, holes, quiet) = (self.plan, self.hole, self.holes, self.quiet);
        let ride = Ride { path: &self.path, tabs: &self.tabs, plan };
        let path = &self.path;
        let tan = plan.angle.to_radians().tan();
        let first_pt = self.first;
        let ring = self.ring;
        let (mut s, mut z_from) = (self.s, self.z_from);
        let z_to = -d;
        if first {
            if plan.entry == Entry::Plunge {
                b.dive(first_pt.x, first_pt.y, z_to);
                z_from = z_to;
            } else {
                // To the surface, then down into the material the way the entry says.
                b.dive(first_pt.x, first_pt.y, 0.0);
            }
        }
        if (z_from - z_to).abs() < 1e-9 {
            // At the depth already (the plunge of the first pass): one lap.
            ride.follow(b, s, s + path.len, b.feed, &|_| z_to, &[]);
            self.s = s;
            self.z_from = z_to;
            return;
        }
        let drop = z_from - z_to;
        match plan.entry {
            Entry::Plunge => {
                b.down_to(z_to);
                ride.follow(b, s, s + path.len, b.feed, &|_| z_to, &[]);
            }
            Entry::RampContour => {
                let l = drop / tan;
                let (s0, feed) = (s, plan.ramp_feed);
                ride.follow(b, s0, s0 + l, feed, &|x| z_from - (x - s0) * tan, &[]);
                ride.follow(b, s0 + l, s0 + l + path.len, b.feed, &|_| z_to, &[]);
                s = (s0 + l).rem_euclid(path.len);
            }
            Entry::RampZigzag => {
                // Back and forth along the first part of the ring, an even number of legs, so that it ends where it began.
                let l = drop / tan;
                let longest = path.len * 0.45;
                let legs = 2 * ((l / (2.0 * longest)).ceil().max(1.0) as usize);
                let leg = l / legs as f64;
                let (s0, feed) = (s, plan.ramp_feed);
                for k in 0..legs {
                    let (a, c) = if k % 2 == 0 { (s0, s0 + leg) } else { (s0 + leg, s0) };
                    let done = k as f64 * leg;
                    let z = move |x: f64| z_from - (done + (x - a).abs()) * tan;
                    if a < c {
                        ride.follow(b, a, c, feed, &z, &[]);
                    } else {
                        // Backwards: the path is walked forwards, so the points are laid down in the other order.
                        zigzag_back(b, &ride, c, a, feed, &z);
                    }
                }
                ride.follow(b, s0, s0 + path.len, b.feed, &|_| z_to, &[]);
                s = s0.rem_euclid(path.len);
            }
            Entry::Helix => {
                let r = (plan.diameter / 2.0).max(0.2);
                let others: Vec<Polyline> = holes.to_vec();
                match (plan.pocket && !hole).then(|| helix_place(ring, &others, path.at(s), r)).flatten() {
                    Some((c, rh)) => {
                        let turns = (drop / (std::f64::consts::TAU * rh * tan)).ceil().max(1.0);
                        let steps = (turns as usize) * HELIX_STEPS;
                        let here = path.at(s);
                        let a0 = (here.y - c.y).atan2(here.x - c.x);
                        // To the start of the helix: the point of the circle nearest to where the bit is.
                        b.push(MoveKind::Cut, [c.x + rh * a0.cos(), c.y + rh * a0.sin(), z_from], b.feed);
                        for k in 1..=steps {
                            let t = k as f64 / steps as f64;
                            let a = a0 + t * turns * std::f64::consts::TAU;
                            b.push(MoveKind::Cut, [c.x + rh * a.cos(), c.y + rh * a.sin(), z_from - drop * t], plan.ramp_feed);
                        }
                        // Back to the line the lap starts on, at the depth.
                        b.push(MoveKind::Cut, [here.x, here.y, z_to], b.feed);
                        ride.follow(b, s, s + path.len, b.feed, &|_| z_to, &[]);
                    }
                    None => {
                        // The inner rings of a pocket are narrow: only the first one says that its helix did not fit.
                        notes.helix_fallback |= !quiet;
                        let l = drop / tan;
                        let (s0, feed) = (s, plan.ramp_feed);
                        ride.follow(b, s0, s0 + l, feed, &|x| z_from - (x - s0) * tan, &[]);
                        ride.follow(b, s0 + l, s0 + l + path.len, b.feed, &|_| z_to, &[]);
                        s = (s0 + l).rem_euclid(path.len);
                    }
                }
            }
        }
        z_from = z_to;
        self.s = s;
        self.z_from = z_from;
    }
}

/// Cut backwards along the ring from `to` to `from` (`from` < `to`): the same points as going forwards, in the other order.
fn zigzag_back(b: &mut Builder, ride: &Ride, from: f64, to: f64, feed: f64, z: &dyn Fn(f64) -> f64) {
    let mut s: Vec<f64> = ride.path.vertices_between(from, to);
    s.push(from);
    s.sort_by(|a, c| c.total_cmp(a));
    for x in s {
        let p = ride.path.at(x);
        b.push(MoveKind::Cut, [p.x, p.y, z(x)], feed);
    }
}

/// Go along an open line at every depth, forwards and backwards in turn. A ramp is a zigzag along the line from where the pass
/// begins; a line that is too short for the slope is walked back and forth as often as it takes.
pub(crate) fn open_passes(b: &mut Builder, line: &Polyline, depths: &[f64], plan: &Plan) {
    if line.pts.len() < 2 || depths.is_empty() {
        return;
    }
    let first = line.pts[0];
    let mut here = vec![];
    here.extend(line.pts.iter().copied());
    let tan = plan.angle.to_radians().tan();
    let mut z_from = 0.0;
    for (i, d) in depths.iter().enumerate() {
        let z_to = -d;
        let pts: Vec<Pt> = if i % 2 == 0 { here.clone() } else { here.iter().rev().copied().collect() };
        if i == 0 {
            if plan.entry == Entry::Plunge {
                b.dive(first.x, first.y, z_to);
                z_from = z_to;
            } else {
                b.dive(first.x, first.y, 0.0);
            }
        }
        if plan.entry != Entry::Plunge && (z_from - z_to).abs() > 1e-9 {
            ramp_along(b, &pts, z_from, z_to, tan, plan.ramp_feed);
        } else if i > 0 {
            b.down_to(z_to);
        }
        for q in pts.iter().skip(1) {
            b.push(MoveKind::Cut, [q.x, q.y, z_to], b.feed);
        }
        z_from = z_to;
    }
}

/// Down from `z_from` to `z_to` along `pts` (forwards and back, an even number of legs, each no longer than the line), the bit ends at the
/// first point at `z_to`.
fn ramp_along(b: &mut Builder, pts: &[Pt], z_from: f64, z_to: f64, tan: f64, feed: f64) {
    let seg: Vec<f64> = pts.windows(2).map(|w| w[0].dist(w[1])).collect();
    let total: f64 = seg.iter().sum();
    if total < 1e-9 {
        b.down_to(z_to);
        return;
    }
    let l = (z_from - z_to) / tan;
    let legs = 2 * ((l / (2.0 * total)).ceil().max(1.0) as usize);
    let per = l / legs as f64;
    let at = |s: f64| -> Pt {
        let mut acc = 0.0;
        for (i, sg) in seg.iter().enumerate() {
            if s <= acc + sg + 1e-12 {
                let t = if *sg > 1e-12 { (s - acc) / sg } else { 0.0 };
                return Pt::new(pts[i].x + (pts[i + 1].x - pts[i].x) * t, pts[i].y + (pts[i + 1].y - pts[i].y) * t);
            }
            acc += sg;
        }
        pts[pts.len() - 1]
    };
    // The places of a leg along the line: the start, the vertices on the way and the end.
    let mut marks: Vec<f64> = vec![0.0, per];
    let mut acc = 0.0;
    for sg in &seg {
        acc += sg;
        if acc > 1e-9 && acc < per - 1e-9 {
            marks.push(acc);
        }
    }
    marks.sort_by(|a, c| a.total_cmp(c));
    for k in 0..legs {
        let forward = k % 2 == 0;
        let ordered: Vec<f64> = if forward { marks.clone() } else { marks.iter().rev().copied().collect() };
        for m in ordered.into_iter().skip(1) {
            let p = at(m);
            let travelled = k as f64 * per + if forward { m } else { per - m };
            b.push(MoveKind::Cut, [p.x, p.y, (z_from - travelled * tan).max(z_to)], feed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(x: f64, y: f64, s: f64) -> Polyline {
        Polyline::new(vec![Pt::new(x, y), Pt::new(x + s, y), Pt::new(x + s, y + s), Pt::new(x, y + s)], true)
    }

    #[test]
    fn a_way_inside_the_region_is_only_one_that_stays_in_it_and_out_of_its_islands() {
        // A pocket 40 mm wide with an island of 10 mm in the middle.
        let region = vec![square(0.0, 0.0, 40.0), square(15.0, 15.0, 10.0)];
        assert!(way_inside(&region, Pt::new(2.0, 2.0), Pt::new(38.0, 2.0)), "along the wall");
        assert!(way_inside(&region, Pt::new(2.0, 2.0), Pt::new(2.0, 38.0)));
        assert!(!way_inside(&region, Pt::new(2.0, 20.0), Pt::new(38.0, 20.0)), "over the island");
        assert!(!way_inside(&region, Pt::new(2.0, 2.0), Pt::new(45.0, 2.0)), "out of the pocket");
        let wide = ops::offset(&region, 0.02);
        assert!(way_inside(&wide, Pt::new(0.0, 0.0), Pt::new(40.0, 0.0)), "the edge of a region a little wider counts as inside");
        assert!(!way_inside(&wide, Pt::new(0.0, 20.0), Pt::new(40.0, 20.0)), "the island is still in the way");
        assert!(way_inside(&region, Pt::new(5.0, 5.0), Pt::new(5.0, 5.0)), "no way at all");
    }
}
