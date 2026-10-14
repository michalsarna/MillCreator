//! The machine: the link to a GRBL controller (from baseCreator), the buttons that use it (connect, home, frame, start,
//! stop, pause) and the little console.
use crate::device::Device;
use crate::toolpath;
use crate::App;
use base_core::machine::{Controller, LinkKind};
use base_ui::i18n::{tr, trf};
use base_ui::link::{self, Cmd, ConsoleLine, Evt, LaserLink, PortInfo, Target};
use base_ui::theme;
use eframe::egui::{self, Color32, RichText};

/// The state of the link to the machine.
pub struct MachineLink {
    link: LaserLink,
    pub connected: bool,
    /// The user asked for the link (so a lost one is worth opening again).
    pub want_link: bool,
    pub reconnect_at: Option<std::time::Instant>,
    /// The state word of the last status report (`Idle`, `Run`, `Hold:0`, `Alarm`…) and the position.
    pub state: String,
    pub pos: [f64; 3],
    /// Lines of the job acknowledged and the number of lines of the job.
    pub progress: (usize, usize),
    pub console: base_ui::console_ui::ConsoleState,
    pub ports: Vec<PortInfo>,
}

impl MachineLink {
    pub fn new(ctx: &egui::Context) -> MachineLink {
        MachineLink { link: LaserLink::spawn(ctx.clone()), connected: false, want_link: false, reconnect_at: None, state: String::new(), pos: [f64::NAN; 3], progress: (0, 0), console: Default::default(), ports: vec![] }
    }

    pub fn refresh_ports(&mut self) {
        self.ports = link::list_port_info();
    }

    fn push(&mut self, l: ConsoleLine) {
        self.console.push_capped(l);
    }

    pub fn job_running(&self) -> bool {
        self.progress.1 > 0 && self.progress.0 < self.progress.1
    }
}

/// Where the device says to connect, if anywhere.
pub fn target_of(dev: &Device) -> Option<Target> {
    if !dev.has_target() {
        return None;
    }
    Some(match dev.connection.link {
        LinkKind::Serial => Target::Serial { port: link::resolve_port(&dev.connection.port, &dev.connection.usb_id, &link::list_port_info()), baud: dev.connection.baud },
        LinkKind::Tcp => Target::Tcp { host: dev.connection.host.trim().to_string(), port: dev.connection.tcp_port },
    })
}

impl App {
    pub(crate) fn connect_machine(&mut self) {
        let Some(target) = target_of(&self.device) else {
            self.status = tr("Choose a port or a host in the device settings first.").to_string();
            return;
        };
        self.machine.want_link = true;
        self.machine.link.send(Cmd::Connect { target, controller: Controller::Grbl, streaming: self.device.connection.streaming });
    }

    pub(crate) fn disconnect_machine(&mut self) {
        self.machine.want_link = false;
        self.machine.reconnect_at = None;
        self.machine.link.send(Cmd::Disconnect);
    }

    /// Take what the worker reports; called every frame.
    pub(crate) fn poll_machine(&mut self) {
        let events = self.machine.link.poll();
        for e in events {
            match e {
                Evt::Log(s) => self.machine.push(ConsoleLine::info(s)),
                Evt::Traffic(l) => self.machine.push(l),
                Evt::Connected(c) => {
                    let lost = self.machine.connected && !c;
                    self.machine.connected = c;
                    if lost && self.machine.want_link && self.device.connection.auto_reconnect {
                        self.machine.reconnect_at = Some(std::time::Instant::now() + std::time::Duration::from_secs(3));
                    }
                    if c {
                        self.machine.reconnect_at = None;
                        if self.device.connection.unlock_on_connect {
                            self.machine.link.send(Cmd::Line("$X".into()));
                        } else if self.device.connection.home_on_connect {
                            self.machine.link.send(Cmd::Line("$H".into()));
                        }
                    } else {
                        self.machine.state.clear();
                        self.machine.progress = (0, 0);
                    }
                }
                Evt::Status { state, x, y, z } => {
                    self.machine.state = state;
                    self.machine.pos = [x, y, z];
                }
                Evt::Progress { done, total } => {
                    let was = self.machine.job_running();
                    self.machine.progress = (done, total);
                    if was && done >= total && total > 0 {
                        self.status = tr("The job is finished.").to_string();
                    }
                }
                Evt::Stalled { secs, state } => {
                    self.status = trf("No answer from the controller for {} s while a job is running (state: {}).", &[&secs, &state]);
                }
            }
        }
        if self.machine.reconnect_at.is_some_and(|t| t <= std::time::Instant::now()) {
            self.machine.reconnect_at = None;
            if self.machine.want_link && !self.machine.connected {
                self.connect_machine();
            }
        }
    }

    pub(crate) fn can_start(&self) -> bool {
        self.machine.connected && !self.machine.job_running()
    }

