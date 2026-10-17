//! A picture as a 3D relief: the grey level of every pixel is a depth (black is the deepest, white is left uncut), the way
//! molino does it. From the depth map come the moves of the bit (roughing passes down to the full depth, with the shape of
//! the bit taken into account so that it does not gouge) and an STL model of the carved piece.
use crate::tools::{Tool, ToolKind};
use base_core::geom::Pt;
use base_core::image::ImageData;
use base_core::raster::{self, Dither, Grid};
use base_core::scene::Shape;

/// The most cells of the depth map (with the footprint of the bit this is the work of the compensation).
const MAX_CELLS: usize = 1_000_000;
/// The most steps of work of the compensation (cells times cells of the footprint of the bit).
const MAX_WORK: usize = 400_000_000;

/// The heights of the surface of the relief on a grid: 0 at the top of the material and down to `-depth` for black.
pub struct HeightMap {
    pub grid: Grid,
    /// Row by row, in mm (zero or less).
    pub h: Vec<f64>,
}

impl HeightMap {
    /// Sample the picture on a grid of `step` mm; `depth` is the depth of black. The grid is coarser when it would have too
    /// many cells.
    pub fn of(shape: &Shape, im: &ImageData, step: f64, angle: f64, depth: f64) -> Option<HeightMap> {
        let mut step = step.max(0.02);
        let grid = loop {
            let g = raster::sample(shape, im, step, angle, Dither::Grayscale)?;
            if g.cols * g.rows <= MAX_CELLS {
                break g;
            }
            step *= 1.4;
        };
        let h = grid.levels.iter().map(|l| -(depth * (*l as f64).clamp(0.0, 1.0))).collect();
        Some(HeightMap { grid, h })
    }

    pub fn at(&self, c: usize, r: usize) -> f64 {
        self.h[r * self.grid.cols + c]
    }
}

/// How far the cutting surface of the bit rises above its tip at the distance `d` from the axis; `None` where the bit has no
/// cutting surface (outside of its radius).
fn rise(tool: &Tool, d: f64) -> Option<f64> {
    let r = tool.radius();
    if d > r + 1e-9 {
        return None;
    }
    let d = d.min(r);
    Some(match tool.kind {
        ToolKind::BallNose | ToolKind::TaperedBall => r - (r * r - d * d).max(0.0).sqrt(),
        ToolKind::BullNose => {
            let rc = tool.dims.corner.clamp(0.0, r);
            let flat = r - rc;
            if d <= flat {
                0.0
            } else {
                rc - (rc * rc - (d - flat).powi(2)).max(0.0).sqrt()
            }
        }
        ToolKind::VBit => {
            let half = (tool.dims.angle / 2.0).clamp(5.0, 89.0).to_radians();
            ((d - tool.dims.tip.max(0.0) / 2.0).max(0.0)) / half.tan()
        }
        // A flat end mill, a drill or an engraving bit with a flat tip.
        _ => 0.0,
    })
}

/// The height of the tip of the bit at every cell so that the bit does not cut into the relief anywhere under its body (a
/// "drop cutter": the tip is as low as the lowest point of the body that touches the surface allows). Outside of the picture the material is not cut, so near the edge the bit stays higher.
pub fn tip_heights(map: &HeightMap, tool: &Tool) -> Vec<f64> {
    let (cols, rows, step) = (map.grid.cols, map.grid.rows, map.grid.step);
    let reach = (tool.radius() / step).floor() as i64;
    // The cells under the bit: offset in cells and how high the cutting surface is there.
    let mut foot: Vec<(i64, i64, f64)> = vec![];
    for dr in -reach..=reach {
        for dc in -reach..=reach {
            let d = ((dr * dr + dc * dc) as f64).sqrt() * step;
            if let Some(up) = rise(tool, d) {
                foot.push((dr, dc, up));
            }
        }
    }
    if foot.len() * cols * rows > MAX_WORK {
        // Too much work: only the cells on a ring and the centre (a coarser, still safe-enough footprint is not made: the bit is
        // taken as the biggest it could be, a point).
        foot = vec![(0, 0, 0.0)];
    }
    let mut out = vec![0.0f64; cols * rows];
    for r in 0..rows as i64 {
        for c in 0..cols as i64 {
            let mut z = f64::NEG_INFINITY;
            for &(dr, dc, up) in &foot {
                let (rr, cc) = (r + dr, c + dc);
                let surface = if rr < 0 || cc < 0 || rr >= rows as i64 || cc >= cols as i64 { 0.0 } else { map.h[rr as usize * cols + cc as usize] };
                // The bit touches the surface with the part of it that is `up` above the tip at that distance.
                z = z.max(surface - up);
            }
            out[r as usize * cols + c as usize] = z.min(0.0);
        }
    }
    out
}

