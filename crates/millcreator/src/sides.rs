//! A job on more than one side (File → Project setup… → Sides to work on): side A and side B of a sheet turned over, the four long
//! faces of a post or the six faces of a block are work areas of one project. The face that is shown is `App::scene` (so the editor
//! only ever touches it); the others wait here with their undo history. Each face shows the others where they lie to it as ghosts
//! (`base_core::faces`): the one under it seen through, those beside it unfolded over its edges. Side B of a sheet is cut as it is
//! drawn, on the bottom (`toolpath::generate_sides`); every face of a post or a block is a job of its own, on a work area as big as
//! the face and with the material as thick as the object is deep under it.
use crate::layers::Side;
use crate::materials::FlipAxis;
use crate::toolpath::Program;
use crate::{App, History};
use base_core::faces::{Body, Faces, Solid};
use base_core::project::Project;
use base_core::scene::Scene;
use eframe::egui;

/// The faces that are not shown, and the object they are the faces of.
pub(crate) struct Sides {
    /// The object. The size of a sheet is the one of the material of the project (`App::solid`).
    pub solid: Solid,
    pub ghosts: bool,
    /// The face shown (in `App::scene`); A is 0.
    pub shown: usize,
    /// The drawing and the undo history of every face; those of the shown face are in the app (here they are empty).
    pub others: Vec<Scene>,
    pub histories: Vec<History>,
}

impl Default for Sides {
    fn default() -> Sides {
        Sides::new(Solid::sheet(1.0, 1.0, 1.0, FlipAxis::SideToSide))
    }
}

impl Sides {
    pub(crate) fn new(solid: Solid) -> Sides {
        let n = solid.count();
        Sides { solid, ghosts: true, shown: 0, others: vec![Scene::default(); n], histories: (0..n).map(|_| History::default()).collect() }
    }
    /// Side B of a sheet is the one shown.
    pub(crate) fn shown_b(&self) -> bool {
        matches!(self.solid.body, Body::Sheet(_)) && self.shown == 1
    }
    /// The way a sheet is turned over.
    pub(crate) fn flip(&self) -> FlipAxis {
        match self.solid.body {
            Body::Sheet(f) => f,
            _ => FlipAxis::SideToSide,
        }
    }
    /// A post or a block (each face its own work area and thickness).
    pub(crate) fn is_solid(&self) -> bool {
        !matches!(self.solid.body, Body::Sheet(_))
    }
    /// The side of a sheet that is not shown.
    pub(crate) fn other(&self) -> &Scene {
        &self.others[1 - self.shown.min(1)]
    }
}

/// Put the faces of a drawing (`shown` is the face on the screen) into a project that is saved: face A is its drawing, the others
/// go into their section. Without faces the sections are left out.
pub(crate) fn sides_into(p: &mut Project, shown: &Scene, two: Option<&Sides>, solid: Option<Solid>) {
    p.remove_section(base_core::setup::SECTION);
    p.remove_section(base_core::faces::SECTION);
    if let (Some(t), Some(solid)) = (two, solid) {
        let scenes = (0..solid.count()).map(|i| if i == t.shown { shown.clone() } else { t.others[i].clone() }).collect();
        Faces { solid, ghosts: t.ghosts, scenes, marks: false }.write(p);
    }
}

/// The object of a job on more than one side, with the size of a sheet from the material (`stock`, or the work area of `device`).
pub(crate) fn solid_of(two: Option<&Sides>, stock: &crate::materials::Stock, device: &crate::device::Device) -> Option<Solid> {
    let t = two?;
    Some(match t.solid.body {
        Body::Sheet(flip) => {
            let w = if stock.width > 0.0 { stock.width } else { device.area.x };
            let h = if stock.height > 0.0 { stock.height } else { device.area.y };
            Solid::sheet(w, h, stock.thickness, flip)
        }
        _ => t.solid,
    })
}

impl App {
    /// The size of the material the sides are turned over in: the project setup, or the work area without one.
    pub(crate) fn sides_size(&self) -> (f64, f64) {
        let w = if self.stock.width > 0.0 { self.stock.width } else { self.device.area.x };
        let h = if self.stock.height > 0.0 { self.stock.height } else { self.device.area.y };
        (w, h)
    }

    /// The object of a job on more than one side, with the size of a sheet from the material of the project.
    pub(crate) fn solid(&self) -> Option<Solid> {
        solid_of(self.two.as_ref(), &self.stock, &self.device)
    }

