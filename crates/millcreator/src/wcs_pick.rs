//! The zero of the program on the work area: a button in the device settings that lets the next click on the work area set it
//! (snapping to the corners and the middles of the shapes and to the corners of the work area), and an icon of the axes on the work
//! area, shown while a work coordinate system (G54 … G59) is chosen, that can be dragged. The fields of the device settings follow at
//! once. One pick or one drag is one step of undo.
use crate::device::Wcs;
use crate::App;
use base_ui::editor::{CanvasView, EditorHost};
use base_core::geom::Pt;
use eframe::egui::{self, Color32, Pos2, Stroke};

/// How near (in points on the screen) the pointer must be to a point to snap to it, and to grab the icon.
const SNAP: f32 = 12.0;
const AXIS: f32 = 44.0;

impl App {
    /// The work coordinate system that is being shown and changed: the one of the draft in the device settings when they are open,
    /// else the one of the device of the project.
    fn wcs_now(&self) -> Wcs {
        match &self.dev_edit {
            Some(e) => e.draft.wcs,
            None => self.device.wcs,
        }
    }

    fn area_y(&self) -> f64 {
        match &self.dev_edit {
            Some(e) => e.draft.area.y,
            None => self.device.area.y,
        }
    }

    /// Put the zero at the point `p` of the drawing (the scene counts Y down, the program Y up).
    fn set_wcs_at(&mut self, p: Pt) {
        let (x, y) = (p.x, self.area_y() - p.y);
        let set = |w: &mut Wcs| {
            w.x = x;
            w.y = y;
        };
        match &mut self.dev_edit {
            Some(e) => set(&mut e.draft.wcs),
            None => {
                set(&mut self.device.wcs);
                if let Some(d) = self.active.and_then(|i| self.devices.get_mut(i)) {
                    set(&mut d.wcs);
                }
            }
        }
    }

    /// Where the zero is on the drawing, or `None` without a work coordinate system.
    fn wcs_point(&self) -> Option<Pt> {
        let w = self.wcs_now();
        (w.code != 0).then(|| Pt::new(w.x, self.area_y() - w.y))
    }

    /// The corners and the middles of the shapes and the corners of the work area, for snapping.
    fn snap_candidates(&self) -> Vec<Pt> {
        let (bw, bh) = self.work_size_mm();
        let mut v = vec![Pt::new(0.0, 0.0), Pt::new(bw, 0.0), Pt::new(0.0, bh), Pt::new(bw, bh)];
        for s in &self.scene.shapes {
            if let Some(b) = s.bounds() {
                let (c, lo, hi) = (b.center(), b.min, b.max);
                v.extend([lo, hi, Pt::new(lo.x, hi.y), Pt::new(hi.x, lo.y), c]);
            }
        }
        v
    }

    /// `p`, or the snapping point near it.
    fn wcs_snap(&self, p: Pt) -> Pt {
        let tol = (SNAP / self.ed.view.zoom.max(0.01)) as f64;
        self.snap_candidates().into_iter().map(|c| (((c.x - p.x).powi(2) + (c.y - p.y).powi(2)).sqrt(), c)).filter(|(d, _)| *d <= tol).min_by(|a, b| a.0.total_cmp(&b.0)).map_or(p, |(_, c)| c)
    }