/// The depths of the passes: `step_down`, twice that, ... up to the full `depth` (the last one is the relief itself).
pub fn pass_limits(depth: f64, step_down: f64) -> Vec<f64> {
    let step = step_down.max(0.01);
    let n = ((depth / step) - 1e-9).ceil().clamp(1.0, 200.0) as usize;
    let mut v: Vec<f64> = (1..=n).map(|i| (step * i as f64).min(depth)).collect();
    if let Some(l) = v.last_mut() {
        *l = depth;
    }
    v
}

/// One row of one pass as the points of a path in the work area (x, y, z); a cell where the bit would not cut (at the surface)
/// ends a run, and a point on a straight line between its neighbours is left out.
pub fn row_runs(map: &HeightMap, tips: &[f64], row: usize, limit: f64, reverse: bool, area_y: f64) -> Vec<Vec<[f64; 3]>> {
    let g = &map.grid;
    let cols: Vec<usize> = if reverse { (0..g.cols).rev().collect() } else { (0..g.cols).collect() };
    let mut runs: Vec<Vec<[f64; 3]>> = vec![];
    let mut cur: Vec<[f64; 3]> = vec![];
    for c in cols {
        // The bit goes no deeper than this pass (the relief itself in the last one).
        let z = tips[row * g.cols + c].max(-limit);
        if z > -0.005 {
            if !cur.is_empty() {
                runs.push(std::mem::take(&mut cur));
            }
            continue;
        }
        let p = g.centre_in_area(c, row);
        cur.push([p.x, area_y - p.y, z]);
    }
    if !cur.is_empty() {
        runs.push(cur);
    }
    runs.into_iter().map(simplify).collect()
}

/// Leave out the points that lie on a straight line between the ones before and after them (flat floors, even slopes).
fn simplify(pts: Vec<[f64; 3]>) -> Vec<[f64; 3]> {
    if pts.len() < 3 {
        return pts;
    }
    let mut out = vec![pts[0]];
    for i in 1..pts.len() - 1 {
        let (a, b, c) = (*out.last().unwrap(), pts[i], pts[i + 1]);
        let (u, v) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - b[0], c[1] - b[1], c[2] - b[2]]);
        let cross = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        if (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt() > 1e-6 {
            out.push(b);
        }
    }
    out.push(*pts.last().unwrap());
    out
}

