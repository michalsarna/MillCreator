//! Devices in the program: the list on the start screen, the window that adds or changes a device, the Device tab of the
//! editor and the camera view of the device.
use crate::device::{self, Device, Origin, DUMMY_NAME, MAX_DEVICES};
use crate::App;
use base_core::camimg::{CameraKind, CameraSource};
use base_core::machine::{GrblVersion, LinkKind, RotaryKind, Streaming};
use base_core::units::Units;
use base_ui::camera::{self, rotate_rgb};
use base_ui::camera_stream::{self, Msg};
use base_ui::editor::EditorHost;
use base_ui::i18n::{tr, trf};
use base_ui::theme;
use base_ui::units_ui::{drag_len, fmt_len};
use base_ui::widgets::centered_row;
use base_ui::window::{delete_button, import_button, popup, save_button_enabled};
use eframe::egui::{self, Color32, RichText};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    General,
    Machine,
    Spindle,
    Connection,
    Camera,
    Rotary,
    Jog,
    Probe,
    Macros,
    Maintenance,
    Notes,
}

impl Page {
    const ALL: [Page; 11] = [Page::General, Page::Machine, Page::Spindle, Page::Connection, Page::Camera, Page::Rotary, Page::Jog, Page::Probe, Page::Macros, Page::Maintenance, Page::Notes];
    fn label(self) -> &'static str {
        match self {
            Page::General => "General",
            Page::Machine => "Machine",
            Page::Spindle => "Spindle",
            Page::Connection => "Connection",
            Page::Rotary => "Rotary",
            Page::Jog => "Jogging",
            Page::Probe => "Probe",
            Page::Macros => "Macros",
            Page::Maintenance => "Maintenance",
            Page::Camera => "Camera",
            Page::Notes => "Notes",
        }
    }
}

/// A device being added or changed.
pub struct DeviceEdit {
    /// The place in the list; `None` for a new device.
    pub idx: Option<usize>,
    pub draft: Device,
    pub page: Page,
    pub error: String,
}

/// The live picture of a camera.
pub struct CamState {
    key: String,
    url: Option<camera_stream::StreamWorker>,
    local: Option<camera::Worker>,
    pub tex: Option<egui::TextureHandle>,
    pub error: String,
}

/// Take what the controller reports into the draft of a device: the travel, the spindle speeds and the speed of moves.
fn apply_suggestion(d: &mut Device, s: &base_core::firmware::Suggestion) {
    if let Some(a) = s.area {
        d.area.x = a[0].clamp(10.0, 10_000.0);
        d.area.y = a[1].clamp(10.0, 10_000.0);
        d.area.z = a[2].clamp(1.0, 2_000.0);
        d.safe_z = d.safe_z.min(d.area.z).max(0.1);
    }
    if let Some((lo, hi)) = s.spindle {
        d.spindle.max_rpm = hi.max(1.0);
        d.spindle.min_rpm = lo.min(d.spindle.max_rpm);
    }
    if let Some(r) = s.rapid {
        d.rapid = r.clamp(1.0, 100_000.0);
    }
}

impl App {
    /// The list of devices that is kept between runs, as JSON text.
    pub(crate) fn devices_text(&self) -> String {
        device::save_all(&self.devices)
    }

    /// The stand-in device: made once, kept in the list, selected.
    pub(crate) fn use_dummy_device(&mut self) -> usize {
        let idx = match self.devices.iter().position(|d| d.name == DUMMY_NAME) {
            Some(i) => i,
            None => {
                self.devices.push(Device::dummy());
                self.devices.len() - 1
            }
        };
        self.start_sel = idx;
        self.dev_edit = None;
        idx
    }

    /// The device that is selected on the start screen becomes the device of the project; without any device it is the
    /// Dummy.
    pub(crate) fn take_selected_device(&mut self) {
        if self.devices.is_empty() {
            self.use_dummy_device();
        }
        if let Some(d) = self.devices.get(self.start_sel) {
            self.device = d.clone();
            self.active = Some(self.start_sel);
        }
    }

    pub(crate) fn start_add_device(&mut self) {
        self.dev_edit = Some(DeviceEdit { idx: None, draft: Device { name: String::new(), ..Device::default() }, page: Page::General, error: String::new() });
    }

    pub(crate) fn start_edit_device(&mut self, idx: usize) {
        self.confirm_delete = false;
        if let Some(d) = self.devices.get(idx) {
            self.dev_edit = Some(DeviceEdit { idx: Some(idx), draft: d.clone(), page: Page::General, error: String::new() });
        }
    }

    fn duplicate_device(&mut self, idx: usize) {
        let Some(src) = self.devices.get(idx).cloned() else { return };
        if self.devices.len() >= MAX_DEVICES {
            return;
        }
        let taken: Vec<String> = self.devices.iter().map(|d| d.name.clone()).collect();
        let mut copy = src;
        copy.name = Device::free_name(&copy.name, &taken);
        self.status = trf("Copied to {}", &[&copy.name]);
        self.devices.push(copy);
        self.start_sel = self.devices.len() - 1;
    }

