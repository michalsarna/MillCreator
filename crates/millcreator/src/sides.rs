//! A two-sided project (File → Project setup… → Two-sided job): side A and side B are two work areas of one project. The side that
//! is shown is `App::scene` (so the editor only ever touches it); the other one waits here with its undo history. Each side shows the
//! other turned over as ghosts. Side B is cut as it is drawn, on the bottom (`toolpath::generate_sides`).
use crate::layers::Side;
use crate::materials::FlipAxis;
use crate::{App, History};
use base_core::project::Project;
use base_core::scene::Scene;
use base_core::setup::{self, TwoSided};
use eframe::egui;

/// The side that is not shown, and how the two lie.
#[derive(Default)]
pub(crate) struct Sides {
    pub flip: FlipAxis,
    pub ghosts: bool,
    /// Side B is the one shown (in `App::scene`); side A waits in `other`.
    pub shown_b: bool,
    pub other: Scene,
    pub other_history: History,
}

impl Sides {
    pub(crate) fn new(flip: FlipAxis) -> Sides {
        Sides { flip, ghosts: true, shown_b: false, other: Scene::default(), other_history: History::default() }
    }
}

/// Put the sides of a drawing (`shown` is the side on the screen) into a project that is saved: its scene becomes side A, and side B
/// goes into the section `two_sided`. Without sides the section is left out.
pub(crate) fn sides_into(p: &mut Project, shown: &Scene, two: Option<&Sides>) {
    p.remove_section(setup::SECTION);
    if let Some(t) = two {
        let (a, b) = if t.shown_b { (&t.other, shown) } else { (shown, &t.other) };
        p.scene = a.clone();
        p.set_section(setup::SECTION, &TwoSided::with_side_b(t.flip, t.ghosts, b));
    }
}

impl App {
    /// The size of the material the sides are turned over in: the project setup, or the work area without one.
    pub(crate) fn sides_size(&self) -> (f64, f64) {
        let w = if self.stock.width > 0.0 { self.stock.width } else { self.device.area.x };
        let h = if self.stock.height > 0.0 { self.stock.height } else { self.device.area.y };
        (w, h)
    }

    /// Show side B (`true`) or side A: the shown drawing and its undo history change places with the waiting ones, and the side that is
    /// cut follows.
    pub(crate) fn show_side(&mut self, b: bool) {
        let Some(two) = self.two.as_mut() else { return };
        if two.shown_b == b {
            return;
        }
        std::mem::swap(&mut self.scene, &mut two.other);
        std::mem::swap(&mut self.history, &mut two.other_history);
        two.shown_b = b;
        self.side = if b { Side::Bottom } else { Side::Top };
        self.ed.sel.clear();
        self.ed.ghost_sel.clear();
        self.ed.node_sel.clear();
        self.reset_picture_layers();
        self.touch_rev();
    }

    /// The drawing of side A and, in a two-sided project, of side B, whichever is shown.
    pub(crate) fn sides_scenes(&self) -> (&Scene, Option<&Scene>) {
        match &self.two {
            Some(t) if t.shown_b => (&t.other, Some(&self.scene)),
            Some(t) => (&self.scene, Some(&t.other)),
            None => (&self.scene, None),
        }
    }

    /// Put the sides into a project that is saved (its scene is side A).
    pub(crate) fn sides_to_project(&self, p: &mut Project) {
        sides_into(p, &self.scene, self.two.as_ref());
    }

    /// Take the sides of a project that was read (side A is shown), and leave the section out of what is kept for the next save.
    pub(crate) fn sides_from_project(&mut self, kept: &mut Project) {
        self.two = kept.section::<TwoSided>(setup::SECTION).ok().flatten().map(|s| Sides { flip: s.flip, ghosts: s.ghosts, shown_b: false, other: s.side_b(), other_history: History::default() });
        if let Some(t) = &self.two {
            self.stock.flip = t.flip;
        }
        kept.remove_section(setup::SECTION);
        self.side = Side::Top;
    }

    /// Make the project two-sided (or change how it is turned over); nothing happens to the drawings.
    pub(crate) fn make_two_sided(&mut self, flip: FlipAxis) {
        self.stock.flip = flip;
        match self.two.as_mut() {
            Some(t) => t.flip = flip,
            None => self.two = Some(Sides::new(flip)),
        }
        self.touch_rev();
    }

    /// Make the project one-sided again: only while side B is empty (its shapes are never thrown away). False when it was kept.
    pub(crate) fn make_one_sided(&mut self) -> bool {
        if self.two.is_none() {
            return true;
        }
        if self.sides_scenes().1.is_some_and(|b| !b.shapes.is_empty()) {
            return false;
        }
        self.show_side(false);
        self.two = None;
        self.side = Side::Top;
        self.touch_rev();
        true
    }

    /// The switch above the work area: side A or B, and the ghosts.
    pub(crate) fn sides_bar(&mut self, ui: &mut egui::Ui) {
        let Some(t) = self.two.as_mut() else { return };
        let (shown_b, flip) = (t.shown_b, t.flip);
        let changed = base_ui::two_sided::bar(ui, shown_b, &mut t.ghosts, flip);
        if changed.ghosts {
            self.touch_rev();
        }
        if changed.side {
            self.show_side(!shown_b);
        }
    }

