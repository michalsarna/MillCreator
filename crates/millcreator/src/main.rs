//! MillCreator: an early skeleton. It opens a window with the shared theme and fonts of baseCreator, imports SVG and
//! PDF files into a scene and lets you draw and edit shapes on the work area with the shared editor (tools, selection,
//! node editing, the usual commands). There are no milling features yet.
use base_core::geom::Xf;
use base_core::scene::{Kind, Scene};
use base_core::units::Units;
use base_ui::dialogs::AboutInfo;
use base_ui::editor::{EditorCanvas, EditorCommands, EditorHost, EditorState, Tool};
use base_ui::menu::{key, std_entries, Entry, MenuHost, Menus, StdAct};
use base_ui::theme::{self, Scheme};
use eframe::egui;

const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
/// The size of the work area in millimetres until the machine is configurable.
const WORK_AREA: (f64, f64) = (300.0, 200.0);
/// How many steps of undo are kept.
const UNDO_STEPS: usize = 100;

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

/// Undo and redo of whole scenes.
#[derive(Default)]
struct History {
    undo: Vec<Scene>,
    redo: Vec<Scene>,
}

impl History {
    /// Remember `before`, the state before a change.
    fn push(&mut self, before: Scene) {
        self.undo.push(before);
        if self.undo.len() > UNDO_STEPS {
            self.undo.remove(0);
        }
        self.redo.clear();
    }
    /// The state to go back to, given the current one (which becomes the redo state).
    fn back(&mut self, now: &Scene) -> Option<Scene> {
        let s = self.undo.pop()?;
        self.redo.push(now.clone());
        Some(s)
    }
    fn forward(&mut self, now: &Scene) -> Option<Scene> {
        let s = self.redo.pop()?;
        self.undo.push(now.clone());
        Some(s)
    }
}

/// What the menus can do.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Act {
    Open,
    Quit,
    Std(StdAct),
    ViewOptions,
    SetScheme(Scheme),
    About,
}

impl From<StdAct> for Act {
    fn from(a: StdAct) -> Act {
        Act::Std(a)
    }
}

fn std(entries: Vec<Entry<StdAct>>) -> Vec<Entry<Act>> {
    entries.into_iter().map(|e| e.map(&Act::from)).collect()
}

fn menus() -> Menus<Act> {
    use Entry::{Item, Sep, Sub};
    vec![
        ("File", vec![Item(Act::Open, "Open SVG / PDF…", key("CmdOrCtrl+O", "Ctrl+O")), Sep, Item(Act::Quit, "Quit", key("CmdOrCtrl+Q", "Ctrl+Q"))]),
        ("Edit", std(std_entries::edit())),
        (
            "Arrange",
            [std(std_entries::align()), vec![Sep], std(std_entries::place()), vec![Sep], std(std_entries::group()), vec![Sep], std(std_entries::order()), vec![Sep], std(std_entries::nodes())].concat(),
        ),
        ("View", [std(std_entries::grid()), std(std_entries::zoom()), vec![Item(Act::ViewOptions, "View options…", None)]].concat()),
        ("Settings", vec![Sub("Colour scheme", Scheme::ALL.iter().map(|s| Item(Act::SetScheme(*s), s.label(), None)).collect())]),
        ("Help", vec![Item(Act::About, "About", None)]),
    ]
}

struct App {
    scene: Scene,
    ed: EditorState,
    history: History,
    scheme: Scheme,
    status: String,
    show_options: bool,
    show_about: bool,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> App {
        base_ui::fonts::install(&cc.egui_ctx);
        base_ui::icons::install(&cc.egui_ctx);
        let scheme = Scheme::ALL[0];
        theme::apply(&cc.egui_ctx, scheme);
        App { scene: Scene::default(), ed: EditorState::new(), history: History::default(), scheme, status: "Ready".into(), show_options: false, show_about: false }
    }

    fn open(&mut self, path: std::path::PathBuf) {
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        match std::fs::read(&path).map_err(|e| e.to_string()).and_then(|d| read_scene(&d)) {
            Ok(scene) => {
                self.status = format!("Opened {name}: {} shapes", scene.shapes.len());
                self.scene = scene;
                self.history = History::default();
                self.ed.sel.clear();
                self.ed.node_sel.clear();
                self.ed.view.need_fit = true;
            }
            Err(e) => self.status = format!("Could not open {name}: {e}"),
        }
    }