    /// The material and the work area of face `i` of a post or a block: as big as the face, as thick as the object is deep there.
    pub(crate) fn face_stock(&self, i: usize) -> (crate::materials::Stock, crate::device::Device) {
        let mut stock = self.stock.clone();
        let mut dev = self.device.clone();
        if let Some(solid) = self.two.as_ref().filter(|t| t.is_solid()).map(|t| t.solid) {
            let f = solid.face(i);
            stock.width = f.w;
            stock.height = f.h;
            stock.thickness = f.depth;
            stock.first_pass_depth = stock.first_pass_depth.min(f.depth);
            dev.area.x = f.w.max(10.0);
            dev.area.y = f.h.max(10.0);
            dev.area.z = dev.area.z.max(f.depth.min(2000.0));
        }
        (stock, dev)
    }

    /// The material and the work area follow the shown face of a post or a block.
    fn apply_face_size(&mut self) {
        let Some(i) = self.two.as_ref().filter(|t| t.is_solid()).map(|t| t.shown) else { return };
        let (stock, dev) = self.face_stock(i);
        self.stock = stock;
        self.device.area = dev.area;
    }

    /// The material of the shown face was changed (its size or thickness): the object follows, and so do the other faces.
    pub(crate) fn solid_from_stock(&mut self) {
        let (w, h, t) = (self.stock.width, self.stock.height, self.stock.thickness);
        if let Some(two) = self.two.as_mut().filter(|t| t.is_solid()) {
            if w > 0.0 && h > 0.0 && t > 0.0 {
                two.solid = two.solid.with_face_size(two.shown, w, h, t);
            }
        }
    }

    /// Show face `i`: the shown drawing and its undo history change places with the waiting ones; the side that is cut, the material
    /// and the work area follow.
    pub(crate) fn show_face(&mut self, i: usize) {
        let Some(two) = self.two.as_mut() else { return };
        if two.shown == i || i >= two.others.len() {
            return;
        }
        let was = two.shown;
        std::mem::swap(&mut self.scene, &mut two.others[was]);
        std::mem::swap(&mut self.history, &mut two.histories[was]);
        std::mem::swap(&mut self.scene, &mut two.others[i]);
        std::mem::swap(&mut self.history, &mut two.histories[i]);
        two.shown = i;
        let solid = two.is_solid();
        self.side = if !solid && i == 1 { Side::Bottom } else { Side::Top };
        self.ed.sel.clear();
        self.ed.ghost_sel.clear();
        self.ed.node_sel.clear();
        if solid {
            self.apply_face_size();
            self.ed.view.need_fit = true;
        }
        self.reset_picture_layers();
        self.touch_rev();
    }

    /// Show side B (`true`) or side A of a sheet.
    pub(crate) fn show_side(&mut self, b: bool) {
        self.show_face(usize::from(b));
    }

    /// The drawing of side A and, in a two-sided project, of side B, whichever is shown (for a post or a block: A and B).
    pub(crate) fn sides_scenes(&self) -> (&Scene, Option<&Scene>) {
        match &self.two {
            Some(t) => (self.face_scene(0), Some(self.face_scene(1)).filter(|_| t.others.len() > 1)),
            None => (&self.scene, None),
        }
    }

    /// The drawing of face `i`, whichever is shown.
    pub(crate) fn face_scene(&self, i: usize) -> &Scene {
        match &self.two {
            Some(t) if t.shown != i => &t.others[i],
            _ => &self.scene,
        }
    }

    /// Put the sides into a project that is saved (its scene is face A).
    pub(crate) fn sides_to_project(&self, p: &mut Project) {
        sides_into(p, &self.scene, self.two.as_ref(), self.solid());
    }

    /// Take the faces of a project that was read (face A is shown, with its size for a post or a block), and leave their sections out
    /// of what is kept for the next save.
    pub(crate) fn sides_from_project(&mut self, kept: &mut Project) {
        let sheet = [self.stock.width, self.stock.height, self.stock.thickness];
        self.two = Faces::read(kept, sheet).map(|f| {
            let mut t = Sides::new(f.solid);
            t.ghosts = f.ghosts;
            for (i, sc) in f.scenes.into_iter().enumerate().skip(1) {
                t.others[i] = sc;
            }
            t
        });
        if let Some(t) = &self.two {
            self.stock.flip = t.flip();
        }
        self.side = Side::Top;
        self.apply_face_size();
    }

