//! The toolpaths of the project: the 3D window and the export of G-code.
use crate::toolpath::{self, Program};
use crate::view3d::{self, Region, Scene3d};
use crate::App;
use base_ui::i18n::{tr, trf};
use base_ui::theme;
use base_ui::window::popup;
use base_ui::editor::EditorHost;
use eframe::egui::{self, Color32, RichText};

impl App {
    /// The toolpaths of the project as it is now.
    pub(crate) fn make_program(&self) -> Program {
        toolpath::generate(&self.scene, &self.layers, &self.tools, &self.device, &self.stock)
    }

    /// The box of the milled shapes (min x, min y, max x, max y) in the coordinates of the work area, for a project that
    /// has no cuts yet.
    pub(crate) fn shapes_box(&self) -> Option<[f64; 4]> {
        let h = self.device.area.y;
        self.scene.shapes.iter().filter_map(|s| s.bounds()).reduce(base_core::geom::Rect::union).map(|b| [b.min.x, h - b.max.y, b.max.x, h - b.min.y])
    }

    /// The program is made again when the project changed since it was made.
    pub(crate) fn refresh_program(&mut self) {
        if self.prog.is_none() || self.prog_rev != self.rev {
            self.prog = Some(self.make_program());
            self.prog_rev = self.rev;
        }
    }

    /// The toolpaths over the work area (the Toolpaths button): cuts in the colour of their layer, the moves above the
    /// material in grey.
    pub(crate) fn paint_toolpaths(&mut self, v: &base_ui::editor::CanvasView) {
        if !self.toolpaths_on {
            return;
        }
        self.refresh_program();
        let Some(prog) = self.prog.as_ref() else { return };
        let h = self.device.area.y;
        let to_screen = |p: toolpath::P3| self.w2s(v.origin, base_core::geom::Pt::new(p[0], h - p[1]));
        let many = prog.moves.len() > 100_000;
        let mut shapes = Vec::with_capacity(prog.moves.len());
        for m in &prog.moves {
            let (a, b) = (to_screen(m.from), to_screen(m.to));
            match m.kind {
                toolpath::MoveKind::Cut => {
                    let c = self.layers.get(m.layer).map_or(Color32::RED, |l| l.color32());
                    shapes.push(egui::Shape::line_segment([a, b], egui::Stroke::new(1.5, c)));
                }
                toolpath::MoveKind::Rapid if !many => shapes.push(egui::Shape::line_segment([a, b], egui::Stroke::new(0.6, Color32::from_rgb(0x70, 0x90, 0xc0).gamma_multiply(0.8)))),
                _ => {}
            }
        }
        v.painter.extend(shapes);
    }

    /// The live picture of the camera in a window of its own (the Camera view button).
    pub(crate) fn camera_window(&mut self, ctx: &egui::Context) {
        if !self.show_camera {
            return;
        }
        let mut open = true;
        popup(ctx, "camera_window", &tr("Camera view"), [560.0, 440.0], &mut open, |ui| self.camera_tab(ui));
        self.show_camera = open;
    }

    /// Open the 3D window with a fresh program.
    pub(crate) fn open_preview(&mut self) {
        self.prog = Some(self.make_program());
        self.prog_rev = self.rev;
        self.show_3d = true;
    }

    /// Ask where to write the G-code and write it.
    pub(crate) fn export_gcode(&mut self) {
        let prog = self.make_program();
        if prog.is_empty() {
            self.status = tr("There is nothing to cut: put shapes on layers that have a bit.").to_string();
            return;
        }
        let name = self.path.as_ref().and_then(|p| p.file_stem()).map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "untitled".into());
        let Some(p) = rfd::FileDialog::new().add_filter("G-code", &["nc", "gcode", "ngc"]).set_file_name(format!("{name}.nc")).save_file() else { return };
        let text = toolpath::to_gcode(&prog, &self.device, &self.layers, &self.tools, &name);
        self.status = match std::fs::write(&p, text) {
            Ok(()) => trf("Saved {}", &[&p.display()]),
            Err(e) => trf("Could not save: {}", &[&e]),
        };
        if !prog.warnings.is_empty() {
            self.status.push_str(&format!(" ({})", prog.warnings.iter().map(|w| w.text()).collect::<Vec<_>>().join("; ")));
        }
    }

    pub(crate) fn preview_window(&mut self, ctx: &egui::Context) {
        if !self.show_3d {
            return;
        }
        if self.prog.is_none() || self.prog_rev != self.rev {
            self.prog = Some(self.make_program());
            self.prog_rev = self.rev;
        }
        let mut open = true;
        popup(ctx, "preview_3d", &tr("Preview"), [980.0, 700.0], &mut open, |ui| {
            let prog = self.prog.as_ref().expect("the program was made above");
            let region = Region::of(prog, self.shapes_box());
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut self.view.top, false, "3D");
                ui.selectable_value(&mut self.view.top, true, tr("From above"));
                ui.separator();
                if ui.button(tr("Reset view")).on_hover_text(tr("Back to the first view. Double click on the picture does the same.")).clicked() {
                    self.view.reset();
                }
                ui.checkbox(&mut self.view.show_stock, tr("Material"));
                ui.checkbox(&mut self.view.show_cuts, tr("Cuts"));
                ui.checkbox(&mut self.view.show_rapids, tr("Moves at the safe height"));
            });
            ui.horizontal(|ui| {
                self.playback.show(ui, &mut self.view.progress);
            });
            let secs = prog.estimate_seconds(self.device.rapid);
            ui.label(RichText::new(trf("Time {} / {}", &[&base_ui::widgets::fmt_time(secs * self.view.progress as f64), &base_ui::widgets::fmt_time(secs)])).strong());
            ui.label(RichText::new(trf("Cuts: {} m, about {} min", &[&format!("{:.2}", prog.cut_length() / 1000.0), &format!("{:.1}", secs / 60.0)])).color(theme::text_dim()));
            for w in &prog.warnings {
                ui.label(RichText::new(format!("• {}", w.text())).color(egui::Color32::from_rgb(0xd0, 0x70, 0x20)));
            }
            match region {
                Some(region) => {
                    let sc = Scene3d { prog, region, stock: &self.stock, dev: &self.device, layers: &self.layers, tools: &self.tools };
                    view3d::show(ui, &mut self.view, &sc);
                }
                None => {
                    ui.add_space(30.0);
                    ui.label(RichText::new(tr("There is nothing to show yet: draw shapes and put them on layers that have a bit.")).color(theme::text_dim()));
                }
            }
        });
        self.show_3d = open;
        if !open {
            self.playback.playing = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::App;
    use base_core::geom::Xf;
    use base_core::scene::Kind;

    #[test]
    fn the_window_follows_the_project_and_is_drawn() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        a.open_preview();
        assert!(a.show_3d && a.prog.as_ref().unwrap().is_empty());
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| a.preview_window(ui.ctx()));
        out.textures_delta.clear();
        a.scene.add(0, Kind::Rect { w: 30.0, h: 30.0 }, Xf::translate(20.0, 20.0));
        a.touch_rev();
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| a.preview_window(ui.ctx()));
        out.textures_delta.clear();
        assert!(!a.prog.as_ref().unwrap().is_empty(), "the new shape is milled");
        assert_eq!(a.prog_rev, a.rev);
    }
}
