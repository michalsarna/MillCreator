//! **Technical drawing**: the part as a drawing of the European kind (first-angle projection) on a white sheet. The view from the front
//! is at the top left, the view from the left side to its right and the view from above under it. The block of material is drawn
//! with full lines, the outlines of the shapes that are milled in the view from above, and in the other two views the cuts under the
//! top as dashed (hidden) lines down to the depth of their layer. Dimensions give the size of the material, of the parts, where they
//! are from the corner of the material, and the depths; a title block names the project, the material and the scale for an A4 sheet.
use crate::App;
use base_core::geom::Pt;
use base_ui::dims::{self, DimStyle};
use base_ui::i18n::{tr, trf};
use egui::{Color32, Pos2, Stroke, Vec2};

/// The kind of technical drawing: how the views are laid out and how the numbers are written. Most of the world draws by ISO
/// (first-angle projection: the view from above goes under the view from the front, the view from the left to its right; the
/// numbers lie along their lines); the USA and Canada by ASME (third-angle projection: the view from above over the front, the view
/// from the right to its right; the numbers always upright); Japan by JIS (third-angle, the numbers along their lines).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Standard {
    #[default]
    Iso,
    Asme,
    Jis,
}

impl Standard {
    pub const ALL: [Standard; 3] = [Standard::Iso, Standard::Asme, Standard::Jis];
    pub fn label(self) -> &'static str {
        match self {
            Standard::Iso => "ISO 128 (first-angle: Europe, most of the world)",
            Standard::Asme => "ASME Y14 (third-angle: USA, Canada)",
            Standard::Jis => "JIS B 0001 (third-angle: Japan)",
        }
    }
    /// The short name in the title block.
    pub fn short(self) -> &'static str {
        match self {
            Standard::Iso => "ISO 128",
            Standard::Asme => "ASME Y14.5",
            Standard::Jis => "JIS B 0001",
        }
    }
    /// Third-angle projection (the view from above over the front one).
    pub fn third_angle(self) -> bool {
        self != Standard::Iso
    }
}

/// The parts of the drawing, in millimetres of the material: X right from its left edge, Y up from its front edge, Z up from its top
/// (cuts go below 0). A part is cut into a face of the object (the top for a job on one side; any face of a two-sided sheet, a post
/// or a block): `lines` are its outlines on that face, `inward` the way into the object from it.
pub struct Part {
    pub depth: f64,
    pub lines: Vec<Vec<[f64; 3]>>,
    pub inward: [f64; 3],
}

impl Part {
    /// The box of its lines seen from above.
    #[cfg(test)]
    pub fn bounds(&self) -> base_core::geom::Rect {
        let pts = self.lines.iter().flatten();
        let f = |k: usize, lo: bool| pts.clone().map(|q| q[k]).fold(if lo { f64::MAX } else { f64::MIN }, if lo { f64::min } else { f64::max });
        base_core::geom::Rect { min: Pt::new(f(0, true), f(1, true)), max: Pt::new(f(0, false), f(1, false)) }
    }
}

/// What the drawing shows.
pub struct Sheet {
    pub w: f64,
    pub h: f64,
    pub t: f64,
    pub parts: Vec<Part>,
    pub title: String,
    pub material: String,
    /// The units of the program (the numbers) and of the machine (beside them when the systems differ).
    pub units: base_core::units::Units,
    pub machine: base_core::units::Units,
    pub standard: Standard,
    /// The outlines and circles of the parts, for the dimensions of the view from above.
    pub features: base_ui::dims::Features,
}

/// The standard scales: the first one that fits the three views on an A4 sheet lying down (the drawing area of about 270 × 180 mm).
pub fn scale_for(w: f64, h: f64, t: f64) -> String {
    let need_x = w + h + 60.0;
    let need_y = t + h + 60.0;
    for (n, k) in [(5.0, "5:1"), (2.0, "2:1"), (1.0, "1:1"), (0.5, "1:2"), (0.2, "1:5"), (0.1, "1:10"), (0.05, "1:20"), (0.02, "1:50")] {
        if need_x * n <= 270.0 && need_y * n <= 180.0 {
            return k.to_string();
        }
    }
    "1:100".into()
}

