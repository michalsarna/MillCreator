//! The 3D view of the job: the block of material, the cuts and the moves above it. The view turns with the mouse, zooms
//! with the wheel and goes back to its first position with a button. It shows only the area of the project, ten per cent
//! larger than the project, so the picture is not lost in the work area of the machine.
use crate::device::Device;
use crate::layers::MillLayer;
use crate::materials::Stock;
use crate::toolpath::{MoveKind, Program, P3};
use egui::{epaint::PathStroke, Color32, Pos2, Rect, Sense, Shape, Stroke, Vec2};
pub use base_ui::view3d::{shade, Affine3, Camera, Carved, Fit, Projector, SolidFace, SolidView};

/// How much larger than the project the shown area is.
pub const MARGIN: f64 = 0.10;

/// The area of the block that is shown, in the coordinates of the work area.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Region {
    pub min: [f64; 2],
    pub max: [f64; 2],
}

impl Region {
    /// The cuts of the program, 10% larger in each direction of the size; or the box of the shapes when there are no
    /// cuts yet; or `None` when there is nothing to show.
    pub fn of(prog: &Program, shapes: Option<[f64; 4]>) -> Option<Region> {
        let (lo, hi) = match (prog.bounds, shapes) {
            (Some((lo, hi)), _) => ([lo[0], lo[1]], [hi[0], hi[1]]),
            (None, Some(s)) => ([s[0], s[1]], [s[2], s[3]]),
            _ => return None,
        };
        let size = [(hi[0] - lo[0]).max(1.0), (hi[1] - lo[1]).max(1.0)];
        let centre = [(lo[0] + hi[0]) / 2.0, (lo[1] + hi[1]) / 2.0];
        let half = [size[0] * (1.0 + MARGIN) / 2.0, size[1] * (1.0 + MARGIN) / 2.0];
        Some(Region { min: [centre[0] - half[0], centre[1] - half[1]], max: [centre[0] + half[0], centre[1] + half[1]] })
    }
    pub fn size(&self) -> [f64; 2] {
        [self.max[0] - self.min[0], self.max[1] - self.min[1]]
    }
    pub fn centre(&self) -> [f64; 2] {
        [(self.min[0] + self.max[0]) / 2.0, (self.min[1] + self.max[1]) / 2.0]
    }
}

/// The workpiece on a rotary attachment: a cylinder along X whose surface is the top of the material. Y of the program is the way round
/// the surface (mm), so a point of the program at (x, y, z) lies on the radius `r + z` at the angle `(y - cy) / r`; the middle of the
/// job (`cy`) faces up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cyl {
    pub r: f64,
    pub cy: f64,
}

impl Cyl {
    /// The point of the model in the picture's space (z up, the axis of the cylinder at z = -r).
    pub fn map(&self, p: P3) -> P3 {
        let th = (p[1] - self.cy) / self.r;
        let rr = (self.r + p[2]).max(0.0);
        [p[0], self.cy + rr * th.sin(), -self.r + rr * th.cos()]
    }
    /// The outward direction of the surface where the program's y is `y`.
    pub fn normal(&self, y: f64) -> [f32; 3] {
        let th = (y - self.cy) / self.r;
        [0.0, th.sin() as f32, th.cos() as f32]
    }
    /// A line of the program as the points of the curve it makes on the cylinder (a long way round is cut into short pieces).
    pub fn curve(&self, a: P3, b: P3) -> Vec<P3> {
        let turn = ((b[1] - a[1]) / self.r).abs();
        let n = (turn / 0.12).ceil().clamp(1.0, 200.0) as usize;
        (0..=n).map(|i| {
            let t = i as f64 / n as f64;
            self.map([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t])
        }).collect()
    }
}

impl Region {
    /// With a rotary the block is the whole way round the workpiece, centred on the job.
    pub fn round(self, circumference: f64) -> (Region, f64) {
        let cy = self.centre()[1];
        (Region { min: [self.min[0], cy - circumference / 2.0], max: [self.max[0], cy + circumference / 2.0] }, cy)
    }
}

/// How the model is looked at (the camera of base: turned, zoomed, moved; `top` and `under` are its) and what is shown.
#[derive(Clone, Debug, PartialEq)]
pub struct View3d {
    pub cam: Camera,
    pub show_cuts: bool,
    pub show_rapids: bool,
    pub show_stock: bool,
    /// The shapes of the drawing over the top of the material, as a layer of their own.
    pub show_project: bool,
    /// The sizes of the block (in 3D) or of the shapes (from above), as on a technical drawing.
    pub show_dims: bool,
    /// How much of the program is shown, 0..=1.
    pub progress: f32,
    /// Asked for by a button, done by the next picture (which knows the size of its window).
    pub fit: Option<Fit>,
    /// The view is being turned or moved by the mouse (the picture of a post or a block is made coarser meanwhile).
    pub moving: bool,
    /// The job plays or its slider is dragged: the pictures are made coarser meanwhile, as while the view is turned.
    pub busy: bool,
}

impl std::ops::Deref for View3d {
    type Target = Camera;
    fn deref(&self) -> &Camera {
        &self.cam
    }
}

impl std::ops::DerefMut for View3d {
    fn deref_mut(&mut self) -> &mut Camera {
        &mut self.cam
    }
}

impl Default for View3d {
    fn default() -> View3d {
        View3d { cam: Camera::default(), show_cuts: true, show_rapids: true, show_stock: true, show_project: false, show_dims: false, progress: 1.0, fit: None, moving: false, busy: false }
    }
}

/// The six faces of the block: the corners, the outward direction and how light the face is.
fn faces(min: P3, max: P3) -> [([P3; 4], [f32; 3], f32); 6] {
    let (x0, y0, z0, x1, y1, z1) = (min[0], min[1], min[2], max[0], max[1], max[2]);
    [
        ([[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]], [0.0, 0.0, 1.0], 1.0),
        ([[x0, y0, z0], [x1, y0, z0], [x1, y1, z0], [x0, y1, z0]], [0.0, 0.0, -1.0], 0.5),
        ([[x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1]], [0.0, -1.0, 0.0], 0.82),
        ([[x0, y1, z0], [x1, y1, z0], [x1, y1, z1], [x0, y1, z1]], [0.0, 1.0, 0.0], 0.7),
        ([[x0, y0, z0], [x0, y1, z0], [x0, y1, z1], [x0, y0, z1]], [-1.0, 0.0, 0.0], 0.76),
        ([[x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1]], [1.0, 0.0, 0.0], 0.88),
    ]
}

/// The stock after the cuts as the picture shows it: the heights of a grid (row 0 at the largest y, 0 = the top of the stock, down
/// to the thickness) and how light each point is.
pub struct Surface {
    /// What it was made for; it is made again when this changes.
    pub key: (u64, usize, [u64; 5]),
    pub cols: usize,
    pub rows: usize,
    pub step: f64,
    pub lo: [f64; 2],
    pub top: f64,
    pub z: Vec<f64>,
    /// 0..1 light of every point, from the slope and the depth.
    pub light: Vec<f32>,
}

