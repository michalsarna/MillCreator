//! Adding a device: the one window of every program (`base_ui::add_device_ui`) with the basic settings of a milling machine: the
//! controller, the connection with a read from the controller, the work area, the machine zero, the safe height and the spindle,
//! an optional camera, the things worth checking. "More settings…" opens the whole device window with what is entered here.
use crate::device::{Device, Origin};
use crate::device_ui::{DeviceEdit, Page};
use crate::App;
use base_core::machine::GrblVersion;
use base_ui::add_device_ui::{self as add, AddAction, ReadState};
use base_ui::i18n::tr;
use base_ui::units_ui::drag_len;
use eframe::egui::{self, Color32, RichText};
use std::time::{Duration, Instant};

/// How long a read may take, from the press of the button to the settings.
const READ_TIMEOUT: Duration = Duration::from_secs(12);

/// The window for a new device, while it is open.
pub(crate) struct AddDev {
    pub draft: Device,
    pub read: ReadState,
    /// When the read started; the link was opened for it and is closed after it.
    pub read_since: Option<Instant>,
    /// Why the device cannot be added yet (its numbers are out of range).
    pub error: String,
}

impl App {
    /// Open the window for a new device: plain defaults, nothing predefined.
    pub(crate) fn start_add_device(&mut self) {
        self.add_dev = Some(AddDev { draft: Device { name: String::new(), ..Device::default() }, read: ReadState::Idle, read_since: None, error: String::new() });
    }

    /// The things worth checking before the device is used.
    pub(crate) fn add_dev_warnings(d: &Device) -> Vec<String> {
        let mut w = add::common_warnings(&d.connection, !d.duet.enabled && !d.linuxcnc, &tr("Nothing here has been tried on your machine: run a first test without a bit, or with the bit well above the material."));
        if d.linuxcnc {
            w.insert(0, tr("LinuxCNC runs the program by itself: Start saves the G-code file for it. Not tried on a real machine.").to_string());
        }
        if d.duet.enabled && d.duet.host.trim().is_empty() {
            w.insert(0, tr("No port or host is set yet: choose it in the device settings before connecting.").to_string());
        }
        w
    }

    /// Is a read from the controller of the new device going on? The link is the new device's then (nothing is homed or unlocked).
    pub(crate) fn add_dev_reading(&self) -> bool {
        self.add_dev.as_ref().is_some_and(|a| a.read == ReadState::Reading)
    }

    /// Open the link to the new device and read its settings.
    fn add_dev_start_read(&mut self) {
        let Some(a) = self.add_dev.as_mut() else { return };
        let Some(target) = crate::machine_ui::target_of(&a.draft) else { return };
        a.read = ReadState::Reading;
        a.read_since = Some(Instant::now());
        self.machine.fw.values.clear();
        self.machine.set_grbl_version(a.draft.connection.grbl_version);
        if self.machine.connected {
            self.machine.disconnect();
        }
        self.machine.connect(target, a.draft.connection.streaming);
    }

    /// Follow the read: the settings fill the draft once they came; the link is closed after it.
    fn add_dev_follow_read(&mut self) {
        let Some(a) = self.add_dev.as_mut() else { return };
        if a.read != ReadState::Reading {
            return;
        }
        let late = a.read_since.is_some_and(|t| t.elapsed() > READ_TIMEOUT);
        let m = &self.machine;
        if m.connected && !m.fw.reading && !m.fw.values.is_empty() {
            let s = m.suggestion();
            crate::device_ui::apply_suggestion(&mut a.draft, &s);
            a.read = ReadState::Done;
        } else if late {
            a.read = ReadState::Failed;
        } else {
            return;
        }
        a.read_since = None;
        self.machine.disconnect();
    }

