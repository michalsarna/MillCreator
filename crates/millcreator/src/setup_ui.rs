//! The project setup: the size and thickness of the piece of material, where the zero of X and Y is on it, whether Z counts from the top
//! or from the bottom of the material, and optionally the work area made as big as the piece. It opens after File > New and from
//! File > Project setup…; skipping it leaves the device's own zero.
use crate::materials::{ZeroPoint, MAX_THICKNESS};
use crate::App;
use base_ui::i18n::{tr, trf};
use base_ui::theme;
use base_ui::units_ui::drag_len;
use base_ui::window::popup;
use eframe::egui::{self, RichText};

pub struct Setup {
    pub open: bool,
    pub w: f64,
    pub h: f64,
    pub t: f64,
    pub zero: ZeroPoint,
    pub z_bottom: bool,
    pub work_area: bool,
}

impl Default for Setup {
    fn default() -> Setup {
        Setup { open: false, w: 200.0, h: 150.0, t: 10.0, zero: ZeroPoint::FrontLeft, z_bottom: false, work_area: false }
    }
}

impl App {
    /// Open the window with what the project has now (or the work area and the thickness when it has nothing yet).
    pub(crate) fn open_setup(&mut self) {
        let s = &self.stock;
        self.setup = Setup {
            open: true,
            w: if s.width > 0.0 { s.width } else { self.device.area.x.min(300.0) },
            h: if s.height > 0.0 { s.height } else { self.device.area.y.min(200.0) },
            t: s.thickness,
            zero: s.zero.unwrap_or(ZeroPoint::FrontLeft),
            z_bottom: s.z_bottom,
            work_area: false,
        };
    }

    /// Put the choices into the project: the material, the zero and the work area.
    pub(crate) fn apply_setup(&mut self) {
        let (uw, uh, ut, zero, z_bottom, work_area) = (self.setup.w, self.setup.h, self.setup.t, self.setup.zero, self.setup.z_bottom, self.setup.work_area);
        let (w, h, t) = (uw.clamp(1.0, 20_000.0), uh.clamp(1.0, 20_000.0), ut.clamp(0.1, MAX_THICKNESS));
        self.stock.width = w;
        self.stock.height = h;
        self.stock.thickness = t;
        self.stock.first_pass_depth = self.stock.first_pass_depth.min(t);
        self.stock.zero = Some(zero);
        self.stock.z_bottom = z_bottom;
        if work_area {
            self.device.area.x = w.max(10.0);
            self.device.area.y = h.max(10.0);
        }
        self.device.area.z = self.device.area.z.max(t.min(2000.0));
        self.touch_rev();
        self.ed.view.need_fit = true;
        self.status = trf("Project set up: material {} x {} x {} mm, zero at {}", &[&format!("{w:.1}"), &format!("{h:.1}"), &format!("{t:.1}"), &tr(zero.label())]);
    }

