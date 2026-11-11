//! Three operations for decorative work: **fluting** (a groove along a line that ramps in and out, deepest in the middle),
//! **moulding** (a profile such as a roundover or a cove swept along a line, cut in passes side by side) and **texture** (short
//! strokes of a ball nose over an area, like a surface carved by hand).
use crate::toolpath::{Builder, MoveKind};
use crate::tools::Tool;
use base_core::geom::{Polyline, Pt};
use serde::{Deserialize, Serialize};

/// Where a flute ramps.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum FluteRamp {
    #[default]
    Both,
    Start,
    End,
}

impl FluteRamp {
    pub const ALL: [FluteRamp; 3] = [FluteRamp::Both, FluteRamp::Start, FluteRamp::End];
    pub fn label(self) -> &'static str {
        match self {
            FluteRamp::Both => "At both ends",
            FluteRamp::Start => "At the start",
            FluteRamp::End => "At the end",
        }
    }
}

/// The cross-section of a moulding, across its width from the line (0) to the outer edge (1).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Profile {
    /// A quarter round that drops from the top at the line to the depth at the edge (a rounded-over edge).
    #[default]
    Roundover,
    /// A quarter round hollow (concave).
    Cove,
    /// An S curve: a cove into a roundover.
    Ogee,
    /// A straight slope to the depth.
    Chamfer,
    /// A half round groove of the width, centred on the line.
    HalfRound,
}

impl Profile {
    pub const ALL: [Profile; 5] = [Profile::Roundover, Profile::Cove, Profile::Ogee, Profile::Chamfer, Profile::HalfRound];
    pub fn label(self) -> &'static str {
        match self {
            Profile::Roundover => "Roundover",
            Profile::Cove => "Cove",
            Profile::Ogee => "Ogee",
            Profile::Chamfer => "Slope",
            Profile::HalfRound => "Half round groove",
        }
    }

    /// The depth (0 … 1 of the depth of the layer) at `u` (0 … 1 of the width).
    pub fn depth_at(self, u: f64) -> f64 {
        let u = u.clamp(0.0, 1.0);
        match self {
            Profile::Roundover => 1.0 - (1.0 - u * u).sqrt(),
            Profile::Cove => (1.0 - (1.0 - u).powi(2)).sqrt(),
            Profile::Ogee => {
                if u < 0.5 {
                    0.5 * (1.0 - (1.0 - (2.0 * u).powi(2)).sqrt())
                } else {
                    0.5 + 0.5 * (1.0 - (2.0 - 2.0 * u).powi(2)).sqrt()
                }
            }
            Profile::Chamfer => u,
            // Centred on the line: u runs from one edge (0) to the other (1).
            Profile::HalfRound => (1.0 - (2.0 * u - 1.0).powi(2)).max(0.0).sqrt(),
        }
    }
}

/// The settings of the three operations, kept with the layer.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Decor {
    /// Fluting: where it ramps and how long a ramp is (percent of the line).
    #[serde(default)]
    pub ramp: FluteRamp,
    #[serde(default = "twenty_five")]
    pub ramp_pct: f64,
    /// Moulding: the profile and its width (mm) to the left of the direction of the line (across it for a half round).
    #[serde(default)]
    pub profile: Profile,
    #[serde(default = "ten")]
    pub width: f64,
    /// Texture: the length of a stroke and the gap between strokes (mm), how much the depth and the length vary (0 … 1), and the
    /// number that makes the pattern (the same number, the same strokes).
    #[serde(default = "eight")]
    pub stroke: f64,
    #[serde(default = "two")]
    pub gap: f64,
    #[serde(default = "half")]
    pub vary: f64,
    #[serde(default)]
    pub seed: u32,
}

fn twenty_five() -> f64 {
    25.0
}
fn ten() -> f64 {
    10.0
}
fn eight() -> f64 {
    8.0
}
fn two() -> f64 {
    2.0
}
fn half() -> f64 {
    0.5
}

impl Default for Decor {
    fn default() -> Decor {
        Decor { ramp: FluteRamp::Both, ramp_pct: 25.0, profile: Profile::Roundover, width: 10.0, stroke: 8.0, gap: 2.0, vary: 0.5, seed: 0 }
    }
}

