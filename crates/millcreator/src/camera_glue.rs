//! The camera view: the one of every program (`base_ui::camera_view`): a window of its own, a tab of the side panel or a tab of the
//! work area, the picture that zooms and moves, and below it the overlay, the photos, video and time-lapse. Here is what is the
//! program's: the picture of the device's camera, where the way of showing it is kept and the tabs it opens.
use crate::{App, SideTab};
use base_ui::camera_view::{self as cv, StreamMode};
use base_ui::i18n::{tr, trf};
use base_ui::theme;
use eframe::egui::{self, Color32, RichText};

impl App {
    /// The device's and the active camera's way of showing the view and its zoom, once the device or the camera changed.
    pub(crate) fn follow_active_camera(&mut self) {
        let now = (self.device.name.clone(), self.device.active_camera);
        if self.cam_view_for.as_ref() == Some(&now) {
            return;
        }
        let before = self.stream_mode;
        self.stream_mode = StreamMode::of(&self.device.camera_view, self.device.cameras.get(self.device.active_camera));
        self.cam_zoom = self.device.camera_view.zoom.clamp(1.0, cv::MAX_ZOOM);
        self.cam_view_for = Some(now);
        if self.show_camera && before != self.stream_mode {
            self.show_mode(self.stream_mode);
        }
    }

    /// Keep the way and the zoom with the device (and its entry in the list) when they changed.
    fn save_camera_view(&mut self) {
        let d = &mut self.device;
        if cv::keep_mode(&mut d.camera_view, &mut d.cameras, d.active_camera, self.stream_mode, self.cam_zoom) {
            self.keep_camera_view();
        }
    }

    /// Copy the camera settings of the device in use to its entry in the list, so they are kept with it.
    fn keep_camera_view(&mut self) {
        if let Some(slot) = self.active.and_then(|i| self.devices.get_mut(i)) {
            slot.camera_view = self.device.camera_view.clone();
            slot.cameras = self.device.cameras.clone();
        }
    }

    /// The view shown the given way must be seen: the side panel opens on its tab, the work area shows its tab.
    fn show_mode(&mut self, mode: StreamMode) {
        match mode {
            StreamMode::Tab => {
                self.side_tab = SideTab::Camera;
                self.tabs.collapsed = false;
            }
            StreamMode::Workspace => {
                self.cam_ws_shown = true;
                self.pv_tab_shown = false;
            }
            StreamMode::Detached => {}
        }
    }

    /// The Camera view button: open the view the way of the camera, or close it.
    pub(crate) fn toggle_camera_view(&mut self) {
        if self.show_camera {
            self.camera_closed();
            return;
        }
        self.cam_view_for = None;
        self.follow_active_camera();
        self.show_camera = true;
        self.show_mode(self.stream_mode);
    }

    /// Close the view, in whatever way it is shown.
    pub(crate) fn camera_closed(&mut self) {
        self.show_camera = false;
        self.cam_ws_shown = false;
        if self.side_tab == SideTab::Camera {
            self.side_tab = SideTab::Properties;
        }
    }

    /// How the view of camera `i` opens (from the view options). The one in use changes at once.
    fn set_camera_mode(&mut self, i: usize, mode: StreamMode) {
        let Some(c) = self.device.cameras.get_mut(i) else { return };
        c.view_mode = Some(mode.index());
        if i == self.device.active_camera {
            self.change_mode(mode);
        }
        self.keep_camera_view();
    }

    #[cfg(test)]
    pub(crate) fn change_mode_for_test(&mut self, mode: StreamMode) {
        self.change_mode(mode);
    }

    #[cfg(test)]
    pub(crate) fn extra_tabs_have_camera(&self) -> bool {
        self.extra_tabs().contains(&"camera")
    }

    fn change_mode(&mut self, mode: StreamMode) {
        let before = self.stream_mode;
        self.stream_mode = mode;
        if before == StreamMode::Tab && mode != StreamMode::Tab && self.side_tab == SideTab::Camera {
            self.side_tab = SideTab::Properties;
        }
        if before == StreamMode::Workspace && mode != StreamMode::Workspace {
            self.cam_ws_shown = false;
        }
        if self.show_camera {
            self.show_mode(mode);
        }
    }

    /// In the view options: where the view of each camera opens.
    pub(crate) fn camera_mode_options(&mut self, ui: &mut egui::Ui) {
        if let Some((i, m)) = cv::mode_options(ui, &self.device.camera_view, &self.device.cameras) {
            self.set_camera_mode(i, m);
        }
    }

