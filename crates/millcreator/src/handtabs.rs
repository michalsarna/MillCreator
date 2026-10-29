//! Tabs placed by hand: dots on the outline of a shape of the layer that is being edited. Click on the outline to add a tab, drag a
//! dot to move it along the outline, click a dot to take it away. They are kept with the layer (`Tabs::manual`): the shape and how far
//! round its outline the tab lies, so that they follow the shape when it is moved or scaled.
use crate::layers::{ManualTab, MillLayer};
use crate::App;
use base_core::geom::{Polyline, Pt};
use base_core::scene::{Scene, Shape};
use base_ui::editor::{CanvasView, EditorHost};
use eframe::egui::{self, Color32, Pos2, Stroke};

/// How near (screen points) the pointer has to be to the outline or to a dot.
const GRAB: f32 = 9.0;

/// The outline of a shape the tabs go on: the first closed one, in the coordinates of the work area.
pub fn outline(shape: &Shape) -> Option<Polyline> {
    shape.polys().into_iter().find(|p| p.closed && p.pts.len() >= 3 && p.length() > 1e-6)
}

/// The point `at` (0 … 1) of the way round the outline of the shape.
pub fn point_on(shape: &Shape, at: f64) -> Option<Pt> {
    let poly = outline(shape)?;
    let total = poly.length();
    let mut want = at.rem_euclid(1.0) * total;
    let n = poly.pts.len();
    for i in 0..n {
        let (a, b) = (poly.pts[i], poly.pts[(i + 1) % n]);
        let l = a.dist(b);
        if want <= l || i == n - 1 {
            let t = if l > 1e-12 { (want / l).clamp(0.0, 1.0) } else { 0.0 };
            return Some(Pt::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t));
        }
        want -= l;
    }
    None
}