impl Decor {
    pub fn is_default(&self) -> bool {
        *self == Decor::default()
    }
    pub fn check(&self) -> Result<(), String> {
        let ok = |v: f64, lo: f64, hi: f64| v.is_finite() && (lo..=hi).contains(&v);
        if !ok(self.ramp_pct, 1.0, 50.0) || !ok(self.width, 0.1, 1000.0) || !ok(self.stroke, 0.5, 1000.0) || !ok(self.gap, 0.0, 1000.0) || !ok(self.vary, 0.0, 1.0) {
            return Err("a number of the fluting, moulding or texture is out of range".into());
        }
        Ok(())
    }
}

/// The points of a line, the closing point added for a closed one, and the length along it at every point.
fn along(line: &Polyline) -> (Vec<Pt>, Vec<f64>) {
    let mut pts = line.pts.clone();
    if line.closed && pts.len() > 2 {
        pts.push(pts[0]);
    }
    let mut s = vec![0.0];
    for w in pts.windows(2) {
        s.push(s.last().unwrap() + w[0].dist(w[1]));
    }
    (pts, s)
}

/// Out of a cut that ends at the surface: up at the plunge feed to just above the material, then away.
fn lift(b: &mut Builder) {
    if b.at[2] < 1.0 && b.at[2] > -1e-6 {
        let [x, y, _] = b.at;
        b.push(MoveKind::Plunge, [x, y, 1.0], b.plunge);
    }
    b.retract();
}

/// A flute along the line: down from the top over the ramp to the full depth, at the full depth, up again over the ramp, in
/// passes of the pass depth (every pass the same shape, scaled). Points are added every 0.5 mm on the ramps.
pub(crate) fn flute(b: &mut Builder, line: &Polyline, depths: &[f64], d: &Decor) {
    let (pts, s) = along(line);
    let total = *s.last().unwrap_or(&0.0);
    if pts.len() < 2 || total < 1e-6 {
        return;
    }
    let ramp = total * d.ramp_pct.clamp(1.0, 50.0) / 100.0;
    let (in_ramp, out_ramp) = match d.ramp {
        FluteRamp::Both => (ramp, ramp),
        FluteRamp::Start => (ramp, 0.0),
        FluteRamp::End => (0.0, ramp),
    };
    let share = |x: f64| -> f64 {
        let a = if in_ramp > 0.0 { (x / in_ramp).min(1.0) } else { 1.0 };
        let e = if out_ramp > 0.0 { ((total - x) / out_ramp).min(1.0) } else { 1.0 };
        a.min(e).max(0.0)
    };
    // The places along the line: its points and every 0.5 mm on the ramps.
    let mut at: Vec<f64> = s.clone();
    let mut x = 0.0;
    while x < total {
        if x < in_ramp || x > total - out_ramp {
            at.push(x);
        }
        x += 0.5;
    }
    at.push(total);
    // Where the ramps end: the full depth begins and ends there.
    at.push(in_ramp);
    at.push(total - out_ramp);
    at.sort_by(f64::total_cmp);
    at.dedup_by(|a, c| (*a - *c).abs() < 1e-9);
    let point = |x: f64| -> Pt {
        let i = s.partition_point(|v| *v < x).clamp(1, s.len() - 1);
        let (a, c) = (pts[i - 1], pts[i]);
        let l = s[i] - s[i - 1];
        let f = if l > 1e-12 { (x - s[i - 1]) / l } else { 0.0 };
        Pt::new(a.x + (c.x - a.x) * f, a.y + (c.y - a.y) * f)
    };
    for depth in depths {
        let first = point(at[0]);
        b.dive(first.x, first.y, -depth * share(at[0]));
        for &x in &at[1..] {
            let p = point(x);
            b.cut_to(p.x, p.y, -depth * share(x));
        }
        lift(b);
    }
}