    /// The window for a new device; called every frame.
    pub(crate) fn add_device_window(&mut self, ctx: &egui::Context) {
        if self.add_dev.is_none() {
            return;
        }
        self.add_dev_follow_read();
        if let Some(cam) = self.cam_test_wanted.take() {
            self.cam_test = base_ui::camera_feed::CamTest::start(cam, ctx);
        }
        let Some(mut a) = self.add_dev.take() else { return };
        let ports = self.machine.ports.clone();
        let (mut open, mut refresh, mut read, mut test_cam) = (true, false, false, None);
        let state = a.read;
        let d = &mut a.draft;
        let error = a.error.clone();
        let action = add::window(ctx, &mut open, |ui| {
            add::grid(ui, |ui| {
                ui.label(tr("Name"));
                ui.add(egui::TextEdit::singleline(&mut d.name).hint_text(tr("e.g. My CNC router")));
                ui.end_row();
                ui.label(tr("Controller"));
                let shown = if d.linuxcnc { tr("LinuxCNC (file)").to_string() } else if d.duet.enabled { tr("Duet / RepRapFirmware (HTTP)").to_string() } else { tr(d.connection.grbl_version.label()).to_string() };
                egui::ComboBox::from_id_salt("add_controller").selected_text(shown).show_ui(ui, |ui| {
                    for v in GrblVersion::ALL {
                        if ui.selectable_label(!d.duet.enabled && !d.linuxcnc && d.connection.grbl_version == v, tr(v.label())).clicked() {
                            d.duet.enabled = false;
                            d.linuxcnc = false;
                            d.connection.grbl_version = v;
                        }
                    }
                    if ui.selectable_label(d.duet.enabled, tr("Duet / RepRapFirmware (HTTP)")).clicked() {
                        d.duet.enabled = true;
                        d.linuxcnc = false;
                    }
                    if ui.selectable_label(d.linuxcnc, tr("LinuxCNC (file)")).clicked() {
                        d.linuxcnc = true;
                        d.duet.enabled = false;
                    }
                });
                ui.end_row();
                if d.linuxcnc {
                    // A file for LinuxCNC: no connection.
                } else if d.duet.enabled {
                    ui.label(tr("Host"));
                    ui.add(egui::TextEdit::singleline(&mut d.duet.host).hint_text("duet.local"));
                    ui.end_row();
                } else {
                    refresh = add::connection_rows(ui, &mut d.connection, &ports);
                    // `$$` is read from a GRBL that has it (not 0.7, not Smoothieware).
                    let can = matches!(d.connection.grbl_version, GrblVersion::Grbl11 | GrblVersion::Grbl10 | GrblVersion::Grbl09 | GrblVersion::Grbl08) && d.has_target();
                    read = add::read_row(ui, state, can);
                }
                ui.label(tr("Units"));
                egui::ComboBox::from_id_salt("add_units").selected_text(tr(d.units.label())).show_ui(ui, |ui| {
                    for u in base_core::units::Units::DEVICE {
                        ui.selectable_value(&mut d.units, u, tr(u.label()));
                    }
                });
                ui.end_row();
                let u = d.units;
                ui.label(tr("Work area"));
                ui.horizontal(|ui| {
                    ui.label("X");
                    drag_len(ui, u, &mut d.area.x, 1.0, Some((10.0, 10000.0)));
                    ui.label("Y");
                    drag_len(ui, u, &mut d.area.y, 1.0, Some((10.0, 10000.0)));
                    ui.label("Z");
                    drag_len(ui, u, &mut d.area.z, 1.0, Some((1.0, 2000.0)));
                });
                ui.end_row();
                ui.label(tr("Machine zero (0,0)"));
                egui::ComboBox::from_id_salt("add_origin").selected_text(tr(d.origin.label())).show_ui(ui, |ui| {
                    for o in Origin::ALL {
                        ui.selectable_value(&mut d.origin, o, tr(o.label()));
                    }
                });
                ui.end_row();
                ui.label(tr("Safe height"));
                drag_len(ui, u, &mut d.safe_z, 0.5, Some((0.1, d.area.z)));
                ui.end_row();
                ui.label(tr("Spindle"));
                ui.add(egui::DragValue::new(&mut d.spindle.max_rpm).range(1.0..=1000000.0).suffix(" rpm")).on_hover_text(tr("Highest speed"));
                ui.end_row();
            });
            ui.add_space(4.0);
            test_cam = add::camera_section(ui, &mut d.cameras, &mut d.active_camera, base_ui::camera::SUPPORTED);
            add::warnings(ui, &Self::add_dev_warnings(d), state == ReadState::Done);
            if !error.is_empty() {
                ui.label(RichText::new(&error).color(Color32::from_rgb(0xd0, 0x50, 0x20)));
            }
            !d.name.trim().is_empty() && !base_ui::camera_feed::lacks_address(&d.cameras)
        });
        if refresh {
            self.machine.refresh_ports();
        }
        if let Some(cam) = test_cam {
            self.cam_test_wanted = Some(cam);
        }
        let reading = a.read == ReadState::Reading;
        let mut keep = open;
        match action {
            Some(AddAction::Add) => {
                a.draft.name = a.draft.name.trim().to_string();
                match a.draft.check() {
                    Ok(()) => {
                        self.commit_device(&DeviceEdit { idx: None, draft: a.draft.clone(), page: Page::General, error: String::new() });
                        self.start_sel = self.devices.len().saturating_sub(1);
                        self.status = tr("The device was added. Press New to start.").to_string();
                        keep = false;
                    }
                    Err(e) => a.error = e,
                }
            }
            Some(AddAction::More) => {
                self.dev_edit = Some(DeviceEdit { idx: None, draft: a.draft.clone(), page: Page::General, error: String::new() });
                keep = false;
            }
            Some(AddAction::Dummy) => {
                self.use_dummy_device();
                keep = false;
            }
            Some(AddAction::Cancel) => keep = false,
            None => {}
        }
        if !keep {
            // A read that is still going on is stopped with the window.
            if reading {
                self.machine.disconnect();
            }
            return;
        }
        self.add_dev = Some(a);
        if read {
            self.add_dev_start_read();
        }
        if reading || read {
            ctx.request_repaint_after(Duration::from_millis(200));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_device_is_added_from_the_window_with_its_basic_settings() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        let before = a.devices.len();
        a.start_add_device();
        let mut out = ctx.run_ui(Default::default(), |ui| a.add_device_window(ui.ctx()));
        out.textures_delta.clear();
        assert!(a.add_dev.is_some(), "the window stays open");
        let w = App::add_dev_warnings(&a.add_dev.as_ref().unwrap().draft);
        assert_eq!(w.len(), 2, "no port yet, and the reminder: {w:?}");
        // Duet: its host is what is asked for.
        let mut duet = a.add_dev.as_ref().unwrap().draft.clone();
        duet.duet.enabled = true;
        assert_eq!(App::add_dev_warnings(&duet).len(), 2);
        // Added by hand: what the button does.
        let mut d = a.add_dev.take().unwrap().draft;
        d.name = " My router ".into();
        a.commit_device(&DeviceEdit { idx: None, draft: d, page: Page::General, error: String::new() });
        assert_eq!(a.devices.len(), before + 1);
        assert_eq!(a.devices.last().unwrap().name, " My router ");
    }

    #[test]
    fn a_read_without_an_answer_ends_as_an_error_and_closes_the_link() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        a.start_add_device();
        let ad = a.add_dev.as_mut().unwrap();
        ad.read = ReadState::Reading;
        ad.read_since = Some(Instant::now() - READ_TIMEOUT - Duration::from_secs(1));
        assert!(a.add_dev_reading());
        a.add_dev_follow_read();
        assert_eq!(a.add_dev.as_ref().unwrap().read, ReadState::Failed);
        assert!(!a.add_dev_reading());
    }

    #[test]
    fn linuxcnc_needs_no_port_and_start_saves_the_file() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        let mut d = Device { name: "Router".into(), ..Device::default() };
        let plain = serde_json::to_string(&d).unwrap();
        assert!(!plain.contains("linuxcnc"), "not written while off: older versions read the device as before");
        d.linuxcnc = true;
        let w = App::add_dev_warnings(&d);
        assert_eq!(w.len(), 2, "{w:?}");
        assert!(w[0].starts_with("LinuxCNC runs the program by itself") && !w.iter().any(|t| t.contains("No port")), "{w:?}");
        let back: Device = serde_json::from_str(&serde_json::to_string(&d).unwrap()).unwrap();
        assert!(back.linuxcnc);
        assert!(!a.can_start(), "a connected machine is needed for the others");
        a.device = d;
        assert!(a.can_start(), "Start is there without a link: it saves the file");
        assert!(!a.machine.connected);
    }
}