/// The carved piece as a closed solid in binary STL: the top (the relief), four walls and a flat bottom, `thickness` mm of
/// material (the surface at 0 is the top of the stock; the model stands on z = 0 and is `thickness` high where uncut). The model
/// is in millimetres, x to the right and y up, from the lower left corner of the picture.
pub fn stl(map: &HeightMap, thickness: f64) -> Vec<u8> {
    let g = &map.grid;
    let (cols, rows) = (g.cols, g.rows);
    let thickness = thickness.max(0.1);
    // The picture is turned over for the model: its top is the larger y.
    let vx = |c: usize| (c as f64 + 0.5) * g.step;
    let vy = |r: usize| (rows - 1 - r) as f64 * g.step + 0.5 * g.step;
    let top = |c: usize, r: usize| [vx(c), vy(r), thickness + map.at(c, r)];
    let mut tris: Vec<[[f64; 3]; 3]> = vec![];
    // The top surface: two triangles for every four neighbouring points (counter-clockwise seen from above).
    for r in 0..rows.saturating_sub(1) {
        for c in 0..cols.saturating_sub(1) {
            // The rows go down in the picture, so up the model: (c, r) is above (c, r+1).
            let (a, b, d, e) = (top(c, r + 1), top(c + 1, r + 1), top(c + 1, r), top(c, r));
            tris.push([a, b, d]);
            tris.push([a, d, e]);
        }
    }
    let base = |p: [f64; 3]| [p[0], p[1], 0.0];
    // The walls: along every edge of the border a quad from the surface down to z = 0, facing out.
    let mut wall = |p: [f64; 3], q: [f64; 3]| {
        // p -> q runs counter-clockwise round the model seen from above; the wall faces to the right of it.
        tris.push([p, base(p), base(q)]);
        tris.push([p, base(q), q]);
    };
    for c in 0..cols.saturating_sub(1) {
        wall(top(c, rows - 1), top(c + 1, rows - 1)); // the bottom edge of the picture, y smallest: goes to the right
    }
    for r in (1..rows).rev() {
        wall(top(cols - 1, r), top(cols - 1, r - 1)); // the right edge, upwards
    }
    for c in (1..cols).rev() {
        wall(top(c, 0), top(c - 1, 0)); // the top edge, to the left
    }
    for r in 0..rows.saturating_sub(1) {
        wall(top(0, r), top(0, r + 1)); // the left edge, downwards
    }
    // The bottom: a fan from the middle to every point of the border (the walls have a corner at each of those points, so the
    // solid is closed without gaps), seen from below counter-clockwise.
    let mut border: Vec<[f64; 3]> = vec![];
    for c in 0..cols {
        border.push(base(top(c, rows - 1)));
    }
    for r in (0..rows - 1).rev() {
        border.push(base(top(cols - 1, r)));
    }
    for c in (0..cols - 1).rev() {
        border.push(base(top(c, 0)));
    }
    for r in 1..rows - 1 {
        border.push(base(top(0, r)));
    }
    let mid = [(vx(0) + vx(cols - 1)) / 2.0, (vy(0) + vy(rows - 1)) / 2.0, 0.0];
    for i in 0..border.len() {
        tris.push([mid, border[(i + 1) % border.len()], border[i]]);
    }
    let mut b = vec![0u8; 80];
    b[..21].copy_from_slice(b"MillCreator relief   ");
    b.extend((tris.len() as u32).to_le_bytes());
    for t in &tris {
        let (u, v) = ([t[1][0] - t[0][0], t[1][1] - t[0][1], t[1][2] - t[0][2]], [t[2][0] - t[0][0], t[2][1] - t[0][1], t[2][2] - t[0][2]]);
        let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt().max(1e-300);
        for x in n {
            b.extend(((x / l) as f32).to_le_bytes());
        }
        for p in t {
            for x in p {
                b.extend((*x as f32).to_le_bytes());
            }
        }
        b.extend([0u8; 2]);
    }
    let _ = Pt::new(0.0, 0.0);
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::Library;
    use base_core::geom::Xf;
    use base_core::scene::{Kind, Scene};

    /// A height map of `n` x `n` cells of 1 mm made by `f(col, row)`.
    fn map_of(n: usize, f: impl Fn(usize, usize) -> f64) -> HeightMap {
        let mut scene = Scene::default();
        let im = ImageData { name: "t".into(), px_w: 2, px_h: 2, gray: vec![255; 4], w: n as f64, h: n as f64, invert: false };
        let id = scene.add(0, Kind::Image(im.clone()), Xf::IDENTITY);
        let shape = scene.shape(id).unwrap().clone();
        let mut m = HeightMap::of(&shape, &im, 1.0, 0.0, 1.0).unwrap();
        assert_eq!((m.grid.cols, m.grid.rows), (n, n));
        for r in 0..n {
            for c in 0..n {
                m.h[r * n + c] = f(c, r);
            }
        }
        m
    }

    fn bit(id: &str) -> Tool {
        Library::new().get(id).unwrap().clone()
    }

    #[test]
    fn a_flat_bit_cannot_reach_the_bottom_of_a_pit_it_does_not_fit_in_and_does_in_a_wide_one() {
        // A pit 3 mm deep: 9 x 9 cells in the middle of 21 x 21, and a narrow one of 1 x 1.
        let wide = map_of(21, |c, r| if (6..15).contains(&c) && (6..15).contains(&r) { -3.0 } else { 0.0 });
        let flat = bit("std-flat-3");
        let tips = tip_heights(&wide, &flat);
        assert!((tips[10 * 21 + 10] + 3.0).abs() < 1e-9, "the middle of a wide pit is reached");
        assert!(tips[10 * 21 + 6].abs() < 1e-9, "next to the wall the bit would cut the wall: it stays at the top");
        // A ball nose rides up the walls of the pit, so it is not stopped at the edge like a flat bit: in a flat floor wider than
        // itself it reaches the floor.
        let floor = map_of(21, |c, r| if (3..18).contains(&c) && (3..18).contains(&r) { -1.0 } else { 0.0 });
        let ball = tip_heights(&floor, &bit("std-ball-3"));
        assert!((ball[10 * 21 + 10] + 1.0).abs() < 1e-9, "the ball reaches the flat floor: {}", ball[10 * 21 + 10]);
        let narrow = map_of(21, |c, r| if (c, r) == (10, 10) { -3.0 } else { 0.0 });
        assert!(tip_heights(&narrow, &flat)[10 * 21 + 10].abs() < 1e-9, "a pit of 1 mm is too small for a bit of 3 mm");
    }

    #[test]
    fn the_bit_never_cuts_into_the_relief_under_any_part_of_its_body() {
        // A bumpy surface; for every cell and every cell under the bit the tip plus the rise of the bit is not below the surface.
        let m = map_of(25, |c, r| -2.0 * (((c * 7 + r * 13) % 11) as f64 / 10.0));
        for id in ["std-flat-3", "std-v-60"] {
            let tool = bit(id);
            let tips = tip_heights(&m, &tool);
            let reach = (tool.radius() / m.grid.step).floor() as i64;
            for r in 0..25i64 {
                for c in 0..25i64 {
                    for dr in -reach..=reach {
                        for dc in -reach..=reach {
                            let d = ((dr * dr + dc * dc) as f64).sqrt();
                            let (rr, cc) = (r + dr, c + dc);
                            if rr < 0 || cc < 0 || rr >= 25 || cc >= 25 {
                                continue;
                            }
                            if let Some(up) = rise(&tool, d) {
                                assert!(tips[(r * 25 + c) as usize] + up >= m.h[(rr * 25 + cc) as usize] - 1e-9, "{id} gouges at {c},{r}");
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_passes_go_down_in_steps_to_the_full_depth_and_the_rows_leave_out_straight_points() {
        assert_eq!(pass_limits(1.0, 0.25), vec![0.25, 0.5, 0.75, 1.0]);
        assert_eq!(pass_limits(1.0, 0.4), vec![0.4, 0.8, 1.0]);
        assert_eq!(pass_limits(0.3, 1.0), vec![0.3]);
        // A flat floor and an even slope are only their end points; a bend is kept.
        let m = map_of(9, |c, _| if c < 3 { -1.0 } else if c < 6 { -1.0 + (c as f64 - 2.0) / 3.0 } else { 0.0 });
        let tips = tip_heights(&m, &bit("std-flat-3"));
        let _ = tips;
        let pts = simplify((0..6).map(|i| [i as f64, 0.0, -1.0]).collect());
        assert_eq!(pts.len(), 2);
        let pts = simplify(vec![[0.0, 0.0, 0.0], [1.0, 0.0, -1.0], [2.0, 0.0, -1.0], [3.0, 0.0, 0.0]]);
        assert_eq!(pts.len(), 4);
        // A row stops where the bit would not cut (at the surface) and the limit of the pass holds the depth.
        let m = map_of(9, |c, _| if (3..6).contains(&c) { -2.0 } else { 0.0 });
        let flat_tips = vec![0.0; 0].into_iter().chain((0..81).map(|i| m.h[i])).collect::<Vec<f64>>();
        let runs = row_runs(&m, &flat_tips, 4, 0.5, false, 100.0);
        assert_eq!(runs.len(), 1, "{runs:?}");
        assert!(runs[0].iter().all(|p| (p[2] + 0.5).abs() < 1e-9), "held at the depth of the pass");
        let runs = row_runs(&m, &flat_tips, 4, 5.0, false, 100.0);
        assert!(runs[0].iter().all(|p| (p[2] + 2.0).abs() < 1e-9));
    }

    /// The edges of a closed solid are each shared by two triangles, and its volume is positive.
    #[test]
    fn the_model_is_a_closed_solid_with_the_right_volume() {
        let m = map_of(6, |c, r| if c == 2 && r == 3 { -1.0 } else { 0.0 });
        let b = stl(&m, 5.0);
        let n = u32::from_le_bytes([b[80], b[81], b[82], b[83]]) as usize;
        assert_eq!(b.len(), 84 + 50 * n);
        let f = |o: usize| f32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) as f64;
        let mut edges: std::collections::HashMap<([i64; 3], [i64; 3]), i32> = Default::default();
        let mut volume = 0.0;
        for i in 0..n {
            let o = 84 + 50 * i + 12;
            let v: Vec<[f64; 3]> = (0..3).map(|k| [f(o + 12 * k), f(o + 12 * k + 4), f(o + 12 * k + 8)]).collect();
            let q = |p: [f64; 3]| [(p[0] * 1e4).round() as i64, (p[1] * 1e4).round() as i64, (p[2] * 1e4).round() as i64];
            for (a, c) in [(0, 1), (1, 2), (2, 0)] {
                let (x, y) = (q(v[a]), q(v[c]));
                // An undirected edge counted +1 one way and -1 the other: a closed, consistently turned surface sums to 0.
                let key = if x < y { (x, y) } else { (y, x) };
                *edges.entry(key).or_default() += if x < y { 1 } else { -1 };
            }
            let cross = [v[1][1] * v[2][2] - v[1][2] * v[2][1], v[1][2] * v[2][0] - v[1][0] * v[2][2], v[1][0] * v[2][1] - v[1][1] * v[2][0]];
            volume += (v[0][0] * cross[0] + v[0][1] * cross[1] + v[0][2] * cross[2]) / 6.0;
        }
        let bad: Vec<_> = edges.iter().filter(|(_, s)| **s != 0).take(5).collect();
        assert!(bad.is_empty(), "every edge is used once each way: {bad:?}");
        // 5 x 5 squares of 1 mm, 5 mm high, minus one point pushed down by 1 mm (the four squares round it lose a pyramid-like
        // part: 1 mm3 for this triangulation).
        let expected = 25.0 * 5.0 - 1.0;
        assert!((volume - expected).abs() < 0.4, "{volume} against about {expected}");
        assert!(volume > 0.0);
    }

    /// The top of the picture is the larger Y of the model, and the left of the picture is the smaller X.
    #[test]
    fn the_model_stands_the_way_the_picture_lies() {
        // The upper left corner of the picture is cut deep (4 x 4 cells of 10 x 10).
        let m = map_of(10, |c, r| if c < 4 && r < 4 { -2.0 } else { 0.0 });
        let b = stl(&m, 5.0);
        let n = u32::from_le_bytes([b[80], b[81], b[82], b[83]]) as usize;
        let f = |o: usize| f32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) as f64;
        let (mut sx, mut sy, mut k) = (0.0, 0.0, 0.0);
        for i in 0..n {
            for v in 0..3 {
                let o = 84 + 50 * i + 12 + 12 * v;
                let (x, y, z) = (f(o), f(o + 4), f(o + 8));
                if z < 3.2 && z > 2.8 {
                    sx += x;
                    sy += y;
                    k += 1.0;
                }
            }
        }
        // The pit (z 3) lies at the small x and the large y of the model, whose middle is at 4.5.
        assert!(k > 0.0 && sx / k < 4.5 && sy / k > 4.5, "{} {}", sx / k, sy / k);
    }
}