    /// The ghosts of the other side as they lie under the shown one (turned over), while they are shown.
    pub(crate) fn ghosts(&self) -> Vec<base_core::scene::Shape> {
        match &self.two {
            Some(t) if t.ghosts => {
                let (w, h) = self.sides_size();
                setup::turned(&t.other, t.flip, w, h)
            }
            _ => vec![],
        }
    }

    /// The question of Save G-code in a two-sided project, and the saving when it is answered.
    pub(crate) fn sides_save_window(&mut self, ctx: &egui::Context) {
        let Some(flip) = self.two.as_ref().map(|t| t.flip) else {
            self.sides_save.open = false;
            return;
        };
        if let Some(c) = base_ui::two_sided::save_window(ctx, &mut self.sides_save, flip) {
            self.export_sides(c);
        }
    }

    /// The programs of side A and side B, each with the name of its side at its head.
    pub(crate) fn side_programs(&self) -> (crate::toolpath::Program, crate::toolpath::Program) {
        use crate::toolpath::{generate_sides, JobOptions};
        let (a, b) = self.sides_scenes();
        let empty = Scene::default();
        let b = b.unwrap_or(&empty);
        let opts = |side| JobOptions { one_by_one: self.one_by_one, side };
        (generate_sides(a, Some(b), &self.layers, &self.tools, &self.device, &self.stock, opts(Side::Top)), generate_sides(a, Some(b), &self.layers, &self.tools, &self.device, &self.stock, opts(Side::Bottom)))
    }

    /// Side B after side A in one program, with a stop to turn the material over between them when `hold`.
    pub(crate) fn sides_in_one(&self, mut a: crate::toolpath::Program, mut b: crate::toolpath::Program, hold: bool) -> crate::toolpath::Program {
        let flip = self.two.as_ref().map_or(FlipAxis::SideToSide, |t| t.flip);
        // The name of side B goes in once, where it starts.
        let note = if b.notes.first().is_some_and(|n| n.at == 0 && n.text.starts_with("Side B")) { b.notes.remove(0).text } else { base_ui::two_sided::side_comment(true, flip) };
        a.then(b, note, hold.then(|| base_ui::two_sided::hold_comment(flip)));
        a
    }