    fn export_device_dialog(&mut self, idx: usize) {
        let Some(d) = self.devices.get(idx) else { return };
        let stem: String = d.name.chars().map(|c| if c.is_alphanumeric() { c } else { '-' }).collect::<String>().trim_matches('-').to_string();
        let name = format!("{}.json", if stem.is_empty() { "device".to_string() } else { stem });
        if let Some(p) = rfd::FileDialog::new().add_filter("MillCreator device", &["json"]).set_file_name(&name).save_file() {
            self.status = match std::fs::write(&p, device::export(d)) {
                Ok(()) => trf("Saved the device to {}", &[&p.display()]),
                Err(e) => trf("Could not save: {}", &[&e]),
            };
        }
    }

    fn import_device_dialog(&mut self) {
        let Some(p) = rfd::FileDialog::new().add_filter("MillCreator device", &["json"]).pick_file() else { return };
        self.import_device_from(&p);
    }

    pub(crate) fn import_device_from(&mut self, p: &std::path::Path) {
        let read = match std::fs::metadata(p) {
            Ok(m) if m.len() > device::MAX_FILE => Err("the file is too large".to_string()),
            Ok(_) => std::fs::read_to_string(p).map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        };
        match read.and_then(|t| device::import(&t)) {
            Ok(mut d) if self.devices.len() < MAX_DEVICES => {
                let taken: Vec<String> = self.devices.iter().map(|d| d.name.clone()).collect();
                d.name = Device::free_name(&d.name, &taken);
                self.status = trf("Imported the device {}", &[&d.name]);
                self.devices.push(d);
                self.start_sel = self.devices.len() - 1;
            }
            Ok(_) => self.status = tr("Too many devices").to_string(),
            Err(e) => self.status = trf("Cannot use the file: {}", &[&e]),
        }
    }