/// A shape cut into the top, with `to` from the page to the drawing.
fn top_part(s: &base_core::scene::Shape, to: &dyn Fn(Pt) -> Pt, depth: f64) -> Option<Part> {
    let lines: Vec<Vec<[f64; 3]>> = s
        .polys()
        .iter()
        .map(|p| {
            let mut v: Vec<[f64; 3]> = p.pts.iter().map(|q| to(*q)).map(|q| [q.x, q.y, 0.0]).collect();
            if p.closed {
                if let Some(f) = v.first().copied() {
                    v.push(f);
                }
            }
            v
        })
        .filter(|v| v.len() >= 2)
        .collect();
    (!lines.is_empty()).then_some(Part { depth, lines, inward: [0.0, 0.0, -1.0] })
}

impl App {
    /// The sheet of the project: the material (from the setup, or the box of the shapes), and the shapes on the layers that are milled
    /// (all of them, shown or hidden in the editor). A job on more than one side draws the whole object with the parts of every face.
    pub(crate) fn drawing_sheet(&self) -> Sheet {
        let title = self.path.as_ref().and_then(|p| p.file_stem()).map_or_else(|| tr("Untitled").to_string(), |s| s.to_string_lossy().into_owned());
        let milled = |s: &&base_core::scene::Shape| self.layers.get(s.layer).is_some_and(|l| l.enabled);
        if let Some(solid) = self.solid() {
            return self.solid_sheet(&solid, title);
        }
        let shapes: Vec<&base_core::scene::Shape> = self.scene.shapes.iter().filter(milled).collect();
        // The material lies at the top left of the page; without a setup the box of the shapes stands for it.
        let (x0, y0, w, h) = if self.stock.width > 0.0 && self.stock.height > 0.0 {
            (0.0, 0.0, self.stock.width, self.stock.height)
        } else {
            let ids: Vec<u64> = shapes.iter().map(|s| s.id).collect();
            match self.scene.bounds_of(&ids) {
                Some(b) => (b.min.x, b.min.y, b.width().max(1.0), b.height().max(1.0)),
                None => (0.0, 0.0, self.device.area.x, self.device.area.y),
            }
        };
        // The page counts Y down from its top; the drawing counts it up from the front edge of the material.
        let to = |p: Pt| Pt::new(p.x - x0, h - (p.y - y0));
        let parts = shapes.iter().filter_map(|s| top_part(s, &to, self.layers[s.layer].depth.min(self.stock.thickness))).collect();
        let features = base_ui::dims::Features::of(shapes.iter().copied(), &to);
        Sheet { w, h, t: self.stock.thickness, parts, title, material: tr(self.stock.kind.label()).to_string(), units: base_ui::editor::EditorHost::units(self), machine: self.device.units, standard: self.drawing_standard, features }
    }