    /// A project made before two-sided jobs had sides A and B gave some layers the bottom side. Their shapes go to side B, turned over
    /// as the material is (side B is drawn as the material lies after it is turned over), and the layers are layers of either side
    /// again. True when something was moved.
    pub(crate) fn sides_from_bottom_layers(&mut self) -> bool {
        let bottom: Vec<usize> = self.layers.iter().enumerate().filter(|(_, l)| l.side == Side::Bottom).map(|(i, _)| i).collect();
        if bottom.is_empty() {
            return false;
        }
        for i in &bottom {
            self.layers[*i].side = Side::Top;
        }
        let moving: Vec<base_core::scene::Shape> = self.scene.shapes.iter().filter(|s| bottom.contains(&s.layer)).cloned().collect();
        if moving.is_empty() || self.two.as_ref().is_some_and(|t| t.is_solid()) {
            return false;
        }
        if self.two.is_none() {
            self.two = Some(Sides::new(Solid::sheet(1.0, 1.0, 1.0, self.stock.flip)));
        }
        let (w, h) = self.sides_size();
        let turn = base_core::setup::turn(self.stock.flip, w, h);
        self.scene.shapes.retain(|s| !bottom.contains(&s.layer));
        let t = self.two.as_mut().expect("made above");
        let side_b = if t.shown == 1 { &mut self.scene } else { &mut t.others[1] };
        for s in moving {
            side_b.add_shape(base_core::scene::Shape { xf: s.xf.then(turn), ..s });
        }
        self.touch_rev();
        true
    }

    /// Make the project two-sided (or change how it is turned over); nothing happens to the drawings.
    pub(crate) fn make_two_sided(&mut self, flip: FlipAxis) -> bool {
        self.stock.flip = flip;
        self.make_faces(Solid::sheet(1.0, 1.0, 1.0, flip))
    }

    /// Make the project a job on the faces of `solid` (or change its size); nothing happens to the drawings. A face that would go
    /// (from a block to a post, or to a sheet) must be empty: false when the project stays as it was.
    pub(crate) fn make_faces(&mut self, solid: Solid) -> bool {
        let n = solid.count();
        if let Some(t) = &self.two {
            if (n..t.others.len()).any(|i| !self.face_scene(i).shapes.is_empty()) {
                return false;
            }
            if t.shown >= n {
                self.show_face(0);
            }
        }
        let t = self.two.get_or_insert_with(|| Sides::new(solid));
        t.solid = solid;
        t.others.resize(n, Scene::default());
        t.histories.resize_with(n, History::default);
        let shown = t.shown;
        self.side = if !t.is_solid() && shown == 1 { Side::Bottom } else { Side::Top };
        self.apply_face_size();
        self.touch_rev();
        true
    }

    /// Make the project one-sided again: only while the other faces are empty (their shapes are never thrown away). False when it
    /// was kept.
    pub(crate) fn make_one_sided(&mut self) -> bool {
        let Some(t) = &self.two else { return true };
        if (0..t.others.len()).any(|i| i != 0 && !self.face_scene(i).shapes.is_empty()) {
            return false;
        }
        self.show_face(0);
        self.two = None;
        self.side = Side::Top;
        self.touch_rev();
        true
    }

    /// The switch above the work area: the side or the face, and the ghosts.
    pub(crate) fn sides_bar(&mut self, ui: &mut egui::Ui) {
        let Some(solid) = self.solid() else { return };
        let Some(t) = self.two.as_mut() else { return };
        let shown = t.shown;
        let changed = base_ui::two_sided::faces_bar(ui, &solid, shown, &mut t.ghosts);
        if changed.ghosts {
            self.touch_rev();
        }
        if let Some(i) = changed.face {
            self.show_face(i);
        }
    }

    /// The ghosts of the other faces where they lie to the shown one, while they are shown. Each face's ghosts get ids of their own
    /// (the drawings of the faces count their ids apart).
    pub(crate) fn ghosts(&self) -> Vec<base_core::scene::Shape> {
        let (Some(t), Some(solid)) = (self.two.as_ref(), self.solid()) else { return vec![] };
        if !t.ghosts {
            return vec![];
        }
        let many = t.others.len() > 2;
        (0..t.others.len())
            .filter(|j| *j != t.shown)
            .flat_map(|j| {
                let mut g = solid.ghosts_of(&t.others[j], j, t.shown);
                if many {
                    for s in &mut g {
                        s.id += (j as u64 + 1) << 40;
                    }
                }
                g
            })
            .collect()
    }