    /// The Recording tab: a panel for each camera, the photos aside and the long lines.
    pub(crate) fn recording_tab(&mut self, ui: &mut egui::Ui) {
        let mpos = self.machine.connected.then(|| base_ui::control_ui::ControlTarget::mpos(&self.machine));
        let view = self.device.camera_view.clone();
        let d = &mut self.device;
        let out = base_ui::recording_ui::tab(ui, &mut self.rigs, &mut d.cameras, &view, &mut d.control.park, &mut d.control.split, &base_ui::recording_ui::MachineCtx { mpos, spindle: true });
        if out.cameras {
            self.keep_camera_view();
        }
        if let Some((i, m)) = out.mode {
            self.set_camera_mode(i, m);
        }
        if out.machine {
            self.keep_control();
        }
    }

    /// Is the view shown as a tab of the side panel?
    pub(crate) fn camera_side_tab(&self) -> bool {
        self.show_camera && self.stream_mode == StreamMode::Tab && self.device.camera().is_some()
    }

    /// Is the view shown as a tab of the work area?
    pub(crate) fn camera_work_tab(&self) -> bool {
        self.show_camera && self.stream_mode == StreamMode::Workspace && self.device.camera().is_some()
    }

    /// The window of its own of the view, when the view is shown that way; called every frame.
    pub(crate) fn camera_window(&mut self, ctx: &egui::Context) {
        if !self.show_camera || self.device.camera().is_none() {
            return;
        }
        self.follow_active_camera();
        if self.stream_mode != StreamMode::Detached {
            return;
        }
        let id = egui::ViewportId::from_hash_of("millcreator-camera");
        let saved = self.device.camera_view.clone();
        let logo = self.logo.clone();
        let seen = cv::window(ctx, id, &tr("Camera view"), logo.as_ref(), &saved, |ui| self.camera_tab(ui));
        if seen.embedded {
            // The program cannot open windows of its own here: the side tab instead.
            self.change_mode(StreamMode::Tab);
            return;
        }
        if let Some((_, size)) = seen.place {
            cv::settle(ctx, id, &mut self.cam_win, &self.cam_fit, size);
        }
        if cv::keep_place(ctx, &mut self.device.camera_view, seen.place) {
            self.keep_camera_view();
        }
        if seen.close {
            self.camera_closed();
        }
    }

    /// The view: the three ways and the overlay, the picture, the photos, video and time-lapse, and an error of the stream. The same
    /// in the window of its own and in the tabs.
    pub(crate) fn camera_tab(&mut self, ui: &mut egui::Ui) {
        self.cam_drawn = true;
        let Some(cam) = self.device.camera().cloned() else {
            ui.label(RichText::new(tr("This device has no camera that is switched on. Add one in the device settings.")).color(theme::text_dim()));
            self.cam = None;
            return;
        };
        let ctx = ui.ctx().clone();
        // While the window of its own is being resized the picture stands still.
        if !(self.stream_mode == StreamMode::Detached && cv::resizing(&self.cam_win)) {
            self.camera_pump(&ctx);
        }
        ui.horizontal(|ui| {
            if let Some(m) = cv::mode_buttons(ui, self.stream_mode) {
                self.change_mode(m);
            }
            ui.separator();
            ui.label(RichText::new(&cam.name).strong());
        });
        let (tex, error) = self.cam.as_ref().map_or((None, String::new()), |f| (f.tex.clone(), f.error.clone()));
        let picture = cv::picture(ui, tex.as_ref(), &mut self.cam_zoom, &mut self.cam_pan, &mut self.cam_fit, 60.0);
        if !error.is_empty() {
            // The address is not shown: it may hold a password.
            let error = match &cam.kind {
                base_core::camimg::CameraKind::Url(u) if !u.is_empty() => error.replace(u.as_str(), "…"),
                _ => error,
            };
            ui.label(RichText::new(trf("Stream error: {}", &[&error])).color(Color32::from_rgb(0xd0, 0x50, 0x20)));
        }
        base_ui::editor::EditorBackground::overlay_toggle_button(self, ui);
        // A photo, a video and the time-lapse of this camera; their settings are on the Recording tab.
        ui.add_space(4.0);
        ui.separator();
        if let Some(r) = self.rigs.rec(self.device.active_camera) {
            r.quick(ui);
        }
        if !ui.input(|i| i.pointer.any_down()) {
            self.save_camera_view();
        }
        if let Some((rect, slack_x)) = picture {
            self.cam_fit.measure(ui, rect, slack_x);
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(120));
    }
}