/// A line moved sideways by `off` (to the left of its direction for a positive one), point by point along the mitred normals.
fn shifted(line: &Polyline, off: f64) -> Vec<Pt> {
    let pts = &line.pts;
    let n = pts.len();
    let closed = line.closed && n > 2;
    let dir = |a: Pt, c: Pt| {
        let l = a.dist(c).max(1e-12);
        Pt::new((c.x - a.x) / l, (c.y - a.y) / l)
    };
    let mut out = Vec::with_capacity(n + 1);
    for i in 0..n {
        let prev = if i > 0 { Some(dir(pts[i - 1], pts[i])) } else if closed { Some(dir(pts[n - 1], pts[0])) } else { None };
        let next = if i + 1 < n { Some(dir(pts[i], pts[i + 1])) } else if closed { Some(dir(pts[n - 1], pts[0])) } else { None };
        let (t, k) = match (prev, next) {
            (Some(a), Some(c)) => {
                let m = Pt::new(a.x + c.x, a.y + c.y);
                let l = (m.x * m.x + m.y * m.y).sqrt();
                if l < 1e-9 {
                    (a, 1.0)
                } else {
                    let m = Pt::new(m.x / l, m.y / l);
                    // The mitre: longer at a corner, but never more than 4 times the offset.
                    (m, (1.0 / (m.x * c.x + m.y * c.y).max(0.25)).min(4.0))
                }
            }
            (Some(a), None) | (None, Some(a)) => (a, 1.0),
            (None, None) => return pts.clone(),
        };
        out.push(Pt::new(pts[i].x - t.y * off * k, pts[i].y + t.x * off * k));
    }
    if closed {
        out.push(out[0]);
    }
    out
}

/// The depth (positive) of the tip of the bit at `u` across a profile `width` wide and `depth` deep, so that the body of the bit
/// does not cut below the profile anywhere (a drop cutter across the section).
fn tip_depth(p: Profile, u_mm: f64, width: f64, depth: f64, tool: &Tool) -> f64 {
    let r = tool.radius();
    let n = 24;
    let mut z = f64::INFINITY;
    for k in -n..=n {
        let dx = r * k as f64 / n as f64;
        let x = u_mm + dx;
        // Outside of the profile the material is uncut: at the top on the line side, at the depth beyond the edge (a moulding
        // cuts the edge away); a half round groove is uncut on both sides.
        let surface = if x < 0.0 {
            0.0
        } else if x > width {
            if p == Profile::HalfRound { 0.0 } else { depth }
        } else {
            depth * p.depth_at(x / width)
        };
        if let Some(up) = crate::relief::rise(tool, dx.abs()) {
            z = z.min(surface + up);
        }
    }
    z.max(0.0)
}

/// A moulding along the line: passes side by side across the width (the step over of the bit apart), each at the depth of the
/// profile there with the shape of the bit taken into account, and every pass in steps of the pass depth.
pub(crate) fn moulding(b: &mut Builder, line: &Polyline, depths: &[f64], d: &Decor, tool: &Tool, step: f64) {
    let width = d.width.max(0.1);
    let full = depths.last().copied().unwrap_or(0.0);
    let n = ((width / step.max(0.05)).ceil() as usize).clamp(1, 2000);
    // For a half round groove the width is centred on the line, otherwise it lies to the left of it.
    let from = if d.profile == Profile::HalfRound { -width / 2.0 } else { 0.0 };
    for k in 0..=n {
        let u = width * k as f64 / n as f64;
        let z = tip_depth(d.profile, u, width, full, tool);
        if z < 0.005 {
            continue;
        }
        let path = shifted(line, from + u);
        let Some(first) = path.first() else { continue };
        for depth in depths {
            let zz = z.min(*depth);
            b.dive(first.x, first.y, -zz);
            for p in &path[1..] {
                b.cut_to(p.x, p.y, -zz);
            }
            b.retract();
        }
    }
}

/// A small, repeatable random number from 0 to 1.
fn noise(seed: u32, i: u64) -> f64 {
    let mut x = (seed as u64) ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51_afd7_ed55_8ccd);
    x ^= x >> 33;
    (x >> 11) as f64 / (1u64 << 53) as f64
}

