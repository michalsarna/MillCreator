//! MillCreator's side of the web server and API of baseCreator (`base_ui::web_api`, port 8766): what it publishes (the machine,
//! the job, the project, the cameras) and the commands that come back.
use crate::App;
use base_ui::web_api::{self, Command, WebApi};
use eframe::egui;
use serde_json::json;

impl App {
    /// Start, restart or stop the server to match the options, carry out the commands and publish the state.
    pub(crate) fn web_tick(&mut self, ctx: &egui::Context) {
        let (cmds, shared) = self.web.tick(ctx);
        for c in cmds {
            self.web_command(c);
        }
        let Some(shared) = shared else { return };
        let (est, elapsed) = match self.web_job {
            Some((t, secs)) if self.machine.job_running() => (secs, t.elapsed().as_secs_f64()),
            Some((_, secs)) => (secs, 0.0),
            None => (0.0, 0.0),
        };
        let (device, job) = web_api::machine_status(&self.machine, &self.device.name, &format!("{:?}", self.device.connection.grbl_version), est, elapsed);
        let status = json!({
            "app": "MillCreator",
            "version": crate::VERSION,
            "theme": self.scheme.id(),
            "device": device,
            "job": job,
            "project": {
                "name": self.path.as_ref().and_then(|p| p.file_name()).map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
                "revision": self.rev,
                "description": self.description,
            },
        });
        let cams = web_api::cameras_json(&self.device.cameras, self.device.active_camera);
        WebApi::publish(&shared, status, cams, self.rev, &|| web_api::drawing_svg(self));
        if let Some(img) = self.cam.as_ref().and_then(|f| f.last_image()) {
            self.web.publish_image(&img);
        }
    }

    /// One command from the API, with the same rules as the buttons it stands for.
    pub(crate) fn web_command(&mut self, c: Command) {
        match c {
            Command::Stop => self.abort_job(),
            Command::SelectCamera(i) => {
                if i < self.device.cameras.len() {
                    base_ui::camera_feed::activate(&mut self.device.cameras, &mut self.device.active_camera, i);
                    if let Some(d) = self.active.and_then(|k| self.devices.get_mut(k)) {
                        d.active_camera = self.device.active_camera;
                    }
                }
            }
            other => {
                let step = self.control_ui.xy_text.trim().replace(',', ".").parse::<f64>().ok().filter(|s| *s > 0.0).unwrap_or(1.0);
                let feed = self.control_ui.feed_xy.unwrap_or(self.device.control.jog.xy_feed);
                web_api::machine_command(&mut self.machine, &other, step, feed);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_web_server_publishes_the_machine_and_the_project_on_its_port() {
        let mut a = crate::tests::app();
        assert!(!a.web.on && a.web.port == base_ui::web_api::PORT_MILLCREATOR);
        assert!(a.settings_pairs().iter().any(|(k, v)| *k == "api_port" && v == "8766"));
        a.web.on = true;
        a.web.port = 0;
        a.device.name = "My mill".into();
        let ctx = eframe::egui::Context::default();
        a.web_tick(&ctx);
        let shared = a.web.server.as_ref().expect("started").shared.clone();
        let st = shared.status.lock().unwrap().clone();
        assert_eq!((st["app"].as_str(), st["device"]["name"].as_str()), (Some("MillCreator"), Some("My mill")));
        assert!(st["machine"].is_null(), "a program with a machine shows it");
        assert!(shared.svg.lock().unwrap().1.contains("<svg"));
        // Without a connection the commands do nothing (and do not fail).
        for c in [base_ui::web_api::Command::Home, base_ui::web_api::Command::CancelJog, base_ui::web_api::Command::Jog(base_ui::web_api::Jog::Up), base_ui::web_api::Command::Stop] {
            a.web_command(c);
        }
    }
}
