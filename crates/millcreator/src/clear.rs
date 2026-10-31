//! The geometry of the 2.5D operations beyond rings: a raster of lines that clears a region, the region of a rest pocket and of a face,
//! the notches of a dogbone and the loops of a trochoid. Everything here is in the coordinates of the toolpaths (the work area, y up).
use crate::tools::{Tool, ToolKind};
use base_core::geom::{Polyline, Pt, Rect};
use base_core::ops::{self, BoolOp};

/// Is `p` inside the region (an odd number of its rings round it)?
pub fn inside(region: &[Polyline], p: Pt) -> bool {
    region.iter().filter(|r| r.closed && r.contains(p)).count() % 2 == 1
}

/// Is `p` inside the region or on its edge (within a hair)?
fn inside_or_on(region: &[Polyline], p: Pt) -> bool {
    if inside(region, p) {
        return true;
    }
    region.iter().any(|r| {
        let n = r.pts.len();
        (0..n).any(|i| {
            let (a, b) = (r.pts[i], r.pts[(i + 1) % n]);
            let (dx, dy) = (b.x - a.x, b.y - a.y);
            let l2 = dx * dx + dy * dy;
            let t = if l2 > 1e-12 { (((p.x - a.x) * dx + (p.y - a.y) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
            Pt::new(a.x + dx * t, a.y + dy * t).dist(p) < 1e-3
        })
    })
}

/// Parallel lines `step` mm apart at `angle` degrees over the region (even-odd, so holes are left), each as the pieces that lie inside it,
/// joined into paths that go back and forth: a piece whose start is reached from the end of the last one by a straight way that stays
/// inside the region continues the path; otherwise a new path begins (the bit retracts between them).
pub fn raster_paths(region: &[Polyline], angle_deg: f64, step: f64) -> Vec<Polyline> {
    let step = step.max(0.01);
    let a = angle_deg.to_radians();
    let (u, n) = ((a.cos(), a.sin()), (-a.sin(), a.cos()));
    let along = |p: Pt| p.x * u.0 + p.y * u.1;
    let across = |p: Pt| p.x * n.0 + p.y * n.1;
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for r in region {
        for p in &r.pts {
            lo = lo.min(across(*p));
            hi = hi.max(across(*p));
        }
    }
    if !lo.is_finite() || hi - lo < 1e-6 {
        return vec![];
    }
    // The lines start a hair inside the edge so that a line along an edge of the region is not lost, and the last one is always there.
    let eps = 1e-4_f64.min((hi - lo) / 4.0);
    let count = (((hi - lo - 2.0 * eps) / step).ceil() as usize).max(1);
    let span = hi - lo - 2.0 * eps;
    // For every line: its offset and the pieces of it inside the region, as (start, end) along the line.
    let mut lines: Vec<(f64, Vec<(f64, f64)>)> = vec![];
    for k in 0..=count {
        let t = lo + eps + span * (k as f64 / count as f64);
        let mut cuts: Vec<f64> = vec![];
        for r in region.iter().filter(|r| r.closed) {
            let m = r.pts.len();
            for i in 0..m {
                let (p, q) = (r.pts[i], r.pts[(i + 1) % m]);
                let (cp, cq) = (across(p), across(q));
                if (cp < t) != (cq < t) {
                    let f = (t - cp) / (cq - cp);
                    cuts.push(along(p) + (along(q) - along(p)) * f);
                }
            }
        }
        cuts.sort_by(|x, y| x.total_cmp(y));
        lines.push((t, cuts.chunks_exact(2).filter(|c| c[1] - c[0] > 1e-6).map(|c| (c[0], c[1])).collect()));
    }
    let at = |s: f64, t: f64| Pt::new(s * u.0 + t * n.0, s * u.1 + t * n.1);
    let mut paths: Vec<Polyline> = vec![];
    let mut cur: Vec<Pt> = vec![];
    for (k, (t, pieces)) in lines.iter().enumerate() {
        let t = *t;
        let ordered: Vec<(f64, f64)> = if k % 2 == 0 { pieces.clone() } else { pieces.iter().rev().map(|(a, b)| (*b, *a)).collect() };
        for (s0, s1) in ordered {
            let (p0, p1) = (at(s0, t), at(s1, t));
            let joined = cur.last().is_some_and(|l| {
                // The straight way from the end of the path to the start of this piece stays inside the region.
                l.dist(p0) < 4.0 * step + 1e-9 && (1..8).all(|i| inside_or_on(region, Pt::new(l.x + (p0.x - l.x) * i as f64 / 8.0, l.y + (p0.y - l.y) * i as f64 / 8.0)))
            });
            if !joined && !cur.is_empty() {
                paths.push(Polyline::new(std::mem::take(&mut cur), false));
            }
            cur.push(p0);
            cur.push(p1);
        }
    }
    if cur.len() >= 2 {
        paths.push(Polyline::new(cur, false));
    }
    paths
}

/// The rings of a pocket with the strip in the middle that the step over leaves when the width is not a multiple of it: after the last
/// ring a ring that goes along the middle of what is left.
pub fn rings_with_middle(region: &[Polyline], step: f64) -> Vec<Polyline> {
    let mut rings = ops::inset_rings(region, step, 500);
    // The rings of the last step: found by going in again until the next step would leave nothing.
    let mut cur = ops::offset(region, 0.0);
    let mut innermost = cur.clone();
    for _ in 0..500 {
        if cur.is_empty() {
            break;
        }
        innermost = cur.clone();
        cur = ops::offset(&cur, -step);
        if cur.iter().all(|p| p.area().abs() < 1e-4) {
            break;
        }
    }
    // A ring at the middle of what is left: as far in as it still fits (half the step, or less).
    let mut d = step / 2.0;
    while d >= 0.02 {
        let mid = ops::offset(&innermost, -d);
        if mid.iter().any(|p| p.area().abs() > 1e-4) {
            rings.extend(mid);
            break;
        }
        d /= 2.0;
    }
    rings
}

/// The region the centre of the bit moves in to plane the box `b` with a bit of radius `r`, the edge of the bit going `overhang` mm
/// over the edge of the box.
pub fn face_region(b: Rect, overhang: f64, r: f64) -> Vec<Polyline> {
    let rect = Polyline::new(vec![Pt::new(b.min.x, b.min.y), Pt::new(b.max.x, b.min.y), Pt::new(b.max.x, b.max.y), Pt::new(b.min.x, b.max.y)], true);
    let grow = overhang - r;
    ops::offset(&[rect], grow)
}

/// The region for the centre of a small bit (radius `r_small`) that clears what a bigger one (radius `r_big`) leaves in the pocket
/// `closed`: the places where the small bit reaches what the big one could not.
pub fn rest_region(closed: &[Polyline], r_big: f64, r_small: f64) -> Vec<Polyline> {
    if r_small >= r_big {
        return vec![];
    }
    // What the big bit reaches: the pocket opened by its radius.
    let reached = ops::offset(&ops::offset(closed, -r_big), r_big);
    let left = ops::boolean(closed, &reached, BoolOp::Difference);
    if left.is_empty() {
        return vec![];
    }
    let centres = ops::offset(closed, -r_small);
    let near = ops::offset(&left, r_small);
    ops::boolean(&centres, &near, BoolOp::Intersect)
}

/// How deep the V-bit goes for a chamfer that is `width` mm wide on the surface when the bit follows the edge, and `None` when the bit
/// is not a pointed one.
pub fn chamfer_depth(tool: &Tool, width: f64) -> Option<f64> {
    if !matches!(tool.kind, ToolKind::VBit | ToolKind::Engraver) {
        return None;
    }
    let half = (tool.dims.angle / 2.0).clamp(5.0, 89.0).to_radians();
    Some(((width - tool.dims.tip.max(0.0) / 2.0).max(0.0)) / half.tan())
}

/// The ring with a notch at each sharp corner: the bit goes on from the corner of the ring towards the corner of the shape until its
/// edge reaches it, and back (for a ring that is the path of the centre of a bit of radius `r` inside the shape). Rings that are holes
/// (`hole`) are left alone.
pub fn dogbone(ring: &Polyline, r: f64, hole: bool) -> Polyline {
    let n = ring.pts.len();
    if hole || n < 3 || r <= 0.0 {
        return ring.clone();
    }
    let sign = ring.area().signum();
    let mut pts = vec![];
    for i in 0..n {
        let (a, p, b) = (ring.pts[(i + n - 1) % n], ring.pts[i], ring.pts[(i + 1) % n]);
        pts.push(p);
        let (v1, v2) = ((p.x - a.x, p.y - a.y), (b.x - p.x, b.y - p.y));
        let (l1, l2) = (v1.0.hypot(v1.1), v2.0.hypot(v2.1));
        if l1 < 1e-9 || l2 < 1e-9 {
            continue;
        }
        let cross = v1.0 * v2.1 - v1.1 * v2.0;
        let dot = v1.0 * v2.0 + v1.1 * v2.1;
        let turn = cross.atan2(dot).abs();
        // A corner of the ring (not a bend of a curve): the ring turns by more than 30 degrees, the way the ring goes round.
        if turn < 30f64.to_radians() || cross * sign <= 0.0 {
            continue;
        }
        // The interior angle at the corner is pi - turn; the corner of the shape is r / sin(half of it) from the ring corner, outwards.
        let half = (std::f64::consts::PI - turn) / 2.0;
        let out = {
            // The outward bisector: opposite to the sum of the unit directions that point along the sides into the ring.
            let (e1, e2) = ((-v1.0 / l1, -v1.1 / l1), (v2.0 / l2, v2.1 / l2));
            let (sx, sy) = (e1.0 + e2.0, e1.1 + e2.1);
            let sl = sx.hypot(sy).max(1e-12);
            (-sx / sl, -sy / sl)
        };
        let reach = r / half.sin() - r;
        if reach > 1e-6 {
            let t = Pt::new(p.x + out.0 * reach, p.y + out.1 * reach);
            pts.push(t);
            pts.push(p);
        }
    }
    Polyline::new(pts, true)
}

/// The loops of a trochoid along `line` (open or closed): circles of `diameter` (or half circles swinging across the line) that move
/// on by `step` with every turn. The result is an open path that starts at the side of the line.
pub fn trochoid(line: &Polyline, diameter: f64, step: f64, half: bool) -> Polyline {
    let r = diameter / 2.0;
    let step = step.max(0.05);
    let mut pts: Vec<Pt> = line.pts.clone();
    if line.closed && pts.len() > 1 {
        pts.push(pts[0]);
    }
    let seg: Vec<f64> = pts.windows(2).map(|w| w[0].dist(w[1])).collect();
    let total: f64 = seg.iter().sum();
    if total < 1e-9 || r <= 0.0 {
        return line.clone();
    }
    let per_turn = 16.0;
    let ds = step / per_turn;
    let n = (total / ds).ceil() as usize;
    let at = |s: f64| -> (Pt, (f64, f64)) {
        let mut acc = 0.0;
        for (i, l) in seg.iter().enumerate() {
            if s <= acc + l + 1e-12 || i == seg.len() - 1 {
                let t = if *l > 1e-12 { ((s - acc) / l).clamp(0.0, 1.0) } else { 0.0 };
                let (a, b) = (pts[i], pts[i + 1]);
                let dir = if *l > 1e-12 { ((b.x - a.x) / l, (b.y - a.y) / l) } else { (1.0, 0.0) };
                return (Pt::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t), dir);
            }
            acc += l;
        }
        (pts[pts.len() - 1], (1.0, 0.0))
    };
    let mut out = Vec::with_capacity(n + 1);
    for k in 0..=n {
        let s = (k as f64 * ds).min(total);
        let (c, t) = at(s);
        let nrm = (-t.1, t.0);
        let phi = std::f64::consts::TAU * s / step;
        let (side, fwd) = if half { (phi.cos() * r, 0.0) } else { (phi.cos() * r, phi.sin() * r) };
        out.push(Pt::new(c.x + nrm.0 * side + t.0 * fwd, c.y + nrm.1 * side + t.1 * fwd));
    }
    Polyline::new(out, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Polyline {
        Polyline::new(vec![Pt::new(x0, y0), Pt::new(x1, y0), Pt::new(x1, y1), Pt::new(x0, y1)], true)
    }

    fn length(paths: &[Polyline]) -> f64 {
        paths.iter().map(|p| p.length()).sum()
    }

    #[test]
    fn a_raster_covers_the_region_line_by_line_and_stays_inside_it() {
        let region = [rect(0.0, 0.0, 20.0, 10.0)];
        let paths = raster_paths(&region, 0.0, 2.0);
        // Lines along x at y = 0.0001 … 9.9999: six of them, each 20 long.
        let mut ys: Vec<f64> = paths.iter().flat_map(|p| p.pts.iter().map(|q| (q.y * 10.0).round() / 10.0)).collect();
        ys.sort_by(|a, b| a.total_cmp(b));
        ys.dedup();
        assert!(ys.first().unwrap() <= &0.1 && ys.last().unwrap() >= &9.9, "{ys:?}");
        assert!(paths.iter().flat_map(|p| &p.pts).all(|p| p.x >= -1e-9 && p.x <= 20.0 + 1e-9 && p.y >= 0.0 && p.y <= 10.0));
        // The links are short and inside: the lines are joined into one path.
        assert_eq!(paths.len(), 1, "{}", paths.len());
        assert!(length(&paths) > 20.0 * 5.0 && length(&paths) < 20.0 * 6.0 + 6.0 * 2.0 + 1.0, "{}", length(&paths));
        // A hole leaves its own area alone: no point of a line lies in it.
        let holed = [rect(0.0, 0.0, 20.0, 10.0), rect(8.0, 3.0, 12.0, 7.0)];
        let paths = raster_paths(&holed, 0.0, 1.0);
        let inside_hole = paths.iter().any(|p| p.pts.windows(2).any(|w| { let m = Pt::new((w[0].x + w[1].x) / 2.0, (w[0].y + w[1].y) / 2.0); m.x > 8.1 && m.x < 11.9 && m.y > 3.1 && m.y < 6.9 }));
        assert!(!inside_hole);
        // At an angle the lines run along it.
        let tilted = raster_paths(&region, 90.0, 5.0);
        assert!(tilted.iter().any(|p| p.pts.windows(2).any(|w| (w[0].x - w[1].x).abs() < 1e-6 && (w[0].y - w[1].y).abs() > 5.0)));
    }

    #[test]
    fn the_strip_that_the_step_over_leaves_in_the_middle_gets_a_ring_of_its_own() {
        // 10 mm wide, rings 4 mm apart: 0, 4 leave 2 mm in the middle (no ring at 8); the middle ring is added.
        let region = [rect(0.0, 0.0, 10.0, 30.0)];
        let plain = ops::inset_rings(&region, 4.0, 500);
        let fixed = rings_with_middle(&region, 4.0);
        assert!(fixed.len() > plain.len(), "{} vs {}", fixed.len(), plain.len());
        let middle = fixed.last().unwrap();
        let b = middle.bounds().unwrap();
        assert!((b.min.x - 5.0).abs() < 1.5 && (b.max.x - 5.0).abs() < 1.5, "{b:?}");
        // A width that is a multiple of the step has nothing to add that is not already there or empty.
        let exact = rings_with_middle(&[rect(0.0, 0.0, 8.0, 30.0)], 4.0);
        assert!(exact.len() <= ops::inset_rings(&[rect(0.0, 0.0, 8.0, 30.0)], 4.0, 500).len() + 1);
    }

    #[test]
    fn a_face_goes_over_the_edge_and_a_rest_pocket_is_what_the_big_bit_leaves() {
        let b = Rect { min: Pt::new(0.0, 0.0), max: Pt::new(50.0, 30.0) };
        // Edge of the bit 1 mm over the edge: the centre is r - 1 inside the edge for r = 3 … no: the centre is 1 - r = -2 away: 2 mm inside.
        let f = face_region(b, 1.0, 3.0);
        let fb = f[0].bounds().unwrap();
        assert!((fb.min.x - 2.0).abs() < 1e-6 && (fb.max.x - 48.0).abs() < 1e-6, "{fb:?}");
        // A bigger overhang goes outside the box.
        let g = face_region(b, 5.0, 3.0)[0].bounds().unwrap();
        assert!(g.min.x < 0.0);
        // A rest pocket: a 30 × 30 square, a bit of radius 5 leaves the corners; a bit of radius 1 clears only near them.
        let sq = [rect(0.0, 0.0, 30.0, 30.0)];
        let rest = rest_region(&sq, 5.0, 1.0);
        assert!(!rest.is_empty());
        let rb = rest.iter().filter_map(|p| p.bounds()).fold(None, |a: Option<Rect>, b| Some(a.map_or(b, |a| a.union(b)))).unwrap();
        assert!(rb.min.x >= 1.0 - 1e-6 && rb.max.x <= 29.0 + 1e-6, "{rb:?}");
        // Not in the middle of the square.
        assert!(!inside(&rest, Pt::new(15.0, 15.0)));
        assert!(inside(&rest, Pt::new(1.2, 1.2)));
        // A bit that is not smaller leaves nothing to do.
        assert!(rest_region(&sq, 1.0, 5.0).is_empty());
    }

    #[test]
    fn a_chamfer_is_as_deep_as_its_width_asks_for_a_v_bit_and_nothing_for_other_bits() {
        let v = Tool::new("v", "V", ToolKind::VBit, crate::tools::Dims { diameter: 10.0, angle: 90.0, tip: 0.0, ..Default::default() });
        // A 90° V-bit: the chamfer is as deep as it is wide.
        assert!((chamfer_depth(&v, 2.0).unwrap() - 2.0).abs() < 1e-9);
        let v60 = Tool::new("v", "V", ToolKind::VBit, crate::tools::Dims { diameter: 10.0, angle: 60.0, tip: 0.0, ..Default::default() });
        assert!((chamfer_depth(&v60, 1.0).unwrap() - 1.0 / 30f64.to_radians().tan()).abs() < 1e-9);
        let flat = Tool::new("f", "F", ToolKind::FlatEnd, crate::tools::Dims::default());
        assert!(chamfer_depth(&flat, 1.0).is_none());
    }

    #[test]
    fn a_dogbone_goes_on_into_every_corner_until_the_edge_of_the_bit_reaches_it() {
        // The path of the centre of a bit of radius 2 in a 20 × 10 slot: the ring is 16 × 6, its corners 2 mm from the walls both ways.
        let ring = rect(2.0, 2.0, 18.0, 8.0);
        let bone = dogbone(&ring, 2.0, false);
        assert_eq!(bone.pts.len(), 4 * 3, "a detour (out and back) at each of the four corners");
        // Towards the corner (0, 0): the centre goes 2 * (sqrt 2 - 1) further along the diagonal, so its edge reaches the corner.
        let reach = bone.pts.iter().map(|p| p.dist(Pt::new(0.0, 0.0))).fold(f64::INFINITY, f64::min);
        assert!((reach - 2.0).abs() < 1e-6, "the centre is a radius from the corner: {reach}");
        // A hole (an island) and a ring without corners are left alone.
        assert_eq!(dogbone(&ring, 2.0, true).pts.len(), 4);
        let round = Polyline::new((0..64).map(|i| { let a = i as f64 / 64.0 * std::f64::consts::TAU; Pt::new(10.0 * a.cos(), 10.0 * a.sin()) }).collect(), true);
        assert_eq!(dogbone(&round, 2.0, false).pts.len(), 64);
    }

    #[test]
    fn a_trochoid_goes_in_loops_along_the_line_and_stays_within_the_width_of_the_slot() {
        let line = Polyline::new(vec![Pt::new(0.0, 0.0), Pt::new(40.0, 0.0)], false);
        let t = trochoid(&line, 4.0, 2.0, false);
        assert!(t.pts.len() > 40 * 8, "{}", t.pts.len());
        let b = t.bounds().unwrap();
        assert!(b.min.y >= -2.0 - 1e-9 && b.max.y <= 2.0 + 1e-9 && b.max.x >= 39.0, "{b:?}");
        // It really loops: the x of the points goes back.
        assert!(t.pts.windows(2).any(|w| w[1].x < w[0].x - 0.05));
        // Half circles only swing across the line: x never goes back.
        let h = trochoid(&line, 4.0, 2.0, true);
        assert!(h.pts.windows(2).all(|w| w[1].x >= w[0].x - 1e-9));
        assert!(h.bounds().unwrap().max.y > 1.9);
    }
}

/// The rings of a V-carve, each with the depth the tip of the bit goes to (positive, below the surface).
///
/// A V-bit with its tip `b` mm from the edge of the shape at the depth `(b - tip/2) / tan(half angle)` just touches the edge at the
/// surface, and the cone of every such ring cuts the sloping wall exactly, so the walls come out true whatever the distance of the
/// rings (`step`). Beyond the depth `depth` the bottom is flat: rings go on at that depth, `floor_step` apart, which is rough with a
/// pointed bit (a flat bit is better for that). Nothing is cut shallower than `start`. With `outside` the rings go round the shape
/// outwards (the wall of a plug) and stop where the wall reaches `depth`. `None` when the bit has no slope.
pub fn vcarve_rings(region: &[Polyline], tool: &Tool, depth: f64, start: f64, step: f64, floor_step: f64, outside: bool) -> Option<Vec<(f64, Polyline)>> {
    if !matches!(tool.kind, ToolKind::VBit | ToolKind::Engraver) {
        return None;
    }
    let tan = (tool.dims.angle / 2.0).clamp(5.0, 89.0).to_radians().tan();
    let tip = tool.dims.tip.max(0.0) / 2.0;
    let b_of = |z: f64| tip + z * tan;
    let alive = |rings: &[Polyline]| rings.iter().any(|p| p.area().abs() > 1e-4);
    let at = |b: f64| if outside { ops::offset(region, b) } else { ops::offset(region, -b) };
    let step = step.max(0.02);
    let b_start = b_of(start.max(0.0));
    let b_max = b_of(depth);
    let mut out: Vec<(f64, Polyline)> = vec![];
    let mut last = b_start;
    let put = |b: f64, z: f64, last: &mut f64, out: &mut Vec<(f64, Polyline)>| -> bool {
        let rings = at(b);
        if !alive(&rings) {
            return false;
        }
        out.extend(rings.into_iter().filter(|p| p.area().abs() > 1e-4).map(|p| (z, p)));
        *last = b;
        true
    };
    // The walls.
    let mut k = 1;
    let mut ended = false;
    while k < 500 {
        let b = b_start + k as f64 * step;
        if b >= b_max {
            break;
        }
        if !put(b, (b - tip) / tan, &mut last, &mut out) {
            ended = true;
            break;
        }
        k += 1;
    }
    if !ended && b_max > last + 1e-6 {
        // The last ring of the wall is at the full depth.
        if put(b_max, depth, &mut last, &mut out) {
            // The flat bottom.
            if !outside {
                let mut j = 1;
                while j < 500 {
                    if !put(b_max + j as f64 * floor_step.max(0.02), depth, &mut last, &mut out) {
                        break;
                    }
                    j += 1;
                }
                ended = true;
            }
        } else {
            ended = true;
        }
    }
    // The crest: a ring as far in as the shape still reaches (found by halving), as deep as the distance from the edge makes it.
    if ended && !outside {
        let (mut lo, mut hi) = (last, last + step);
        for _ in 0..14 {
            let mid = (lo + hi) / 2.0;
            if alive(&at(mid)) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        if lo > last + 1e-4 {
            put(lo, ((lo - tip) / tan).min(depth), &mut last, &mut out);
        }
    }
    Some(out)
}