impl Surface {
    /// The surface of the stock with the cuts of `moves`; `None` when nothing is cut.
    /// `detail`: how fine the grid is over the one of the view (`base_ui::view3d::quality_detail` of the rendering quality).
    pub fn of(key: (u64, usize, [u64; 5]), moves: &[crate::toolpath::Move], tool_of: impl Fn(usize) -> Option<crate::tools::Tool>, region: Region, thickness: f64, detail: f64) -> Option<Surface> {
        let v = crate::relief::Limits::VIEW;
        let d2 = detail * detail;
        let limits = crate::relief::Limits { cells: ((v.cells as f64) * d2) as usize, work: v.work * d2, ..v };
        let (cols, rows, step, z) = crate::relief::carved_surface(moves, tool_of, (region.min, region.max), thickness, limits);
        if z.iter().all(|h| *h == 0.0) {
            return None;
        }
        let at = |c: i64, r: i64| z[(r.clamp(0, rows as i64 - 1) as usize) * cols + c.clamp(0, cols as i64 - 1) as usize];
        // A light from the upper left in front: the slope of the surface and the depth give the shade.
        let light_dir = [-0.35f64, -0.45, 0.82];
        let mut light = Vec::with_capacity(z.len());
        for r in 0..rows as i64 {
            for c in 0..cols as i64 {
                let dzdx = (at(c + 1, r) - at(c - 1, r)) / (2.0 * step);
                // Rows go down the picture, so up in y is the row before.
                let dzdy = (at(c, r - 1) - at(c, r + 1)) / (2.0 * step);
                let n = [-dzdx, -dzdy, 1.0];
                let len = (n[0] * n[0] + n[1] * n[1] + 1.0).sqrt();
                let lit = ((n[0] * light_dir[0] + n[1] * light_dir[1] + n[2] * light_dir[2]) / len).max(0.0);
                let depth = (-at(c, r) / thickness.max(0.1)).clamp(0.0, 1.0);
                light.push(((0.45 + 0.55 * lit) * (1.0 - 0.3 * depth)) as f32);
            }
        }
        Some(Surface { key, cols, rows, step, lo: region.min, top: region.min[1] + rows as f64 * step, z, light })
    }

    /// The pieces that fall out of the material once they are cut round the whole thickness `t` (windows without tabs): the points of
    /// the grid inside them go down through the material. `pieces` are closed outlines in the coordinates of the program.
    pub fn fall_out(&mut self, pieces: &[base_core::geom::Polyline], t: f64) {
        for p in pieces {
            let Some(b) = p.bounds() else { continue };
            let c0 = (((b.min.x - self.lo[0]) / self.step - 0.5).floor().max(0.0)) as usize;
            let c1 = (((b.max.x - self.lo[0]) / self.step - 0.5).ceil().max(0.0) as usize).min(self.cols.saturating_sub(1));
            let r0 = (((self.top - b.max.y) / self.step - 0.5).floor().max(0.0)) as usize;
            let r1 = (((self.top - b.min.y) / self.step - 0.5).ceil().max(0.0) as usize).min(self.rows.saturating_sub(1));
            for r in r0..=r1 {
                for c in c0..=c1 {
                    let q = base_core::geom::Pt::new(self.lo[0] + (c as f64 + 0.5) * self.step, self.top - (r as f64 + 0.5) * self.step);
                    if p.contains(q) {
                        self.z[r * self.cols + c] = -t;
                    }
                }
            }
        }
    }

    /// A face with no cuts on the same grid.
    pub fn flat(&self) -> Surface {
        Surface { key: self.key, cols: self.cols, rows: self.rows, step: self.step, lo: self.lo, top: self.top, z: vec![0.0; self.z.len()], light: vec![1.0; self.z.len()] }
    }

    /// How deep the surface is cut at (x, y) (mm, from its face; 0 off the grid). Between the points of the grid the depth goes
    /// over smoothly from one to the next, so that the walls of a cut are not steps of the grid in the picture.
    pub fn depth_at(&self, x: f64, y: f64) -> f64 {
        let fx = (x - self.lo[0]) / self.step - 0.5;
        let fy = (self.top - y) / self.step - 0.5;
        if fx < -0.5 || fy < -0.5 || fx >= self.cols as f64 - 0.5 || fy >= self.rows as f64 - 0.5 {
            return 0.0;
        }
        let (last_c, last_r) = (self.cols - 1, self.rows - 1);
        let (c0, r0) = ((fx.floor().max(0.0) as usize).min(last_c), (fy.floor().max(0.0) as usize).min(last_r));
        let (c1, r1) = ((c0 + 1).min(last_c), (r0 + 1).min(last_r));
        let (tx, ty) = ((fx - c0 as f64).clamp(0.0, 1.0), (fy - r0 as f64).clamp(0.0, 1.0));
        let z = |c: usize, r: usize| self.z[r * self.cols + c];
        let top = z(c0, r0) + (z(c1, r0) - z(c0, r0)) * tx;
        let bottom = z(c0, r1) + (z(c1, r1) - z(c0, r1)) * tx;
        -(top + (bottom - top) * ty)
    }
}

impl Carved for Surface {
    fn depth_at(&self, x: f64, y: f64) -> f64 {
        Surface::depth_at(self, x, y)
    }
    fn step(&self) -> f64 {
        self.step
    }
    fn deepest(&self) -> f64 {
        self.z.iter().fold(0.0f64, |m, z| m.max(-z))
    }
}

/// Is the material cut through at (x, y): the cut of this face (`depth`) and the one of the other face meet (a two-sided project),
/// or this cut alone goes the whole thickness `t`.
pub fn cut_through(depth: f64, other: Option<&Surface>, x: f64, y: f64, t: f64) -> bool {
    depth + other.map_or(0.0, |o| o.depth_at(x, y)) >= t - 0.01
}

/// The colour behind the model (what is seen through a hole cut through the material).
pub fn background(dark: bool) -> Color32 {
    if dark { Color32::from_rgb(0x1c, 0x1e, 0x24) } else { Color32::from_rgb(0xe9, 0xec, 0xf1) }
}

/// A line of a shape of the drawing, in the coordinates of the program.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectLine {
    pub layer: usize,
    pub pts: Vec<[f64; 2]>,
}

/// The lines of the shapes on the layers that are shown. Y goes up in the program and down in the drawing; `area_h` is the height of
/// the work area.
pub fn project_lines(scene: &base_core::scene::Scene, layers: &[MillLayer], area_h: f64) -> Vec<ProjectLine> {
    let mut out = vec![];
    for s in &scene.shapes {
        if !layers.get(s.layer).is_some_and(|l| l.visible) {
            continue;
        }
        for p in s.polys() {
            let mut pts: Vec<[f64; 2]> = p.pts.iter().map(|q| [q.x, area_h - q.y]).collect();
            if p.closed {
                if let Some(first) = pts.first().copied() {
                    pts.push(first);
                }
            }
            if pts.len() >= 2 {
                out.push(ProjectLine { layer: s.layer, pts });
            }
        }
    }
    out
}

/// What the 3D window needs from the project.
pub struct Scene3d<'a> {
    pub surface: Option<&'a Surface>,
    pub prog: &'a Program,
    pub region: Region,
    pub stock: &'a Stock,
    pub dev: &'a Device,
    pub layers: &'a [MillLayer],
    /// The shapes of the drawing, for [`View3d::show_project`].
    pub project: &'a [ProjectLine],
    /// A workpiece on a rotary attachment (the picture shows the cylinder).
    pub rotary: Option<Cyl>,
    /// The outlines and circles of the shapes that are milled (program coordinates, Y up), for their dimensions.
    pub parts: &'a base_ui::dims::Features,
    /// The units of the program, for the dimensions (those of the machine are in `dev`).
    pub units: base_core::units::Units,
    /// A two-sided project: the cuts of side B, coming from the bottom face.
    pub bottom: Option<Bottom<'a>>,
    /// A post or a block: all its faces, carved, round the shown one.
    pub solid: Option<SolidView<'a>>,
    /// The quality of the rendering (1 to 5) of the program options.
    pub quality: u8,
}

/// The bottom of a two-sided project in the view: its program as seen from side A (`toolpath::bottom_seen_from_a`, the depths from
/// the bottom face), the bottom face carved by it, and whether it is the side that plays (the other side is shown done).
pub struct Bottom<'a> {
    pub prog: &'a Program,
    pub surface: Option<&'a Surface>,
    pub animated: bool,
}

