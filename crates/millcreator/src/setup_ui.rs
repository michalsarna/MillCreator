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
        let units = self.device.units;
        let mut open = true;
        let (mut apply, mut skip) = (false, false);
        let st = &mut self.setup;
        let title = tr("Project setup…").trim_end_matches('…').to_string();
        popup(ctx, "setup_window", &title, [380.0, 330.0], &mut open, |ui| {
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