    /// The question of Save G-code (or of Preview G-code) in a job on more than one side, and the saving or the showing when it is
    /// answered.
    pub(crate) fn sides_save_window(&mut self, ctx: &egui::Context) {
        let Some(solid) = self.solid() else {
            self.sides_save.open = false;
            return;
        };
        let show = self.sides_save.show;
        if let Some(c) = base_ui::two_sided::save_window_for(ctx, &mut self.sides_save, &solid, false) {
            if show {
                self.show_sides(c);
            } else {
                self.export_sides(c);
            }
        }
    }

    /// The programs of side A and side B, each with the name of its side at its head.
    pub(crate) fn side_programs(&self) -> (Program, Program) {
        use crate::toolpath::{generate_sides, JobOptions};
        let (a, b) = self.sides_scenes();
        let empty = Scene::default();
        let b = b.unwrap_or(&empty);
        let opts = |side| JobOptions { one_by_one: self.one_by_one, side };
        (generate_sides(a, Some(b), &self.layers, &self.tools, &self.device, &self.stock, opts(Side::Top)), generate_sides(a, Some(b), &self.layers, &self.tools, &self.device, &self.stock, opts(Side::Bottom)))
    }

    /// The program of every face of a post or a block, each made for its own material and work area (also given), with the name of
    /// the face at its head.
    pub(crate) fn face_programs(&self) -> Vec<(Program, crate::materials::Stock, crate::device::Device)> {
        let Some(solid) = self.solid() else { return vec![] };
        (0..solid.count())
            .map(|i| {
                let (stock, dev) = self.face_stock(i);
                let opts = crate::toolpath::JobOptions { one_by_one: self.one_by_one, side: Side::Top };
                let mut p = crate::toolpath::generate_sides(self.face_scene(i), None, &self.layers, &self.tools, &dev, &stock, opts);
                p.notes.insert(0, crate::toolpath::Note { at: 0, text: solid.head_comment(i), before_layer: true });
                (p, stock, dev)
            })
            .collect()
    }

    /// Side B after side A in one program, with a stop to turn the material over between them when `hold`.
    pub(crate) fn sides_in_one(&self, mut a: Program, mut b: Program, hold: bool) -> Program {
        let flip = self.two.as_ref().map_or(FlipAxis::SideToSide, |t| t.flip());
        // The name of side B goes in once, where it starts.
        let note = if b.notes.first().is_some_and(|n| n.at == 0 && n.text.starts_with("Side B")) { b.notes.remove(0).text } else { base_ui::two_sided::side_comment(true, flip) };
        a.then(b, note, hold.then(|| base_ui::two_sided::hold_comment(flip)));
        a
    }

    /// The faces of a post or a block in one program, each after a stop to turn the object when `hold`. The program of each face is
    /// moved so that the device of face A writes it where the device of its own face would (the zero of the program, Z from the
    /// bottom): every face counts from its own corner of the zero.
    pub(crate) fn faces_in_one(&self, faces: Vec<(Program, crate::materials::Stock, crate::device::Device)>, hold: bool) -> Option<Program> {
        let solid = self.solid()?;
        let mut it = faces.into_iter().enumerate();
        let (_, (mut first, s0, d0)) = it.next()?;
        let out0 = d0.with_stock(&s0);
        let zero = |d: &crate::device::Device| {
            let (x, y) = crate::toolpath::machine_xy(d, 0.0, 0.0);
            [-x, -y, -d.z_offset]
        };
        let z0 = zero(&out0);
        for (i, (mut p, s, d)) in it {
            let zi = zero(&d.with_stock(&s));
            let shift = [z0[0] - zi[0], z0[1] - zi[1], z0[2] - zi[2]];
            for m in &mut p.moves {
                for k in 0..3 {
                    m.from[k] += shift[k];
                    m.to[k] += shift[k];
                }
            }
            p.bounds = p.bounds.map(|(lo, hi)| ([lo[0] + shift[0], lo[1] + shift[1], lo[2] + shift[2]], [hi[0] + shift[0], hi[1] + shift[1], hi[2] + shift[2]]));
            let note = if p.notes.first().is_some_and(|n| n.at == 0) { p.notes.remove(0).text } else { solid.head_comment(i) };
            // The top of the next face is at another height (unless Z counts from the table): its zero is set again.
            if !s.z_bottom {
                p.notes.insert(0, crate::toolpath::Note { at: 0, text: format!("Set the zero of Z on the top of face {}", Solid::letter(i)), before_layer: true });
            }
            first.then(p, note, hold.then(|| solid.turn_comment(i)));
        }
        Some(first)
    }