    /// The sheet of a job on more than one side: the box of the object as it lies for face A, the parts of every face where they are on
    /// it.
    fn solid_sheet(&self, solid: &base_core::faces::Solid, title: String) -> Sheet {
        let [bx, by, bz] = solid.bbox();
        // The space of the object (Y from the back to the front, Z up from the bottom) to that of the drawing.
        let to3 = |q: base_core::faces::V3| [q[0], by - q[1], q[2] - bz];
        let mut parts = vec![];
        for i in 0..solid.count() {
            let f = solid.face(i);
            // Into the object from the face, in the drawing (its Y the other way round).
            let inward = [-f.out[0], f.out[1], -f.out[2]];
            for s in self.face_scene(i).shapes.iter().filter(|s| self.layers.get(s.layer).is_some_and(|l| l.enabled)) {
                let depth = self.layers[s.layer].depth.min(f.depth);
                let lines: Vec<Vec<[f64; 3]>> = s
                    .polys()
                    .iter()
                    .map(|p| {
                        let mut v: Vec<[f64; 3]> = p.pts.iter().map(|q| to3(f.at(*q, 0.0))).collect();
                        if p.closed {
                            if let Some(first) = v.first().copied() {
                                v.push(first);
                            }
                        }
                        v
                    })
                    .collect();
                if !lines.is_empty() {
                    parts.push(Part { depth, lines, inward });
                }
            }
        }
        let tops: Vec<&base_core::scene::Shape> = self.face_scene(0).shapes.iter().filter(|s| self.layers.get(s.layer).is_some_and(|l| l.enabled)).collect();
        let features = base_ui::dims::Features::of(tops.into_iter(), &|p: Pt| Pt::new(p.x, by - p.y));
        let material = format!("{} · {}", tr(self.stock.kind.label()), base_ui::two_sided::job_title(solid));
        Sheet { w: bx, h: by, t: bz, parts, title, material, units: base_ui::editor::EditorHost::units(self), machine: self.device.units, standard: self.drawing_standard, features }
    }

    /// The window of the drawing (when it is not a tab of the work area).
    pub(crate) fn drawing_window(&mut self, ctx: &egui::Context) {
        if !self.drawing_open || self.drawing_docked {
            return;
        }
        let mut open = true;
        base_ui::window::popup(ctx, "technical_drawing", &tr("Technical drawing"), [980.0, 720.0], &mut open, |ui| self.drawing_body(ui));
        if !open {
            self.drawing_open = false;
        }
    }

    /// The drawing with its bar: the export, the place of the drawing (a tab or a window), and the picture that zooms and moves.
    pub(crate) fn drawing_body(&mut self, ui: &mut egui::Ui) {
        let sheet = self.drawing_sheet();
        ui.horizontal(|ui| {
            if ui.button(tr("Export…")).on_hover_text(tr("Save the drawing as a picture (PNG, JPG) or as a PDF page (A4)")).clicked() {
                self.export_drawing(&sheet);
            }
            ui.separator();
            ui.label(tr("Kind of drawing"));
            egui::ComboBox::from_id_salt("drawing_standard").selected_text(tr(self.drawing_standard.label())).show_ui(ui, |ui| {
                for k in Standard::ALL {
                    ui.selectable_value(&mut self.drawing_standard, k, tr(k.label()));
                }
            });
            ui.separator();
            if self.drawing_docked {
                if ui.button(tr("Open in a separate window")).clicked() {
                    self.drawing_docked = false;
                }
            } else if ui.button(tr("Dock as a tab")).on_hover_text(tr("Show the drawing as a tab of the work area, beside the drawings and the preview")).clicked() {
                self.drawing_docked = true;
            }
        });
        let size = base_ui::window::fill_size(ui).max(Vec2::new(300.0, 220.0));
        let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
        let (mut zoom, mut pan) = self.drawing_view;
        if resp.dragged() {
            pan += resp.drag_delta();
        }
        if resp.double_clicked() {
            (zoom, pan) = (1.0, Vec2::ZERO);
        }
        if resp.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                let f = (scroll * 0.0015).exp();
                let z = (zoom * f).clamp(0.3, 12.0);
                // About the pointer: the point under it stays there.
                if let Some(m) = resp.hover_pos() {
                    let from_centre = m - rect.center() - pan;
                    pan -= from_centre * (z / zoom - 1.0);
                }
                zoom = z;
            }
        }
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, base_ui::theme::panel_dark());
        // The sheet keeps the proportions of A4 lying down, as big as the room lets, then zoomed.
        let a4 = 297.0 / 210.0;
        let fit = if rect.width() / rect.height() > a4 { Vec2::new(rect.height() * a4, rect.height()) } else { Vec2::new(rect.width(), rect.width() / a4) } * 0.96;
        let sheet_rect = egui::Rect::from_center_size(rect.center() + pan, fit * zoom);
        paint(&painter, sheet_rect, &sheet);
        let step = base_ui::zoom_bar::STEP;
        match base_ui::zoom_bar::overlay_with(ui, rect, zoom, &tr("Drag: move · wheel: zoom"), false) {
            Some(base_ui::zoom_bar::ZoomCmd::In) => zoom = (zoom * step).min(12.0),
            Some(base_ui::zoom_bar::ZoomCmd::Out) => zoom = (zoom / step).max(0.3),
            Some(base_ui::zoom_bar::ZoomCmd::Whole) | Some(base_ui::zoom_bar::ZoomCmd::FitObjects) => (zoom, pan) = (1.0, Vec2::ZERO),
            None => {}
        }
        self.drawing_view = (zoom, pan);
    }

    /// Save the drawing: PNG or JPG at 300 dots per inch on an A4 sheet, or a PDF page of A4.
    fn export_drawing(&mut self, sheet: &Sheet) {
        let stem = self.drawing_file_stem();
        let Some(path) = rfd::FileDialog::new().add_filter("PDF", &["pdf"]).add_filter("PNG", &["png"]).add_filter("JPEG", &["jpg", "jpeg"]).set_file_name(format!("{stem}.pdf")).save_file() else { return };
        self.status = match save_sheet(sheet, &path) {
            Ok(()) => trf("Exported {}", &[&path.display()]),
            Err(e) => trf("Export failed: {}", &[&e]),
        };
    }

    fn drawing_file_stem(&self) -> String {
        self.path.as_ref().and_then(|p| p.file_stem()).map_or_else(|| "drawing".to_string(), |s| s.to_string_lossy().into_owned())
    }
}

