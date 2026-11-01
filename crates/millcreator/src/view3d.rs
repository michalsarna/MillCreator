//! The 3D view of the job: the block of material, the cuts and the moves above it. The view turns with the mouse, zooms
//! with the wheel and goes back to its first position with a button. It shows only the area of the project, ten per cent
//! larger than the project, so the picture is not lost in the work area of the machine.
use crate::device::Device;
use crate::layers::MillLayer;
use crate::materials::Stock;
use crate::toolpath::{MoveKind, Program, P3};
use egui::{epaint::PathStroke, Color32, Pos2, Rect, Sense, Shape, Stroke, Vec2};

/// How much larger than the project the shown area is.
pub const MARGIN: f64 = 0.10;
const START_YAW: f32 = 0.55;
const START_PITCH: f32 = 0.62;
const MIN_PITCH: f32 = 0.03;
const MAX_PITCH: f32 = 1.5;

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

/// How the model is looked at.
#[derive(Clone, Debug, PartialEq)]
pub struct View3d {
    pub yaw: f32,
    pub pitch: f32,
    /// 1.0 = the whole block fits in the window.
    pub zoom: f32,
    pub pan: Vec2,
    pub show_cuts: bool,
    pub show_rapids: bool,
    pub show_stock: bool,
    /// How much of the program is shown, 0..=1.
    pub progress: f32,
    /// Looked at from straight above: the plan of the job. It cannot be turned.
    pub top: bool,
    /// Asked for by a button, done by the next picture (which knows the size of its window).
    pub fit: Option<Fit>,
}

/// What the view is fitted to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fit {
    /// The whole work area of the machine.
    Area,
    /// The cuts of the program (the shapes when there are none yet).
    Elements,
}

impl Default for View3d {
    fn default() -> View3d {
        View3d { yaw: START_YAW, pitch: START_PITCH, zoom: 1.0, pan: Vec2::ZERO, show_cuts: true, show_rapids: true, show_stock: true, progress: 1.0, top: false, fit: None }
    }
}

impl View3d {
    /// Back to the first position of the view (the choices of what to show stay).
    pub fn reset(&mut self) {
        let keep = View3d { show_cuts: self.show_cuts, show_rapids: self.show_rapids, show_stock: self.show_stock, top: self.top, ..View3d::default() };
        *self = keep;
    }

    pub fn rotate(&mut self, dx: f32, dy: f32) {
        if self.top {
            return;
        }
        self.yaw = (self.yaw - dx * 0.01).rem_euclid(std::f32::consts::TAU);
        self.pitch = (self.pitch + dy * 0.01).clamp(MIN_PITCH, MAX_PITCH);
    }

    /// Look at the model from a direction of the world (from the model to the eye), as the navigation cube asks. The camera cannot
    /// go below the horizon or straight above (the plan has its own button), so those are as near as it goes.
    pub fn look_from(&mut self, d: [f32; 3]) {
        if self.top {
            return;
        }
        self.pitch = d[2].clamp(-1.0, 1.0).asin().clamp(MIN_PITCH, MAX_PITCH);
        if d[0].hypot(d[1]) > 1e-4 {
            self.yaw = d[0].atan2(-d[1]).rem_euclid(std::f32::consts::TAU);
        }
    }

    pub fn zoom_by(&mut self, factor: f32) {
        self.zoom = (self.zoom * factor).clamp(0.2, 40.0);
    }

    /// The direction from the model to the eye, and the right and up directions of the picture.
    fn axes(&self) -> ([f32; 3], [f32; 3], [f32; 3]) {
        let (yaw, pitch) = if self.top { (0.0, std::f32::consts::FRAC_PI_2) } else { (self.yaw, self.pitch) };
        let (sy, cy, sp, cp) = (yaw.sin(), yaw.cos(), pitch.sin(), pitch.cos());
        ([sy * cp, -cy * cp, sp], [cy, sy, 0.0], [-sy * sp, cy * sp, cp])
    }
}

/// Turns points of the model into points of the picture.
pub struct Projector {
    right: [f32; 3],
    up: [f32; 3],
    target: [f32; 3],
    scale: f32,
    centre: Pos2,
}

