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
    /// A cylinder on the rotary attachment instead of a block.
    pub round: bool,
    /// The diameter and the length of the cylinder (mm).
    pub diameter: f64,
    pub length: f64,
}

impl Default for Setup {
    fn default() -> Setup {
        Setup { open: false, w: 200.0, h: 150.0, t: 10.0, zero: ZeroPoint::FrontLeft, z_bottom: false, work_area: false, round: false, diameter: 50.0, length: 150.0 }
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
            round: s.round || self.device.rotary.enabled,
            diameter: self.device.rotary.diameter.clamp(1.0, 1000.0),
            length: if s.round && s.width > 0.0 { s.width } else { self.device.area.x.min(150.0) },
        };
    }

    /// Put the choices into the project: the material, the zero and the work area.
    pub(crate) fn apply_setup(&mut self) {
        if self.setup.round {
            return self.apply_cylinder();
        }
        // A block: the rotary attachment of the device is not used.
        let was_round = self.device.rotary.enabled;
        self.stock.round = false;
        self.set_rotary(false, self.device.rotary.diameter);
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
        if was_round {
            self.status = format!("{} {}", self.status, tr("The rotary attachment of the device is off."));
        }
    }

    /// A cylinder on the rotary: its length along X, the way round it along Y, its radius the deepest cut; the rotary attachment of
    /// the device is turned on with its diameter (also in the list of devices, where it is kept).
    fn apply_cylinder(&mut self) {
        let (d, len, zero, work_area) = (self.setup.diameter.clamp(1.0, 1000.0), self.setup.length.clamp(1.0, 20_000.0), self.setup.zero, self.setup.work_area);
        self.set_rotary(true, d);
        let round = self.device.rotary.circumference();
        self.stock.round = true;
        self.stock.width = len;
        self.stock.height = round;
        self.stock.thickness = (d / 2.0).min(MAX_THICKNESS);
        self.stock.first_pass_depth = self.stock.first_pass_depth.min(self.stock.thickness);
        self.stock.zero = Some(zero);
        self.stock.z_bottom = false;
        if work_area {
            self.device.area.x = len.max(10.0);
            self.device.area.y = round.max(10.0);
        }
        self.device.area.z = self.device.area.z.max((d / 2.0).min(2000.0));
        self.touch_rev();
        self.ed.view.need_fit = true;
        self.status = trf("Project set up: a cylinder of Ø {} mm and {} mm long on the rotary attachment (switched on), zero at {}", &[&format!("{d:.1}"), &format!("{len:.1}"), &tr(zero.label())]);
    }

    /// Switch the rotary attachment of the device in use on or off, with the diameter of the workpiece, in the project and in the
    /// list of devices.
    pub(crate) fn set_rotary(&mut self, on: bool, diameter: f64) {
        for r in std::iter::once(&mut self.device.rotary).chain(self.active.and_then(|i| self.devices.get_mut(i)).map(|d| &mut d.rotary)) {
            r.enabled = on;
            if on {
                r.diameter = diameter;
            }
        }
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
        popup(ctx, "setup_window", &title, [400.0, 600.0], &mut open, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("Shape of the material"));
                ui.selectable_value(&mut st.round, false, tr("Block"));
                ui.selectable_value(&mut st.round, true, tr("Cylinder (rotary)")).on_hover_text(tr("A round workpiece turned by the rotary attachment of the device. Applying switches the rotary attachment on with this diameter."));
            });
            ui.add_space(4.0);
            if st.round {
                cylinder_picture(ui, st, colour, units);
                ui.add_space(4.0);
                egui::Grid::new("setup_grid_round").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                    ui.label(tr("Workpiece diameter"));
                    drag_len(ui, units, &mut st.diameter, 0.5, Some((1.0, 1000.0)));
                    ui.end_row();
                    ui.label(tr("Workpiece length"));
                    drag_len(ui, units, &mut st.length, 1.0, Some((1.0, 20_000.0)));
                    ui.end_row();
                    ui.label(tr("Circumference"));
                    ui.label(base_ui::units_ui::fmt_len(units, std::f64::consts::PI * st.diameter, 1)).on_hover_text(tr("The way round the workpiece: the design may not be higher."));
                    ui.end_row();
                    ui.label(tr("Zero of the program"));
                    egui::ComboBox::from_id_salt("setup_zero_round").selected_text(tr(st.zero.label())).show_ui(ui, |ui| {
                        for z in ZeroPoint::ALL {
                            ui.selectable_value(&mut st.zero, z, tr(z.label()));
                        }
                    });
                    ui.end_row();
                    ui.label("");
                    ui.checkbox(&mut st.work_area, tr("Work area is the material"));
                    ui.end_row();
                });
                ui.label(RichText::new(tr("The cylinder lies along X on the rotary attachment; Y of the drawing goes round it and the drawing is unrolled from its surface. Z counts from the top of the surface, and the deepest cut is the radius. The zero is on the surface; set the same one on the machine.")).color(theme::text_dim()));
                ui.horizontal(|ui| {
                    apply = ui.button(tr("Apply")).clicked();
                    skip = ui.button(tr("Skip")).clicked();
                });
                return;
            }
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
    fn a_cylinder_switches_the_rotary_on_and_a_block_switches_it_off_again() {
        let mut a = crate::tests::app();
        a.open_setup();
        assert!(!a.setup.round, "a block by default");
        a.setup.round = true;
        a.setup.diameter = 40.0;
        a.setup.length = 120.0;
        a.apply_setup();
        let round = std::f64::consts::PI * 40.0;
        assert!(a.device.rotary.enabled && a.device.rotary.diameter == 40.0);
        assert!(a.stock.round && a.stock.width == 120.0 && (a.stock.height - round).abs() < 1e-9 && a.stock.thickness == 20.0);
        if let Some(i) = a.active {
            assert!(a.devices[i].rotary.enabled && a.devices[i].rotary.diameter == 40.0, "kept with the device");
        }
        // Opened again, the window shows the cylinder.
        a.open_setup();
        assert!(a.setup.round && a.setup.diameter == 40.0 && a.setup.length == 120.0);
        // The wrap lines of the work area follow the way round.
        assert_eq!(base_ui::rotary_ui::wrap_lines(&a.device.rotary, 2.5 * round).len(), 2);
        a.setup.round = false;
        a.apply_setup();
        assert!(!a.device.rotary.enabled && !a.stock.round);
        assert!(a.status.contains("off"), "{}", a.status);
        // An old project has no cylinder.
        let old: crate::materials::Stock = serde_json::from_str("{\"kind\":\"WoodLight\",\"color\":[1,2,3],\"thickness\":8.0}").unwrap();
        assert!(!old.round);
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

/// The picture of a cylinder on the rotary: the workpiece along X seen at a slant, its length and diameter, the axis of the rotary with
/// the arrow of the turn (Y goes round), the jaws of a chuck at its left end, and the zero of the program on the surface.
fn cylinder_picture(ui: &mut egui::Ui, st: &Setup, colour: [u8; 3], units: base_core::units::Units) {
    use egui::{Color32, Pos2, Stroke};
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width().min(380.0), 200.0), egui::Sense::hover());
    let p = ui.painter_at(rect);
    let shade = |f: f32| Color32::from_rgb((colour[0] as f32 * f) as u8, (colour[1] as f32 * f) as u8, (colour[2] as f32 * f) as u8);
    // The proportions of the drawing follow the numbers, within limits so that a long thin bar and a short thick one both show.
    let ratio = (st.length / st.diameter.max(1e-6)).clamp(1.2, 6.0) as f32;
    // Room on the left for the chuck, on the right for the diameter and the arrow of the turn.
    let room_w = rect.width() - 60.0 - 150.0;
    let dia = (room_w / ratio).min(rect.height() - 80.0).max(20.0);
    let len = (dia * ratio).min(room_w);
    let rx = dia * 0.18; // the ends are ellipses: the cylinder is seen a little from the side
    let left = rect.min.x + 60.0 + (room_w - len) / 2.0;
    let cy = rect.center().y - 4.0;
    let (top, bottom) = (cy - dia / 2.0, cy + dia / 2.0);
    let right = left + len;
    // The chuck at the left end: a block with three jaws.
    let chuck = egui::Rect::from_min_max(Pos2::new(left - rx - 26.0, cy - dia * 0.62), Pos2::new(left - rx + 2.0, cy + dia * 0.62));
    p.rect_filled(chuck, 3.0, Color32::from_gray(0x70));
    for k in [-0.45f32, 0.0, 0.45] {
        p.rect_filled(egui::Rect::from_center_size(Pos2::new(left - rx + 4.0, cy + k * dia), egui::vec2(8.0, dia * 0.16)), 1.0, Color32::from_gray(0x55));
    }
    // The axis of the rotary, behind the workpiece.
    let axis = Stroke::new(1.0, Color32::from_gray(0x90));
    let mut x = chuck.min.x - 8.0;
    while x < right + rx + 84.0 {
        p.line_segment([Pos2::new(x, cy), Pos2::new((x + 7.0).min(right + rx + 84.0), cy)], axis);
        x += 11.0;
    }
    // The body: the side, light at the top and darker below, then the end on the right.
    let n = 24;
    let mut body: Vec<Pos2> = vec![Pos2::new(left, top), Pos2::new(right, top)];
    for i in 0..=n {
        let a = -std::f32::consts::FRAC_PI_2 + std::f32::consts::PI * i as f32 / n as f32;
        body.push(Pos2::new(right + rx * a.cos(), cy + dia / 2.0 * a.sin()));
    }
    body.push(Pos2::new(left, bottom));
    p.add(egui::Shape::convex_polygon(body, shade(0.85), Stroke::NONE));
    p.rect_filled(egui::Rect::from_min_max(Pos2::new(left, top), Pos2::new(right, cy - dia * 0.15)), 0.0, shade(1.0));
    let edge = Stroke::new(1.0, shade(0.45));
    p.line_segment([Pos2::new(left, top), Pos2::new(right, top)], edge);
    p.line_segment([Pos2::new(left, bottom), Pos2::new(right, bottom)], edge);
    let end: Vec<Pos2> = (0..=48).map(|i| {
        let a = std::f32::consts::TAU * i as f32 / 48.0;
        Pos2::new(right + rx * a.cos(), cy + dia / 2.0 * a.sin())
    }).collect();
    p.add(egui::Shape::convex_polygon(end.clone(), shade(0.7), edge));
    let green = Color32::from_rgb(0x30, 0xa0, 0x40);
    let turn: Vec<Pos2> = (0..=20).map(|i| {
        let a = -2.3 + 2.0 * i as f32 / 20.0;
        Pos2::new(right + rx + 66.0 + 8.0 * a.cos(), cy + dia * 0.42 * a.sin())
    }).collect();
    let tip = turn[turn.len() - 1];
    let before = turn[turn.len() - 3];
    p.add(egui::Shape::line(turn, Stroke::new(2.0, green)));
    let dir = (tip - before).normalized();
    let side = egui::vec2(-dir.y, dir.x);
    p.add(egui::Shape::convex_polygon(vec![tip + dir * 4.0, tip - dir * 5.0 + side * 4.0, tip - dir * 5.0 - side * 4.0], green, Stroke::NONE));
    p.text(Pos2::new(right + rx + 80.0, cy - dia * 0.42), egui::Align2::LEFT_CENTER, "Y", egui::FontId::proportional(12.0), green);
    // The length under the cylinder and the diameter at its right end.
    let ink = base_ui::dims::DimStyle::on(theme::panel());
    let num = |v: f64| base_ui::dims::number(units, v);
    base_ui::dims::dim(&p, Pos2::new(left, bottom), Pos2::new(right, bottom), egui::vec2(0.0, 1.0), 16.0, &num(st.length), &ink);
    base_ui::dims::dim(&p, Pos2::new(right + rx, top), Pos2::new(right + rx, bottom), egui::vec2(1.0, 0.0), 16.0, &format!("Ø {}", num(st.diameter)), &ink);
    // The zero of the program on the top of the surface: at the left or the right end, or in the middle.
    let zx = match st.zero {
        ZeroPoint::FrontLeft | ZeroPoint::BackLeft => left,
        ZeroPoint::FrontRight | ZeroPoint::BackRight => right,
        ZeroPoint::Center => (left + right) / 2.0,
    };
    let zero = Pos2::new(zx, top);
    let red = Color32::from_rgb(0xd0, 0x30, 0x30);
    let blue = Color32::from_rgb(0x30, 0x60, 0xd0);
    for (end, name, c) in [(zero + egui::vec2(28.0, 0.0), "X", red), (zero + egui::vec2(0.0, -26.0), "Z", blue)] {
        p.line_segment([zero, end], Stroke::new(2.0, c));
        p.text(end + (end - zero).normalized() * 8.0, egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(12.0), c);
    }
    p.circle_filled(zero, 4.5, Color32::from_rgb(0xe0, 0x30, 0x30));
    p.circle_stroke(zero, 4.5, Stroke::new(1.2, Color32::WHITE));
}