    fn undo(&mut self) {
        if let Some(s) = self.history.back(&self.scene) {
            self.scene = s;
            self.forget_missing();
        }
    }

    fn redo(&mut self) {
        if let Some(s) = self.history.forward(&self.scene) {
            self.scene = s;
            self.forget_missing();
        }
    }

    /// Drop selected shapes that the undo took away.
    fn forget_missing(&mut self) {
        let scene = &self.scene;
        self.ed.sel.retain(|id| scene.shape(*id).is_some());
        self.ed.node_sel.retain(|(id, _, _)| scene.shape(*id).is_some());
        self.ed.active_seg = None;
    }

    fn shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.egui_wants_keyboard_input() {
            return;
        }
        let (cmd, shift) = ctx.input(|i| (i.modifiers.command, i.modifiers.shift));
        let pressed = |k: egui::Key| ctx.input(|i| i.key_pressed(k));
        if cmd {
            if pressed(egui::Key::Z) {
                if shift {
                    self.redo();
                } else {
                    self.undo();
                }
            } else if pressed(egui::Key::Y) {
                self.redo();
            } else if pressed(egui::Key::A) {
                self.select_all();
            } else if pressed(egui::Key::C) {
                self.copy();
            } else if pressed(egui::Key::V) {
                self.paste();
            } else if pressed(egui::Key::D) {
                self.duplicate();
            } else if pressed(egui::Key::G) {
                if shift {
                    self.ungroup_selection();
                } else {
                    self.group_selection();
                }
            }
        } else if pressed(egui::Key::Delete) || pressed(egui::Key::Backspace) {
            if self.ed.tool == Tool::Node && !self.ed.node_sel.is_empty() {
                self.delete_nodes();
            } else {
                self.delete_selection();
            }
        } else if pressed(egui::Key::Escape) {
            self.ed.sel.clear();
            self.ed.node_sel.clear();
            self.ed.pen_pts.clear();
        } else {
            for (tool, _, key) in Tool::ALL {
                if pressed(key) {
                    self.ed.tool = tool;
                }
            }
        }
    }
}

impl EditorHost for App {
    fn ed(&self) -> &EditorState {
        &self.ed
    }
    fn ed_mut(&mut self) -> &mut EditorState {
        &mut self.ed
    }
    fn scene(&self) -> &Scene {
        &self.scene
    }
    fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
    fn layer_visible(&self, _layer: usize) -> bool {
        true
    }
    fn checkpoint(&mut self) {
        let before = self.scene.clone();
        self.history.push(before);
    }
    fn touch(&mut self) {}
    fn undo(&mut self) {
        App::undo(self)
    }
    fn redo(&mut self) {
        App::redo(self)
    }
    fn can_undo(&self) -> bool {
        !self.history.undo.is_empty()
    }
    fn can_redo(&self) -> bool {
        !self.history.redo.is_empty()
    }
    fn drop_checkpoint(&mut self) {
        self.history.undo.pop();
    }
    fn say(&mut self, msg: String) {
        self.status = msg;
    }
    fn said(&self) -> &str {
        &self.status
    }
    fn work_size(&self) -> (f64, f64) {
        WORK_AREA
    }
    fn units(&self) -> Units {
        Units::Mm
    }
    fn workspace_fill(&self) -> egui::Color32 {
        theme::workspace()
    }
    fn bed_fill(&self) -> egui::Color32 {
        if self.scheme.is_dark() {
            egui::Color32::from_rgb(0xea, 0xea, 0xed)
        } else {
            egui::Color32::WHITE
        }
    }
    fn shape_color(&self, layer: usize) -> Option<egui::Color32> {
        Some(self.layer_color(layer))
    }
    fn layer_color(&self, _layer: usize) -> egui::Color32 {
        egui::Color32::from_rgb(0x20, 0x20, 0x30)
    }
}

