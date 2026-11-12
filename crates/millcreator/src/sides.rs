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

    /// The ghosts of the other side, under the shapes of the shown one, and the name of the shown side.
    pub(crate) fn paint_sides(&self, v: &base_ui::editor::CanvasView) {
        let Some(t) = &self.two else { return };
        if t.ghosts {
            let (w, h) = self.sides_size();
            let ghosts = setup::turned(&t.other, t.flip, w, h);
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
}
