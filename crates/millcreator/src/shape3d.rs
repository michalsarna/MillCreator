//! 3D shapes from closed vectors: every closed shape of the layer becomes a height map — a dome, a slope (a pyramid or a bevel)
//! or a flat top — raised out of the material (the box round the shapes cut down to the depth of the layer) or sunk into it. It is
//! then cut like a relief: roughing in passes, a finish with a ball nose, or both, with the shape of the bit taken into account.
use crate::relief::HeightMap;
use base_core::geom::{Polyline, Pt, Xf};
use base_core::raster::Grid;
use serde::{Deserialize, Serialize};

/// The most cells of a height map made from shapes.
const MAX_CELLS: usize = 250_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Form {
    /// A round top: a quarter circle from the edge up to the middle of the shape.
    #[default]
    Dome,
    /// A straight slope from the edge over `Model::slope` mm (a pyramid when the shape is narrower).
    Slope,
    /// Straight walls and a flat top (extruded).
    Flat,
}

impl Form {
    pub const ALL: [Form; 3] = [Form::Dome, Form::Slope, Form::Flat];
    pub fn label(self) -> &'static str {
        match self {
            Form::Dome => "Dome",
            Form::Slope => "Slope",
            Form::Flat => "Flat top",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model {
    #[serde(default)]
    pub form: Form,
    /// Raised out of the material (else sunk into it).
    #[serde(default = "yes")]
    pub raised: bool,
    /// The width of the slope (mm).
    #[serde(default = "five")]
    pub slope: f64,
}

fn yes() -> bool {
    true
}
fn five() -> f64 {
    5.0
}

impl Default for Model {
    fn default() -> Model {
        Model { form: Form::Dome, raised: true, slope: 5.0 }
    }
}

impl Model {
    pub fn is_default(&self) -> bool {
        *self == Model::default()
    }
    pub fn check(&self) -> Result<(), String> {
        if !(self.slope.is_finite() && (0.1..=1000.0).contains(&self.slope)) {
            return Err("the slope of the 3D shape is out of range".into());
        }
        Ok(())
    }
    /// How high the shape is (0 … 1) at the distance `d` from its edge, for a shape whose middle is `deepest` from its edge.
    pub fn rise(&self, d: f64, deepest: f64) -> f64 {
        match self.form {
            Form::Dome => {
                let u = (d / deepest.max(1e-9)).clamp(0.0, 1.0);
                (1.0 - (1.0 - u).powi(2)).sqrt()
            }
            Form::Slope => (d / self.slope.max(1e-9)).clamp(0.0, 1.0),
            Form::Flat => 1.0,
        }
    }
}

fn seg_dist(a: Pt, b: Pt, p: Pt) -> f64 {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let l2 = dx * dx + dy * dy;
    let t = if l2 > 1e-18 { (((p.x - a.x) * dx + (p.y - a.y) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
    p.dist(Pt::new(a.x + t * dx, a.y + t * dy))
}

/// The height map of the shapes (each a list of closed outlines, holes included, in the coordinates of the page) on a grid of
/// `step` mm round them, `margin` mm wider than their box (where a raised shape has its background cut away).
pub fn height_map(shapes: &[Vec<Polyline>], model: &Model, depth: f64, step: f64, margin: f64) -> Option<HeightMap> {
    let all: Vec<&Polyline> = shapes.iter().flatten().filter(|p| p.closed && p.pts.len() >= 3).collect();
    let mut lo = Pt::new(f64::INFINITY, f64::INFINITY);
    let mut hi = Pt::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
    for p in all.iter().flat_map(|p| &p.pts) {
        lo = Pt::new(lo.x.min(p.x), lo.y.min(p.y));
        hi = Pt::new(hi.x.max(p.x), hi.y.max(p.y));
    }
    if !lo.x.is_finite() {
        return None;
    }
    let m = if model.raised { margin.max(0.0) } else { 0.0 };
    let (lo, hi) = (Pt::new(lo.x - m, lo.y - m), Pt::new(hi.x + m, hi.y + m));
    let mut step = step.max(0.05);
    while ((hi.x - lo.x) / step).ceil() * ((hi.y - lo.y) / step).ceil() > MAX_CELLS as f64 {
        step *= 1.3;
    }
    let cols = ((hi.x - lo.x) / step).ceil().max(1.0) as usize;
    let rows = ((hi.y - lo.y) / step).ceil().max(1.0) as usize;
    let grid = Grid { cols, rows, step, min: lo, rot: Xf::IDENTITY, levels: vec![0.0; cols * rows] };
    // For every shape: whether a cell is inside it (even-odd over its outlines) and its distance to the edge.
    let mut rise = vec![0.0f64; cols * rows];
    let mut inside_any = vec![false; cols * rows];
    for shape in shapes {
        let rings: Vec<&Polyline> = shape.iter().filter(|p| p.closed && p.pts.len() >= 3).collect();
        if rings.is_empty() {
            continue;
        }
        let mut dist = vec![-1.0f64; cols * rows];
        let mut deepest = 0.0f64;
        for r in 0..rows {
            for c in 0..cols {
                let p = grid.centre(c, r);
                if rings.iter().filter(|q| q.contains(p)).count() % 2 == 1 {
                    let d = rings.iter().flat_map(|q| (0..q.pts.len()).map(move |i| seg_dist(q.pts[i], q.pts[(i + 1) % q.pts.len()], p))).fold(f64::INFINITY, f64::min);
                    dist[r * cols + c] = d;
                    deepest = deepest.max(d);
                }
            }
        }
        for (i, d) in dist.iter().enumerate() {
            if *d >= 0.0 {
                inside_any[i] = true;
                rise[i] = rise[i].max(model.rise(*d, deepest));
            }
        }
    }
    let h = (0..cols * rows)
        .map(|i| {
            if model.raised {
                // The background is cut down to the depth; the shape rises out of it to the top of the material.
                if inside_any[i] { -depth * (1.0 - rise[i]) } else { -depth }
            } else if inside_any[i] {
                -depth * rise[i]
            } else {
                0.0
            }
        })
        .collect();
    Some(HeightMap { grid, h })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(x: f64, y: f64, s: f64) -> Vec<Polyline> {
        vec![Polyline::new(vec![Pt::new(x, y), Pt::new(x + s, y), Pt::new(x + s, y + s), Pt::new(x, y + s)], true)]
    }

    #[test]
    fn a_dome_rises_to_the_top_in_the_middle_and_a_sunken_slope_goes_down() {
        let m = Model::default();
        let map = height_map(&[square(0.0, 0.0, 20.0)], &m, 4.0, 0.5, 3.0).unwrap();
        let at = |x: f64, y: f64| {
            let c = ((x - map.grid.min.x) / map.grid.step) as usize;
            let r = ((y - map.grid.min.y) / map.grid.step) as usize;
            map.at(c, r)
        };
        assert!(at(10.0, 10.0) > -0.1, "the top of the dome: {}", at(10.0, 10.0));
        assert!(at(0.3, 10.0) < -2.0, "near the edge it is low: {}", at(0.3, 10.0));
        assert!((at(-2.0, 10.0) + 4.0).abs() < 1e-9, "the background is cut to the depth");
        let sunk = Model { form: Form::Slope, raised: false, slope: 4.0 };
        let map = height_map(&[square(0.0, 0.0, 20.0)], &sunk, 4.0, 0.5, 3.0).unwrap();
        let at = |x: f64, y: f64| {
            let c = ((x - map.grid.min.x) / map.grid.step) as usize;
            let r = ((y - map.grid.min.y) / map.grid.step) as usize;
            map.at(c, r)
        };
        assert!((at(10.0, 10.0) + 4.0).abs() < 1e-9 && at(0.6, 10.0) > -1.0);
        assert!(map.grid.min.x >= -1e-9, "a sunken shape needs no margin");
        assert!(Model::default().check().is_ok() && Model { slope: 0.0, ..Model::default() }.check().is_err());
        let d: Model = serde_json::from_str("{}").unwrap();
        assert!(d.is_default());
    }

    #[test]
    fn a_hole_of_a_shape_stays_low() {
        let mut ring = square(0.0, 0.0, 30.0);
        let mut hole = square(10.0, 10.0, 10.0);
        hole[0].pts.reverse();
        ring.append(&mut hole);
        let map = height_map(&[ring], &Model { form: Form::Flat, raised: true, slope: 5.0 }, 2.0, 0.5, 2.0).unwrap();
        let at = |x: f64, y: f64| map.at(((x - map.grid.min.x) / map.grid.step) as usize, ((y - map.grid.min.y) / map.grid.step) as usize);
        assert!(at(5.0, 5.0).abs() < 1e-9 && (at(15.0, 15.0) + 2.0).abs() < 1e-9);
    }
}