impl Region {
    /// The region of two programs together (the two sides of a project), or of one of them.
    pub fn of_two(a: &Program, b: &Program, shapes: Option<[f64; 4]>) -> Option<Region> {
        let (ra, rb) = (Region::of(a, shapes), Region::of(b, None));
        match (ra, rb) {
            (Some(x), Some(y)) => Some(Region { min: [x.min[0].min(y.min[0]), x.min[1].min(y.min[1])], max: [x.max[0].max(y.max[0]), x.max[1].max(y.max[1])] }),
            (x, y) => x.or(y),
        }
    }
}

/// The middle of the picture and the radius of the sphere that holds what is shown, the same for the picture and for the fitting.
pub(crate) fn framing(sc: &Scene3d) -> (P3, f64) {
    let r = sc.region;
    let [w, h] = r.size();
    let t = sc.stock.thickness;
    let safe = sc.dev.safe_z;
    match sc.rotary {
        Some(c) => ([r.centre()[0], c.cy, -c.r], ((w * w) + (2.0 * (c.r + safe)).powi(2)).sqrt() / 2.0),
        None => ([r.centre()[0], r.centre()[1], -t / 2.0], ((w * w + h * h + (t + safe).powi(2)).sqrt()) / 2.0),
    }
}

/// The point of the program after `part` (0..=1) of its length, and the moves that are done by then.
pub fn position_at(prog: &Program, part: f32) -> (usize, Option<P3>) {
    let total: f64 = prog.moves.iter().map(|m| m.length()).sum();
    if total <= 0.0 || part >= 1.0 {
        return (prog.moves.len(), prog.moves.last().map(|m| m.to));
    }
    let want = total * part.max(0.0) as f64;
    let mut done = 0.0;
    for (i, m) in prog.moves.iter().enumerate() {
        let l = m.length();
        if done + l >= want {
            let t = if l > 0.0 { (want - done) / l } else { 0.0 };
            let p = [m.from[0] + (m.to[0] - m.from[0]) * t, m.from[1] + (m.to[1] - m.from[1]) * t, m.from[2] + (m.to[2] - m.from[2]) * t];
            return (i, Some(p));
        }
        done += l;
    }
    (prog.moves.len(), prog.moves.last().map(|m| m.to))
}

/// The picture, in the space the ui gives. Turns, pans and zooms with the mouse.
pub fn show(ui: &mut egui::Ui, view: &mut View3d, sc: &Scene3d) {
    let (rect, resp) = ui.allocate_exact_size(base_ui::window::fill_size(ui).max(Vec2::splat(120.0)), Sense::click_and_drag());
    view.moving = base_ui::view3d::handle_input(ui, &resp, &mut view.cam) || view.busy;
    if let Some(f) = view.fit.take() {
        let area = [0.0, 0.0, sc.dev.area.x, sc.dev.area.y];
        let r = sc.region;
        let (lo, hi) = match f {
            Fit::Area => ([area[0], area[1]], [area[2], area[3]]),
            Fit::Elements => (r.min, r.max),
        };
        fit_to(view, rect, sc, lo, hi);
    }
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 4.0, background(ui.visuals().dark_mode));
    draw(&painter, rect, view, sc);
    // The navigation cube (not on the plan from above) and the zoom bar: its third button is the first view fitted to the work
    // area, its fourth fits all the objects.
    if let Some(f) = base_ui::view3d::overlays(ui, rect, &mut view.cam) {
        view.fit = Some(f);
    }
}

/// Zoom and pan so that the box `lo`..`hi` (X and Y, from the material's top down to the safe height) fills `rect`.
pub fn fit_to(view: &mut View3d, rect: Rect, sc: &Scene3d, lo: [f64; 2], hi: [f64; 2]) {
    let t = sc.stock.thickness;
    let (target, radius) = framing(sc);
    // The picture at zoom 1 and no pan: its points are a fixed multiple of their distance from the middle of the window.
    let base = View3d { cam: Camera { zoom: 1.0, pan: Vec2::ZERO, ..view.cam }, fit: None, ..view.clone() };
    let pj = Projector::new(&base, target, radius, rect);
    let (mut min, mut max) = (Pos2::new(f32::MAX, f32::MAX), Pos2::new(f32::MIN, f32::MIN));
    let mut take = |p: P3| {
        let p = pj.project(p);
        min = min.min(p);
        max = max.max(p);
    };
    match sc.rotary {
        // The cylinder is as long as the way along X asks for and as big round as it is.
        Some(c) => {
            for x in [lo[0], hi[0]] {
                for k in 0..16 {
                    let a = k as f64 / 16.0 * std::f64::consts::TAU;
                    for rr in [c.r, c.r + sc.dev.safe_z] {
                        take([x, c.cy + rr * a.sin(), -c.r + rr * a.cos()]);
                    }
                }
            }
        }
        None => {
            for x in [lo[0], hi[0]] {
                for y in [lo[1], hi[1]] {
                    for z in [-t, 0.0] {
                        take([x, y, z]);
                    }
                }
            }
        }
    }
    let (bw, bh) = ((max.x - min.x).max(1.0), (max.y - min.y).max(1.0));
    let zoom = (rect.width() / bw).min(rect.height() / bh) * 0.92;
    let offset = (min + (max - min) / 2.0) - rect.center();
    view.zoom = zoom.clamp(0.2, 40.0);
    view.pan = -offset * view.zoom;
}

/// Lines of many short moves drawn as few shapes: moves that follow each other in the same stroke are one polyline, points closer
/// than half a pixel to the last one kept are left out, and what lies wholly outside `clip` is not drawn. A program of hundreds of
/// thousands of moves is then as many points as the picture can show, not a shape for every move.
pub struct Lines {
    shapes: Vec<Shape>,
    pts: Vec<Pos2>,
    /// The end of the last move, when it was not kept (too close to the last point kept).
    pending: Option<Pos2>,
    stroke: Stroke,
    dashed: bool,
    clip: Rect,
}

impl Lines {
    /// The smallest distance (in points) between two points kept.
    const MIN_STEP: f32 = 0.5;

    /// More rapid moves than this are drawn plain: hundreds of dashes for each of thousands of moves would hold the picture up.
    pub const MAX_DASHED: usize = 2_000;

    pub fn new(clip: Rect) -> Lines {
        Lines { shapes: vec![], pts: vec![], pending: None, stroke: Stroke::NONE, dashed: false, clip: clip.expand(4.0) }
    }

    /// The end of the polyline that is being made.
    fn end(&self) -> Option<Pos2> {
        self.pending.or_else(|| self.pts.last().copied())
    }

    /// A piece from `a` to `b`, plain or dashed.
    pub fn add(&mut self, a: Pos2, b: Pos2, stroke: Stroke, dashed: bool) {
        // Wholly on one side of the picture: not seen.
        let c = self.clip;
        if (a.x < c.min.x && b.x < c.min.x) || (a.x > c.max.x && b.x > c.max.x) || (a.y < c.min.y && b.y < c.min.y) || (a.y > c.max.y && b.y > c.max.y) {
            self.flush();
            return;
        }
        let follows = stroke == self.stroke && dashed == self.dashed && self.end().is_some_and(|e| (e - a).length_sq() < 0.01);
        if !follows {
            self.flush();
            self.stroke = stroke;
            self.dashed = dashed;
            self.pts.push(a);
        }
        if self.pts.last().is_some_and(|l| (*l - b).length() < Self::MIN_STEP) {
            self.pending = Some(b);
            return;
        }
        self.pending = None;
        // A point on the straight line between the one before it and the new one (within a fifth of a pixel, going on the same
        // way) is not needed: a straight run of many moves is one piece.
        let n = self.pts.len();
        if n >= 2 {
            let (p0, p1) = (self.pts[n - 2], self.pts[n - 1]);
            let (d, e) = (b - p0, p1 - p0);
            let len = d.length();
            if len > 0.0 && e.dot(d) > 0.0 && e.length() <= len && (d.x * e.y - d.y * e.x).abs() / len < 0.2 {
                self.pts[n - 1] = b;
                return;
            }
        }
        self.pts.push(b);
    }