    pub(crate) fn setup_window(&mut self, ctx: &egui::Context) {
        if !self.setup.open {
            return;
        }
        let units = base_ui::editor::EditorHost::units(self);
        let mut open = true;
        let (mut apply, mut skip) = (false, false);
        let colour = self.stock.color;
        let st = &mut self.setup;
        let title = tr("Project setup…").trim_end_matches('…').to_string();
        popup(ctx, "setup_window", &title, [400.0, 560.0], &mut open, |ui| {
            block_picture(ui, st, colour, units);
            ui.add_space(4.0);
            egui::Grid::new("setup_grid").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                ui.label(tr("Material width"));
                drag_len(ui, units, &mut st.w, 1.0, Some((1.0, 20_000.0)));
                ui.end_row();
                ui.label(tr("Material height"));
                drag_len(ui, units, &mut st.h, 1.0, Some((1.0, 20_000.0)));
                ui.end_row();
                ui.label(tr("Thickness"));
                drag_len(ui, units, &mut st.t, 0.5, Some((0.1, MAX_THICKNESS)));
                ui.end_row();
                ui.label(tr("Zero of the program"));
                egui::ComboBox::from_id_salt("setup_zero").selected_text(tr(st.zero.label())).show_ui(ui, |ui| {
                    for z in ZeroPoint::ALL {
                        ui.selectable_value(&mut st.zero, z, tr(z.label()));
                    }
                });
                ui.end_row();
                ui.label("");
                ui.checkbox(&mut st.z_bottom, tr("Z counts from the bottom of the material"));
                ui.end_row();
                ui.label("");
                ui.checkbox(&mut st.work_area, tr("Work area is the material"));
                ui.end_row();
            });
            ui.label(RichText::new(tr("The material lies at the top left of the page. The program is written for the work coordinate system G54 with its zero at the place you choose: set that zero on the machine there. Z counts from the top of the material, or from its bottom.")).color(theme::text_dim()));
            ui.horizontal(|ui| {
                apply = ui.button(tr("Apply")).clicked();
                skip = ui.button(tr("Skip")).clicked();
            });
        });
        if apply {
            self.apply_setup();
            self.setup.open = false;
        } else if skip || !open {
            self.setup.open = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::materials::ZeroPoint;
    use crate::toolpath::machine_xy;

    #[test]
    fn the_setup_puts_the_material_and_the_zero_into_the_project_and_the_program_counts_from_there() {
        let mut a = crate::tests::app();
        a.open_setup();
        assert!(a.setup.open);
        a.setup.w = 100.0;
        a.setup.h = 60.0;
        a.setup.t = 12.0;
        a.setup.zero = ZeroPoint::Center;
        a.setup.z_bottom = true;
        a.apply_setup();
        assert_eq!((a.stock.width, a.stock.height, a.stock.thickness), (100.0, 60.0, 12.0));
        // The centre of the piece is the zero: a point at the centre is (0, 0) in the program, whatever the work area.
        let dev = a.device.with_stock(&a.stock);
        let centre = (50.0, a.device.area.y - 30.0);
        let (x, y) = machine_xy(&dev, centre.0, centre.1);
        assert!(x.abs() < 1e-9 && y.abs() < 1e-9, "{x} {y}");
        assert_eq!(dev.wcs.word().as_deref(), Some("G54"));
        assert_eq!(dev.z_offset, 12.0, "Z counts from the bottom");
        // The device itself is untouched, and a project without a setup keeps the zero of the device.
        assert_eq!(a.device.wcs.code, 0);
        let plain = a.device.with_stock(&crate::materials::Stock::default());
        assert_eq!((plain.wcs, plain.z_offset), (a.device.wcs, 0.0));
        // The work area can be made the piece.
        a.open_setup();
        a.setup.work_area = true;
        a.apply_setup();
        assert_eq!((a.device.area.x, a.device.area.y), (100.0, 60.0));
        // From the corner: front left of a piece that fills the work area is the machine zero.
        a.setup.zero = ZeroPoint::FrontLeft;
        a.apply_setup();
        let dev = a.device.with_stock(&a.stock);
        assert_eq!(machine_xy(&dev, 0.0, 0.0), (0.0, 0.0));
    }

    #[test]
    fn a_z_offset_moves_every_z_of_the_program_and_an_old_project_has_no_setup() {
        use crate::materials::Stock;
        let mut s = Stock::default();
        s.z_bottom = true;
        let lib = crate::tools::Library::new();
        let mut dev = crate::device::Device::dummy();
        dev.z_offset = s.thickness;
        let layers = crate::layers::default_layers(&lib);
        let g = crate::toolpath::to_gcode(&crate::toolpath::Program::default(), &dev, &layers, &lib, "t");
        assert!(g.contains(&format!("G0 Z{}", dev.safe_z + 10.0)), "{g}");
        // A project file from before has no width, height, zero or z_bottom.
        let old: Stock = serde_json::from_str("{\"kind\":\"WoodLight\",\"color\":[1,2,3],\"thickness\":8.0}").unwrap();
        assert_eq!((old.width, old.height, old.zero, old.z_bottom), (0.0, 0.0, None, false));
    }
}