    /// The G-code of all the sides of the job the way `how` says, with the postprocessors: one text (side A, a stop to turn the
    /// material over or none, side B...) or one for each side, each saying at its head which side it is. `None` when there is nothing
    /// to make or a postprocessor failed (the reason is in the status line).
    pub(crate) fn sides_texts(&mut self, how: base_ui::two_sided::SaveSides) -> Option<Vec<String>> {
        use base_ui::two_sided::SaveSides;
        if !self.gcode_variables_ok() {
            return None;
        }
        let info = self.gcode_info(&self.gcode_name());
        if self.two.as_ref().is_some_and(|t| t.is_solid()) {
            let faces = self.face_programs();
            if faces.iter().all(|(p, _, _)| p.is_empty()) {
                self.status = self.nothing_to_cut();
                return None;
            }
            let text = |p: &Program, s: &crate::materials::Stock, d: &crate::device::Device| crate::toolpath::to_gcode_info(p, &d.with_stock(s), &self.layers, &self.tools, &info);
            let texts = match how {
                SaveSides::Separate => faces.iter().map(|(p, s, d)| text(p, s, d)).collect(),
                SaveSides::OneFile | SaveSides::OneFileWithHold => {
                    let (s0, d0) = (faces[0].1.clone(), faces[0].2.clone());
                    let one = self.faces_in_one(faces, how == SaveSides::OneFileWithHold)?;
                    vec![text(&one, &s0, &d0)]
                }
            };
            return texts.into_iter().map(|t| self.postprocessed(t)).collect();
        }
        let (a, b) = self.side_programs();
        if a.is_empty() && b.is_empty() {
            self.status = self.nothing_to_cut();
            return None;
        }
        let dev = self.device.with_stock(&self.stock);
        let text = |prog: &Program| crate::toolpath::to_gcode_info(prog, &dev, &self.layers, &self.tools, &info);
        let texts = match how {
            SaveSides::Separate => vec![text(&a), text(&b)],
            SaveSides::OneFile | SaveSides::OneFileWithHold => vec![text(&self.sides_in_one(a, b, how == SaveSides::OneFileWithHold))],
        };
        texts.into_iter().map(|t| self.postprocessed(t)).collect()
    }

    /// The number of sides or faces of the job (1 without).
    pub(crate) fn face_count(&self) -> usize {
        self.two.as_ref().map_or(1, |t| t.others.len())
    }