    /// The devices on the start screen with the buttons for them.
    pub(crate) fn device_section(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(tr("Device")).size(16.0).strong());
        ui.label(RichText::new(tr("Choose the machine you will work with.")).color(theme::text_dim()));
        ui.add_space(6.0);
        self.start_sel = self.start_sel.min(self.devices.len().saturating_sub(1));
        if self.devices.is_empty() {
            ui.label(RichText::new(tr("No device yet. Add one, or work without a machine.")).color(theme::text_dim()));
        }
        egui::ScrollArea::vertical().id_salt("device_list").max_height(150.0).auto_shrink([false, true]).show(ui, |ui| {
            for i in 0..self.devices.len() {
                let d = &self.devices[i];
                let text = format!("{}    {} × {}", d.name, fmt_len(d.units, d.area.x, 0), fmt_len(d.units, d.area.y, 0));
                if ui.add_sized([ui.available_width(), 26.0], egui::Button::selectable(self.start_sel == i, text)).clicked() {
                    self.start_sel = i;
                    self.confirm_delete = false;
                }
            }
        });
        ui.add_space(6.0);
        let has = !self.devices.is_empty();
        centered_row(ui, "device_buttons_1", |ui| {
            if ui.button(tr("Add device…")).clicked() {
                self.start_add_device();
            }
            if ui.add_enabled(has, egui::Button::new(tr("Edit…"))).clicked() {
                self.start_edit_device(self.start_sel);
            }
            if ui.add_enabled(has, egui::Button::new(tr("Duplicate"))).clicked() {
                self.duplicate_device(self.start_sel);
            }
            if import_button(ui, &tr("Import…")).on_hover_text(tr("Read a device from a file")).clicked() {
                self.import_device_dialog();
            }
        });
        centered_row(ui, "device_buttons_2", |ui| {
            if ui.button(tr("I have no device")).on_hover_text(tr("Work on a project without a machine: a plain device called Dummy, with the default work area and speeds")).clicked() {
                self.use_dummy_device();
            }
        });
    }

    /// The window that adds or changes a device.
    pub(crate) fn device_window(&mut self, ctx: &egui::Context) {
        let Some(mut edit) = self.dev_edit.take() else { return };
        let mut open = true;
        let mut save = false;
        let mut cancel = false;
        let mut export = false;
        let mut ask_delete = false;
        let mut delete = false;
        let confirm = self.confirm_delete;
        let ports = self.machine.ports.clone();
        let mut refresh_ports = false;
        let machine = &mut self.machine;
        let new = edit.idx.is_none();
        let title = if new { tr("Add a device") } else { tr("Device settings") };
        popup(ctx, "device_window", &title, [620.0, 540.0], &mut open, |ui| {
            ui.horizontal(|ui| {
                for p in Page::ALL {
                    if ui.add(egui::Button::selectable(edit.page == p, RichText::new(tr(p.label())).strong())).clicked() {
                        edit.page = p;
                    }
                }
            });
            ui.separator();
            let units = edit.draft.units;
            egui::ScrollArea::vertical().id_salt("device_page").max_height((ui.ctx().content_rect().height() - 220.0).clamp(160.0, 420.0)).auto_shrink([false, true]).show(ui, |ui| {
                let d = &mut edit.draft;
                match edit.page {
                    Page::General => {
                        egui::Grid::new("dev_general").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                            ui.label(tr("Name"));
                            ui.add(egui::TextEdit::singleline(&mut d.name).hint_text(tr("e.g. My CNC router")));
                            ui.end_row();
                            ui.label(tr("Controller"));
                            ui.label("GRBL");
                            ui.end_row();
                            if new {
                                ui.label(tr("Start from"));
                                egui::ComboBox::from_id_salt("dev_preset").selected_text(tr("A usual machine size…")).show_ui(ui, |ui| {
                                    for p in base_core::presets::with_plugins(base_core::presets::MILL, &self.plugins.registry, "millcreator", base_core::plugins::PresetClass::Mill) {
                                        if ui.selectable_label(false, &p.name).clicked() {
                                            d.area.x = p.area[0];
                                            d.area.y = p.area[1];
                                            d.area.z = p.area[2];
                                            d.safe_z = d.safe_z.min(p.area[2]);
                                            d.rapid = p.rapid;
                                            d.spindle.max_rpm = p.spindle_max;
                                            d.spindle.min_rpm = d.spindle.min_rpm.min(p.spindle_max);
                                        }
                                    }
                                });
                                ui.end_row();
                            }
                            ui.label(tr("Units"));
                            egui::ComboBox::from_id_salt("dev_units").selected_text(tr(d.units.label())).show_ui(ui, |ui| {
                                for u in Units::ALL {
                                    ui.selectable_value(&mut d.units, u, tr(u.label()));
                                }
                            });
                            ui.end_row();
                            ui.label(tr("Machine zero (0,0)"));
                            egui::ComboBox::from_id_salt("dev_origin").selected_text(tr(d.origin.label())).show_ui(ui, |ui| {
                                for o in Origin::ALL {
                                    ui.selectable_value(&mut d.origin, o, tr(o.label()));
                                }
                            });
                            ui.end_row();
                        });
                    }
                    Page::Machine => {
                        egui::Grid::new("dev_machine").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                            ui.label(tr("Work area"));
                            ui.horizontal(|ui| {
                                ui.label("X");
                                drag_len(ui, units, &mut d.area.x, 1.0, Some((10.0, 10000.0)));
                                ui.label("Y");
                                drag_len(ui, units, &mut d.area.y, 1.0, Some((10.0, 10000.0)));
                                ui.label("Z");
                                drag_len(ui, units, &mut d.area.z, 1.0, Some((1.0, 2000.0)));
                            });
                            ui.end_row();
                            ui.label(tr("Safe height"));
                            drag_len(ui, units, &mut d.safe_z, 0.5, Some((0.1, d.area.z)));
                            ui.end_row();
                            ui.label(tr("Speed of moves above the material"));
                            ui.add(egui::DragValue::new(&mut d.rapid).range(1.0..=100000.0).suffix(" mm/min"));
                            ui.end_row();
                        });
                        ui.add_space(4.0);
                        ui.label(RichText::new(tr("Z is how far the spindle goes down from the top of the material. The safe height is where the spindle moves between cuts.")).color(theme::text_dim()));
                        ui.add_space(8.0);
                        ui.separator();
                        let sugg = machine.suggestion();
                        if base_ui::device_pages::suggestion_ui(ui, machine, &sugg) {
                            apply_suggestion(d, &sugg);
                        }
                        ui.add_space(6.0);
                        base_ui::device_pages::copies_section(ui, machine, &mut d.control);
                    }
                    Page::Jog => base_ui::device_pages::jog_page(ui, &mut d.control),
                    Page::Probe => base_ui::device_pages::probe_page(ui, &mut d.control),
                    Page::Macros => base_ui::device_pages::macros_page(ui, &mut d.control),
                    Page::Maintenance => base_ui::device_pages::maintenance_page(ui, &mut d.control),
                    Page::Spindle => {
                        egui::Grid::new("dev_spindle").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                            ui.label(tr("Lowest speed"));
                            ui.add(egui::DragValue::new(&mut d.spindle.min_rpm).range(0.0..=1000000.0).suffix(" rpm"));
                            ui.end_row();
                            ui.label(tr("Highest speed"));
                            ui.add(egui::DragValue::new(&mut d.spindle.max_rpm).range(1.0..=1000000.0).suffix(" rpm"));
                            ui.end_row();
                            ui.label(tr("Wait after switching on"));
                            ui.add(egui::DragValue::new(&mut d.spindle.spin_up).range(0.0..=600.0).suffix(" s"));
                            ui.end_row();
                        });
                    }
                    Page::Connection => {
                        egui::Grid::new("dev_link").num_columns(2).min_col_width(150.0).spacing([12.0, 6.0]).show(ui, |ui| {
                            ui.label(tr("Connection type"));
                            egui::ComboBox::from_id_salt("link_kind").width(230.0).selected_text(tr(d.connection.link.label())).show_ui(ui, |ui| {
                                for k in LinkKind::ALL {
                                    ui.selectable_value(&mut d.connection.link, k, tr(k.label()));
                                }
                            });
                            ui.end_row();
                            if d.connection.link == LinkKind::Serial {
                                ui.label(tr("Port"));
                                ui.horizontal_wrapped(|ui| {
                                    egui::ComboBox::from_id_salt("cfg_port").width(220.0).selected_text(if d.connection.port.is_empty() { tr("No port") } else { d.connection.port.as_str() }).show_ui(ui, |ui| {
                                        for p in &ports {
                                            ui.selectable_value(&mut d.connection.port, p.name.clone(), p.label());
                                        }
                                    });
                                    if ui.button(tr("Refresh")).clicked() {
                                        refresh_ports = true;
                                    }
                                });
                                ui.end_row();
                                ui.label(tr("Port or USB id"));
                                ui.add(egui::TextEdit::singleline(&mut d.connection.port).hint_text("/dev/ttyUSB0  or  0403:6001").desired_width(220.0)).on_hover_text(tr("A port name, or the USB vendor and product id (VID:PID, hexadecimal): the port that has these ids is used."));
                                ui.end_row();
                                ui.label(tr("Bound USB device"));
                                ui.horizontal(|ui| {
                                    let ids = ports.iter().find(|p| p.name == d.connection.port).and_then(|p| p.usb);
                                    if d.connection.usb_id.is_empty() {
                                        ui.label(RichText::new(tr("none")).color(theme::text_dim()));
                                    } else {
                                        ui.label(RichText::new(&d.connection.usb_id).monospace());
                                    }
                                    if ui.add_enabled(ids.is_some(), egui::Button::new(tr("Bind to this port"))).on_hover_text(tr("Remember the USB ids of the chosen port: the port is found again by them when its name changes.")).clicked() {
                                        if let Some((v, p)) = ids {
                                            d.connection.usb_id = format!("{v:04x}:{p:04x}");
                                        }
                                    }
                                    if ui.add_enabled(!d.connection.usb_id.is_empty(), egui::Button::new(tr("Unbind"))).clicked() {
                                        d.connection.usb_id.clear();
                                    }
                                });
                                ui.end_row();
                                ui.label(tr("Baud rate"));
                                ui.add(egui::DragValue::new(&mut d.connection.baud).range(1200..=1_000_000));
                                ui.end_row();
                            } else {
                                ui.label(tr("Host"));
                                ui.add(egui::TextEdit::singleline(&mut d.connection.host).hint_text("raspberrypi.local").desired_width(200.0));
                                ui.end_row();
                                ui.label(tr("TCP port"));
                                ui.add(egui::DragValue::new(&mut d.connection.tcp_port).range(1..=65535));
                                ui.end_row();
                            }
                            ui.label(tr("Reconnect"));
                            ui.checkbox(&mut d.connection.auto_reconnect, tr("Connect again automatically")).on_hover_text(tr("When the link drops by itself, try again every few seconds until you press Disconnect."));
                            ui.end_row();
                            ui.label(tr("After connecting"));
                            ui.vertical(|ui| {
                                ui.checkbox(&mut d.connection.home_on_connect, tr("Home the machine"));
                                ui.checkbox(&mut d.connection.unlock_on_connect, tr("Clear an alarm"));
                                if d.connection.unlock_on_connect {
                                    ui.label(RichText::new(tr("Clearing the alarm skips homing: use it only when the head is known to stand in a safe place.")).color(Color32::from_rgb(0xe8, 0x8a, 0x1f)));
                                }
                            });
                            ui.end_row();
                            ui.label(tr("GRBL version"));
                            egui::ComboBox::from_id_salt("grbl_dialect").selected_text(tr(d.connection.grbl_version.label())).show_ui(ui, |ui| {
                                for k in GrblVersion::ALL {
                                    ui.selectable_value(&mut d.connection.grbl_version, k, tr(k.label()));
                                }
                            })
                            .response
                            .on_hover_text(tr("Older GRBL (0.9) reports positions in another format and has no jog command: the program moves the head with ordinary moves instead."));
                            ui.end_row();
                            ui.label(tr("Streaming"));
                            egui::ComboBox::from_id_salt("streaming").selected_text(tr(d.connection.streaming.label())).show_ui(ui, |ui| {
                                for m in Streaming::ALL {
                                    ui.selectable_value(&mut d.connection.streaming, m, tr(m.label()));
                                }
                            })
                            .response
                            .on_hover_text(tr("Counting fills the controller's buffer and is fast. Simple sends one line and waits for its ok: slow, but for firmware that answers wrongly."));
                            ui.end_row();
                        });
                    }
                    Page::Rotary => {
                        ui.checkbox(&mut d.rotary.enabled, tr("A rotary attachment is connected"));
                        ui.add_enabled_ui(d.rotary.enabled, |ui| {
                            egui::Grid::new("dev_rotary").num_columns(2).min_col_width(150.0).spacing([12.0, 6.0]).show(ui, |ui| {
                                ui.label(tr("Kind"));
                                ui.horizontal(|ui| {
                                    ui.radio_value(&mut d.rotary.kind, RotaryKind::Chuck, tr("Chuck"));
                                    ui.radio_value(&mut d.rotary.kind, RotaryKind::Roller, tr("Rollers"));
                                });
                                ui.end_row();
                                ui.label(tr("Workpiece diameter"));
                                drag_len(ui, units, &mut d.rotary.diameter, 0.5, Some((1.0, 1000.0)));
                                ui.end_row();
                                if d.rotary.kind == RotaryKind::Roller {
                                    ui.label(tr("Roller diameter"));
                                    drag_len(ui, units, &mut d.rotary.roller_diameter, 0.5, Some((1.0, 1000.0)));
                                    ui.end_row();
                                }
                                ui.label(tr("Microsteps per turn"));
                                ui.add(egui::DragValue::new(&mut d.rotary.microsteps_per_rev).range(1.0..=1_000_000.0)).on_hover_text(tr("Microsteps of the motor for one full turn of what it drives (the chuck, or one roller)."));
                                ui.end_row();
                                ui.label(tr("Circumference"));
                                ui.label(fmt_len(units, d.rotary.circumference(), 2)).on_hover_text(tr("The way round the workpiece: the design may not be higher."));
                                ui.end_row();
                            });
                            ui.label(RichText::new(tr("GRBL 1.1 has three axes: the A axis is not available, Y is used.")).color(Color32::from_rgb(0xe8, 0x8a, 0x1f)));
                            ui.label(trf("The controller needs {} steps per mm on the Y axis ($101).", &[&format!("{:.3}", d.rotary.steps_per_mm())]));
                        });
                        ui.add_space(4.0);
                        ui.label(RichText::new(tr("With a rotary attachment the Y axis turns the work: the drawing may not be higher than the way round the workpiece. The model is simple (a chuck turns by the workpiece diameter, rollers move the surface like their own); a real machine needs testing.")).color(theme::text_dim()));
                    }
                    Page::Camera => camera_page(ui, d),
                    Page::Notes => {
                        ui.add(egui::TextEdit::multiline(&mut d.notes).desired_rows(10).desired_width(f32::INFINITY).hint_text(tr("Notes about this machine")));
                    }
                }
            });
            ui.add_space(6.0);
            if !edit.error.is_empty() {
                ui.label(RichText::new(&edit.error).color(Color32::from_rgb(0xd0, 0x50, 0x20)));
            }
            ui.horizontal(|ui| {
                let label = if new { tr("Add device") } else { tr("Save") };
                let go = egui::Button::new(RichText::new(label).color(Color32::WHITE).strong()).fill(Color32::from_rgb(0x2e, 0xa0, 0x4f));
                if ui.add_enabled(!edit.draft.name.trim().is_empty(), go).clicked() {
                    save = true;
                }
                if ui.button(tr("Cancel")).clicked() {
                    cancel = true;
                }
                if !new {
                    ui.separator();
                    if save_button_enabled(ui, true, &tr("Export…")).on_hover_text(tr("Save this device to a file")).clicked() {
                        export = true;
                    }
                    if confirm {
                        if delete_button(ui, &tr("Confirm delete")).clicked() {
                            delete = true;
                        }
                    } else if delete_button(ui, &tr("Delete")).clicked() {
                        ask_delete = true;
                    }
                }
            });
        });
        if refresh_ports {
            self.machine.refresh_ports();
        }
        if let Some(i) = edit.idx {
            if export {
                self.export_device_dialog(i);
            }
            if ask_delete {
                self.confirm_delete = true;
            }
            if delete && i < self.devices.len() {
                self.devices.remove(i);
                self.confirm_delete = false;
                self.active = match self.active {
                    Some(a) if a == i => None,
                    Some(a) if a > i => Some(a - 1),
                    other => other,
                };
                self.start_sel = self.start_sel.min(self.devices.len().saturating_sub(1));
                return;
            }
        }
        if cancel {
            self.confirm_delete = false;
            open = false;
        }
        if save {
            edit.draft.name = edit.draft.name.trim().to_string();
            match edit.draft.check() {
                Ok(()) => {
                    self.commit_device(&edit);
                    return;
                }
                Err(e) => edit.error = e,
            }
        }
        if open {
            self.dev_edit = Some(edit);
        }
    }

    /// The edited device goes into the list (and into the project, when it is the device in use).
    fn commit_device(&mut self, edit: &DeviceEdit) {
        let mut d = edit.draft.clone();
        match edit.idx {
            None => {
                if self.devices.len() >= MAX_DEVICES {
                    self.status = tr("Too many devices").to_string();
                    return;
                }
                let taken: Vec<String> = self.devices.iter().map(|d| d.name.clone()).collect();
                d.name = Device::free_name(&d.name, &taken);
                self.devices.push(d);
                self.start_sel = self.devices.len() - 1;
                self.status = tr("The device was added.").to_string();
            }
            Some(i) => {
                let others: Vec<String> = self.devices.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, d)| d.name.clone()).collect();
                d.name = Device::free_name(&d.name, &others);
                if self.active == Some(i) {
                    self.device = d.clone();
                    self.touch();
                }
                if let Some(slot) = self.devices.get_mut(i) {
                    *slot = d;
                }
                self.status = tr("The device was saved.").to_string();
            }
        }
    }

    /// The Device tab of the editor: the device in use and a way to change it.
    pub(crate) fn device_tab(&mut self, ui: &mut egui::Ui) {
        let d = self.device.clone();
        ui.label(RichText::new(&d.name).size(16.0).strong());
        ui.add_space(4.0);
        egui::Grid::new("dev_summary").num_columns(2).spacing([14.0, 4.0]).show(ui, |ui| {
            fn row(ui: &mut egui::Ui, k: &'static str, v: String) {
                ui.label(RichText::new(tr(k)).color(theme::text_dim()));
                ui.label(v);
                ui.end_row();
            }
            row(ui, "Work area", format!("{} × {} × {}", fmt_len(d.units, d.area.x, 0), fmt_len(d.units, d.area.y, 0), fmt_len(d.units, d.area.z, 0)));
            row(ui, "Machine zero (0,0)", tr(d.origin.label()).to_string());
            row(ui, "Safe height", fmt_len(d.units, d.safe_z, 1));
            row(ui, "Spindle", format!("{}…{} rpm", d.spindle.min_rpm, d.spindle.max_rpm));
            row(ui, "Speed of moves above the material", format!("{} mm/min", d.rapid));
            row(ui, "Cameras", d.cameras.len().to_string());
        });
        ui.add_space(8.0);
        if let Some(i) = self.active {
            if ui.button(tr("Device settings…")).clicked() {
                self.start_edit_device(i);
            }
        } else if ui.button(tr("Device settings…")).on_hover_text(tr("This device is not in the list: it came with the project")).clicked() {
            self.dev_edit = Some(DeviceEdit { idx: None, draft: d.clone(), page: Page::General, error: String::new() });
        }
        if d.is_dummy() {
            ui.add_space(6.0);
            ui.label(RichText::new(tr("This is the stand-in device (Dummy): the default work area and speeds.")).color(theme::text_dim()));
        }
        if !d.notes.trim().is_empty() {
            ui.add_space(8.0);
            ui.label(RichText::new(tr("Notes")).strong());
            ui.label(&d.notes);
        }
    }

    // ───────────── camera ─────────────

    /// The Camera tab: the live picture of the active camera of the device.
    pub(crate) fn camera_tab(&mut self, ui: &mut egui::Ui) {
        let Some(cam) = self.device.camera().cloned() else {
            ui.label(RichText::new(tr("This device has no camera that is switched on. Add one in the device settings.")).color(theme::text_dim()));
            self.cam = None;
            return;
        };
        let ctx = ui.ctx().clone();
        let key = format!("{:?}", cam.kind);
        if self.cam.as_ref().map(|c| c.key.as_str()) != Some(key.as_str()) {
            let mut st = CamState { key, url: None, local: None, tex: None, error: String::new() };
            match &cam.kind {
                CameraKind::Url(u) => st.url = Some(camera_stream::spawn(u.clone(), ctx.clone())),
                CameraKind::Local(i) if camera::SUPPORTED => st.local = Some(camera::spawn(*i, cam.image.prefer_yuyv, ctx.clone())),
                CameraKind::Local(_) => st.error = tr("This build cannot use the cameras of the computer.").to_string(),
            }
            self.cam = Some(st);
        }
        let Some(st) = self.cam.as_mut() else { return };
        let mut latest: Option<(u32, u32, Vec<u8>)> = None;
        if let Some(w) = &st.url {
            while let Ok(m) = w.rx.try_recv() {
                match m {
                    Msg::Frame(f) => {
                        st.error.clear();
                        latest = Some((f.w, f.h, f.rgb));
                    }
                    Msg::Error(e) => st.error = e,
                }
            }
        }
        if let Some(w) = &st.local {
            while let Ok(m) = w.rx.try_recv() {
                match m {
                    Ok(f) => {
                        st.error.clear();
                        latest = Some((f.w, f.h, f.rgb));
                    }
                    Err(e) => st.error = e,
                }
            }
        }
        if let Some((w, h, rgb)) = latest {
            let (w, h, rgb) = process_frame(&cam, rgb, w, h);
            let img = egui::ColorImage::from_rgb([w as usize, h as usize], &rgb);
            match &mut st.tex {
                Some(t) if t.size() == [w as usize, h as usize] => t.set(img, egui::TextureOptions::LINEAR),
                _ => st.tex = Some(ctx.load_texture("camera-view", img, egui::TextureOptions::LINEAR)),
            }
        }
        ui.label(RichText::new(&cam.name).strong());
        match &st.tex {
            Some(t) => {
                let size = t.size_vec2();
                // Fitted to the window, up as well as down; the line of a stream error under it is left out of the height, or the
                // window would grow by that on every frame.
                let line = ui.text_style_height(&egui::TextStyle::Body) + ui.spacing().item_spacing.y;
                let under = ui.spacing().item_spacing.y + if st.error.is_empty() { 0.0 } else { line };
                let scale = (ui.available_width().max(60.0) / size.x).min((ui.available_height() - under).max(60.0) / size.y);
                ui.add(egui::Image::new(egui::load::SizedTexture::new(t.id(), size * scale)));
            }
            None => {
                ui.label(RichText::new(tr("Waiting for the picture…")).color(theme::text_dim()));
            }
        }
        if !st.error.is_empty() {
            ui.label(RichText::new(&st.error).color(Color32::from_rgb(0xd0, 0x50, 0x20)));
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(120));
    }
}