    /// Every piece of a path given as points.
    pub fn add_path(&mut self, pts: &[Pos2], stroke: Stroke, dashed: bool) {
        for w in pts.windows(2) {
            self.add(w[0], w[1], stroke, dashed);
        }
    }

    fn flush(&mut self) {
        if let Some(p) = self.pending.take() {
            self.pts.push(p);
        }
        if self.pts.len() >= 2 {
            let pts = std::mem::take(&mut self.pts);
            if self.dashed {
                self.shapes.extend(Shape::dashed_line(&pts, self.stroke, 5.0, 4.0));
            } else {
                self.shapes.push(Shape::line(pts, self.stroke));
            }
        }
        self.pts.clear();
    }

    /// The shapes, onto the painter.
    pub fn paint(mut self, painter: &egui::Painter) {
        self.flush();
        painter.extend(self.shapes);
    }
}

/// The surface as a mesh of triangles, the far ones first (a height field is drawn correctly from any side that way).
/// `under`: the surface of the bottom face of a block of that thickness (its depths count up from the bottom).
/// Where the material is cut through (`cut_through` with the cuts of the other face, `other`, and the thickness `t`) the cell is a
/// hole: the background is seen through it.
fn draw_surface(painter: &egui::Painter, pj: &Projector, s: &Surface, color: [u8; 3], eye: [f32; 3], under: Option<f64>, through: (Option<&Surface>, f64, Color32)) {
    let at = |c: usize, r: usize| {
        let z = s.z[r * s.cols + c];
        [s.lo[0] + (c as f64 + 0.5) * s.step, s.top - (r as f64 + 0.5) * s.step, under.map_or(z, |t| -t - z)]
    };
    let (other, t, hole) = through;
    let open: Vec<bool> = (0..s.rows * s.cols).map(|i| {
        let (r, c) = (i / s.cols, i % s.cols);
        cut_through(-s.z[i], other, s.lo[0] + (c as f64 + 0.5) * s.step, s.top - (r as f64 + 0.5) * s.step, t)
    }).collect();
    let mut mesh = egui::epaint::Mesh::default();
    mesh.vertices.reserve(s.cols * s.rows);
    for r in 0..s.rows {
        for c in 0..s.cols {
            // A point where the material is cut through has the colour of the background; a narrow slot through the material (a
            // few points of the grid wide) shows as a dark line.
            let i = r * s.cols + c;
            mesh.colored_vertex(pj.project(at(c, r)), if open[i] { hole } else { shade(color, s.light[i]) });
        }
    }
    // The far side first: the columns and rows are walked from the side the eye is not on.
    let cs: Vec<usize> = if eye[0] > 0.0 { (0..s.cols - 1).collect() } else { (0..s.cols - 1).rev().collect() };
    let rs: Vec<usize> = if eye[1] > 0.0 { (0..s.rows - 1).rev().collect() } else { (0..s.rows - 1).collect() };
    for r in rs {
        for c in cs.iter().copied() {
            let i = (r * s.cols + c) as u32;
            let w = s.cols as u32;
            mesh.add_triangle(i, i + 1, i + w);
            mesh.add_triangle(i + 1, i + w + 1, i + w);
        }
    }
    painter.add(Shape::mesh(mesh));
}

/// A cylinder that has not been cut: the same grid as a carved surface, all at the top.
fn plain_surface(region: Region, c: Cyl) -> Surface {
    let step = (2.0 * std::f64::consts::PI * c.r / 160.0).max(0.05);
    let rows = 160;
    let cols = ((region.size()[0] / step).round() as usize).clamp(2, 600);
    Surface { key: (0, 0, [0; 5]), cols, rows, step, lo: [region.min[0], region.min[1]], top: region.min[1] + rows as f64 * step, z: vec![0.0; cols * rows], light: vec![0.85; cols * rows] }
}

/// The workpiece on a rotary attachment: the surface (carved by the cuts so far) wrapped round the axis, the near side only and the
/// far cells first, and the two ends.
fn draw_cylinder(painter: &egui::Painter, pj: &Projector, sc: &Scene3d, c: Cyl, eye: [f32; 3]) {
    let flat;
    let s = match sc.surface {
        Some(s) => s,
        None => {
            flat = plain_surface(sc.region, c);
            &flat
        }
    };
    let at = |col: usize, row: usize| [s.lo[0] + (col as f64 + 0.5) * s.step, s.top - (row as f64 + 0.5) * s.step, s.z[row * s.cols + col]];
    let e = [eye[0] as f64, eye[1] as f64, eye[2] as f64];
    let mut mesh = egui::epaint::Mesh::default();
    mesh.vertices.reserve(s.cols * s.rows);
    for row in 0..s.rows {
        for col in 0..s.cols {
            let p = at(col, row);
            let n = c.normal(p[1]);
            // The side the light comes from is brighter; the part that turns away from the eye is not drawn at all.
            let facing = (n[1] * eye[1] + n[2] * eye[2]).max(0.0);
            mesh.colored_vertex(pj.project(c.map(p)), shade(sc.stock.color, s.light[row * s.cols + col] * (0.62 + 0.38 * facing)));
        }
    }
    let mut cells: Vec<(f64, u32)> = vec![];
    for row in 0..s.rows - 1 {
        for col in 0..s.cols - 1 {
            let (a, b) = (at(col, row), at(col + 1, row + 1));
            let mid = [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0, (a[2] + b[2]) / 2.0];
            let n = c.normal(mid[1]);
            if n[1] as f64 * e[1] + n[2] as f64 * e[2] <= 0.02 {
                continue;
            }
            let m = c.map(mid);
            cells.push((m[0] * e[0] + m[1] * e[1] + m[2] * e[2], (row * s.cols + col) as u32));
        }
    }
    // Far first.
    cells.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let w = s.cols as u32;
    for (_, i) in cells {
        mesh.add_triangle(i, i + 1, i + w);
        mesh.add_triangle(i + 1, i + w + 1, i + w);
    }
    painter.add(Shape::mesh(mesh));
    // The ends: circles of the whole radius, the one that faces the eye.
    let (x0, x1) = (s.lo[0] + 0.5 * s.step, s.lo[0] + (s.cols as f64 - 0.5) * s.step);
    for (x, nx) in [(x0, -1.0f32), (x1, 1.0)] {
        if nx * eye[0] <= 0.0 {
            continue;
        }
        let pts: Vec<Pos2> = (0..72).map(|k| {
            let a = k as f64 / 72.0 * std::f64::consts::TAU;
            pj.project([x, c.cy + c.r * a.sin(), -c.r + c.r * a.cos()])
        }).collect();
        painter.add(Shape::convex_polygon(pts, shade(sc.stock.color, 0.78), Stroke::new(1.0, shade(sc.stock.color, 0.5))));
    }
}

