//! The toolpaths of the project: the 3D window and the export of G-code.
use crate::toolpath::{self, Program};
use crate::view3d::{self, Region, Scene3d};
use crate::{App, Screen};
use base_ui::i18n::{tr, trf};
use base_ui::theme;
use base_ui::editor::EditorHost;
use eframe::egui::{self, Color32, RichText};

impl App {
    /// The toolpaths of the project as it is now.
    pub(crate) fn make_program(&self) -> Program {
        toolpath::generate_with(&self.job_scene(), &self.layers, &self.tools, &self.device, &self.stock, toolpath::JobOptions { one_by_one: self.one_by_one })
    }

    /// What the job is made of: the whole drawing, or with "Selection only" the shapes that are selected now.
    pub(crate) fn job_scene(&self) -> std::borrow::Cow<'_, base_core::scene::Scene> {
        if self.selection_only {
            let (next_id, next_group) = self.scene.counters();
            let shapes = self.scene.shapes.iter().filter(|s| self.ed.sel.contains(&s.id)).cloned().collect();
            std::borrow::Cow::Owned(base_core::scene::Scene::from_parts(shapes, next_id, next_group))
        } else {
            std::borrow::Cow::Borrowed(&self.scene)
        }
    }

    /// Changes whenever the project or what the job makes of it changes: the key of the program that is kept.
    pub(crate) fn job_stamp(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        (self.rev, self.one_by_one, self.selection_only).hash(&mut h);
        if self.selection_only {
            self.ed.sel.hash(&mut h);
        }
        h.finish()
    }

    /// The box of the milled shapes (min x, min y, max x, max y) in the coordinates of the work area, for a project that
    /// has no cuts yet.
    pub(crate) fn shapes_box(&self) -> Option<[f64; 4]> {
        let h = self.device.area.y;
        self.scene.shapes.iter().filter_map(|s| s.bounds()).reduce(base_core::geom::Rect::union).map(|b| [b.min.x, h - b.max.y, b.max.x, h - b.min.y])
    }

    /// The program is made again when the project changed since it was made.
    pub(crate) fn refresh_program(&mut self) {
        if self.prog.is_none() || self.prog_rev != self.job_stamp() {
            self.prog = Some(self.make_program());
            self.prog_rev = self.job_stamp();
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

    /// Is the camera view open, as a tab of the work area (so that the bar of the tabs has one for it)?
    pub(crate) fn camera_work_tab(&self) -> bool {
        self.show_camera && self.device.camera().is_some()
    }

    /// The camera view in the work area, in place of the drawing.
    pub(crate) fn camera_work_panel(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().id_salt("camera_work_area").auto_shrink([false, false]).show(ui, |ui| {
            egui::Frame::new().inner_margin(8).show(ui, |ui| self.camera_tab(ui));
        });
    }

    /// Open the 3D window with a fresh program.
    pub(crate) fn open_preview(&mut self) {
        self.prog = Some(self.make_program());
        self.prog_rev = self.job_stamp();
        self.show_3d = true;
    }

    /// The G-code of the drawing as text (with the postprocessors), and the program it was made from. `None` (the reason is in the
    /// status line) when there is nothing to cut or a postprocessor failed.
    pub(crate) fn gcode_text(&mut self, name: &str) -> Option<(String, toolpath::Program)> {
        if !self.gcode_variables_ok() {
            return None;
        }
        let prog = self.make_program();
        if prog.is_empty() {
            self.status = self.nothing_to_cut();
            return None;
        }
        let text = toolpath::to_gcode_info(&prog, &self.device.with_stock(&self.stock), &self.layers, &self.tools, &self.gcode_info(name));
        let text = self.postprocessed(text)?;
        Some((text, prog))
    }

    /// What the variables of the own G-code are made of: the project, the machine and the moment (the box and the layer are filled
    /// in when the program is written).
    pub(crate) fn gcode_info(&self, name: &str) -> base_core::gcode_out::Info {
        base_core::gcode_out::Info { project_name: name.to_string(), project_path: self.path.as_ref().map(|p| p.display().to_string()).unwrap_or_default(), notes: self.description.clone(), ..base_core::gcode_out::Info::now() }
    }

    /// Are the variables in the own G-code of the device all known? When not, the reason goes to the status line and the file is
    /// not made.
    pub(crate) fn gcode_variables_ok(&mut self) -> bool {
        match self.device.gcode.check_variables() {
            Ok(()) => true,
            Err(e) => {
                self.status = trf("The own G-code of the device: {}", &[&e]);
                false
            }
        }
    }

    pub(crate) fn gcode_name(&self) -> String {
        self.path.as_ref().and_then(|p| p.file_stem()).map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "untitled".into())
    }

    /// Ask where to write the G-code and write it.
    pub(crate) fn export_gcode(&mut self) {
        let name = self.gcode_name();
        let Some((text, prog)) = self.gcode_text(&name) else { return };
        let Some(p) = rfd::FileDialog::new().add_filter("G-code", &["nc", "gcode", "ngc"]).set_file_name(format!("{name}.nc")).save_file() else { return };
        self.status = match std::fs::write(&p, &text) {
            Ok(()) => trf("Saved {}", &[&p.display()]),
            Err(e) => trf("Could not save: {}", &[&e]),
        };
        if let Some(n) = &self.post_note {
            self.status.push_str(&format!(" — {n}"));
        }
        if !prog.warnings.is_empty() {
            self.status.push_str(&format!(" ({})", prog.warnings.iter().map(|w| w.text()).collect::<Vec<_>>().join("; ")));
        }
        if self.ed.grid_prefs.gcode_preview_after_save && p.exists() {
            if let Err(e) = base_ui::gcode_view_ui::open(&mut self.gcode_view, p) {
                self.status = e;
            }
        }
    }

    /// The G-code of the project as text in a tab of its own (coloured, open to edits); nothing is written to the disk. A project has
    /// one such tab: when it is open already it is shown (and made again when nothing was changed by hand).
    pub(crate) fn preview_gcode(&mut self) {
        let name = format!("{}.nc", self.gcode_name());
        let slot = self.slot;
        let Some((text, prog)) = self.gcode_text(&name) else { return };
        match self.gcode_texts.iter_mut().find(|g| g.slot == slot) {
            Some(g) => {
                if !g.dirty() {
                    g.replace(text);
                }
                g.name = name;
                self.panes.select(base_ui::work_panes::Content::GcodeText(slot));
            }
            None => self.gcode_texts.push(base_ui::gcode_text_ui::GcodeText::new(slot, &name, text)),
        }
        self.status = if prog.warnings.is_empty() { tr("The G-code is shown, nothing is saved").to_string() } else { format!("{} ({})", tr("The G-code is shown, nothing is saved"), prog.warnings.iter().map(|w| w.text()).collect::<Vec<_>>().join("; ")) };
    }

    /// The picture a relief can be made of: the selected one on a layer that engraves it as a relief, else the first such picture.
    pub(crate) fn relief_picture(&self) -> Option<&base_core::scene::Shape> {
        use base_core::scene::Kind;
        let picks = |id: Option<u64>| {
            self.scene.shapes.iter().find(|s| matches!(s.kind, Kind::Image(_)) && id.map_or(true, |i| s.id == i) && self.layers.get(s.layer).is_some_and(|l| l.op == crate::layers::Operation::Picture && l.picture == crate::layers::PictureMethod::Relief))
        };
        self.ed.sel.iter().find_map(|id| picks(Some(*id))).or_else(|| picks(None))
    }

    /// The relief of that picture as an STL model.
    pub(crate) fn relief_model(&self) -> Option<Vec<u8>> {
        use base_core::scene::Kind;
        let shape = self.relief_picture()?;
        let Kind::Image(im) = &shape.kind else { return None };
        let layer = &self.layers[shape.layer];
        let tool = layer.tool.as_deref().and_then(|id| self.tools.get(id))?;
        let step = (tool.dims.diameter * layer.step_over_pct / 100.0).max(0.05);
        // The model is not turned by the scan angle: it is the picture as it lies.
        let map = crate::relief::HeightMap::of(shape, im, step, 0.0, layer.depth)?;
        Some(crate::relief::stl(&map, self.stock.thickness))
    }

    /// File > Import > Import 3D model as relief…
    pub(crate) fn import_model_dialog(&mut self) {
        if let Some(p) = rfd::FileDialog::new().add_filter("3D model", &["stl", "obj", "3mf"]).pick_file() {
            self.import_model(p);
        }
    }

    /// A 3D model (STL, OBJ, 3MF) seen from above becomes a picture of depths on a layer that cuts a 3D relief: the highest
    /// point of the model is left uncut, the lowest is the full depth (the height of the model, or the thickness of the material).
    pub(crate) fn import_model(&mut self, path: std::path::PathBuf) {
        use crate::layers::{Operation, PictureMethod};
        if self.screen == Screen::Start {
            self.new_design();
        }
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let read = std::fs::read(&path).map_err(|e| e.to_string()).and_then(|b| crate::model3d::read(&name, &b));
        let tris = match read {
            Ok(t) => t,
            Err(e) => {
                self.status = trf("Could not import the model {}: {}", &[&name, &e]);
                return;
            }
        };
        let [w, h, height] = crate::model3d::size(&tris);
        let Some(im) = crate::model3d::depth_map(&tris, &name, 0.25) else {
            self.status = trf("The model {} has no area seen from above", &[&name]);
            return;
        };
        // A layer that already cuts a relief, else the first layer with nothing on it.
        let used: std::collections::HashSet<usize> = self.scene.shapes.iter().map(|s| s.layer).collect();
        let relief = |l: &crate::layers::MillLayer| l.op == Operation::Picture && l.picture == PictureMethod::Relief;
        let li = self.layers.iter().position(relief).or_else(|| (0..self.layers.len()).find(|i| !used.contains(i))).unwrap_or(self.layers.len() - 1);
        self.checkpoint();
        if !relief(&self.layers[li]) {
            let ball = self.tools.all().find(|t| t.kind == crate::tools::ToolKind::BallNose).cloned();
            let l = &mut self.layers[li];
            l.op = Operation::Picture;
            l.picture = PictureMethod::Relief;
            l.depth = height.min(self.stock.thickness).max(0.1);
            if let Some(t) = ball {
                l.use_tool(&t);
            }
        }
        let id = self.scene.add(li, base_core::scene::Kind::Image(im), base_core::geom::Xf::IDENTITY);
        self.ed.sel = std::iter::once(id).collect();
        self.ed.node_sel.clear();
        self.touch_rev();
        self.ed.view.need_fit = true;
        self.status = trf("Imported the model {} as a relief on layer {}: {} x {} x {} mm", &[&name, &self.layers[li].name, &format!("{w:.1}"), &format!("{h:.1}"), &format!("{height:.1}")]);
        if height > self.stock.thickness + 1e-9 {
            self.status.push_str(&format!(" — {}", tr("the model is higher than the material: the relief is cut only as deep as the layer says")));
        }
    }

    /// Milling > Export relief as STL…
    pub(crate) fn export_relief(&mut self) {
        let Some(bytes) = self.relief_model() else {
            self.status = tr("There is no relief to export: put a picture on a layer with the Picture operation and the 3D relief method.").to_string();
            return;
        };
        self.save_stl(bytes, "relief");
    }

    /// Ask where to put the STL model and write it.
    fn save_stl(&mut self, bytes: Vec<u8>, fallback: &str) {
        let name = self.path.as_ref().and_then(|p| p.file_stem()).map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| fallback.into());
        let Some(p) = rfd::FileDialog::new().add_filter("STL", &["stl"]).set_file_name(format!("{name}.stl")).save_file() else { return };
        self.status = match std::fs::write(&p, bytes) {
            Ok(()) => trf("Saved {}", &[&p.display()]),
            Err(e) => trf("Could not save: {}", &[&e]),
        };
    }

    /// The milled piece as the 3D view shows it (the cuts done up to the slider) as an STL model: the stock with the grooves of
    /// the bits in it. `None` when no cut has been made yet.
    pub(crate) fn view_model(&self) -> Option<Vec<u8>> {
        let prog = self.prog.as_ref()?;
        let region = Region::of(prog, self.shapes_box())?;
        let (done, _) = view3d::position_at(prog, self.view.progress);
        let moves = &prog.moves[..done.min(prog.moves.len())];
        let tool_of = |layer: usize| self.layers.get(layer).and_then(|l| l.tool.as_deref()).and_then(|id| self.tools.get(id)).cloned();
        let (cols, rows, step, z) = crate::relief::carved_surface(moves, tool_of, (region.min, region.max), self.stock.thickness, crate::relief::Limits::EXPORT);
        if z.iter().all(|h| *h == 0.0) {
            return None;
        }
        Some(crate::relief::stl_of(cols, rows, step, |c, r| z[r * cols + c], self.stock.thickness))
    }

    /// With a rotary attachment the block is the cylinder round the workpiece: the whole way round, centred on the job.
    pub(crate) fn rotary_view(&self, region: Option<Region>) -> (Option<Region>, Option<view3d::Cyl>) {
        let Some(r) = region else { return (None, None) };
        let rot = &self.device.rotary;
        if !rot.enabled || rot.diameter <= 0.0 {
            return (Some(r), None);
        }
        let (round, cy) = r.round(rot.circumference());
        (Some(round), Some(view3d::Cyl { r: rot.diameter / 2.0, cy }))
    }

    /// The button of the preview: Export the milled piece as STL.
    pub(crate) fn export_view(&mut self) {
        if self.device.rotary.enabled {
            // The model is a plate with the cuts on it; wrapped round a workpiece it would be wrong.
            self.status = tr("The milled piece cannot be exported as STL while a rotary attachment is chosen.").to_string();
            return;
        }
        match self.view_model() {
            Some(bytes) => self.save_stl(bytes, "milled"),
            None => self.status = tr("Nothing to export: the job has no cuts yet.").to_string(),
        }
    }

    pub(crate) fn preview_window(&mut self, _ctx: &egui::Context) {
        if !self.show_3d {
            self.pv_tab_seen = false;
            self.pv_tab_shown = false;
            return;
        }
        // The preview is a tab of the work area (drawn by `preview_work_panel`); it is shown the first time it opens. A window of its
        // own is the tab that the person took out (`work_panes`), never a second copy.
        if !self.pv_tab_seen {
            self.pv_tab_shown = true;
            self.cam_ws_shown = false;
        }
        self.pv_tab_seen = true;
    }

    /// The options, totals and the picture (the same in the window and in the tab).
    pub(crate) fn preview_body(&mut self, ui: &mut egui::Ui) {
        let prog = self.prog.as_ref().expect("the program was made above");
        let (region, rotary) = self.rotary_view(Region::of(prog, self.shapes_box()));
        // The depth the cuts can go: the thickness of the material, or the radius of a workpiece on a rotary.
        let depth = rotary.map_or(self.stock.thickness, |c| c.r);
        let mut stl = false;
        ui.horizontal_wrapped(|ui| {
            ui.selectable_value(&mut self.view.top, false, "3D");
            ui.selectable_value(&mut self.view.top, true, tr("From above"));
            ui.separator();
            if ui.button(tr("Reset view")).on_hover_text(tr("Back to the first view. Double click on the picture does the same.")).clicked() {
                self.view.reset();
            }
            // The two buttons that fit the picture: the work area, and all the objects.
            if let Some(f) = Self::preview_fit_buttons(ui) {
                self.view.fit = Some(f);
            }
            ui.checkbox(&mut self.view.show_stock, tr("Material"));
            ui.checkbox(&mut self.view.show_cuts, tr("Cuts"));
            ui.checkbox(&mut self.view.show_rapids, tr("Moves at the safe height"));
            ui.separator();
            // Only the icon: what it does is in the tooltip.
            stl = Self::preview_stl_button(ui);
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
                // The carved stock is made again only when the program, the slider or the stock changed.
                let done = view3d::position_at(prog, self.view.progress).0.min(prog.moves.len());
                let bits = |v: f64| v.to_bits();
                let key = (self.prog_rev, done, [bits(region.min[0]), bits(region.min[1]), bits(region.max[0]), bits(region.max[1]), bits(depth)]);
                if self.view.show_stock && self.surface.as_ref().map(|s| s.key) != Some(key) {
                    let tool_of = |layer: usize| self.layers.get(layer).and_then(|l| l.tool.as_deref()).and_then(|id| self.tools.get(id)).cloned();
                    self.surface = view3d::Surface::of(key, &prog.moves[..done], tool_of, region, depth);
                    if self.surface.is_none() {
                        // Nothing is cut: remember that by a surface that is not there (no key to compare), so it is not tried again each frame.
                        self.surface_none = Some(key);
                    }
                }
                let surface = if self.surface_none == Some(key) { None } else { self.surface.as_ref() };
                let sc = Scene3d { surface, prog, region, stock: &self.stock, dev: &self.device, layers: &self.layers, rotary };
                view3d::show(ui, &mut self.view, &sc);
            }
            None => {
                ui.add_space(30.0);
                ui.label(RichText::new(tr("There is nothing to show yet: draw shapes and put them on layers that have a bit.")).color(theme::text_dim()));
            }
        }
        if stl {
            self.export_view();
        }
    }

    /// The button that exports the view as an STL model.
    fn preview_stl_button(ui: &mut egui::Ui) -> bool {
        base_ui::widgets::icon_button(ui, "download", &tr("Export the milled piece as STL…"), true, 22.0)
    }

    /// The two buttons that fit the picture, the work area on the left.
    fn preview_fit_buttons(ui: &mut egui::Ui) -> Option<view3d::Fit> {
        let mut fit = None;
        for (what, icon, tip) in [(view3d::Fit::Area, "scan", "Fit work area"), (view3d::Fit::Elements, "expand", "Fit all objects")] {
            if base_ui::widgets::icon_button(ui, icon, &tr(tip), true, 22.0) {
                fit = Some(what);
            }
        }
        fit
    }

    /// The preview as a tab of the work area, in place of the drawing.
    pub(crate) fn preview_work_panel(&mut self, ui: &mut egui::Ui) {
        self.refresh_program();
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            ui.label(RichText::new(tr("Preview")).strong());

        });
        egui::Frame::new().inner_margin(egui::Margin { left: 8, right: 8, top: 0, bottom: 3 }).show(ui, |ui| self.preview_body(ui));
    }

    /// Is the preview a tab of the work area (so that the bar of the tabs has one for it)?
    pub(crate) fn preview_work_tab(&self) -> bool {
        // The preview is a tab (or a window of its own that the person took the tab out into); it is not a floating window of the program.
        self.show_3d
    }

    /// The tab was closed with its cross.
    pub(crate) fn preview_close(&mut self) {
        self.show_3d = false;
        self.pv_tab_shown = false;
        self.playback.playing = false;
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
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            a.preview_window(ui.ctx());
            egui::CentralPanel::no_frame().show(ui, |ui| a.preview_work_panel(ui));
        });
        out.textures_delta.clear();
        a.scene.add(0, Kind::Rect { w: 30.0, h: 30.0 }, Xf::translate(20.0, 20.0));
        a.touch_rev();
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            a.preview_window(ui.ctx());
            egui::CentralPanel::no_frame().show(ui, |ui| a.preview_work_panel(ui));
        });
        out.textures_delta.clear();
        assert!(!a.prog.as_ref().unwrap().is_empty(), "the new shape is milled");
        assert_eq!(a.prog_rev, a.job_stamp());
        // The two fits: the whole work area shows more than the small shape, so it zooms less.
        let mut zoom_of = |fit| {
            a.view.fit = Some(fit);
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            a.preview_window(ui.ctx());
            egui::CentralPanel::no_frame().show(ui, |ui| a.preview_work_panel(ui));
        });
            out.textures_delta.clear();
            assert!(a.view.fit.is_none(), "the request is done by the picture");
            a.view.zoom
        };
        let area = zoom_of(crate::view3d::Fit::Area);
        let elements = zoom_of(crate::view3d::Fit::Elements);
        assert!(elements > area, "{elements} > {area}");
    }

    #[test]
    fn the_preview_is_always_a_tab_of_the_work_area_and_draws_there() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        assert!(!a.preview_work_tab(), "closed to begin with");
        a.open_preview();
        assert!(a.preview_work_tab(), "a tab, not a floating window");
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            a.preview_window(ui.ctx());
            egui::CentralPanel::no_frame().show(ui, |ui| a.preview_work_panel(ui));
        });
        out.textures_delta.clear();
        a.preview_close();
        assert!(!a.preview_work_tab() && !a.show_3d);
    }

    #[test]
    fn the_tab_opens_shown_again_after_it_was_closed_and_the_camera_tab_gives_way() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        a.open_preview();
        a.cam_ws_shown = true;
        a.preview_close();
        assert!(!a.show_3d && !a.pv_tab_shown);
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            a.preview_window(ui.ctx());
            egui::CentralPanel::no_frame().show(ui, |ui| a.preview_work_panel(ui));
        });
        out.textures_delta.clear();
        // Opened again: it is still a tab, and the shown one.
        a.open_preview();
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            a.preview_window(ui.ctx());
            egui::CentralPanel::no_frame().show(ui, |ui| a.preview_work_panel(ui));
        });
        out.textures_delta.clear();
        assert!(a.preview_work_tab() && a.pv_tab_shown && !a.cam_ws_shown);
    }

    #[test]
    fn with_a_rotary_the_block_is_the_cylinder_round_the_workpiece_and_the_stl_is_not_made() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        a.new_design();
        a.scene.add(0, base_core::scene::Kind::Rect { w: 40.0, h: 20.0 }, base_core::geom::Xf::translate(50.0, 60.0));
        a.layers[0].tool = Some("std-flat-3".into());
        a.open_preview();
        let prog = a.make_program();
        let r = crate::view3d::Region::of(&prog, None);
        let (plain, none) = a.rotary_view(r);
        assert!(none.is_none() && plain == r);
        a.device.rotary.enabled = true;
        a.device.rotary.diameter = 40.0;
        let (round, cyl) = a.rotary_view(r);
        let cyl = cyl.expect("a cylinder");
        assert_eq!(cyl.r, 20.0);
        assert!((round.unwrap().size()[1] - a.device.rotary.circumference()).abs() < 1e-9, "the whole way round");
        // The window is drawn, and the STL is refused with a word.
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            a.preview_window(ui.ctx());
            egui::CentralPanel::no_frame().show(ui, |ui| a.preview_work_panel(ui));
        });
        out.textures_delta.clear();
        a.export_view();
        assert!(a.status.contains("rotary"), "{}", a.status);
    }

    #[test]
    fn the_view_is_exported_as_a_closed_stl_with_the_cuts_done_up_to_the_slider() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        a.scene.add(0, Kind::Rect { w: 30.0, h: 30.0 }, Xf::translate(20.0, 20.0));
        a.touch_rev();
        a.open_preview();
        assert!(!a.prog.as_ref().unwrap().is_empty());
        a.view.progress = 0.0;
        assert!(a.view_model().is_none(), "nothing is cut at the start");
        a.view.progress = 1.0;
        let stl = a.view_model().expect("the whole job is cut");
        let n = u32::from_le_bytes([stl[80], stl[81], stl[82], stl[83]]) as usize;
        assert!(n > 100 && stl.len() == 84 + 50 * n);
        a.view.progress = 0.5;
        assert!(a.view_model().is_some(), "half of the job has cuts too");
    }
}