    /// The icon of the work coordinate system, and the hint while the zero is being picked.
    pub(crate) fn paint_wcs_icon(&mut self, v: &CanvasView) {
        if let Some(p) = self.wcs_point() {
            let o = self.w2s(v.origin, p);
            let (x_end, y_end) = (o + egui::vec2(AXIS, 0.0), o + egui::vec2(0.0, -AXIS));
            let red = Color32::from_rgb(0xd0, 0x30, 0x30);
            let green = Color32::from_rgb(0x30, 0xa0, 0x40);
            for (end, c, label) in [(x_end, red, "X"), (y_end, green, "Y")] {
                let dir = (end - o).normalized();
                let side = egui::vec2(-dir.y, dir.x);
                v.painter.line_segment([o, end], Stroke::new(2.0, c));
                v.painter.add(egui::Shape::convex_polygon(vec![end + dir * 3.0, end - dir * 7.0 + side * 4.0, end - dir * 7.0 - side * 4.0], c, Stroke::NONE));
                v.painter.text(end + dir * 8.0 + side * 2.0, egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(12.0), c);
            }
            v.painter.circle(o, 4.0, Color32::WHITE, Stroke::new(1.5, Color32::from_gray(0x40)));
            v.painter.text(o + egui::vec2(6.0, 6.0), egui::Align2::LEFT_TOP, self.wcs_now().word().unwrap_or_default(), egui::FontId::proportional(11.0), Color32::from_gray(0x40));
        }
        if self.pick_origin {
            v.painter.text(v.rect.center_top() + egui::vec2(0.0, 10.0), egui::Align2::CENTER_TOP, base_ui::i18n::tr("Click the work area to set the zero of the program (Esc cancels)"), egui::FontId::proportional(13.0), Color32::from_rgb(0xc0, 0x50, 0x10));
        }
    }