impl MenuHost for App {
    type Act = Act;
    fn menus(&self) -> Menus<Act> {
        menus()
    }
    fn act_enabled(&self, a: Act) -> bool {
        match a {
            Act::Std(s) => s.enabled(self),
            _ => true,
        }
    }
    fn act_checked(&self, a: Act) -> Option<bool> {
        match a {
            Act::Std(s) => s.checked(self),
            Act::SetScheme(s) => Some(self.scheme == s),
            _ => None,
        }
    }
    fn act_icon(&self, a: Act) -> Option<&'static str> {
        match a {
            Act::Std(s) => Some(s.icon()),
            Act::Open => Some("folder-open"),
            Act::Quit => Some("x"),
            Act::ViewOptions => Some("sliders-horizontal"),
            Act::About => Some("info"),
            Act::SetScheme(_) => None,
        }
    }
    fn run_act(&mut self, ctx: &egui::Context, a: Act) {
        match a {
            Act::Open => {
                if let Some(p) = rfd::FileDialog::new().add_filter("SVG / PDF / Illustrator", &["svg", "pdf", "ai"]).pick_file() {
                    self.open(p);
                }
            }
            Act::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            Act::Std(s) => s.run(self),
            Act::ViewOptions => self.show_options = true,
            Act::SetScheme(s) => {
                self.scheme = s;
                theme::apply(ctx, s);
            }
            Act::About => self.show_about = true,
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.shortcuts(&ctx);
        let chrome = egui::Frame::new().fill(theme::panel()).stroke(egui::Stroke::new(1.0, theme::border())).inner_margin(egui::Margin::symmetric(8, 4));
        egui::Panel::top("menu").frame(chrome).show(ui, |ui| base_ui::menu::menu_bar(self, ui));
        egui::Panel::bottom("status").frame(chrome).show(ui, |ui| {
            ui.label(format!("MillCreator {VERSION} (early skeleton, no milling features yet): {}", self.status));
        });
        egui::Panel::left("tools").frame(chrome).exact_size(52.0).resizable(false).show(ui, |ui| base_ui::editor::tool_palette(self, ui, None));
        egui::Panel::right("arrange_bar").frame(chrome.inner_margin(egui::Margin::symmetric(2, 4))).exact_size(40.0).resizable(false).show(ui, |ui| base_ui::editor::arrange_bar(self, ui));
        egui::CentralPanel::no_frame().show(ui, |ui| self.canvas(ui));
        let mut show = self.show_options;
        base_ui::editor::view_options_window(self, &ctx, &mut show);
        self.show_options = show;
        let about = AboutInfo {
            title: "About MillCreator",
            name: "MillCreator",
            version: VERSION,
            lines: vec!["Open-source CNC mill design and control software (early skeleton).".into(), "Written in Rust with egui; shares its core and interface with LightCreator through baseCreator.".into()],
            url: "https://github.com/michalsarna/MillCreator",
            licence: "MIT OR Apache-2.0",
        };
        base_ui::dialogs::about_window(&ctx, &mut self.show_about, &about, None);
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
    use base_core::geom::Pt;

    const SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><rect x="10" y="10" width="40" height="20" stroke="#ff0000" fill="none"/><circle cx="70" cy="70" r="10"/></svg>"##;

    fn app() -> App {
        App { scene: Scene::default(), ed: EditorState::new(), history: History::default(), scheme: Scheme::ALL[0], status: String::new(), show_options: false, show_about: false }
    }

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

    #[test]
    fn edits_can_be_undone_and_redone() {
        let mut a = app();
        a.scene = read_scene(SVG).unwrap();
        a.select_all();
        assert_eq!(a.ed.sel.len(), 2);
        a.delete_selection();
        assert!(a.scene.shapes.is_empty());
        a.undo();
        assert_eq!(a.scene.shapes.len(), 2);
        a.redo();
        assert!(a.scene.shapes.is_empty());
        a.undo();
        // A shape that the undo took away is no longer selected.
        a.ed.sel = vec![999];
        a.forget_missing();
        assert!(a.ed.sel.is_empty());
    }

    #[test]
    fn the_editor_draws_in_the_window_for_every_tool() {
        let mut a = app();
        a.scene = read_scene(SVG).unwrap();
        a.ed.sel = vec![1];
        let ctx = egui::Context::default();
        for (tool, _, _) in Tool::ALL {
            a.ed.tool = tool;
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                egui::CentralPanel::no_frame().show(ui, |ui| a.canvas(ui));
            });
            out.textures_delta.clear();
        }
        a.ed.pen_pts = vec![Pt::new(0.0, 0.0), Pt::new(10.0, 10.0)];
        a.finish_pen(false);
        assert_eq!(a.scene.shapes.len(), 3);
        assert!(!a.history.undo.is_empty(), "drawing is undoable");
    }
}
