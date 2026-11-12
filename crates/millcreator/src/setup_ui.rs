//! The project setup of MillCreator: the shared window of baseCreator (`base_ui::project_setup`: a block or a cylinder, its size, the
//! zero of the program, a two-sided job) and what the choices do to the project. It opens after File > New and from
//! File > Project setup…; skipping it leaves the device's own zero.
use crate::materials::{ZeroPoint, MAX_THICKNESS};
use crate::App;
use base_ui::i18n::{tr, trf};
use base_ui::project_setup::{Answer, Kind};
use eframe::egui;

pub use base_ui::project_setup::Setup;

/// What MillCreator asks of the shared window.
fn kind(colour: [u8; 3]) -> Kind {
    Kind {
        zero: true,
        cylinder: true,
        marks_tip: "Two holes on the line the material is turned over round, drilled through it into the table: put pins in them and the material lies in the same place after it is turned over.",
        block_note: "The material lies at the top left of the page. The program is written for the work coordinate system G54 with its zero at the place you choose: set that zero on the machine there. Z counts from the top of the material, or from its bottom.",
        cylinder_note: "The cylinder lies along X on the rotary attachment; Y of the drawing goes round it and the drawing is unrolled from its surface. Z counts from the top of the surface, and the deepest cut is the radius. The zero is on the surface; set the same one on the machine.",
        colour,
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
            two_sided: self.two.is_some(),
            flip: s.flip,
            // The marks are offered for a project that becomes two-sided; one that is already has them (or not) by now.
            marks: self.two.is_none(),
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
        self.apply_two_sided();
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

    /// A block chosen as two-sided: the project gets side B (or the new way of turning it over) and, when asked, the alignment holes
    /// on side A; chosen as one-sided, side B goes when it is empty.
    fn apply_two_sided(&mut self) {
        if self.setup.two_sided {
            let first = self.two.is_none();
            self.make_two_sided(self.setup.flip);
            if first && self.setup.marks {
                self.show_side(false);
                self.add_alignment_holes();
            }
            self.status = format!("{} — {}", self.status, base_ui::project_setup::flip_name(self.setup.flip));
        } else if !self.make_one_sided() {
            self.status = tr("Side B has shapes: the project stays two-sided. Delete them first to make it one-sided.").to_string();
        }
    }

    pub(crate) fn setup_window(&mut self, ctx: &egui::Context) {
        let units = base_ui::editor::EditorHost::units(self);
        if base_ui::project_setup::window(ctx, &mut self.setup, units, kind(self.stock.color)) == Some(Answer::Apply) {
            self.apply_setup();
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