impl Projector {
    pub fn new(v: &View3d, target: P3, radius: f64, area: Rect) -> Projector {
        let (_, right, up) = v.axes();
        let fit = area.width().min(area.height()) / (2.0 * radius.max(1.0) as f32) * 0.92;
        Projector { right, up, target: [target[0] as f32, target[1] as f32, target[2] as f32], scale: fit * v.zoom, centre: area.center() + v.pan }
    }
    pub fn project(&self, p: P3) -> Pos2 {
        let d = [p[0] as f32 - self.target[0], p[1] as f32 - self.target[1], p[2] as f32 - self.target[2]];
        let dot = |a: &[f32; 3]| a[0] * d[0] + a[1] * d[1] + a[2] * d[2];
        Pos2::new(self.centre.x + dot(&self.right) * self.scale, self.centre.y - dot(&self.up) * self.scale)
    }
}

fn shade(c: [u8; 3], f: f32) -> Color32 {
    let m = |v: u8| ((v as f32) * f).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgb(m(c[0]), m(c[1]), m(c[2]))
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
    pub fn of(key: (u64, usize, [u64; 5]), moves: &[crate::toolpath::Move], tool_of: impl Fn(usize) -> Option<crate::tools::Tool>, region: Region, thickness: f64) -> Option<Surface> {
        let (cols, rows, step, z) = crate::relief::carved_surface(moves, tool_of, (region.min, region.max), thickness, crate::relief::Limits::VIEW);
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
}

/// What the 3D window needs from the project.
pub struct Scene3d<'a> {
    pub surface: Option<&'a Surface>,
    pub prog: &'a Program,
    pub region: Region,
    pub stock: &'a Stock,
    pub dev: &'a Device,
    pub layers: &'a [MillLayer],
    /// A workpiece on a rotary attachment (the picture shows the cylinder).
    pub rotary: Option<Cyl>,
}

/// The middle of the picture and the radius of the sphere that holds what is shown, the same for the picture and for the fitting.
fn framing(sc: &Scene3d) -> (P3, f64) {
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
    if resp.dragged_by(egui::PointerButton::Primary) {
        let d = resp.drag_delta();
        view.rotate(d.x, d.y);
    }
    if resp.dragged_by(egui::PointerButton::Secondary) || resp.dragged_by(egui::PointerButton::Middle) {
        view.pan += resp.drag_delta();
    }
    if resp.hovered() {
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll != 0.0 {
            view.zoom_by((scroll * 0.004).exp());
        }
    }
    if resp.double_clicked() {
        view.reset();
    }
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
    painter.rect_filled(rect, 4.0, if ui.visuals().dark_mode { Color32::from_rgb(0x1c, 0x1e, 0x24) } else { Color32::from_rgb(0xe9, 0xec, 0xf1) });
    draw(&painter, rect, view, sc);
    // The navigation cube (the plan from above cannot be turned).
    if !view.top {
        let (eye, right, up) = view.axes();
        match base_ui::navicube::show(ui, rect, &[right, up, eye]) {
            Some(base_ui::navicube::Nav::LookFrom(d)) => view.look_from(d),
            Some(base_ui::navicube::Nav::Home) => view.reset(),
            None => {}
        }
    }
    // The slider of the zoom, below the cube: the same as the wheel, for a mouse without one.
    let track = Rect::from_min_max(Pos2::new(rect.right() - 28.0, rect.top() + 150.0), Pos2::new(rect.right() - 8.0, (rect.bottom() - 24.0).max(rect.top() + 200.0)));
    let mut z = view.zoom.max(0.2).ln();
    let (lo, hi) = (0.2f32.ln(), 40.0f32.ln());
    let r = ui.put(track, egui::Slider::new(&mut z, lo..=hi).vertical().show_value(false));
    if r.on_hover_text(base_ui::i18n::tr("Zoom")).changed() {
        view.zoom_by((z - view.zoom.max(0.2).ln()).exp());
    }
}