/// Draw the model into `rect`.
pub fn draw(painter: &egui::Painter, rect: Rect, view: &View3d, sc: &Scene3d) {
    let r = sc.region;
    let t = sc.stock.thickness;
    let safe = sc.dev.safe_z;
    let [w, h] = r.size();
    // The sphere that holds the block and the height of the moves above it.
    let (target, radius) = framing(sc);
    let pj = Projector::new(view, target, radius, rect);
    // A path of the program as the points of the picture: straight, or the curve it makes round a cylinder.
    let line = |a: P3, b: P3| -> Vec<Pos2> {
        match sc.rotary {
            Some(c) => c.curve(a, b).iter().map(|q| pj.project(*q)).collect(),
            None => vec![pj.project(a), pj.project(b)],
        }
    };
    let place = |p: P3| pj.project(match sc.rotary {
        Some(c) => c.map(p),
        None => p,
    });
    let (eye, _, _) = view.axes();
    let min = [r.min[0], r.min[1], -t];
    let max = [r.max[0], r.max[1], 0.0];
    if let (true, Some(c)) = (view.show_stock, sc.rotary) {
        draw_cylinder(painter, &pj, sc, c, eye);
    } else if let (true, Some(solid)) = (view.show_stock, sc.solid.as_ref()) {
        base_ui::view3d::draw_solid(painter, &pj, solid, sc.quality, view.moving);
    } else if view.show_stock {
        for (corners, normal, light) in faces(min, max) {
            if normal[0] * eye[0] + normal[1] * eye[1] + normal[2] * eye[2] <= 0.0 {
                continue;
            }
            let pts: Vec<Pos2> = corners.iter().map(|c| pj.project(*c)).collect();
            painter.add(Shape::convex_polygon(pts, shade(sc.stock.color, light), Stroke::new(1.0, shade(sc.stock.color, light * 0.6))));
        }
        // The cuts made so far, carved into the top of the stock (the whole top when nothing is cut yet); in a two-sided project
        // the bottom face carved by side B, seen when the block is turned to show its underside.
        let hole = background(painter.ctx().global_style().visuals.dark_mode);
        let bottom_surface = sc.bottom.as_ref().and_then(|b| b.surface);
        // A face that is not cut is drawn flat when the other face has cuts, so that those that go through show on it too.
        let flat;
        if eye[2] >= 0.0 || sc.bottom.is_none() {
            let top = match (sc.surface, bottom_surface) {
                (Some(s), _) => Some(s),
                (None, Some(b)) => {
                    flat = b.flat();
                    Some(&flat)
                }
                (None, None) => None,
            };
            if let Some(s) = top {
                draw_surface(painter, &pj, s, sc.stock.color, eye, None, (bottom_surface, t, hole));
            }
        } else {
            let under = match (bottom_surface, sc.surface) {
                (Some(s), _) => Some(s),
                (None, Some(a)) => {
                    flat = a.flat();
                    Some(&flat)
                }
                (None, None) => None,
            };
            if let Some(s) = under {
                draw_surface(painter, &pj, s, sc.stock.color, eye, Some(t), (sc.surface, t, hole));
            }
        }
    }
    // The side that plays goes as far as the slider; the other one is shown done.
    let bottom_plays = sc.bottom.as_ref().is_some_and(|b| b.animated);
    let (done, mut here) = if bottom_plays { (sc.prog.moves.len(), None) } else { position_at(sc.prog, view.progress) };
    let (b_done, b_here) = match &sc.bottom {
        Some(b) if b.animated => position_at(b.prog, view.progress),
        Some(b) => (b.prog.moves.len(), None),
        None => (0, None),
    };
    // A point of side B in the block: its depth counts up from the bottom face.
    let up = |p: P3| [p[0], p[1], -t - p[2]];
    let cut_color = |layer: usize| sc.layers.get(layer).map(|l| l.color32()).unwrap_or(Color32::RED);
    // In a two-sided project each side's paths are seen from its own face.
    let from_below = eye[2] < 0.0;
    if let (Some(b), true) = (&sc.bottom, from_below) {
        if view.show_cuts {
            let mut lines = Lines::new(painter.clip_rect());
            for m in b.prog.moves.iter().take(b_done) {
                if m.kind == MoveKind::Rapid || m.to[2] > 0.0 {
                    continue;
                }
                lines.add(pj.project(up(m.from)), pj.project(up(m.to)), Stroke::new(1.2, cut_color(m.layer)), false);
            }
            lines.paint(painter);
        }
        if view.show_rapids {
            let rapid = Color32::from_rgb(0x30, 0x7f, 0xd0);
            let mut lines = Lines::new(painter.clip_rect());
            let above = |m: &&crate::toolpath::Move| !(m.kind == MoveKind::Cut || (m.kind == MoveKind::Plunge && m.to[2] <= 0.0 && m.from[2] <= 0.0));
            let dashed = b.prog.moves.iter().take(b_done).filter(above).count() <= Lines::MAX_DASHED;
            let stroke = if dashed { Stroke::new(1.0, rapid) } else { Stroke::new(0.7, rapid.gamma_multiply(0.6)) };
            for m in b.prog.moves.iter().take(b_done).filter(above) {
                lines.add(pj.project(up(m.from)), pj.project(up(m.to)), stroke, dashed);
            }
            lines.paint(painter);
        }
        if let (true, Some(p)) = (b.animated, b_here) {
            if view.progress < 1.0 && !b.prog.is_empty() {
                // The bit comes from below.
                let tip = pj.project(up(p));
                let top = pj.project([p[0], p[1], -t - p[2] - 12.0]);
                painter.line_segment([top, tip], Stroke::new(3.0, Color32::from_rgb(0xe0, 0x50, 0x20)));
                painter.circle_filled(tip, 3.5, Color32::from_rgb(0xe0, 0x50, 0x20));
            }
        }
    }
    if bottom_plays || (sc.bottom.is_some() && from_below) {
        here = None;
    }
    // A post or a block: the paths of the shown face only while it is turned to the eye.
    let top_paths = (sc.bottom.is_none() && sc.solid.is_none()) || !from_below;
    if view.show_cuts && top_paths {
        // The paths of the bit in the colour of the layer, over the carved stock.
        let mut lines = Lines::new(painter.clip_rect());
        // A straight move needs no list of points, and the end of one move is the start of the next (projected once).
        let mut last: Option<(P3, Pos2)> = None;
        for m in sc.prog.moves.iter().take(done) {
            if m.kind == MoveKind::Rapid || m.to[2] > 0.0 {
                continue;
            }
            let stroke = Stroke::new(1.2, cut_color(m.layer));
            if sc.rotary.is_some() {
                lines.add_path(&line(m.from, m.to), stroke, false);
                continue;
            }
            let a = match last {
                Some((p, q)) if p == m.from => q,
                _ => pj.project(m.from),
            };
            let b = pj.project(m.to);
            lines.add(a, b, stroke, false);
            last = Some((m.to, b));
        }
        lines.paint(painter);
    }
    if view.show_rapids && top_paths {
        // The safe height as a frame over the block, and the moves above the material as dashed lines.
        let z = safe;
        let frame = PathStroke::new(1.0, Color32::from_rgba_unmultiplied(0x40, 0x90, 0xe0, 120));
        match sc.rotary {
            // Round a cylinder: a ring at each end, at the safe height above the surface.
            Some(c) => {
                for x in [r.min[0], r.max[0]] {
                    let pts: Vec<Pos2> = (0..=72).map(|k| {
                        let a = k as f64 / 72.0 * std::f64::consts::TAU;
                        pj.project([x, c.cy + (c.r + z) * a.sin(), -c.r + (c.r + z) * a.cos()])
                    }).collect();
                    painter.add(Shape::line(pts, frame.clone()));
                }
            }
            None => {
                let ring = [[r.min[0], r.min[1], z], [r.max[0], r.min[1], z], [r.max[0], r.max[1], z], [r.min[0], r.max[1], z], [r.min[0], r.min[1], z]];
                let pts: Vec<Pos2> = ring.iter().map(|c| pj.project(*c)).collect();
                painter.add(Shape::line(pts, frame));
            }
        }
        let rapid = Color32::from_rgb(0x30, 0x7f, 0xd0);
        let mut lines = Lines::new(painter.clip_rect());
        let above = |m: &&crate::toolpath::Move| !(m.kind == MoveKind::Cut || (m.kind == MoveKind::Plunge && m.to[2] <= 0.0 && m.from[2] <= 0.0));
        let dashed = sc.prog.moves.iter().take(done).filter(above).count() <= Lines::MAX_DASHED;
        let stroke = if dashed { Stroke::new(1.0, rapid) } else { Stroke::new(0.7, rapid.gamma_multiply(0.6)) };
        for m in sc.prog.moves.iter().take(done).filter(above) {
            lines.add_path(&line(m.from, m.to), stroke, dashed);
        }
        lines.paint(painter);
    }
    if view.show_project {
        // Over everything else, at the height of the top of the material.
        for l in sc.project {
            let mut pts: Vec<Pos2> = Vec::with_capacity(l.pts.len());
            for w in l.pts.windows(2) {
                pts.extend(line([w[0][0], w[0][1], 0.0], [w[1][0], w[1][1], 0.0]));
            }
            painter.add(Shape::line(pts, Stroke::new(1.6, cut_color(l.layer))));
        }
    }
    if let Some(p) = here {
        if !sc.prog.is_empty() && view.progress < 1.0 {
            let top = place([p[0], p[1], p[2] + 12.0]);
            let tip = place(p);
            painter.line_segment([top, tip], Stroke::new(3.0, Color32::from_rgb(0xe0, 0x50, 0x20)));
            painter.circle_filled(tip, 3.5, Color32::from_rgb(0xe0, 0x50, 0x20));
        }
    }
    if view.show_dims {
        let st = base_ui::dims::DimStyle::on(background(painter.ctx().global_style().visuals.dark_mode)).in_units(sc.units, sc.dev.units);
        if view.top {
            let to = |p: base_core::geom::Pt| place([p.x, p.y, 0.0]);
            base_ui::dims::drawing(painter, &to, sc.parts, Some(base_core::geom::Pt::new(0.0, 0.0)), &st);
        } else {
            let (lo, hi) = match sc.rotary {
                Some(c) => ([r.min[0], c.cy - c.r, -2.0 * c.r], [r.max[0], c.cy + c.r, 0.0]),
                None => (min, max),
            };
            base_ui::dims::box3d(painter, &|p| pj.project(p), lo, hi, &st);
            // The milled shapes on the top of the block: their outsides and holes, and the diameters of the round ones.
            if sc.rotary.is_none() {
                base_ui::dims::features3d(painter, &|p| pj.project(p), sc.parts, 0.0, &st);
            }
        }
    }
    // The axes at the corner of the block.
    let o = [r.min[0], sc.rotary.map_or(r.min[1], |c| c.cy), 0.0];
    let len = (w.min(if sc.rotary.is_some() { w } else { h }) * 0.2).max(5.0);
    for (d, name, color) in [([len, 0.0, 0.0], "X", Color32::from_rgb(0xd0, 0x30, 0x30)), ([0.0, len, 0.0], "Y", Color32::from_rgb(0x30, 0xa0, 0x30)), ([0.0, 0.0, len], "Z", Color32::from_rgb(0x30, 0x60, 0xd0))] {
        let a = place(o);
        let b = place([o[0] + d[0], o[1] + d[1], o[2] + d[2]]);
        painter.line_segment([a, b], Stroke::new(2.0, color));
        painter.text(b, egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(13.0), color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base_ui::view3d::{MAX_PITCH, MIN_PITCH, START_PITCH, START_YAW};
    use crate::layers::default_layers;
    use crate::tools::Library;
    use crate::toolpath::generate;
    use base_core::geom::Xf;
    use base_core::scene::{Kind, Scene};

    pub(super) fn job() -> (Program, Device, Stock, Vec<MillLayer>, Library) {
        let lib = Library::new();
        let dev = Device::dummy();
        let stock = Stock::default();
        let layers = default_layers(&lib);
        let mut scene = Scene::default();
        scene.add(0, Kind::Rect { w: 40.0, h: 20.0 }, Xf::translate(50.0, 60.0));
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        (prog, dev, stock, layers, lib)
    }

    #[test]
    fn the_region_is_ten_per_cent_larger_than_the_project_around_the_same_centre() {
        let (prog, ..) = job();
        let (lo, hi) = prog.bounds.unwrap();
        let r = Region::of(&prog, None).unwrap();
        let size = [hi[0] - lo[0], hi[1] - lo[1]];
        assert!((r.size()[0] / size[0] - 1.1).abs() < 1e-9 && (r.size()[1] / size[1] - 1.1).abs() < 1e-9);
        assert!((r.centre()[0] - (lo[0] + hi[0]) / 2.0).abs() < 1e-9);
        // Without cuts the box of the shapes is used; with nothing there is no region.
        let none = Program::default();
        let r = Region::of(&none, Some([10.0, 10.0, 30.0, 20.0])).unwrap();
        assert!((r.size()[0] - 22.0).abs() < 1e-9 && (r.size()[1] - 11.0).abs() < 1e-9);
        assert!(Region::of(&none, None).is_none());
    }

    #[test]
    fn the_project_is_in_the_coordinates_of_the_program_without_the_hidden_layers() {
        let (prog, dev, _, mut layers, _) = job();
        let mut scene = Scene::default();
        scene.add(0, Kind::Rect { w: 40.0, h: 20.0 }, Xf::translate(50.0, 60.0));
        scene.add(1, Kind::Rect { w: 5.0, h: 5.0 }, Xf::translate(0.0, 0.0));
        layers[1].visible = false;
        let lines = project_lines(&scene, &layers, dev.area.y);
        assert_eq!(lines.len(), 1, "the shape on the hidden layer is left out");
        assert_eq!(lines[0].layer, 0);
        assert_eq!(lines[0].pts.first(), lines[0].pts.last(), "a closed shape comes back to its start");
        // The same box as the shape in the program: a line along the outside of it, whatever the bit does.
        let ys: Vec<f64> = lines[0].pts.iter().map(|p| p[1]).collect();
        let (lo, hi) = (ys.iter().cloned().fold(f64::MAX, f64::min), ys.iter().cloned().fold(f64::MIN, f64::max));
        assert!((lo - (dev.area.y - 80.0)).abs() < 1e-6 && (hi - (dev.area.y - 60.0)).abs() < 1e-6, "{lo} {hi}");
        assert!(prog.bounds.is_some());
    }

    #[test]
    fn the_choice_of_the_project_stays_when_the_view_goes_back() {
        let mut v = View3d::default();
        assert!(!v.show_project, "it is off until asked for");
        v.show_project = true;
        v.reset();
        assert!(v.show_project);
    }

    #[test]
    fn the_navigation_cube_sets_the_camera_from_a_direction() {
        let mut v = View3d::default();
        // From the front: the eye is at -Y, low.
        v.look_from([0.0, -1.0, 0.0]);
        assert!(v.yaw.abs() < 1e-5 && v.pitch == MIN_PITCH);
        // From the right-hand side (+X) the yaw is a quarter turn.
        v.look_from([1.0, 0.0, 0.0]);
        assert!((v.yaw - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
        // From a corner above: a pitch of about 35 degrees.
        v.look_from([0.577, -0.577, 0.577]);
        assert!((v.pitch - 0.6155).abs() < 1e-3 && v.yaw > 0.0);
        // The camera of the picture agrees with the cube: the direction to the eye is the one that was asked for.
        let (eye, _, _) = v.axes();
        for (got, want) in eye.iter().zip([0.57735, -0.57735, 0.57735]) {
            assert!((got - want).abs() < 0.01, "{eye:?}");
        }
        // From straight above or below it goes as far as the camera does and keeps the yaw.
        let yaw = v.yaw;
        v.look_from([0.0, 0.0, 1.0]);
        assert_eq!((v.yaw, v.pitch), (yaw, MAX_PITCH));
        v.look_from([0.0, 0.0, -1.0]);
        assert_eq!(v.pitch, MIN_PITCH);
        // The plan is not turned.
        let mut plan = View3d { cam: Camera { top: true, ..Camera::default() }, ..View3d::default() };
        plan.look_from([1.0, 0.0, 0.0]);
        assert_eq!((plan.yaw, plan.pitch), (START_YAW, START_PITCH));
    }

    #[test]
    fn the_view_turns_zooms_and_goes_back() {
        let mut v = View3d::default();
        v.show_rapids = false;
        v.rotate(30.0, 20.0);
        v.zoom_by(2.0);
        v.pan = Vec2::new(5.0, 5.0);
        assert!(v != View3d { show_rapids: false, ..View3d::default() });
        v.reset();
        assert_eq!(v, View3d { show_rapids: false, ..View3d::default() }, "back to the first view, the choices stay");
        // The tilt cannot go under the block or over the top.
        v.rotate(0.0, 10_000.0);
        assert!(v.pitch <= MAX_PITCH);
        v.rotate(0.0, -10_000.0);
        assert!(v.pitch >= MIN_PITCH);
        v.zoom_by(1e9);
        assert!(v.zoom <= 40.0);
    }

    #[test]
    fn from_straight_above_the_plan_is_drawn_as_on_the_map_and_cannot_be_turned() {
        let area = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        let mut v = View3d { cam: Camera { top: true, ..Camera::default() }, ..View3d::default() };
        v.rotate(200.0, 100.0);
        assert_eq!((v.yaw, v.pitch), (START_YAW, START_PITCH), "turning does nothing");
        let pj = Projector::new(&v, [0.0, 0.0, 0.0], 50.0, area);
        let o = pj.project([0.0, 0.0, 0.0]);
        let right = pj.project([10.0, 0.0, 0.0]);
        let away = pj.project([0.0, 10.0, 0.0]);
        let high = pj.project([0.0, 0.0, 10.0]);
        assert!(right.x > o.x && (right.y - o.y).abs() < 1e-3, "X to the right");
        assert!(away.y < o.y && (away.x - o.x).abs() < 1e-3, "Y up the screen");
        assert!((high - o).length() < 1e-3, "height does not move the point");
        v.reset();
        assert!(v.top, "the choice of the view stays");
    }

    #[test]
    fn the_target_is_in_the_middle_and_turning_moves_the_sides() {
        let area = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        let mut v = View3d::default();
        let pj = Projector::new(&v, [10.0, 20.0, -3.0], 50.0, area);
        let c = pj.project([10.0, 20.0, -3.0]);
        assert!((c - area.center()).length() < 1e-3);
        // Seen from the front, a point to the right of the target is to the right on the screen.
        v.yaw = 0.0;
        let pj = Projector::new(&v, [0.0, 0.0, 0.0], 50.0, area);
        assert!(pj.project([10.0, 0.0, 0.0]).x > pj.project([0.0, 0.0, 0.0]).x);
        // A higher point is higher on the screen (smaller y).
        assert!(pj.project([0.0, 0.0, 10.0]).y < pj.project([0.0, 0.0, 0.0]).y);
        // Zoom makes distances larger.
        let d1 = (pj.project([10.0, 0.0, 0.0]) - pj.project([0.0, 0.0, 0.0])).length();
        v.zoom_by(2.0);
        let pj = Projector::new(&v, [0.0, 0.0, 0.0], 50.0, area);
        let d2 = (pj.project([10.0, 0.0, 0.0]) - pj.project([0.0, 0.0, 0.0])).length();
        assert!((d2 / d1 - 2.0).abs() < 1e-3);
    }

    #[test]
    fn from_above_only_the_top_face_is_seen_and_from_the_front_the_front_one_too() {
        let visible = |v: &View3d| {
            let (eye, _, _) = v.axes();
            faces([0.0; 3], [1.0; 3]).iter().filter(|(_, n, _)| n[0] * eye[0] + n[1] * eye[1] + n[2] * eye[2] > 1e-3).count()
        };
        let top = View3d { cam: Camera { pitch: 1.5707, yaw: 0.0, ..Camera::default() }, ..View3d::default() };
        assert_eq!(visible(&top), 1);
        let front = View3d { cam: Camera { pitch: 0.4, yaw: 0.0, ..Camera::default() }, ..View3d::default() };
        assert_eq!(visible(&front), 2);
        let corner = View3d { cam: Camera { pitch: 0.4, yaw: 0.7, ..Camera::default() }, ..View3d::default() };
        assert_eq!(visible(&corner), 3);
    }

    #[test]
    fn the_position_follows_the_part_of_the_program() {
        let (prog, ..) = job();
        let (n, end) = position_at(&prog, 1.0);
        assert_eq!(n, prog.moves.len());
        assert_eq!(end, prog.moves.last().map(|m| m.to));
        let (n0, p0) = position_at(&prog, 0.0);
        assert!(n0 <= 1 && p0.is_some());
        let (nh, _) = position_at(&prog, 0.5);
        assert!(nh > 0 && nh < prog.moves.len());
        assert!(position_at(&Program::default(), 0.5).1.is_none());
    }

    #[test]
    fn the_picture_is_drawn_for_a_job() {
        let (prog, dev, stock, layers, _lib) = job();
        let region = Region::of(&prog, None).unwrap();
        let sc = Scene3d { surface: None, prog: &prog, region, stock: &stock, dev: &dev, layers: &layers, project: &[], rotary: None, parts: &base_ui::dims::Features::default(), units: base_core::units::Units::Mm, bottom: None, solid: None, quality: 3 };
        let ctx = egui::Context::default();
        let mut view = View3d::default();
        view.progress = 0.5;
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            show(ui, &mut view, &sc);
        });
        out.textures_delta.clear();
        assert!(!out.shapes.is_empty());
    }

    #[test]
    fn a_rotary_wraps_y_round_the_workpiece_with_the_top_of_the_material_on_its_surface() {
        let c = Cyl { r: 10.0, cy: 50.0 };
        let near = |a: P3, b: P3| a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < 1e-9);
        // The middle of the job faces up and the surface is at z = 0; deeper is nearer the axis.
        assert!(near(c.map([5.0, 50.0, 0.0]), [5.0, 50.0, 0.0]));
        assert!(near(c.map([5.0, 50.0, -2.0]), [5.0, 50.0, -2.0]));
        // A quarter of the way round (the arc is r * pi / 2 long) is at the side, at the height of the axis.
        let q = c.map([5.0, 50.0 + 10.0 * std::f64::consts::FRAC_PI_2, 0.0]);
        assert!(near(q, [5.0, 60.0, -10.0]), "{q:?}");
        // Half way round it is under the axis; the same point as a quarter and a quarter.
        let h = c.map([5.0, 50.0 + 10.0 * std::f64::consts::PI, 0.0]);
        assert!(near(h, [5.0, 50.0, -20.0]) || (h[1] - 50.0).abs() < 1e-6 && (h[2] + 20.0).abs() < 1e-6, "{h:?}");
        // The outward direction is up at the middle and sideways a quarter away.
        assert_eq!(c.normal(50.0), [0.0, 0.0, 1.0]);
        // A long cut is a curve of short pieces that all lie on the radius of its depth.
        let curve = c.curve([0.0, 50.0, -1.0], [20.0, 50.0 + 10.0 * std::f64::consts::PI, -1.0]);
        assert!(curve.len() > 20);
        for p in &curve {
            let r = ((p[1] - 50.0).powi(2) + (p[2] + 10.0).powi(2)).sqrt();
            assert!((r - 9.0).abs() < 1e-9, "{r}");
        }
        // A straight cut along X is not cut into pieces.
        assert_eq!(c.curve([0.0, 50.0, -1.0], [20.0, 50.0, -1.0]).len(), 2);
        // The region is the whole way round, centred on the job.
        let (r, cy) = Region { min: [0.0, 40.0], max: [100.0, 60.0] }.round(2.0 * std::f64::consts::PI * 10.0);
        assert!((r.size()[1] - 62.83185307).abs() < 1e-6 && cy == 50.0 && r.size()[0] == 100.0);
    }

    /// Moves that follow each other are one polyline; a straight run is one piece, the corners stay, points that are too close are
    /// left out and what is out of sight is not drawn.
    #[test]
    fn many_short_moves_are_few_lines() {
        let clip = egui::Rect::from_min_size(Pos2::ZERO, egui::vec2(800.0, 600.0));
        let st = Stroke::new(1.0, Color32::RED);
        let mut l = Lines::new(clip);
        // A straight row of a thousand moves a tenth of a pixel to 0.6 pixels long, then a corner and a short way down.
        let mut x = 10.0;
        for k in 0..1000 {
            let step = if k % 2 == 0 { 0.1 } else { 0.6 };
            l.add(Pos2::new(x, 100.0), Pos2::new(x + step, 100.0), st, false);
            x += step;
        }
        for k in 0..10 {
            l.add(Pos2::new(x, 100.0 + k as f32), Pos2::new(x, 101.0 + k as f32), st, false);
        }
        // Another colour, and a move wholly out of sight.
        l.add(Pos2::new(x, 110.0), Pos2::new(x + 5.0, 110.0), Stroke::new(1.0, Color32::BLUE), false);
        l.add(Pos2::new(-50.0, -50.0), Pos2::new(-10.0, -60.0), st, false);
        l.flush();
        assert_eq!(l.shapes.len(), 2, "one line for each stroke: {:?}", l.shapes);
        let Shape::Path(p) = &l.shapes[0] else { panic!("a polyline") };
        assert_eq!(p.points.len(), 3, "the row, the corner and the end: {:?}", p.points);
        assert_eq!(p.points[0], Pos2::new(10.0, 100.0));
        assert!((p.points[1].x - x).abs() < 1e-3 && (p.points[2].y - 110.0).abs() < 1e-3);
    }

    #[test]
    fn the_cylinder_is_drawn_plain_and_carved_from_every_side_and_the_view_fits_it() {
        let (prog, mut dev, stock, layers, _lib) = job();
        dev.rotary.enabled = true;
        dev.rotary.diameter = 30.0;
        let region = Region::of(&prog, None).unwrap();
        let (region, cy) = region.round(dev.rotary.circumference());
        let rotary = Some(Cyl { r: 15.0, cy });
        let ctx = egui::Context::default();
        let mut view = View3d::default();
        view.progress = 0.6;
        let mut counts = vec![];
        for (yaw, top) in [(0.3f32, false), (2.0, false), (4.0, false), (0.0, true)] {
            view.yaw = yaw;
            view.top = top;
            let surface = {
                let moves = &prog.moves;
                let lib = Library::new();
                let tool_of = |l: usize| layers.get(l).and_then(|l| l.tool.as_deref()).and_then(|id| lib.get(id)).cloned();
                Surface::of((0, 0, [0; 5]), moves, tool_of, region, 15.0, 1.0)
            };
            for sf in [None, surface.as_ref()] {
                let sc = Scene3d { surface: sf, prog: &prog, region, stock: &stock, dev: &dev, layers: &layers, project: &[], rotary, parts: &base_ui::dims::Features::default(), units: base_core::units::Units::Mm, bottom: None, solid: None, quality: 3 };
                view.fit = Some(Fit::Elements);
                let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                    show(ui, &mut view, &sc);
                });
                out.textures_delta.clear();
                counts.push(out.shapes.len());
            }
        }
        assert!(counts.iter().all(|n| *n > 0), "{counts:?}");
        assert!(view.zoom > 0.2 && view.zoom.is_finite());
    }
}
#[cfg(test)]
mod through_tests {
    /// A cut the whole thickness goes through; so do two cuts from both faces that meet; a shallower one does not.
    #[test]
    fn a_cut_through_the_material_is_a_hole_and_so_are_two_that_meet() {
        use crate::toolpath::generate;
        use base_core::geom::Xf;
        use base_core::scene::{Kind, Scene};
        let lib = crate::tools::Library::new();
        let dev = crate::device::Device::dummy();
        let mut stock = crate::materials::Stock::default();
        stock.thickness = 10.0;
        let mut layers = crate::layers::default_layers(&lib);
        layers[2].depth = 6.0;
        let mut scene = Scene::default();
        scene.add(2, Kind::Rect { w: 30.0, h: 30.0 }, Xf::translate(20.0, 20.0));
        let prog = generate(&scene, &layers, &lib, &dev, &stock);
        let region = super::Region::of(&prog, None).unwrap();
        let tool_of = |l: usize| layers.get(l).and_then(|l| l.tool.as_deref()).and_then(|id| lib.get(id)).cloned();
        let s = super::Surface::of((0, 0, [0; 5]), &prog.moves, tool_of, region, 10.0, 1.0).unwrap();
        // The middle of the pocket, in the coordinates of the program (Y up from the bottom of the work area).
        let (x, y) = (35.0, dev.area.y - 35.0);
        assert!((s.depth_at(x, y) - 6.0).abs() < 1e-6, "{}", s.depth_at(x, y));
        assert!(!super::cut_through(s.depth_at(x, y), None, x, y, 10.0), "6 of 10 mm");
        assert!(super::cut_through(s.depth_at(x, y), Some(&s), x, y, 10.0), "6 from each face meet");
        assert!(super::cut_through(10.0, None, x, y, 10.0), "the whole thickness");
        assert_eq!(s.depth_at(-1000.0, 0.0), 0.0, "off the grid");
        assert!(s.flat().z.iter().all(|z| *z == 0.0));
    }

