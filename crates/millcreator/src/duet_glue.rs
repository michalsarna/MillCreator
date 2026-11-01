//! A Duet board (RepRapFirmware) in the Control tab: the program is made as usual, sent over HTTP, started, and the state of the board is
//! asked for. The network work runs in a thread; its answer comes back as a line for the status bar.
use crate::App;
use base_ui::duet;
use base_ui::i18n::{tr, trf};
use eframe::egui::{self, RichText};
use std::sync::mpsc;

/// What the thread of a request does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DuetJob {
    SendAndStart,
    State,
    Stop,
}

impl App {
    /// The G-code as the board gets it: the program of the project as written for this device (the S values are rpm, no GRBL range).
    pub(crate) fn duet_program(&mut self) -> Option<(String, String)> {
        if !self.gcode_variables_ok() {
            return None;
        }
        let prog = self.make_program();
        if prog.is_empty() {
            self.status = self.nothing_to_cut();
            return None;
        }
        let name = self.path.as_ref().and_then(|p| p.file_stem()).map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "untitled".into());
        let text = crate::toolpath::to_gcode_info(&prog, &self.device.with_stock(&self.stock), &self.layers, &self.tools, &self.gcode_info(&name));
        Some((name, text))
    }

    /// Start a request in a thread; the answer is picked up by `poll_duet`.
    pub(crate) fn duet_request(&mut self, job: DuetJob) {
        if self.duet_rx.is_some() {
            return;
        }
        let (host, password) = (self.device.duet.host.trim().to_string(), self.device.duet.password.clone());
        if !duet::valid_host(&host) {
            self.status = tr("Set the address of the board first (Device settings → Connection).").to_string();
            return;
        }
        let program = if job == DuetJob::SendAndStart {
            match self.duet_program() {
                Some(p) => Some(p),
                None => return,
            }
        } else {
            None
        };
        let (tx, rx) = mpsc::channel();
        self.duet_rx = Some(rx);
        self.status = tr("Talking to the Duet…").to_string();
        std::thread::spawn(move || {
            let run = || -> Result<String, String> {
                duet::connect(&host, &password)?;
                match (job, program) {
                    (DuetJob::SendAndStart, Some((name, text))) => {
                        duet::upload(&host, &name, &text)?;
                        duet::start(&host, &name)?;
                    }
                    (DuetJob::Stop, _) => duet::stop(&host)?,
                    _ => {}
                }
                let (state, pct) = duet::status(&host)?;
                Ok(match pct {
                    Some(p) => format!("{state} ({p} %)"),
                    None => state,
                })
            };
            let _ = tx.send((job, run()));
        });
    }

    /// Take the answer of the request, if it came.
    pub(crate) fn poll_duet(&mut self, ctx: &egui::Context) {
        let Some(rx) = &self.duet_rx else { return };
        match rx.try_recv() {
            Ok((job, r)) => {
                self.duet_rx = None;
                self.status = match (job, r) {
                    (DuetJob::SendAndStart, Ok(s)) => trf("Sent to the Duet and started. State: {}", &[&s]),
                    (_, Ok(s)) => trf("State of the Duet: {}", &[&s]),
                    (_, Err(e)) => trf("Duet: {}", &[&e]),
                };
            }
            Err(mpsc::TryRecvError::Empty) => ctx.request_repaint_after(std::time::Duration::from_millis(200)),
            Err(mpsc::TryRecvError::Disconnected) => self.duet_rx = None,
        }
    }

    /// The buttons of the Duet in the Control tab.
    pub(crate) fn duet_buttons(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("Duet").strong());
        ui.label(RichText::new(tr("The Duet runs the program by itself: send it and start it, read its state, stop it. Jogging and overrides are not available. The password is not saved.")).small().color(base_ui::theme::text_dim()));
        let idle = self.duet_rx.is_none();
        ui.horizontal_wrapped(|ui| {
            if ui.add_enabled(idle, egui::Button::new(tr("Send and start"))).on_hover_text(tr("Upload the program to the board and start it there")).clicked() {
                self.duet_request(DuetJob::SendAndStart);
            }
            if ui.add_enabled(idle, egui::Button::new(tr("State"))).clicked() {
                self.duet_request(DuetJob::State);
            }
            if ui.add_enabled(idle, egui::Button::new(tr("Stop"))).clicked() {
                self.duet_request(DuetJob::Stop);
            }
        });
        ui.separator();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    /// A board that answers every request, and keeps what it was sent.
    fn board() -> (String, std::sync::Arc<std::sync::Mutex<Vec<String>>>) {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = l.local_addr().unwrap().to_string();
        let seen = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let seen2 = seen.clone();
        std::thread::spawn(move || {
            for stream in l.incoming().take(10) {
                let Ok(mut s) = stream else { break };
                let _ = s.set_read_timeout(Some(std::time::Duration::from_secs(2)));
                let mut buf = vec![0u8; 1 << 20];
                let mut n = 0;
                loop {
                    match s.read(&mut buf[n..]) {
                        Ok(0) | Err(_) => break,
                        Ok(k) => n += k,
                    }
                    let t = String::from_utf8_lossy(&buf[..n]).to_string();
                    if let Some(h) = t.find("\r\n\r\n") {
                        let len = t.lines().find_map(|l| l.to_ascii_lowercase().strip_prefix("content-length:").and_then(|v| v.trim().parse::<usize>().ok())).unwrap_or(0);
                        if n >= h + 4 + len {
                            break;
                        }
                    }
                }
                let t = String::from_utf8_lossy(&buf[..n]).to_string();
                let line = t.lines().next().unwrap_or("").to_string();
                let body = if line.contains("rr_status") { "{\"status\":\"P\",\"fractionPrinted\":3}" } else { "{\"err\":0}" };
                seen2.lock().unwrap().push(format!("{line} || {}", t.split("\r\n\r\n").nth(1).unwrap_or("").lines().take(3).collect::<Vec<_>>().join("|")));
                let _ = write!(s, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body);
            }
        });
        (addr, seen)
    }

    #[test]
    fn the_program_is_sent_to_the_duet_started_and_the_state_comes_back_without_blocking() {
        let (host, seen) = board();
        let mut a = crate::tests::app();
        let id = a.scene.add(0, base_core::scene::Kind::Rect { w: 20.0, h: 20.0 }, base_core::geom::Xf::translate(10.0, 10.0));
        a.ed.sel = vec![id];
        a.layers[0].tool = Some("std-flat-3".into());
        a.device.duet.enabled = true;
        a.device.duet.host = host;
        a.duet_request(DuetJob::SendAndStart);
        assert!(a.duet_rx.is_some(), "the request runs in a thread: {}", a.status);
        assert!(a.status.contains("Duet") || a.status.contains("Duet"), "{}", a.status);
        let ctx = egui::Context::default();
        let t = std::time::Instant::now();
        while a.duet_rx.is_some() && t.elapsed().as_secs() < 8 {
            a.poll_duet(&ctx);
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        assert!(a.status.contains("Printing"), "{}", a.status);
        let seen = seen.lock().unwrap();
        assert!(seen.iter().any(|l| l.starts_with("POST /rr_upload?name=0%3A%2Fgcodes%2Funtitled.nc") && l.contains("G21 G90 G17")), "{seen:?}");
        assert!(seen.iter().any(|l| l.contains("rr_gcode?gcode=M32")), "{seen:?}");
    }

    #[test]
    fn nothing_is_sent_without_an_address_or_without_a_job() {
        let mut a = crate::tests::app();
        a.device.duet.enabled = true;
        a.duet_request(DuetJob::State);
        assert!(a.duet_rx.is_none() && a.status.contains("address"), "{}", a.status);
        a.device.duet.host = "127.0.0.1:1".into();
        a.duet_request(DuetJob::SendAndStart);
        assert!(a.duet_rx.is_none(), "an empty drawing is not sent: {}", a.status);
        // The password never goes into the file of the device.
        a.device.duet.password = "secret".into();
        let json = serde_json::to_string(&a.device).unwrap();
        assert!(!json.contains("secret"), "{json}");
        let back: crate::device::Device = serde_json::from_str(&json).unwrap();
        assert!(back.duet.enabled && back.duet.password.is_empty());
    }
}