    /// The G-code of all the sides of the job: in one file or in one file for each, named `_A`, `_B`...
    pub(crate) fn export_sides(&mut self, how: base_ui::two_sided::SaveSides) {
        use base_ui::i18n::trf;
        use base_ui::two_sided::{face_paths, SaveSides};
        let name = self.gcode_name();
        let n = self.face_count();
        let Some(texts) = self.sides_texts(how) else { return };
        let Some(p) = rfd::FileDialog::new().add_filter("G-code", &["nc", "gcode", "ngc"]).set_file_name(format!("{name}.nc")).save_file() else { return };
        let paths = match how {
            SaveSides::Separate => face_paths(&p, n),
            _ => vec![p],
        };
        let mut saved = vec![];
        for (path, text) in paths.into_iter().zip(texts) {
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

    /// The G-code of all the sides of the job in tabs, the way `how` says: one tab with the whole job, or a tab for each side
    /// (`name_A`, `name_B`...). Nothing is saved.
    pub(crate) fn show_sides(&mut self, how: base_ui::two_sided::SaveSides) {
        use base_ui::i18n::tr;
        use base_ui::two_sided::{face_text, text_face, text_slot, SaveSides};
        let n = self.face_count();
        let Some(texts) = self.sides_texts(how) else { return };
        let stem = self.gcode_name();
        let slot = self.slot;
        let names: Vec<String> = if how == SaveSides::Separate { (0..n).map(|i| format!("{stem}_{}.nc", Solid::letter(i))).collect() } else { vec![format!("{stem}.nc")] };
        // The tabs of the other faces alone go when the whole job is in one (unless they have edits), and so do those of faces
        // that are no more.
        let keep = names.len();
        self.gcode_texts.retain(|g| text_slot(g.slot) != slot || text_face(g.slot) < keep || g.dirty());
        let stamp = self.gcode_stamp();
        for (i, (name, text)) in names.into_iter().zip(texts).enumerate() {
            let key = face_text(slot, i);
            match self.gcode_texts.iter_mut().find(|g| g.slot == key) {
                Some(g) => {
                    if !g.dirty() {
                        g.replace(text);
                        g.stamp = stamp;
                    }
                    g.name = name;
                    g.sides = Some(how);
                }
                None => {
                    let mut g = base_ui::gcode_text_ui::GcodeText::new(key, &name, text);
                    g.stamp = stamp;
                    g.sides = Some(how);
                    self.gcode_texts.push(g);
                }
            }
        }
        self.panes.select(base_ui::work_panes::Content::GcodeText(slot));
        self.status = tr("The G-code is shown, nothing is saved").to_string();
    }

    /// The ghosts of the other faces, the outlines of those beside the shown one (unfolded round it), and the name of the shown face.
    pub(crate) fn paint_sides(&self, v: &base_ui::editor::CanvasView) {
        let (Some(t), Some(solid)) = (&self.two, self.solid()) else { return };
        let to = |p: base_core::geom::Pt| base_ui::editor::EditorHost::w2s(self, v.origin, p);
        // On the work area the ghosts of the face under it; round it, on the workspace, those of the faces beside it.
        let colour = base_ui::two_sided::ghost_colour(self.ed.grid_prefs.bed_fill(self.scheme.is_dark()));
        let outside = base_ui::two_sided::ghost_colour(self.ed.grid_prefs.workspace_fill());
        if t.ghosts {
            for j in (0..t.others.len()).filter(|j| *j != t.shown) {
                let c = if solid.relation(j, t.shown) == base_core::faces::Relation::Opposite { colour } else { outside };
                base_ui::two_sided::paint_ghosts(v.painter, &to, &solid.ghosts_of(&t.others[j], j, t.shown), c);
            }
        }
        if t.others.len() > 2 {
            base_ui::two_sided::paint_net(v.painter, &to, base_core::geom::Pt::new(0.0, 0.0), &solid, t.shown, outside);
        }
        base_ui::two_sided::paint_face_name(v.painter, v.bed.left_top() + egui::vec2(6.0, 6.0), &solid, t.shown);
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
        assert!(!t.shown_b() && t.other().shape(on_b).is_some());
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

    /// An old project with a layer for the bottom: its shapes become side B, turned over, and the layer is an ordinary one again.
    #[test]
    fn the_shapes_of_an_old_bottom_layer_become_side_b() {
        let mut a = crate::tests::app();
        a.stock.width = 200.0;
        a.stock.height = 100.0;
        a.layers[1].side = crate::layers::Side::Bottom;
        let top = a.scene.add(0, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(5.0, 5.0));
        a.scene.add(1, Kind::Rect { w: 20.0, h: 10.0 }, Xf::translate(10.0, 20.0));
        assert!(a.sides_from_bottom_layers());
        assert!(a.layers.iter().all(|l| l.side == crate::layers::Side::Top));
        assert!(a.scene.shape(top).is_some() && a.scene.shapes.len() == 1, "side A keeps the rest");
        let (_, b) = a.sides_scenes();
        let b = b.expect("two-sided now");
        let bb = b.shapes[0].bounds().unwrap();
        assert!((bb.min.x - 170.0).abs() < 1e-9 && (bb.max.x - 190.0).abs() < 1e-9 && b.shapes[0].layer == 1, "turned over side to side: {bb:?}");
        assert!(!a.sides_from_bottom_layers(), "once");
    }

    /// Preview G-code of a two-sided job asks how, then shows one tab with the whole job or a tab for each side; the tab of side B is
    /// made again with the project like the other one.
    #[test]
    fn the_g_code_of_a_two_sided_job_is_shown_whole_or_a_tab_for_each_side() {
        use base_ui::two_sided::{SaveSides, SIDE_B_TEXT};
        let mut a = crate::tests::app();
        a.stock.width = 200.0;
        a.stock.height = 100.0;
        a.scene.add(0, Kind::Rect { w: 20.0, h: 20.0 }, Xf::translate(20.0, 20.0));
        a.make_two_sided(crate::materials::FlipAxis::SideToSide);
        a.show_side(true);
        a.scene.add(0, Kind::Rect { w: 20.0, h: 20.0 }, Xf::translate(120.0, 40.0));
        a.preview_gcode();
        assert!(a.sides_save.open && a.sides_save.show, "asked first");
        let slot = a.slot;
        a.show_sides(SaveSides::Separate);
        let tab = |a: &crate::App, key: usize| a.gcode_texts.iter().find(|g| g.slot == key).map(|g| (g.name.clone(), g.text.clone(), g.sides));
        let (ta, tb) = (tab(&a, slot).unwrap(), tab(&a, slot | SIDE_B_TEXT).unwrap());
        assert!(ta.0.ends_with("_A.nc") && tb.0.ends_with("_B.nc"));
        assert!(ta.1.contains("(Side A)") && !ta.1.contains("Side B") && tb.1.contains("(Side B"), "{}", ta.1);
        // The tab of side B follows the project (a new note changes what the G-code is made of).
        let before = tb.1.clone();
        a.scene.add(0, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(150.0, 60.0));
        a.description = "a note".into();
        a.refresh_gcode_text_after(std::time::Duration::ZERO);
        assert_ne!(tab(&a, slot | SIDE_B_TEXT).unwrap().1, before, "side B made again");
        // One file: one tab with both sides and the stop; the tab of side B goes.
        a.show_sides(SaveSides::OneFileWithHold);
        assert!(tab(&a, slot | SIDE_B_TEXT).is_none());
        let whole = tab(&a, slot).unwrap();
        assert!(whole.1.contains("(Side A)") && whole.1.contains("M0 (Turn") && whole.1.contains("(Side B"), "{}", whole.1);
        assert_eq!(whole.2, Some(SaveSides::OneFileWithHold));
    }

    fn post_app() -> crate::App {
        let mut a = crate::tests::app();
        a.open_setup();
        a.setup.sides = base_ui::project_setup::Sides::Post;
        a.setup.post = [60.0, 40.0, 250.0];
        a.apply_setup();
        a
    }

    /// A post: every face is its own work area, as big as the face, and the material under it is as thick as the post is deep
    /// there; the drawings of the faces stay apart and the project keeps them all.
    #[test]
    fn the_faces_of_a_post_have_their_own_size_and_drawings_and_are_saved() {
        let mut a = post_app();
        let t = a.two.as_ref().unwrap();
        assert!(t.is_solid() && t.others.len() == 4 && t.shown == 0);
        assert_eq!((a.stock.width, a.stock.height, a.stock.thickness, a.device.area.x, a.device.area.y), (250.0, 60.0, 40.0, 250.0, 60.0));
        let on_a = a.scene.add(0, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(5.0, 5.0));
        a.show_face(1);
        assert_eq!((a.stock.width, a.stock.height, a.stock.thickness, a.device.area.y), (250.0, 40.0, 60.0, 40.0), "face B: H × b, a deep");
        assert!(a.scene.shapes.is_empty());
        let on_b = a.scene.add(0, Kind::Rect { w: 20.0, h: 5.0 }, Xf::translate(50.0, 10.0));
        a.show_face(3);
        a.scene.add(0, Kind::Ellipse { w: 8.0, h: 8.0 }, Xf::translate(100.0, 10.0));
        // The ghosts on face D: face A beside it (at the front, below the drawing), face B under it (seen through), C at the back.
        let ghosts = a.ghosts();
        assert_eq!(ghosts.len(), 2);
        assert!(ghosts.iter().all(|g| g.id > 1 << 40), "ids of their own");
        // Saved while face D is shown: face A is the drawing of the file and the size of the post is kept.
        let p = base_core::project::Project::from_json(&a.to_project().to_json()).unwrap();
        assert!(p.scene.shape(on_a).is_some() && p.scene.shapes.len() == 1);
        let mut kept = p.clone();
        let mut b = crate::tests::app();
        b.stock = a.stock.clone();
        b.sides_from_project(&mut kept);
        let t = b.two.as_ref().unwrap();
        assert_eq!(t.solid, base_core::faces::Solid::post(60.0, 40.0, 250.0));
        assert!(t.others[1].shape(on_b).is_some() && t.others[3].shapes.len() == 1);
        assert_eq!((b.stock.width, b.stock.height, b.stock.thickness), (250.0, 60.0, 40.0), "face A shown, with its size");
        // A thicker material on face B makes the post deeper from front to back (its side a).
        a.show_face(1);
        a.stock.thickness = 70.0;
        a.solid_from_stock();
        assert_eq!(a.two.as_ref().unwrap().solid, base_core::faces::Solid::post(70.0, 40.0, 250.0));
        a.show_face(0);
        assert_eq!(a.stock.height, 70.0, "face A follows");
        // Not one-sided, nor a sheet, while the faces have shapes.
        assert!(!a.make_one_sided() && !a.make_two_sided(crate::materials::FlipAxis::SideToSide));
    }

    /// All the faces of a post in one file: each with its name, a stop with how to roll it before each face after A, the spindle
    /// starting again; every face counts from its own zero (here the middle of each face), and from the top of its own material.
    #[test]
    fn the_faces_of_a_post_in_one_file_and_in_a_file_each() {
        use base_ui::two_sided::SaveSides;
        let mut a = crate::tests::app();
        a.open_setup();
        a.setup.sides = base_ui::project_setup::Sides::Post;
        a.setup.post = [60.0, 40.0, 250.0];
        a.setup.zero = crate::materials::ZeroPoint::Center;
        a.apply_setup();
        // The same square in the middle of every face.
        for i in 0..4 {
            a.show_face(i);
            let (w, h) = (a.stock.width, a.stock.height);
            a.scene.add(0, Kind::Rect { w: 10.0, h: 10.0 }, Xf::translate(w / 2.0 - 5.0, h / 2.0 - 5.0));
        }
        a.show_face(0);
        let one = a.sides_texts(SaveSides::OneFileWithHold).unwrap();
        assert_eq!(one.len(), 1);
        let g = &one[0];
        let at = |t: &str| g.find(t).unwrap_or_else(|| panic!("no {t:?} in\n{g}"));
        let order = [at("(Face A of 4"), at("M0 (Roll it a quarter turn back: face B up"), at("(Face B of 4"), at("M0 (Roll it a quarter turn back: face C up"), at("(Face D of 4")];
        assert!(order.windows(2).all(|w| w[0] < w[1]), "{g}");
        assert_eq!(g.matches("M30").count(), 1);
        // Every face is cut round its own middle: the X and Y of the cuts are the same small numbers on each face.
        let xs = |text: &str| -> (f64, f64) {
            let mut lo = f64::MAX;
            let mut hi = f64::MIN;
            for l in text.lines().filter(|l| l.starts_with("G1")) {
                for w in l.split_whitespace().filter(|w| w.starts_with('X')) {
                    let v: f64 = w[1..].parse().unwrap();
                    lo = lo.min(v);
                    hi = hi.max(v);
                }
            }
            (lo, hi)
        };
        let (lo, hi) = xs(g);
        assert!(lo > -10.0 && hi < 10.0, "every face from its own middle: {lo} {hi}");
        let sep = a.sides_texts(SaveSides::Separate).unwrap();
        assert_eq!(sep.len(), 4);
        assert!(sep[2].contains("(Face C of 4, bottom of the post: 250 x 60 mm, 40 mm high)") && !sep[2].contains("M0 (Roll"), "{}", sep[2]);
        // The tabs of the faces: one each, and one for the whole when it is shown in one.
        a.show_sides(SaveSides::Separate);
        assert_eq!(a.gcode_texts.iter().filter(|t| base_ui::two_sided::text_slot(t.slot) == a.slot).count(), 4);
        assert!(a.gcode_texts.iter().any(|t| t.name.ends_with("_D.nc")));
        a.show_sides(SaveSides::OneFile);
        assert_eq!(a.gcode_texts.iter().filter(|t| base_ui::two_sided::text_slot(t.slot) == a.slot).count(), 1);
    }

    /// A block has six faces, the ends E and F turned up; going back to a post takes E and F away only while they are empty.
    #[test]
    fn a_block_has_six_faces_and_becomes_a_post_only_when_its_ends_are_empty() {
        let mut a = crate::tests::app();
        a.open_setup();
        a.setup.sides = base_ui::project_setup::Sides::Block;
        a.setup.block = [120.0, 80.0, 50.0];
        a.apply_setup();
        a.show_face(4);
        assert_eq!((a.stock.width, a.stock.height, a.stock.thickness), (50.0, 80.0, 120.0));
        a.scene.add(0, Kind::Rect { w: 5.0, h: 5.0 }, Xf::IDENTITY);
        a.open_setup();
        assert_eq!(a.setup.sides, base_ui::project_setup::Sides::Block);
        a.setup.sides = base_ui::project_setup::Sides::Post;
        a.apply_setup();
        assert_eq!(a.two.as_ref().unwrap().others.len(), 6, "kept: face E has a shape");
        a.scene.shapes.clear();
        a.apply_setup();
        assert_eq!(a.two.as_ref().unwrap().others.len(), 4);
        assert_eq!(a.two.as_ref().unwrap().shown, 0, "back on face A");
    }
}