/// The picture of a camera as it is shown: lens correction, adjustments and the turn.
pub fn process_frame(cam: &CameraSource, rgb: Vec<u8>, w: u32, h: u32) -> (u32, u32, Vec<u8>) {
    let (wu, hu) = (w as usize, h as usize);
    let mut out = if cam.lens.is_none() { rgb } else { base_core::camimg::undistort(&rgb, wu, hu, &cam.lens) };
    base_core::camimg::adjust(&mut out, wu, hu, &cam.image);
    rotate_rgb(&out, w, h, cam.rotation % 4)
}

fn camera_page(ui: &mut egui::Ui, d: &mut Device) {
    let mut remove = None;
    for (i, c) in d.cameras.iter_mut().enumerate() {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut c.enabled, "").on_hover_text(tr("Switched on"));
                ui.add(egui::TextEdit::singleline(&mut c.name).hint_text(tr("Camera name")).desired_width(200.0));
                if base_ui::window::delete_icon(ui, &tr("Delete this camera")).clicked() {
                    remove = Some(i);
                }
            });
            ui.horizontal(|ui| {
                let local = matches!(c.kind, CameraKind::Local(_));
                let mut want_local = local;
                ui.radio_value(&mut want_local, false, tr("Address"));
                ui.radio_value(&mut want_local, true, tr("Camera of this computer"));
                if want_local != local {
                    c.kind = if want_local { CameraKind::Local(0) } else { CameraKind::Url(String::new()) };
                }
            });
            match &mut c.kind {
                CameraKind::Url(u) => {
                    ui.add(egui::TextEdit::singleline(u).hint_text("http://raspberrypi.local:8080/stream.mjpg").desired_width(340.0));
                    if !u.trim().is_empty() && !base_core::camimg::valid_url(u) {
                        ui.label(RichText::new(tr("Not an http or https address.")).color(Color32::from_rgb(0xc0, 0x30, 0x30)));
                    }
                }
                CameraKind::Local(n) => {
                    ui.horizontal(|ui| {
                        ui.label(tr("Camera number"));
                        ui.add(egui::DragValue::new(n).range(0..=15));
                    });
                }
            }
            ui.horizontal(|ui| {
                ui.label(tr("Camera rotation"));
                for q in 0..4u8 {
                    ui.selectable_value(&mut c.rotation, q, format!("{}°", q as u32 * 90));
                }
            });
        });
    }
    if let Some(i) = remove {
        d.cameras.remove(i);
        d.active_camera = d.active_camera.min(d.cameras.len().saturating_sub(1));
    }
    if d.cameras.len() < 8 && ui.button(tr("Add a camera")).clicked() {
        d.cameras.push(CameraSource::url(&tr("Camera"), "", 0));
    }
    if d.cameras.len() > 1 {
        ui.horizontal(|ui| {
            ui.label(tr("Shown in the Camera tab"));
            egui::ComboBox::from_id_salt("dev_active_cam").selected_text(d.cameras.get(d.active_camera).map(|c| c.name.clone()).unwrap_or_default()).show_ui(ui, |ui| {
                for (i, c) in d.cameras.iter().enumerate() {
                    ui.selectable_value(&mut d.active_camera, i, &c.name);
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        App::build(&egui::Context::default(), None, false)
    }

    #[test]
    fn without_a_device_the_dummy_is_made_once_and_selected() {
        let mut a = app();
        a.devices.clear();
        let i = a.use_dummy_device();
        assert_eq!((i, a.devices.len(), a.devices[0].name.as_str()), (0, 1, DUMMY_NAME));
        assert_eq!(a.use_dummy_device(), 0);
        assert_eq!(a.devices.len(), 1, "the same one the second time");
        a.take_selected_device();
        assert!(a.device.is_dummy() && a.active == Some(0));
        assert_eq!(a.work_size_mm(), (300.0, 200.0));
    }

    #[test]
    fn a_new_project_without_any_device_gets_the_dummy() {
        let mut a = app();
        a.devices.clear();
        a.new_design();
        assert_eq!((a.screen, a.devices.len(), a.device.is_dummy()), (crate::Screen::Editor, 1, true));
    }

    #[test]
    fn a_device_is_added_changed_and_the_name_stays_unique() {
        let mut a = app();
        a.devices.clear();
        a.start_add_device();
        {
            let e = a.dev_edit.as_mut().unwrap();
            e.draft.name = "Router".into();
            e.draft.area.x = 600.0;
        }
        let e = a.dev_edit.take().unwrap();
        a.commit_device(&e);
        assert_eq!(a.devices[0].area.x, 600.0);
        // The same name again gets a number.
        let mut e2 = DeviceEdit { idx: None, draft: Device { name: "Router".into(), ..Device::default() }, page: Page::General, error: String::new() };
        e2.draft.area.y = 400.0;
        a.commit_device(&e2);
        assert_eq!(a.devices[1].name, "Router (2)");
        // Changing the device in use changes the device of the project.
        a.start_sel = 0;
        a.take_selected_device();
        a.start_edit_device(0);
        a.dev_edit.as_mut().unwrap().draft.safe_z = 8.0;
        let e = a.dev_edit.take().unwrap();
        a.commit_device(&e);
        assert_eq!(a.device.safe_z, 8.0);
    }

    #[test]
    fn a_device_file_is_imported_and_a_bad_one_is_refused() {
        let mut a = app();
        a.devices.clear();
        let dir = std::env::temp_dir().join(format!("mc-dev-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut d = Device::dummy();
        d.name = "From file".into();
        let ok = dir.join("d.json");
        std::fs::write(&ok, device::export(&d)).unwrap();
        a.import_device_from(&ok);
        assert_eq!(a.devices.len(), 1);
        a.import_device_from(&ok);
        assert_eq!(a.devices[1].name, "From file (2)");
        let bad = dir.join("bad.json");
        std::fs::write(&bad, "nonsense").unwrap();
        a.import_device_from(&bad);
        assert_eq!(a.devices.len(), 2);
        assert!(a.status.starts_with("Cannot use the file"), "{}", a.status);
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn the_windows_and_tabs_draw() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        a.devices = vec![Device::dummy()];
        a.start_add_device();
        for page in Page::ALL {
            a.dev_edit.as_mut().unwrap().page = page;
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| a.device_window(ui.ctx()));
            out.textures_delta.clear();
        }
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            a.device_section(ui);
            a.device_tab(ui);
            a.camera_tab(ui);
        });
        out.textures_delta.clear();
    }

    #[test]
    fn the_camera_picture_is_fitted_to_the_window_up_as_well_as_down_and_the_window_keeps_its_size() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        a.device.cameras = vec![CameraSource::url("c", "http://127.0.0.1:9/none.mjpg", 1)];
        a.device.active_camera = 0;
        let mut rects = vec![];
        for i in 0..8 {
            let input = egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1600.0, 1200.0))), ..Default::default() };
            let mut shown = None;
            let mut out = ctx.run_ui(input, |ui| {
                shown = egui::Window::new("cam").default_size([1000.0, 800.0]).resizable(true).show(ui.ctx(), |ui| a.camera_tab(ui)).map(|r| r.response.rect);
            });
            out.textures_delta.clear();
            if i == 0 {
                a.cam.as_mut().expect("the camera state").tex = Some(ctx.load_texture("t", egui::ColorImage::from_rgb([320, 180], &vec![0u8; 320 * 180 * 3]), egui::TextureOptions::LINEAR));
            }
            rects.push(shown.expect("the window"));
        }
        assert!((rects[3].height() - rects[7].height()).abs() < 1.0, "the window grew: {rects:?}");
        assert!(rects[7].width() > 900.0 && rects[7].height() > 500.0, "{:?}", rects[7]);
    }

    #[test]
    fn a_frame_is_turned_with_the_camera() {
        let mut cam = CameraSource::url("c", "http://x/y", 1);
        cam.lens = Default::default();
        let rgb = vec![0u8; 4 * 2 * 3];
        let (w, h, out) = process_frame(&cam, rgb, 4, 2);
        assert_eq!((w, h, out.len()), (2, 4, 24));
    }
}