    /// Frame: the tool goes round the box where the work will be done, at the safe height, with the spindle off.
    pub(crate) fn frame_job(&mut self) {
        if !self.machine.connected {
            self.status = tr("Connect to the machine first (Device tab)").to_string();
            return;
        }
        if self.machine.job_running() {
            return;
        }
        let prog = self.make_program();
        let b = prog.bounds.map(|(lo, hi)| [lo[0], lo[1], hi[0], hi[1]]).or_else(|| self.shapes_box());
        let Some(b) = b else {
            self.status = tr("There is nothing to frame: put shapes on layers that have a bit.").to_string();
            return;
        };
        for l in link::clean_gcode(&toolpath::frame_gcode(b, &self.device)) {
            self.machine.link.send(Cmd::Line(l));
        }
        self.status = tr("Framing the work area").to_string();
    }

    pub(crate) fn start_job(&mut self) {
        if !self.machine.connected {
            self.status = tr("Connect to the machine first (Device tab)").to_string();
            return;
        }
        let prog = self.make_program();
        if prog.is_empty() {
            self.status = tr("There is nothing to cut: put shapes on layers that have a bit.").to_string();
            return;
        }
        let name = self.path.as_ref().and_then(|p| p.file_stem()).map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "untitled".into());
        let lines = link::clean_gcode(&toolpath::to_gcode(&prog, &self.device, &self.layers, &self.tools, &name));
        self.machine.push(ConsoleLine::info(trf("Starting job: {} lines, ~{}", &[&lines.len(), &base_ui::widgets::fmt_time(prog.estimate_seconds(self.device.rapid))])));
        self.machine.link.send(Cmd::Job(lines));
        self.status = tr("The job is running.").to_string();
    }

    pub(crate) fn abort_job(&mut self) {
        self.machine.link.send(Cmd::Abort);
    }

    pub(crate) fn pause_job(&mut self) {
        self.machine.link.send(Cmd::Realtime(b'!'));
    }

    pub(crate) fn resume_job(&mut self) {
        self.machine.link.send(Cmd::Realtime(b'~'));
    }

    pub(crate) fn machine_line(&mut self, line: &str) {
        self.machine.link.send(Cmd::Line(line.to_string()));
    }

    /// The Connection part of the Device tab: connect, the state, the positions and the buttons that need the link.
    pub(crate) fn machine_section(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(tr("Machine")).strong());
        ui.horizontal_wrapped(|ui| {
            if self.machine.connected {
                if ui.button(tr("Disconnect")).clicked() {
                    self.disconnect_machine();
                }
            } else if ui.add_enabled(self.device.has_target(), egui::Button::new(tr("Connect"))).on_hover_text(tr("Open the link to the controller set in the device settings")).clicked() {
                self.connect_machine();
            }
            let (text, color) = if self.machine.connected { (tr("Connected"), Color32::from_rgb(0x2e, 0xa0, 0x4f)) } else { (tr("Disconnected"), theme::text_dim()) };
            ui.label(RichText::new(text).color(color));
            if !self.machine.state.is_empty() {
                ui.label(RichText::new(&self.machine.state).strong());
            }
        });
        if self.machine.connected && self.machine.pos.iter().all(|v| v.is_finite()) {
            let p = self.machine.pos;
            ui.label(RichText::new(format!("X {:.3}   Y {:.3}   Z {:.3}", p[0], p[1], p[2])).monospace());
        }
        ui.horizontal_wrapped(|ui| {
            let idle = self.machine.connected && !self.machine.job_running();
            if ui.add_enabled(idle, egui::Button::new(tr("Home"))).clicked() {
                self.machine_line("$H");
            }
            if ui.add_enabled(idle, egui::Button::new(tr("Unlock"))).on_hover_text(tr("Clear an alarm ($X)")).clicked() {
                self.machine_line("$X");
            }
            if ui.add_enabled(self.machine.connected, egui::Button::new(tr("Pause"))).on_hover_text(tr("Feed hold")).clicked() {
                self.pause_job();
            }
            if ui.add_enabled(self.machine.connected, egui::Button::new(tr("Resume"))).clicked() {
                self.resume_job();
            }
        });
        if self.machine.job_running() {
            let (d, t) = self.machine.progress;
            ui.add(egui::ProgressBar::new(d as f32 / t.max(1) as f32).text(format!("{d} / {t}")));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_device_without_a_port_or_host_has_no_target_and_a_port_gives_one() {
        let mut d = Device::dummy();
        assert!(target_of(&d).is_none());
        d.connection.port = "/dev/ttyUSB0".into();
        assert!(matches!(target_of(&d), Some(Target::Serial { baud: 115_200, .. })));
        d.connection.link = LinkKind::Tcp;
        assert!(target_of(&d).is_none());
        d.connection.host = "pi.local".into();
        assert!(matches!(target_of(&d), Some(Target::Tcp { port: 3333, .. })));
    }
}