/// The drawing as a file: an A4 sheet lying down (297 × 210 mm) at 300 dots per inch; the kind by the extension of the path.
pub fn save_sheet(sheet: &Sheet, path: &std::path::Path) -> Result<(), String> {
    // A4 in points of 1/96 inch, drawn at 300/96 pixels a point: 3508 × 2480 pixels.
    let size = Vec2::new(1122.5, 793.7);
    let img = base_ui::picture_out::render(size, 300.0 / 96.0, |ui| paint(ui.painter(), egui::Rect::from_min_size(Pos2::ZERO, size), sheet));
    base_ui::picture_out::save(&img, path)
}

/// Draw the sheet into `rect`.
pub fn paint(p: &egui::Painter, rect: egui::Rect, s: &Sheet) {
    let paper = Color32::WHITE;
    let ink = Color32::from_rgb(0x18, 0x18, 0x20);
    let st = DimStyle { ink, size: 11.0, units: s.units, second: None, upright: s.standard == Standard::Asme }.in_units(s.units, s.machine);
    p.rect_filled(rect, 0.0, paper);
    let frame = rect.shrink(10.0);
    p.rect_stroke(frame, 0.0, Stroke::new(1.4, ink), egui::StrokeKind::Inside);
    // The title block in the lower right corner.
    let tb = egui::Rect::from_min_max(Pos2::new(frame.right() - 300.0, frame.bottom() - 74.0), frame.right_bottom());
    p.rect_stroke(tb, 0.0, Stroke::new(1.2, ink), egui::StrokeKind::Inside);
    let row = tb.height() / 3.0;
    for i in 1..3 {
        p.line_segment([Pos2::new(tb.left(), tb.top() + row * i as f32), Pos2::new(tb.right(), tb.top() + row * i as f32)], Stroke::new(0.8, ink));
    }
    let font = |sz: f32| egui::FontId::proportional(sz);
    let units = trf("Dimensions in {}", &[&s.units.symbol()]);
    p.text(Pos2::new(tb.left() + 8.0, tb.top() + row * 0.5), egui::Align2::LEFT_CENTER, &s.title, font(15.0), ink);
    p.text(Pos2::new(tb.left() + 8.0, tb.top() + row * 1.5), egui::Align2::LEFT_CENTER, format!("{} · {}", s.material, st.text(s.t)), font(12.0), ink);
    p.text(Pos2::new(tb.left() + 8.0, tb.top() + row * 2.5), egui::Align2::LEFT_CENTER, format!("{}   {}   {}", trf("Scale {}", &[&scale_for(s.w, s.h, s.t)]), units, s.standard.short()), font(12.0), ink);
    // The three views and the room for their dimensions: front (w × t) at the top left, side (h × t) right of it, top (w × h) below.
    let room = 70.0_f32;
    let area = egui::Rect::from_min_max(frame.min + Vec2::splat(room), Pos2::new(frame.right() - room, tb.top() - room * 0.6));
    let (w, h, t) = (s.w.max(1.0), s.h.max(1.0), s.t.max(0.1));
    let gap = room as f64 * 1.75;
    let k = ((area.width() as f64 - gap) / (w + h)).min((area.height() as f64 - gap) / (t + h)).max(0.01);
    // First-angle: the front at the top left, the view from the left right of it, the view from above under it. Third-angle: the view
    // from above at the top left, the front under it, the view from the right right of the front.
    let third = s.standard.third_angle();
    let (front_o, top_o) = if third {
        let top_o = Pos2::new(area.left(), area.top() + (h * k) as f32);
        (Pos2::new(area.left(), top_o.y + (gap + t * k) as f32), top_o)
    } else {
        let front_o = Pos2::new(area.left(), area.top() + (t * k) as f32);
        (front_o, Pos2::new(area.left(), front_o.y + (gap + h * k) as f32))
    };
    let side_o = Pos2::new(area.left() + (w * k + gap) as f32, front_o.y);
    // Points of each view: (x, z) for the front, (y, z) for the side (seen from the left the front edge is on the right, seen from the
    // right on the left), (x, y) from above.
    let front = |x: f64, z: f64| {
        Pos2::new(front_o.x + (x * k) as f32, front_o.y - (z * k) as f32 - (t * k) as f32)
    };
    let side = |y: f64, z: f64| Pos2::new(side_o.x + ((if third { y } else { h - y }) * k) as f32, side_o.y - (z * k) as f32 - (t * k) as f32);
    let top = |x: f64, y: f64| Pos2::new(top_o.x + (x * k) as f32, top_o.y - (y * k) as f32);
    let full = Stroke::new(1.5, ink);
    let thin = Stroke::new(1.0, ink);
    let hidden = Stroke::new(0.9, ink);
    let rect_of = |a: Pos2, b: Pos2| egui::Rect::from_two_pos(a, b);
    // The block in the three views.
    let fr = rect_of(front(0.0, 0.0), front(w, -t));
    let sr = rect_of(side(0.0, 0.0), side(h, -t));
    let tr_ = rect_of(top(0.0, 0.0), top(w, h));
    for r in [fr, sr, tr_] {
        p.rect_stroke(r, 0.0, full, egui::StrokeKind::Middle);
    }
    // The parts in each view: seen (the face they are cut into is turned to the view) as full lines, on the far side of the object
    // as hidden (dashed) lines, and on a face across the view as the dashed box of the cut, from that face to its depth.
    let dashed = |pts: &[Pos2]| p.extend(egui::Shape::dashed_line(pts, hidden, 5.0, 3.0));
    // Each view: the way to the eye, the two coordinates it shows and where they are on the sheet, and the size of the block in it.
    let side_eye = if third { [1.0, 0.0, 0.0] } else { [-1.0, 0.0, 0.0] };
    let views: [([f64; 3], [usize; 2], &dyn Fn(f64, f64) -> Pos2, [f64; 4]); 3] = [
        ([0.0, -1.0, 0.0], [0, 2], &front, [0.0, -t, w, 0.0]),
        (side_eye, [1, 2], &side, [0.0, -t, h, 0.0]),
        ([0.0, 0.0, 1.0], [0, 1], &top, [0.0, 0.0, w, h]),
    ];
    for part in &s.parts {
        let d = part.depth.clamp(0.0, t.max(w).max(h));
        if d <= 0.0 && part.inward[2] > -0.5 {
            continue;
        }
        for (eye, [ia, ib], at, bx) in &views {
            let out = [-part.inward[0], -part.inward[1], -part.inward[2]];
            let k = out[0] * eye[0] + out[1] * eye[1] + out[2] * eye[2];
            if k > 0.5 {
                for line in &part.lines {
                    let pts: Vec<Pos2> = line.iter().map(|q| at(q[*ia], q[*ib])).collect();
                    p.add(egui::Shape::line(pts, thin));
                }
            } else if k < -0.5 {
                for line in &part.lines {
                    let pts: Vec<Pos2> = line.iter().map(|q| at(q[*ia], q[*ib])).collect();
                    dashed(&pts);
                }
            } else {
                // The box of the cut seen from the side: its edges that are not edges of the block.
                let (mut lo, mut hi) = ([f64::MAX; 2], [f64::MIN; 2]);
                for q in part.lines.iter().flatten() {
                    for e in [0.0, d] {
                        let r = [q[0] + part.inward[0] * e, q[1] + part.inward[1] * e, q[2] + part.inward[2] * e];
                        for (n, ax) in [*ia, *ib].iter().enumerate() {
                            lo[n] = lo[n].min(r[*ax]);
                            hi[n] = hi[n].max(r[*ax]);
                        }
                    }
                }
                if lo[0] > hi[0] {
                    continue;
                }
                let on_edge = |v: f64, edge: f64| (v - edge).abs() < 1e-6;
                let edges = [
                    (on_edge(lo[1], bx[1]), [lo[0], lo[1]], [hi[0], lo[1]]),
                    (on_edge(hi[1], bx[3]), [lo[0], hi[1]], [hi[0], hi[1]]),
                    (on_edge(lo[0], bx[0]), [lo[0], lo[1]], [lo[0], hi[1]]),
                    (on_edge(hi[0], bx[2]), [hi[0], lo[1]], [hi[0], hi[1]]),
                ];
                for (skip, a2, b2) in edges {
                    if !skip {
                        dashed(&[at(a2[0], a2[1]), at(b2[0], b2[1])]);
                    }
                }
            }
        }
    }
    // Centre lines of the views (dash and dot as on a drawing are dashes here).
    let centre = Stroke::new(0.6, Color32::from_rgb(0x50, 0x50, 0x60));
    p.extend(egui::Shape::dashed_line(&[top(w / 2.0, -4.0), top(w / 2.0, h + 4.0)], centre, 12.0, 4.0));
    p.extend(egui::Shape::dashed_line(&[top(-4.0, h / 2.0), top(w + 4.0, h / 2.0)], centre, 12.0, 4.0));
    // Dimensions: the material in each view, the parts from above, and the depths in the front view.
    let n = |v: f64| st.text(v);
    dims::dim(p, fr.left_top(), fr.right_top(), Vec2::new(0.0, -1.0), 18.0, &n(w), &st);
    dims::dim(p, fr.right_top(), fr.right_bottom(), Vec2::new(1.0, 0.0), 18.0, &n(t), &st);
    dims::dim(p, sr.left_top(), sr.right_top(), Vec2::new(0.0, -1.0), 18.0, &n(h), &st);
    dims::dim(p, tr_.left_top(), tr_.left_bottom(), Vec2::new(-1.0, 0.0), 40.0, &n(h), &st);
    dims::dim(p, tr_.left_bottom(), tr_.right_bottom(), Vec2::new(0.0, 1.0), 40.0, &n(w), &st);
    let to = |q: Pt| top(q.x, q.y);
    if !s.parts.is_empty() {
        dims::drawing(p, &to, &s.features, Some(Pt::new(0.0, 0.0)), &st);
    }
    let mut depths: Vec<f64> = s.parts.iter().filter(|q| q.inward[2] < -0.5).map(|q| q.depth.clamp(0.0, t)).filter(|d| *d > 0.0 && *d < t - 1e-6).collect();
    depths.sort_by(|a, b| a.total_cmp(b));
    depths.dedup_by(|a, b| (*a - *b).abs() < 0.05);
    for (i, d) in depths.iter().take(4).enumerate() {
        dims::dim(p, fr.left_top(), Pos2::new(fr.left(), front(0.0, -d).y), Vec2::new(-1.0, 0.0), 16.0 + 22.0 * i as f32, &n(*d), &st);
    }
    // The names of the views.
    let label = |at: Pos2, text: String| p.text(at, egui::Align2::LEFT_BOTTOM, text, font(12.0), ink);
    // Under each view, clear of its dimensions (the view from above has two rows of them under it).
    label(fr.left_bottom() + Vec2::new(0.0, 24.0), tr("Front view").to_string());
    label(sr.left_bottom() + Vec2::new(0.0, 24.0), tr("Side view").to_string());
    label(Pos2::new(tr_.left(), tr_.bottom() + 66.0), tr("Top view").to_string());
    // The sign of the projection in the title block: a truncated cone seen from the side and from its end; the end view on the right
    // for first-angle, on the left for third-angle.
    let sym = egui::Rect::from_min_size(Pos2::new(tb.right() - 78.0, tb.top() + row + 4.0), Vec2::new(70.0, row - 8.0));
    let (cone, circles) = if third { (sym.right() - 34.0, sym.left() + 14.0) } else { (sym.left() + 4.0, sym.right() - 14.0) };
    let mid = sym.center().y;
    let hh = (sym.height() / 2.0).min(10.0);
    let thin_ink = Stroke::new(1.0, ink);
    p.add(egui::Shape::closed_line(vec![Pos2::new(cone, mid - hh * 0.5), Pos2::new(cone + 30.0, mid - hh), Pos2::new(cone + 30.0, mid + hh), Pos2::new(cone, mid + hh * 0.5)], thin_ink));
    p.circle_stroke(Pos2::new(circles, mid), hh, thin_ink);
    p.circle_stroke(Pos2::new(circles, mid), hh * 0.5, thin_ink);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sheet_has_the_material_the_parts_and_their_depths_and_a_scale_for_a4() {
        let mut a = crate::tests::app();
        a.stock.width = 200.0;
        a.stock.height = 100.0;
        a.stock.thickness = 12.0;
        a.layers[0].enabled = true;
        a.layers[0].depth = 5.0;
        a.scene.add(0, base_core::scene::Kind::Rect { w: 40.0, h: 20.0 }, base_core::geom::Xf::translate(10.0, 10.0));
        let s = a.drawing_sheet();
        assert_eq!((s.w, s.h, s.t), (200.0, 100.0, 12.0));
        assert_eq!(s.parts.len(), 1);
        // Y counts up from the front edge: a rectangle 10 mm under the top edge of the page is 70..90 from the front.
        let b = s.parts[0].bounds();
        assert!((b.min.x - 10.0).abs() < 1e-9 && (b.min.y - 70.0).abs() < 1e-9 && (b.max.y - 90.0).abs() < 1e-9, "{b:?}");
        assert_eq!(s.parts[0].depth, 5.0);
        assert_eq!(scale_for(200.0, 100.0, 12.0), "1:2", "the three views do not fit A4 at 1:1");
        assert_eq!(scale_for(150.0, 60.0, 10.0), "1:1");
        assert_eq!(scale_for(600.0, 400.0, 18.0), "1:5");
        let ctx = egui::Context::default();
        let mut out = ctx.run_ui(Default::default(), |ui| paint(ui.painter(), egui::Rect::from_min_size(Pos2::ZERO, Vec2::new(900.0, 640.0)), &s));
        out.textures_delta.clear();
        assert!(out.shapes.len() > 40, "the views, the dashed cuts, the dimensions and the title: {}", out.shapes.len());
        // Every kind is drawn; ISO is first-angle, ASME and JIS third-angle; ASME writes the numbers upright.
        assert!(!Standard::Iso.third_angle() && Standard::Asme.third_angle() && Standard::Jis.third_angle());
        for k in Standard::ALL {
            let sheet = Sheet { standard: k, ..a.drawing_sheet() };
            let mut out = ctx.run_ui(Default::default(), |ui| paint(ui.painter(), egui::Rect::from_min_size(Pos2::ZERO, Vec2::new(900.0, 640.0)), &sheet));
            out.textures_delta.clear();
            assert!(out.shapes.len() > 40, "{k:?}");
        }
        // Saved as a PDF page and a picture of an A4 sheet at 300 dots per inch.
        let dir = std::env::temp_dir().join(format!("mill-drawing-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        save_sheet(&s, &dir.join("d.png")).unwrap();
        let png = std::fs::read(dir.join("d.png")).unwrap();
        let be = |i: usize| u32::from_be_bytes([png[i], png[i + 1], png[i + 2], png[i + 3]]);
        assert_eq!((be(16), be(20)), (3508, 2480), "A4 at 300 dots per inch");
        save_sheet(&s, &dir.join("d.pdf")).unwrap();
        assert!(std::fs::read(dir.join("d.pdf")).unwrap().starts_with(b"%PDF"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A post: the drawing is of the whole object, with the parts of every face where they are on it; a layer hidden in the editor is
    /// still milled, so it is drawn.
    #[test]
    fn the_drawing_of_a_post_has_the_parts_of_all_its_faces_and_of_hidden_layers() {
        let mut a = crate::tests::app();
        a.open_setup();
        a.setup.sides = base_ui::project_setup::Sides::Post;
        a.setup.post = [60.0, 40.0, 250.0];
        a.apply_setup();
        a.layers[2].depth = 5.0;
        a.layers[2].visible = false;
        a.scene.add(2, base_core::scene::Kind::Rect { w: 40.0, h: 20.0 }, base_core::geom::Xf::translate(10.0, 10.0));
        a.show_face(1);
        a.scene.add(2, base_core::scene::Kind::Ellipse { w: 20.0, h: 20.0 }, base_core::geom::Xf::translate(100.0, 10.0));
        a.show_face(2);
        a.scene.add(2, base_core::scene::Kind::Rect { w: 30.0, h: 10.0 }, base_core::geom::Xf::translate(200.0, 5.0));
        let s = a.drawing_sheet();
        assert_eq!((s.w, s.h, s.t), (250.0, 60.0, 40.0), "the post as it lies for face A");
        assert_eq!(s.parts.len(), 3);
        // Into the post from the top, from the front (Y up from the front) and from the bottom.
        let ins: Vec<[f64; 3]> = s.parts.iter().map(|p| p.inward).collect();
        assert!(ins.contains(&[0.0, 0.0, -1.0]) && ins.contains(&[0.0, 1.0, 0.0]) && ins.contains(&[0.0, 0.0, 1.0]), "{ins:?}");
        // The round part of face B lies on the front of the post: Y = 0, Z between the top (0) and the bottom (-40).
        let b = s.parts.iter().find(|p| p.inward == [0.0, 1.0, 0.0]).unwrap();
        assert!(b.lines.iter().flatten().all(|q| q[1].abs() < 1e-9 && q[2] <= 1e-9 && q[2] >= -40.0 - 1e-9));
        if let Ok(dir) = std::env::var("SHOT_OUT") {
            save_sheet(&s, &std::path::Path::new(&dir).join("post-drawing.png")).unwrap();
        }
    }
}