/// A texture over the inside of the rings: rows at the angle of the layer, the step over apart; along every row strokes of about
/// the stroke length with gaps between them, each a shallow scoop (deepest in its middle) of a depth that varies, the rows shifted
/// so the strokes do not line up. Only the inside of the rings is cut (a hole stays smooth).
pub(crate) fn texture(b: &mut Builder, rings: &[Polyline], depth: f64, d: &Decor, step: f64, angle: f64) {
    let lines = crate::clear::raster_paths(rings, angle, step.max(0.1));
    let mut i: u64 = 0;
    for line in lines {
        let (pts, s) = along(&line);
        let total = *s.last().unwrap_or(&0.0);
        if total < 1e-6 {
            continue;
        }
        let point = |x: f64| -> Pt {
            let j = s.partition_point(|v| *v < x).clamp(1, s.len() - 1);
            let (a, c) = (pts[j - 1], pts[j]);
            let l = s[j] - s[j - 1];
            let f = if l > 1e-12 { (x - s[j - 1]) / l } else { 0.0 };
            Pt::new(a.x + (c.x - a.x) * f, a.y + (c.y - a.y) * f)
        };
        let mut x = noise(d.seed, i) * (d.stroke + d.gap);
        i += 1;
        while x < total {
            let len = (d.stroke * (1.0 - d.vary * 0.5 + d.vary * noise(d.seed, i))).max(0.5);
            let deep = depth * (1.0 - d.vary * 0.6 * noise(d.seed, i + 7));
            i += 1;
            let end = (x + len).min(total);
            let n = (((end - x) / 0.5).ceil() as usize).max(2);
            for k in 0..=n {
                let t = k as f64 / n as f64;
                let p = point(x + (end - x) * t);
                // A scoop: in and out of the material along a half sine.
                let z = -deep * (std::f64::consts::PI * t).sin();
                if k == 0 {
                    b.dive(p.x, p.y, z.min(-0.001));
                } else {
                    b.push(MoveKind::Cut, [p.x, p.y, z.min(0.0)], b.feed);
                }
            }
            lift(b);
            x = end + d.gap.max(0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_profiles_go_from_the_top_to_the_depth() {
        for p in [Profile::Roundover, Profile::Cove, Profile::Ogee, Profile::Chamfer] {
            assert!(p.depth_at(0.0).abs() < 1e-9 && (p.depth_at(1.0) - 1.0).abs() < 1e-9, "{p:?}");
            assert!(p.depth_at(0.5) > 0.0 && p.depth_at(0.5) < 1.0);
        }
        assert!(Profile::Roundover.depth_at(0.5) < 0.5 && Profile::Cove.depth_at(0.5) > 0.5, "the roundover is convex, the cove hollow");
        assert!((Profile::HalfRound.depth_at(0.5) - 1.0).abs() < 1e-9 && Profile::HalfRound.depth_at(0.0).abs() < 1e-9);
        assert!(Decor::default().check().is_ok());
        assert!(Decor { vary: 2.0, ..Decor::default() }.check().is_err());
        let d: Decor = serde_json::from_str("{}").unwrap();
        assert!(d.is_default());
    }

    #[test]
    fn a_line_is_moved_to_its_left() {
        let l = Polyline::new(vec![Pt::new(0.0, 0.0), Pt::new(10.0, 0.0)], false);
        let s = shifted(&l, 2.0);
        assert!(s[0].dist(Pt::new(0.0, 2.0)) < 1e-9 && s[1].dist(Pt::new(10.0, 2.0)) < 1e-9, "{s:?}");
        let sq = Polyline::new(vec![Pt::new(0.0, 0.0), Pt::new(10.0, 0.0), Pt::new(10.0, 10.0), Pt::new(0.0, 10.0)], true);
        let s = shifted(&sq, 1.0);
        assert_eq!(s.len(), 5);
        assert!(s[1].dist(Pt::new(9.0, 1.0)) < 1e-9, "inside an anticlockwise square: {:?}", s[1]);
    }

    #[test]
    fn the_noise_repeats_for_the_same_seed() {
        assert_eq!(noise(3, 10), noise(3, 10));
        assert_ne!(noise(3, 10), noise(4, 10));
        assert!((0..1000).all(|i| (0.0..1.0).contains(&noise(1, i))));
    }
}