    /// The pointer on the work area while the zero is picked or the icon is dragged. Returns whether it took the pointer.
    pub(crate) fn wcs_pointer(&mut self, resp: &egui::Response, origin: Pos2, hover: Option<Pos2>) -> bool {
        let at = hover.map(|h| self.s2w(origin, h));
        if self.pick_origin {
            if resp.ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.pick_origin = false;
                return true;
            }
            resp.ctx.set_cursor_icon(egui::CursorIcon::Crosshair);
            if resp.clicked() {
                if let Some(p) = at {
                    self.remember_wcs();
                    let p = self.wcs_snap(p);
                    self.set_wcs_at(p);
                    self.pick_origin = false;
                    self.status = base_ui::i18n::trf("The zero of the program is at X {} Y {}", &[&format!("{:.2}", self.wcs_now().x), &format!("{:.2}", self.wcs_now().y)]);
                }
            }
            return true;
        }
        let Some(zero) = self.wcs_point() else { return false };
        let near = |p: Option<Pos2>| p.is_some_and(|h| (h - self.w2s(origin, zero)).length() <= SNAP);
        if resp.drag_started() && near(hover) {
            self.remember_wcs();
            self.wcs_drag = true;
        }
        if self.wcs_drag {
            if let Some(p) = at {
                let p = self.wcs_snap(p);
                self.set_wcs_at(p);
            }
            if resp.drag_stopped() {
                self.wcs_drag = false;
            }
            return true;
        }
        false
    }

    /// Keep the zero as it is now, for one step of undo.
    fn remember_wcs(&mut self) {
        self.wcs_undo = Some(self.wcs_now());
    }

    /// Undo the last change of the zero, when it was the last thing done. Returns whether it did.
    pub(crate) fn undo_wcs(&mut self) -> bool {
        let Some(prev) = self.wcs_undo.take() else { return false };
        let set = |w: &mut Wcs| *w = prev;
        match &mut self.dev_edit {
            Some(e) => set(&mut e.draft.wcs),
            None => {
                set(&mut self.device.wcs);
                if let Some(d) = self.active.and_then(|i| self.devices.get_mut(i)) {
                    set(&mut d.wcs);
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base_core::geom::Xf;
    use base_core::scene::Kind;

    fn app() -> App {
        let mut a = App::build(&egui::Context::default(), None, false);
        a.device = crate::device::Device::dummy();
        a.device.wcs = Wcs { code: 54, x: 0.0, y: 0.0 };
        a.devices = vec![a.device.clone()];
        a.active = Some(0);
        a.ed.view.zoom = 2.0;
        a
    }

    #[test]
    fn the_zero_snaps_to_a_corner_of_a_shape_and_counts_y_up() {
        let mut a = app();
        a.scene.add(0, Kind::Rect { w: 40.0, h: 20.0 }, Xf::translate(30.0, 50.0));
        let area_y = a.device.area.y;
        // Near the lower-left corner of the shape on the drawing (30, 70): snapped there.
        let p = a.wcs_snap(Pt::new(31.5, 69.0));
        assert!((p.x - 30.0).abs() < 1e-9 && (p.y - 70.0).abs() < 1e-9, "{p:?}");
        // Far from everything: the point itself.
        let far = a.wcs_snap(Pt::new(100.0, 100.0));
        assert!((far.x - 100.0).abs() < 1e-9 && (far.y - 100.0).abs() < 1e-9);
        // The middle of the shape and the corner of the work area are candidates too.
        assert!(a.snap_candidates().iter().any(|c| (c.x - 50.0).abs() < 1e-9 && (c.y - 60.0).abs() < 1e-9));
        assert!(a.snap_candidates().iter().any(|c| c.x == 0.0 && c.y == 0.0));
        // The zero of the program: X as drawn, Y up from the front.
        a.set_wcs_at(p);
        assert!((a.device.wcs.x - 30.0).abs() < 1e-9 && (a.device.wcs.y - (area_y - 70.0)).abs() < 1e-9);
        assert_eq!(a.devices[0].wcs, a.device.wcs, "the device in the list follows");
        assert!(a.wcs_point().is_some_and(|z| (z.x - 30.0).abs() < 1e-9 && (z.y - 70.0).abs() < 1e-9));
    }

    #[test]
    fn the_draft_in_the_device_settings_is_what_changes_while_they_are_open_and_one_undo_takes_it_back() {
        let mut a = app();
        a.open_device_settings();
        assert!(a.dev_edit.is_some());
        let before = a.wcs_now();
        a.remember_wcs();
        a.set_wcs_at(Pt::new(12.0, 34.0));
        let e = a.dev_edit.as_ref().unwrap();
        assert!((e.draft.wcs.x - 12.0).abs() < 1e-9, "the fields of the settings follow");
        assert_eq!(a.device.wcs, before, "the device of the project is not changed until the settings are saved");
        assert!(a.undo_wcs());
        assert_eq!(a.dev_edit.as_ref().unwrap().draft.wcs, before);
        assert!(!a.undo_wcs(), "one step only");
        // A change of the drawing after the pick makes the undo go to the drawing.
        a.dev_edit = None;
        a.remember_wcs();
        a.set_wcs_at(Pt::new(5.0, 5.0));
        EditorHost::checkpoint(&mut a);
        assert!(!a.undo_wcs());
        // Without a work coordinate system there is no icon and no zero to pick.
        a.device.wcs.code = 0;
        assert!(a.wcs_point().is_none());
    }

    #[test]
    fn picking_takes_the_pointer_and_escape_cancels_it() {
        let mut a = app();
        let ctx = egui::Context::default();
        a.pick_origin = true;
        let mut claimed = false;
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            let (resp, _) = ui.allocate_painter(egui::vec2(100.0, 100.0), egui::Sense::click());
            claimed = a.wcs_pointer(&resp, Pos2::ZERO, Some(Pos2::new(10.0, 10.0)));
        });
        out.textures_delta.clear();
        assert!(claimed && a.pick_origin, "the pick goes on until a click or Escape");
        let mut input = egui::RawInput::default();
        input.events.push(egui::Event::Key { key: egui::Key::Escape, physical_key: None, pressed: true, repeat: false, modifiers: Default::default() });
        let mut out = ctx.run_ui(input, |ui| {
            let (resp, _) = ui.allocate_painter(egui::vec2(100.0, 100.0), egui::Sense::click());
            a.wcs_pointer(&resp, Pos2::ZERO, None);
        });
        out.textures_delta.clear();
        assert!(!a.pick_origin, "Escape cancelled it");
    }
}