    /// A higher quality makes the grid of the carved surface finer, also with a large bit (it was half the bit, whatever the
    /// quality); between the points of the grid the depth goes over smoothly, with no steps.
    #[test]
    fn a_higher_quality_makes_a_finer_surface_and_its_depth_has_no_steps() {
        let (prog, _dev, _stock, layers, lib) = super::tests::job();
        let region = super::Region::of(&prog, None).unwrap();
        let tool_of = |l: usize| layers.get(l).and_then(|l| l.tool.as_deref()).and_then(|id| lib.get(id)).cloned();
        let steps: Vec<f64> = [1u8, 3, 5].iter().map(|q| super::Surface::of((0, 0, [0; 5]), &prog.moves, tool_of, region, 10.0, base_ui::view3d::quality_detail(*q)).unwrap().step).collect();
        assert!(steps[0] > steps[1] && steps[1] > steps[2], "{steps:?}");
        let s = super::Surface { key: (0, 0, [0; 5]), cols: 2, rows: 1, step: 1.0, lo: [0.0, 0.0], top: 1.0, z: vec![0.0, -4.0], light: vec![1.0; 2] };
        assert_eq!(s.depth_at(0.5, 0.5), 0.0);
        assert_eq!(s.depth_at(1.5, 0.5), 4.0);
        assert!((s.depth_at(1.0, 0.5) - 2.0).abs() < 1e-9, "half way between: {}", s.depth_at(1.0, 0.5));
        assert_eq!(s.depth_at(2.5, 0.5), 0.0, "off the grid");
    }

    /// A piece cut round falls out: the points inside it go through the material, those outside stay as they were.
    #[test]
    fn a_piece_cut_round_falls_out() {
        use base_core::geom::{Polyline, Pt};
        let region = super::Region { min: [0.0, 0.0], max: [20.0, 20.0] };
        let mut s = super::Surface { key: (0, 0, [0; 5]), cols: 20, rows: 20, step: 1.0, lo: region.min, top: 20.0, z: vec![-1.0; 400], light: vec![1.0; 400] };
        let square = Polyline::new(vec![Pt::new(5.0, 5.0), Pt::new(15.0, 5.0), Pt::new(15.0, 15.0), Pt::new(5.0, 15.0)], true);
        s.fall_out(&[square], 10.0);
        assert_eq!(s.depth_at(10.0, 10.0), 10.0);
        assert_eq!(s.depth_at(2.0, 2.0), 1.0);
        assert!(super::cut_through(s.depth_at(10.0, 10.0), None, 10.0, 10.0, 10.0));
    }
}
