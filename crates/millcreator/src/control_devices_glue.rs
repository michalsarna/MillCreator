//! The USB keyboard or pad in this program: what its presses do to the machine (`base_ui::control_devices_ui` has the rest).
use crate::menu::Act;
use crate::App;
use base_core::control_devices::{Action, Fired};
use base_ui::control_devices_ui::JogCfg;
use base_ui::control_ui;
use base_ui::menu::MenuHost;
use eframe::egui;

/// What the keys of a mill can do.
pub(crate) const ACTIONS: &[Action] = &[
    Action::MoveLeft, Action::MoveRight, Action::MoveForward, Action::MoveBack, Action::MoveUp, Action::MoveDown, Action::Stop, Action::Pause, Action::Resume, Action::Home, Action::Unlock,
    Action::StartJob, Action::Frame, Action::Spindle, Action::Air, Action::Water,
];

impl App {
    /// Once a frame: read the device and do what it says. Moves are shared; the rest is done as the buttons of the Control tab do it.
    pub(crate) fn control_devices_frame(&mut self, ctx: &egui::Context) {
        let listening = self.machine.connected && self.screen == crate::Screen::Editor;
        let own = self.device.control_device.clone();
        let fired = self.control_devices.poll(ctx, own.as_deref(), listening);
        let (step_xy, step_z, feed_xy, feed_z) = control_ui::jog_numbers(&self.control_ui, &self.device.control);
        let hold = self.device.connection.grbl_version == base_core::machine::GrblVersion::Grbl11;
        let cfg = JogCfg { step_xy, step_z, feed_xy, feed_z, hold, z_axis: true };
        for f in self.control_devices.apply_moves(ctx, fired, &mut self.machine, &cfg) {
            let Fired::Press(a) = f else { continue };
            match a {
                Action::Stop => self.control_device_stop(),
                Action::Pause => self.pause_job(),
                Action::Resume => self.resume_job(),
                Action::Home => self.machine.home(),
                Action::Unlock => self.machine.unlock(),
                Action::StartJob if self.act_enabled(Act::StartJob) => self.start_job(),
                Action::Frame if self.act_enabled(Act::Frame) => self.frame_job(),
                Action::Spindle => self.control_device_spindle(),
                Action::Air | Action::Water => self.control_device_coolant(a),
                _ => {}
            }
        }
    }

    /// Stop what runs; with nothing running, switch off what the keys switched on.
    fn control_device_stop(&mut self) {
        if self.machine.working() {
            self.abort_job();
        } else {
            if self.control_devices.is_on(Action::Spindle) {
                self.machine.line("M5");
            }
            if self.control_devices.is_on(Action::Air) || self.control_devices.is_on(Action::Water) {
                self.machine.line("M9");
            }
        }
        self.control_devices.all_off();
    }

    fn control_device_spindle(&mut self) {
        if self.control_devices.toggle(Action::Spindle) {
            let sp = &self.device.spindle;
            let rpm = self.control_ui.spindle_rpm.unwrap_or((sp.max_rpm * 0.5).max(sp.min_rpm));
            let s = self.machine.spindle_s(rpm, sp.max_rpm);
            self.machine.line(format!("M3 S{s}"));
        } else {
            self.machine.line("M5");
        }
    }

    /// Air is M7 and water M8, as on the Control tab. M9 switches both off, so the one that stays on is asked for again.
    fn control_device_coolant(&mut self, a: Action) {
        if self.control_devices.toggle(a) {
            self.machine.line(if a == Action::Air { "M7" } else { "M8" });
        } else {
            self.machine.line("M9");
            let other = if a == Action::Air { Action::Water } else { Action::Air };
            if self.control_devices.is_on(other) {
                self.machine.line(if other == Action::Air { "M7" } else { "M8" });
            }
        }
    }

    /// The windows of the settings. What is chosen for this machine stays with it.
    pub(crate) fn control_devices_windows(&mut self, ctx: &egui::Context) {
        let mut own = self.device.control_device.clone();
        let mut changed = self.control_devices.window(ctx, ACTIONS, &mut own, true);
        changed |= self.control_devices.choice_window(ctx, &mut own);
        if changed {
            self.device.control_device = own.clone();
            if let Some(d) = self.active.and_then(|i| self.devices.get_mut(i)) {
                d.control_device = own;
            }
        }
    }
}