/// The picture of the setup: the block of material seen at a slant with its width, height and thickness, and the zero of the program
/// with its axes where it was chosen (on the top of the block, or on its bottom when Z counts from there). The block is drawn thicker
/// than it is when it is thin, so that it can be seen; the numbers are the real ones.
fn block_picture(ui: &mut egui::Ui, st: &Setup, colour: [u8; 3], units: base_core::units::Units) {
    use egui::{Color32, Pos2, Stroke};
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width().min(380.0), 210.0), egui::Sense::hover());
    let p = ui.painter_at(rect);
    let big = st.w.max(st.h).max(1e-6);
    let (w, h) = (st.w / big, st.h / big);
    let t = (st.t / big).clamp(0.1, 0.6);
    // A slanted view: X to the right, Y back and up at a slant, Z up; the block in the middle with room for the dimensions.
    let (sx, sy) = (0.55, 0.32);
    let (span_x, span_y) = (w + h * sx, h * sy + t);
    let k = ((rect.width() as f64 - 110.0) / span_x).min((rect.height() as f64 - 70.0) / span_y).max(1.0);
    let o = Pos2::new(rect.center().x - (span_x * k / 2.0) as f32 - 10.0, rect.center().y + (span_y * k / 2.0) as f32 - 6.0);
    let at = |q: [f64; 3]| Pos2::new(o.x + ((q[0] + q[1] * sx) * k) as f32, o.y - ((q[1] * sy + q[2]) * k) as f32);
    let shade = |f: f32| Color32::from_rgb((colour[0] as f32 * f) as u8, (colour[1] as f32 * f) as u8, (colour[2] as f32 * f) as u8);
    let edge = Stroke::new(1.0, shade(0.45));
    let face = |pts: [[f64; 3]; 4], f: f32| p.add(egui::Shape::convex_polygon(pts.iter().map(|q| at(*q)).collect(), shade(f), edge));
    face([[0.0, 0.0, 0.0], [w, 0.0, 0.0], [w, 0.0, t], [0.0, 0.0, t]], 0.82);
    face([[w, 0.0, 0.0], [w, h, 0.0], [w, h, t], [w, 0.0, t]], 0.68);
    face([[0.0, 0.0, t], [w, 0.0, t], [w, h, t], [0.0, h, t]], 1.0);
    // The width under the front, the height along the right side, the thickness at the back right (away from the zero).
    let ink = base_ui::dims::DimStyle::on(theme::panel());
    let n = |v: f64| base_ui::dims::number(units, v);
    base_ui::dims::dim(&p, at([0.0, 0.0, 0.0]), at([w, 0.0, 0.0]), egui::vec2(0.0, 1.0), 16.0, &n(st.w), &ink);
    let along = (at([w, h, 0.0]) - at([w, 0.0, 0.0])).normalized();
    base_ui::dims::dim(&p, at([w, 0.0, 0.0]), at([w, h, 0.0]), egui::vec2(-along.y, along.x), 16.0, &n(st.h), &ink);
    base_ui::dims::dim(&p, at([w, h, 0.0]), at([w, h, t]), egui::vec2(1.0, 0.0), 14.0, &n(st.t), &ink);
    // The zero of the program and its axes.
    let (zx, zy) = match st.zero {
        ZeroPoint::FrontLeft => (0.0, 0.0),
        ZeroPoint::FrontRight => (w, 0.0),
        ZeroPoint::BackLeft => (0.0, h),
        ZeroPoint::BackRight => (w, h),
        ZeroPoint::Center => (w / 2.0, h / 2.0),
    };
    let zz = if st.z_bottom { 0.0 } else { t };
    let zero = at([zx, zy, zz]);
    let len = 0.18;
    for (d, name, c) in [([len, 0.0, 0.0], "X", Color32::from_rgb(0xd0, 0x30, 0x30)), ([0.0, len, 0.0], "Y", Color32::from_rgb(0x30, 0xa0, 0x30)), ([0.0, 0.0, len], "Z", Color32::from_rgb(0x30, 0x60, 0xd0))] {
        let end = at([zx + d[0], zy + d[1], zz + d[2]]);
        p.line_segment([zero, end], Stroke::new(2.0, c));
        p.text(end + (end - zero).normalized() * 8.0, egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(12.0), c);
    }
    p.circle_filled(zero, 4.5, Color32::from_rgb(0xe0, 0x30, 0x30));
    p.circle_stroke(zero, 4.5, Stroke::new(1.2, Color32::WHITE));
}