    /// The G-code of both sides of a two-sided project: in one file (side A, a stop to turn the material over or none, side B) or in
    /// two files named `_A` and `_B`. Each side says at its head which one it is.
    pub(crate) fn export_sides(&mut self, how: base_ui::two_sided::SaveSides) {
        use base_ui::i18n::trf;
        use base_ui::two_sided::{side_paths, SaveSides};
        if !self.gcode_variables_ok() {
            return;
        }
        let name = self.gcode_name();
        let (a, b) = self.side_programs();
        if a.is_empty() && b.is_empty() {
            self.status = self.nothing_to_cut();
            return;
        }
        let Some(p) = rfd::FileDialog::new().add_filter("G-code", &["nc", "gcode", "ngc"]).set_file_name(format!("{name}.nc")).save_file() else { return };
        let info = self.gcode_info(&name);
        let dev = self.device.with_stock(&self.stock);
        let text = |prog: &crate::toolpath::Program| crate::toolpath::to_gcode_info(prog, &dev, &self.layers, &self.tools, &info);
        let files: Vec<(std::path::PathBuf, String)> = match how {
            SaveSides::Separate => {
                let (pa, pb) = side_paths(&p);
                vec![(pa, text(&a)), (pb, text(&b))]
            }
            SaveSides::OneFile | SaveSides::OneFileWithHold => vec![(p, text(&self.sides_in_one(a, b, how == SaveSides::OneFileWithHold)))],
        };
        let mut saved = vec![];
        for (path, text) in files {
            let Some(text) = self.postprocessed(text) else { return };
            if let Err(e) = std::fs::write(&path, text) {
                self.status = trf("Could not save: {}", &[&e]);
                return;
            }
            saved.push(path);
        }
        self.status = trf("Saved {}", &[&saved.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")]);
        if let Some(first) = saved.into_iter().next().filter(|p| self.ed.grid_prefs.gcode_preview_after_save && p.exists()) {
            self.time_gcode_for_device();
            if let Err(e) = base_ui::gcode_view_ui::open(&mut self.gcode_view, first) {
                self.status = e;
            }
        }
    }

    /// The ghosts of the other side, under the shapes of the shown one, and the name of the shown side.
    pub(crate) fn paint_sides(&self, v: &base_ui::editor::CanvasView) {
        let Some(t) = &self.two else { return };
        if t.ghosts {
            let ghosts = self.ghosts();
            let colour = base_ui::two_sided::ghost_colour(self.ed.grid_prefs.bed_fill(self.scheme.is_dark()));
            base_ui::two_sided::paint_ghosts(v.painter, &|p| base_ui::editor::EditorHost::w2s(self, v.origin, p), &ghosts, colour);
        }
        base_ui::two_sided::paint_side_name(v.painter, v.bed.left_top() + egui::vec2(6.0, 6.0), t.shown_b);
    }
}

#[cfg(test)]
mod tests {
    use base_core::geom::Xf;
    use base_core::scene::Kind;

    #[test]
    fn the_sides_change_places_keep_their_drawings_and_are_saved_in_the_project() {
        let mut a = crate::tests::app();
        a.stock.width = 200.0;
        a.stock.height = 100.0;
        let on_a = a.scene.add(0, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(5.0, 5.0));
        a.make_two_sided(crate::materials::FlipAxis::SideToSide);
        a.show_side(true);
        assert!(a.scene.shapes.is_empty() && a.side == crate::layers::Side::Bottom, "side B is empty, and it is the side cut");
        let on_b = a.scene.add(0, Kind::Rect { w: 20.0, h: 5.0 }, Xf::translate(50.0, 50.0));
        let (sa, sb) = a.sides_scenes();
        assert!(sa.shape(on_a).is_some() && sb.unwrap().shape(on_b).is_some());
        // Saved while side B is shown: the project's drawing is side A, side B is in its section.
        let p = base_core::project::Project::from_json(&a.to_project().to_json()).unwrap();
        assert!(p.scene.shape(on_a).is_some() && p.scene.shapes.len() == 1);
        let mut kept = p.clone();
        let mut b = crate::tests::app();
        b.sides_from_project(&mut kept);
        assert!(kept.section_names().iter().all(|n| *n != base_core::setup::SECTION), "not kept twice");
        let t = b.two.as_ref().unwrap();
        assert!(!t.shown_b && t.other.shape(on_b).is_some());
        // One-sided again only while side B is empty.
        assert!(!a.make_one_sided());
        a.scene.shapes.clear();
        assert!(a.make_one_sided() && a.two.is_none() && a.scene.shape(on_a).is_some(), "back on side A");
    }

    /// Both sides in one file: each says which side it is, the spindle stops with the bit up before the stop that waits for the
    /// material to be turned over, starts again after it, and the job ends once.
    #[test]
    fn both_sides_in_one_file_with_a_stop_between_them() {
        let mut a = crate::tests::app();
        a.stock.width = 200.0;
        a.stock.height = 100.0;
        a.scene.add(0, Kind::Rect { w: 20.0, h: 20.0 }, Xf::translate(20.0, 20.0));
        a.make_two_sided(crate::materials::FlipAxis::SideToSide);
        a.show_side(true);
        a.scene.add(0, Kind::Rect { w: 20.0, h: 20.0 }, Xf::translate(120.0, 40.0));
        let (pa, pb) = a.side_programs();
        assert!(!pa.is_empty() && !pb.is_empty());
        let dev = a.device.with_stock(&a.stock);
        let info = a.gcode_info("job");
        let g = crate::toolpath::to_gcode_info(&a.sides_in_one(pa.clone(), pb.clone(), true), &dev, &a.layers, &a.tools, &info);
        let at = |t: &str| g.find(t).unwrap_or_else(|| panic!("no {t:?} in\n{g}"));
        let (side_a, stop, side_b) = (at("(Side A)"), at("M0 (Turn the material over"), at("(Side B:"));
        assert!(side_a < stop && stop < side_b, "{g}");
        let last_m5 = g[..stop].rfind("M5").unwrap();
        assert!(!g[last_m5..stop].contains("G1"), "nothing is cut after the spindle stopped");
        assert!(g[stop..].contains("M3 S"), "the spindle starts again for side B");
        assert_eq!(g.matches("M30").count(), 1);
        // Without the stop; and each side alone says which it is.
        let g = crate::toolpath::to_gcode_info(&a.sides_in_one(pa.clone(), pb.clone(), false), &dev, &a.layers, &a.tools, &info);
        assert!(!g.contains("M0 (Turn") && g.contains("(Side B:"));
        let g = crate::toolpath::to_gcode_info(&pa, &dev, &a.layers, &a.tools, &info);
        assert!(g.contains("(Side A)") && !g.contains("Side B"));
    }

    /// The ghosts are the shapes of the other side turned over; they can be picked to align to (base-ui does the rest).
    #[test]
    fn the_ghosts_are_the_other_side_turned_over() {
        let mut a = crate::tests::app();
        a.stock.width = 200.0;
        a.stock.height = 100.0;
        a.scene.add(0, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(5.0, 5.0));
        a.make_two_sided(crate::materials::FlipAxis::SideToSide);
        a.show_side(true);
        let g = base_ui::editor::EditorHost::ghost_shapes(&a);
        let b = g[0].bounds().unwrap();
        assert!((b.min.x - 185.0).abs() < 1e-9 && (b.max.x - 195.0).abs() < 1e-9, "{b:?}");
        a.two.as_mut().unwrap().ghosts = false;
        assert!(base_ui::editor::EditorHost::ghost_shapes(&a).is_empty());
    }
}
