//! MillCreator: an early skeleton. It opens a window with the shared theme and fonts of baseCreator, imports SVG and
//! PDF files into a scene and shows the outlines (pan with the mouse, zoom with the wheel). There are no milling
//! features yet.
use base_core::geom::{Pt, Xf};
use base_core::scene::{Kind, Scene};
use base_ui::theme::{self, Scheme};
use eframe::egui;

const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

/// Read an SVG or PDF (by its content) into a new scene. Every outline becomes one shape on layer 0.
fn read_scene(data: &[u8]) -> Result<Scene, String> {
    let is_pdf = data.windows(5).take(1024).any(|w| w == b"%PDF-");
    let outlines = if is_pdf { base_core::pdf::parse(data)? } else { base_core::svg::parse(data)? };
    let mut scene = Scene::default();
    for o in outlines {
        scene.add(0, Kind::Bezier(o.contours), Xf::IDENTITY);
    }
    Ok(scene)
}

struct App {
    scene: Scene,
    scheme: Scheme,
    zoom: f32,
    pan: egui::Vec2,
    status: String,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> App {
        base_ui::fonts::install(&cc.egui_ctx);
        let scheme = Scheme::ALL[0];
        theme::apply(&cc.egui_ctx, scheme);
        App { scene: Scene::default(), scheme, zoom: 3.0, pan: egui::vec2(40.0, 40.0), status: "Ready".into() }
    }

    fn open(&mut self, path: std::path::PathBuf) {
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        match std::fs::read(&path).map_err(|e| e.to_string()).and_then(|d| read_scene(&d)) {
            Ok(scene) => {
                self.status = format!("Opened {name}: {} shapes", scene.shapes.len());
                self.scene = scene;
            }
            Err(e) => self.status = format!("Could not open {name}: {e}"),
        }
    }

    fn canvas(&mut self, ui: &mut egui::Ui) {
        let (resp, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
        painter.rect_filled(resp.rect, 0.0, theme::workspace());
        if resp.dragged() {
            self.pan += resp.drag_delta();
        }
        if resp.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                let before = ui.input(|i| i.pointer.hover_pos()).unwrap_or(resp.rect.center()) - resp.rect.min - self.pan;
                let k = (1.0 + scroll * 0.002).clamp(0.5, 2.0);
                let zoom = (self.zoom * k).clamp(0.05, 200.0);
                self.pan -= before * (zoom / self.zoom - 1.0);
                self.zoom = zoom;
            }
        }
        let to_screen = |p: Pt| resp.rect.min + self.pan + egui::vec2(p.x as f32 * self.zoom, p.y as f32 * self.zoom);
        let stroke = egui::Stroke::new(1.5, theme::accent());
        for shape in &self.scene.shapes {
            for poly in shape.polys() {
                let mut pts: Vec<egui::Pos2> = poly.pts.iter().map(|p| to_screen(*p)).collect();
                if poly.closed {
                    if let Some(first) = pts.first().copied() {
                        pts.push(first);
                    }
                }
                painter.add(egui::Shape::line(pts, stroke));
            }
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        egui::Panel::top("menu").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open SVG / PDF…").clicked() {
                        ui.close();
                        if let Some(p) = rfd::FileDialog::new().add_filter("SVG / PDF / Illustrator", &["svg", "pdf", "ai"]).pick_file() {
                            self.open(p);
                        }
                    }
                });
                ui.menu_button("View", |ui| {
                    for s in Scheme::ALL {
                        if ui.selectable_label(self.scheme == s, s.label()).clicked() {
                            self.scheme = s;
                            theme::apply(&ctx, s);
                        }
                    }
                });
            });
        });
        egui::Panel::bottom("status").show(ui, |ui| {
            ui.label(format!("MillCreator {VERSION} (early skeleton, no milling features yet): {}", self.status));
        });
        egui::CentralPanel::no_frame().show(ui, |ui| self.canvas(ui));
    }
}

fn main() -> eframe::Result {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title(format!("MillCreator {VERSION}")).with_app_id("millcreator").with_inner_size([1100.0, 740.0]),
        ..Default::default()
    };
    eframe::run_native(
        "MillCreator",
        opts,
        Box::new(|cc| {
            let mut app = App::new(cc);
            if let Some(p) = std::env::args().nth(1) {
                app.open(p.into());
            }
            Ok(Box::new(app))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><rect x="10" y="10" width="40" height="20" stroke="#ff0000" fill="none"/><circle cx="70" cy="70" r="10"/></svg>"##;

    #[test]
    fn an_svg_becomes_one_shape_per_outline() {
        let scene = read_scene(SVG).unwrap();
        assert_eq!(scene.shapes.len(), 2);
        assert!(scene.shapes.iter().all(|s| s.bounds().is_some()));
    }

    #[test]
    fn nonsense_is_an_error_and_not_a_panic() {
        assert!(read_scene(b"not a picture").is_err());
        assert!(read_scene(b"%PDF-1.5 broken").is_err());
        assert!(read_scene(b"").is_err());
    }
}
