//! The 3D view of the job: the block of material, the cuts and the moves above it. The view turns with the mouse, zooms
//! with the wheel and goes back to its first position with a button. It shows only the area of the project, ten per cent
//! larger than the project, so the picture is not lost in the work area of the machine.
use crate::device::Device;
use crate::layers::MillLayer;
use crate::materials::Stock;
use crate::toolpath::{MoveKind, Program, P3};
use crate::tools::Library;
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
    pub fn scale(&self) -> f32 {
        self.scale
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

/// What the 3D window needs from the project.
pub struct Scene3d<'a> {
    pub prog: &'a Program,
    pub region: Region,
    pub stock: &'a Stock,
    pub dev: &'a Device,
    pub layers: &'a [MillLayer],
    pub tools: &'a Library,
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
}

/// Zoom and pan so that the box `lo`..`hi` (X and Y, from the material's top down to the safe height) fills `rect`.
pub fn fit_to(view: &mut View3d, rect: Rect, sc: &Scene3d, lo: [f64; 2], hi: [f64; 2]) {
    let r = sc.region;
    let [w, h] = r.size();
    let t = sc.stock.thickness;
    let radius = ((w * w + h * h + (t + sc.dev.safe_z).powi(2)).sqrt()) / 2.0;
    let target = [r.centre()[0], r.centre()[1], -t / 2.0];
    // The picture at zoom 1 and no pan: its points are a fixed multiple of their distance from the middle of the window.
    let base = View3d { zoom: 1.0, pan: Vec2::ZERO, fit: None, ..view.clone() };
    let pj = Projector::new(&base, target, radius, rect);
    let (mut min, mut max) = (Pos2::new(f32::MAX, f32::MAX), Pos2::new(f32::MIN, f32::MIN));
    for x in [lo[0], hi[0]] {
        for y in [lo[1], hi[1]] {
            for z in [-t, 0.0] {
                let p = pj.project([x, y, z]);
                min = min.min(p);
                max = max.max(p);
            }
        }
    }
    let (bw, bh) = ((max.x - min.x).max(1.0), (max.y - min.y).max(1.0));
    let zoom = (rect.width() / bw).min(rect.height() / bh) * 0.92;
    let offset = (min + (max - min) / 2.0) - rect.center();
    view.zoom = zoom.clamp(0.2, 40.0);
    view.pan = -offset * view.zoom;
}

/// Draw the model into `rect`.
pub fn draw(painter: &egui::Painter, rect: Rect, view: &View3d, sc: &Scene3d) {
    let r = sc.region;
    let t = sc.stock.thickness;
    let safe = sc.dev.safe_z;
    let [w, h] = r.size();
    let target = [r.centre()[0], r.centre()[1], -t / 2.0];
    // The sphere that holds the block and the height of the moves above it.
    let radius = ((w * w + h * h + (t + safe).powi(2)).sqrt()) / 2.0;
    let pj = Projector::new(view, target, radius, rect);
    let (eye, _, _) = view.axes();
    let min = [r.min[0], r.min[1], -t];
    let max = [r.max[0], r.max[1], 0.0];
    if view.show_stock {
        for (corners, normal, light) in faces(min, max) {
            if normal[0] * eye[0] + normal[1] * eye[1] + normal[2] * eye[2] <= 0.0 {
                continue;
            }
            let pts: Vec<Pos2> = corners.iter().map(|c| pj.project(*c)).collect();
            painter.add(Shape::convex_polygon(pts, shade(sc.stock.color, light), Stroke::new(1.0, shade(sc.stock.color, light * 0.6))));
        }
    }
    let (done, here) = position_at(sc.prog, view.progress);
    let cut_color = |layer: usize| sc.layers.get(layer).map(|l| l.color32()).unwrap_or(Color32::RED);
    let groove = |layer: usize, z: f64| {
        let tool = sc.layers.get(layer).and_then(|l| l.tool.as_deref()).and_then(|id| sc.tools.get(id));
        tool.map_or(1.0, |tl| tl.cut_width(-z))
    };
    if view.show_cuts {
        // The grooves first (as wide as the bit), then the centre lines in the colour of the layer.
        let dark = shade(sc.stock.color, 0.28);
        for m in sc.prog.moves.iter().take(done) {
            if m.kind == MoveKind::Rapid || m.to[2] > 0.0 {
                continue;
            }
            let wd = (groove(m.layer, m.to[2]) as f32 * pj.scale()).clamp(1.5, 400.0);
            painter.line_segment([pj.project(m.from), pj.project(m.to)], Stroke::new(wd, dark));
        }
        for m in sc.prog.moves.iter().take(done) {
            if m.kind == MoveKind::Rapid || m.to[2] > 0.0 {
                continue;
            }
            painter.line_segment([pj.project(m.from), pj.project(m.to)], Stroke::new(1.2, cut_color(m.layer)));
        }
    }
    if view.show_rapids {
        // The safe height as a frame over the block, and the moves above the material as dashed lines.
        let z = safe;
        let ring = [[r.min[0], r.min[1], z], [r.max[0], r.min[1], z], [r.max[0], r.max[1], z], [r.min[0], r.max[1], z], [r.min[0], r.min[1], z]];
        let pts: Vec<Pos2> = ring.iter().map(|c| pj.project(*c)).collect();
        painter.add(Shape::line(pts, PathStroke::new(1.0, Color32::from_rgba_unmultiplied(0x40, 0x90, 0xe0, 120))));
        let rapid = Color32::from_rgb(0x30, 0x7f, 0xd0);
        for m in sc.prog.moves.iter().take(done) {
            if m.kind == MoveKind::Cut || (m.kind == MoveKind::Plunge && m.to[2] <= 0.0 && m.from[2] <= 0.0) {
                continue;
            }
            let a = pj.project(m.from);
            let b = pj.project(m.to);
            painter.extend(Shape::dashed_line(&[a, b], Stroke::new(1.0, rapid), 5.0, 4.0));
        }
    }
    if let Some(p) = here {
        if !sc.prog.is_empty() && view.progress < 1.0 {
            let top = pj.project([p[0], p[1], p[2] + 12.0]);
            let tip = pj.project(p);
            painter.line_segment([top, tip], Stroke::new(3.0, Color32::from_rgb(0xe0, 0x50, 0x20)));
            painter.circle_filled(tip, 3.5, Color32::from_rgb(0xe0, 0x50, 0x20));
        }
    }
    // The axes at the corner of the block.
    let o = [r.min[0], r.min[1], 0.0];
    let len = (w.min(h) * 0.2).max(5.0);
    for (d, name, color) in [([len, 0.0, 0.0], "X", Color32::from_rgb(0xd0, 0x30, 0x30)), ([0.0, len, 0.0], "Y", Color32::from_rgb(0x30, 0xa0, 0x30)), ([0.0, 0.0, len], "Z", Color32::from_rgb(0x30, 0x60, 0xd0))] {
        let a = pj.project(o);
        let b = pj.project([o[0] + d[0], o[1] + d[1], o[2] + d[2]]);
        painter.line_segment([a, b], Stroke::new(2.0, color));
        painter.text(b, egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(13.0), color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layers::default_layers;
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
        let (prog, dev, stock, layers, lib) = job();
        let region = Region::of(&prog, None).unwrap();
        let sc = Scene3d { prog: &prog, region, stock: &stock, dev: &dev, layers: &layers, tools: &lib };
        let ctx = egui::Context::default();
        let mut view = View3d::default();
        view.progress = 0.5;
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            show(ui, &mut view, &sc);
        });
        out.textures_delta.clear();
        assert!(!out.shapes.is_empty());
    }
}