/// Zoom and pan so that the box `lo`..`hi` (X and Y, from the material's top down to the safe height) fills `rect`.
pub fn fit_to(view: &mut View3d, rect: Rect, sc: &Scene3d, lo: [f64; 2], hi: [f64; 2]) {
    let t = sc.stock.thickness;
    let (target, radius) = framing(sc);
    // The picture at zoom 1 and no pan: its points are a fixed multiple of their distance from the middle of the window.
    let base = View3d { zoom: 1.0, pan: Vec2::ZERO, fit: None, ..view.clone() };
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

/// The surface as a mesh of triangles, the far ones first (a height field is drawn correctly from any side that way).
fn draw_surface(painter: &egui::Painter, pj: &Projector, s: &Surface, color: [u8; 3], eye: [f32; 3]) {
    let at = |c: usize, r: usize| [s.lo[0] + (c as f64 + 0.5) * s.step, s.top - (r as f64 + 0.5) * s.step, s.z[r * s.cols + c]];
    let mut mesh = egui::epaint::Mesh::default();
    mesh.vertices.reserve(s.cols * s.rows);
    for r in 0..s.rows {
        for c in 0..s.cols {
            mesh.colored_vertex(pj.project(at(c, r)), shade(color, s.light[r * s.cols + c]));
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
    } else if view.show_stock {
        for (corners, normal, light) in faces(min, max) {
            if normal[0] * eye[0] + normal[1] * eye[1] + normal[2] * eye[2] <= 0.0 {
                continue;
            }
            let pts: Vec<Pos2> = corners.iter().map(|c| pj.project(*c)).collect();
            painter.add(Shape::convex_polygon(pts, shade(sc.stock.color, light), Stroke::new(1.0, shade(sc.stock.color, light * 0.6))));
        }
        // The cuts made so far, carved into the top of the stock (the whole top when nothing is cut yet).
        if let Some(s) = sc.surface {
            draw_surface(painter, &pj, s, sc.stock.color, eye);
        }
    }
    let (done, here) = position_at(sc.prog, view.progress);
    let cut_color = |layer: usize| sc.layers.get(layer).map(|l| l.color32()).unwrap_or(Color32::RED);
    if view.show_cuts {
        // The paths of the bit in the colour of the layer, over the carved stock.
        for m in sc.prog.moves.iter().take(done) {
            if m.kind == MoveKind::Rapid || m.to[2] > 0.0 {
                continue;
            }
            painter.add(Shape::line(line(m.from, m.to), Stroke::new(1.2, cut_color(m.layer))));
        }
    }
    if view.show_rapids {
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
        for m in sc.prog.moves.iter().take(done) {
            if m.kind == MoveKind::Cut || (m.kind == MoveKind::Plunge && m.to[2] <= 0.0 && m.from[2] <= 0.0) {
                continue;
            }
            painter.extend(Shape::dashed_line(&line(m.from, m.to), Stroke::new(1.0, rapid), 5.0, 4.0));
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
    use crate::layers::default_layers;
    use crate::tools::Library;
    use crate::toolpath::generate;
    use base_core::geom::Xf;
    use base_core::scene::{Kind, Scene};

    fn job() -> (Program, Device, Stock, Vec<MillLayer>, Library) {
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
        let mut plan = View3d { top: true, ..View3d::default() };
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
        let mut v = View3d { top: true, ..View3d::default() };
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
        let top = View3d { pitch: 1.5707, yaw: 0.0, ..View3d::default() };
        assert_eq!(visible(&top), 1);
        let front = View3d { pitch: 0.4, yaw: 0.0, ..View3d::default() };
        assert_eq!(visible(&front), 2);
        let corner = View3d { pitch: 0.4, yaw: 0.7, ..View3d::default() };
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
        let sc = Scene3d { surface: None, prog: &prog, region, stock: &stock, dev: &dev, layers: &layers, rotary: None };
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
                Surface::of((0, 0, [0; 5]), moves, tool_of, region, 15.0)
            };
            for sf in [None, surface.as_ref()] {
                let sc = Scene3d { surface: sf, prog: &prog, region, stock: &stock, dev: &dev, layers: &layers, rotary };
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