/// The point of the outline nearest to `p`: how far round it is (0 … 1) and how far away.
pub fn nearest(shape: &Shape, p: Pt) -> Option<(f64, f64)> {
    let poly = outline(shape)?;
    let total = poly.length();
    let n = poly.pts.len();
    let mut walked = 0.0;
    let mut best: Option<(f64, f64)> = None;
    for i in 0..n {
        let (a, b) = (poly.pts[i], poly.pts[(i + 1) % n]);
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        let l2 = dx * dx + dy * dy;
        let t = if l2 > 1e-12 { (((p.x - a.x) * dx + (p.y - a.y) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
        let d = Pt::new(a.x + dx * t, a.y + dy * t).dist(p);
        if best.is_none_or(|(_, bd)| d < bd) {
            best = Some(((walked + t * l2.sqrt()) / total, d));
        }
        walked += l2.sqrt();
    }
    best
}

/// The shapes the tabs of layer `li` go on.
fn shapes_of<'a>(scene: &'a Scene, li: usize) -> impl Iterator<Item = &'a Shape> + 'a {
    scene.shapes.iter().filter(move |s| s.layer == li && outline(s).is_some())
}

/// Add a tab at the point of an outline of layer `li` nearest to `p`, when one is within `tol` mm. Returns whether it was added.
pub fn add(scene: &Scene, layer: &mut MillLayer, li: usize, p: Pt, tol: f64) -> bool {
    let best = shapes_of(scene, li).filter_map(|s| nearest(s, p).map(|(at, d)| (s.id, at, d))).min_by(|a, b| a.2.total_cmp(&b.2));
    match best {
        Some((shape, at, d)) if d <= tol => {
            layer.tabs.manual.push(ManualTab { shape, at });
            true
        }
        _ => false,
    }
}

/// The tab of the layer nearest to `p`, when within `tol` mm.
pub fn pick(scene: &Scene, layer: &MillLayer, p: Pt, tol: f64) -> Option<usize> {
    layer.tabs.manual.iter().enumerate().filter_map(|(i, t)| scene.shape(t.shape).and_then(|s| point_on(s, t.at)).map(|q| (i, q.dist(p)))).filter(|(_, d)| *d <= tol).min_by(|a, b| a.1.total_cmp(&b.1)).map(|(i, _)| i)
}

/// Where the tab `i` of the layer is now, in the coordinates of the work area.
pub fn place(scene: &Scene, layer: &MillLayer, i: usize) -> Option<Pt> {
    let t = layer.tabs.manual.get(i)?;
    point_on(scene.shape(t.shape)?, t.at)
}

/// What a press on the canvas is doing.
#[derive(Default)]
pub struct Editing {
    /// The tab being dragged, and whether it has moved.
    drag: Option<(usize, bool)>,
}

impl App {
    /// Is the layer that is being edited one whose tabs can be placed by hand?
    pub(crate) fn hand_tabs_possible(&self) -> bool {
        self.layers.get(self.ed.active_layer).is_some_and(|l| l.tabs.enabled && matches!(l.op, crate::layers::Operation::ProfileOutside | crate::layers::Operation::ProfileInside | crate::layers::Operation::OnPath))
    }

    /// The dots of the tabs and, over the outline, the place where one would be added; drawn while the tabs are being edited.
    pub(crate) fn paint_hand_tabs(&mut self, v: &CanvasView) {
        if !self.tab_edit || !self.hand_tabs_possible() {
            return;
        }
        let li = self.ed.active_layer;
        let layer = &self.layers[li];
        let accent = layer.color32();
        for s in shapes_of(&self.scene, li) {
            if let Some(poly) = outline(s) {
                let pts: Vec<Pos2> = poly.pts.iter().map(|p| self.w2s(v.origin, *p)).collect();
                v.painter.add(egui::Shape::closed_line(pts, Stroke::new(1.5, accent.gamma_multiply(0.6))));
            }
        }
        for i in 0..layer.tabs.manual.len() {
            if let Some(p) = place(&self.scene, layer, i) {
                let c = self.w2s(v.origin, p);
                v.painter.circle_filled(c, 5.5, Color32::from_rgb(0xff, 0xc0, 0x20));
                v.painter.circle_stroke(c, 5.5, Stroke::new(1.5, Color32::BLACK));
            }
        }
    }

    /// The pointer while tabs are edited: a click on the outline adds a tab, a click on a dot takes it away, dragging a dot moves it.
    /// True while the editing has the pointer.
    pub(crate) fn hand_tabs_pointer(&mut self, resp: &egui::Response, origin: Pos2, hover: Option<Pos2>) -> bool {
        if !self.tab_edit || !self.hand_tabs_possible() {
            return false;
        }
        let li = self.ed.active_layer;
        let zoom = self.ed.view.zoom.max(0.01) as f64;
        let tol = GRAB as f64 / zoom;
        let at_pointer = hover.map(|h| self.s2w(origin, h));
        if resp.drag_started() {
            if let Some(p) = at_pointer {
                if let Some(i) = pick(&self.scene, &self.layers[li], p, tol) {
                    self.hand_tab_drag.drag = Some((i, false));
                }
            }
        }
        if let (Some((i, _)), true, Some(p)) = (self.hand_tab_drag.drag, resp.dragged(), at_pointer) {
            let shape = self.layers[li].tabs.manual.get(i).map(|t| t.shape);
            if let Some(s) = shape.and_then(|id| self.scene.shape(id)) {
                if let Some((at, _)) = nearest(s, p) {
                    self.layers[li].tabs.manual[i].at = at;
                    self.hand_tab_drag.drag = Some((i, true));
                    self.touch_rev();
                }
            }
        }
        if resp.drag_stopped() {
            self.hand_tab_drag.drag = None;
        }
        if resp.clicked() {
            if let Some(p) = at_pointer {
                if let Some(i) = pick(&self.scene, &self.layers[li], p, tol) {
                    self.layers[li].tabs.manual.remove(i);
                    self.touch_rev();
                } else {
                    let scene = &self.scene;
                    if add(scene, &mut self.layers[li], li, p, tol) {
                        self.touch_rev();
                    }
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

    fn scene_with_square() -> (Scene, u64) {
        let mut scene = Scene::default();
        let id = scene.add(0, Kind::Rect { w: 40.0, h: 40.0 }, Xf::translate(10.0, 20.0));
        (scene, id)
    }

    #[test]
    fn a_tab_is_a_place_on_the_outline_that_follows_the_shape() {
        let (mut scene, id) = scene_with_square();
        let shape = scene.shape(id).unwrap();
        // A quarter of the way round a 40 mm square starting at its corner: the next corner.
        let q = point_on(shape, 0.25).unwrap();
        assert!((q.x - 50.0).abs() < 1e-9 && (q.y - 20.0).abs() < 1e-9, "{q:?}");
        let (at, d) = nearest(shape, Pt::new(30.0, 18.0)).unwrap();
        assert!((at - 0.125).abs() < 1e-9 && (d - 2.0).abs() < 1e-9, "{at} {d}");
        // Moved: the tab goes with the shape.
        scene.shapes[0].xf = Xf::translate(100.0, 100.0);
        let q = point_on(scene.shape(id).unwrap(), 0.25).unwrap();
        assert!((q.x - 140.0).abs() < 1e-9 && (q.y - 100.0).abs() < 1e-9);
    }

    #[test]
    fn tabs_are_added_picked_and_only_near_an_outline() {
        let (scene, _) = scene_with_square();
        let mut layer = crate::layers::default_layers(&crate::tools::Library::new()).remove(0);
        assert!(!add(&scene, &mut layer, 0, Pt::new(30.0, 40.0), 1.0), "the middle of the square is not on the outline");
        assert!(add(&scene, &mut layer, 0, Pt::new(30.0, 20.4), 1.0));
        assert!(add(&scene, &mut layer, 0, Pt::new(10.2, 40.0), 1.0));
        assert_eq!(layer.tabs.manual.len(), 2);
        assert_eq!(pick(&scene, &layer, Pt::new(30.0, 20.0), 1.5), Some(0));
        assert_eq!(pick(&scene, &layer, Pt::new(30.0, 30.0), 1.5), None);
        // Another layer has no shape of its own: nothing to add to.
        assert!(!add(&scene, &mut layer, 1, Pt::new(30.0, 20.0), 1.0));
    }

    #[test]
    fn tabs_placed_by_hand_are_the_only_ones_of_that_ring_and_are_saved_with_the_layer() {
        use crate::materials::Stock;
        use crate::toolpath::generate;
        let lib = crate::tools::Library::new();
        let dev = crate::device::Device::dummy();
        let mut layers = crate::layers::default_layers(&lib);
        layers[0].depth = 6.0;
        layers[0].step_down = 2.0;
        layers[0].tabs = crate::layers::Tabs { enabled: true, kind: crate::layers::TabKind::Partial, length: 6.0, height: 2.0, spacing: 20.0, min: 4, max: 8, ..Default::default() };
        let (scene, _) = scene_with_square();
        let raised = |layers: &[MillLayer]| -> usize {
            let prog = generate(&scene, layers, &lib, &dev, &Stock::default());
            prog.moves.iter().filter(|m| m.kind == crate::toolpath::MoveKind::Cut && (m.to[2] + 4.0).abs() < 1e-9 && m.from[2] < -4.0 - 1e-6).count()
        };
        let automatic = raised(&layers);
        assert!(automatic >= 4, "four tabs or more by spacing: {automatic}");
        assert!(add(&scene, &mut layers[0], 0, Pt::new(30.0, 20.0), 1.0));
        let by_hand = raised(&layers);
        assert_eq!(by_hand, 1, "one tab by hand, none of the automatic ones");
        // The tab goes through the project file with the layer.
        let json = serde_json::to_string(&layers[0]).unwrap();
        let back: MillLayer = serde_json::from_str(&json).unwrap();
        assert_eq!(back.tabs.manual, layers[0].tabs.manual);
        // An older layer has none.
        let mut v: serde_json::Value = serde_json::from_str(&json).unwrap();
        v["tabs"].as_object_mut().unwrap().remove("manual");
        let old: MillLayer = serde_json::from_value(v).unwrap();
        assert!(old.tabs.manual.is_empty());
    }
}
