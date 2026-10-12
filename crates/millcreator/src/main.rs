//! MillCreator: an early skeleton. It looks like LightCreator (the window frame, the menus, the control and status bars,
//! the tool palette, the Properties tab, the colour schemes and the languages all come from baseCreator), imports SVG and
//! PDF files into a scene and lets you draw and edit shapes on the work area with the shared editor. There are no
//! milling features yet.
mod menu;
#[cfg(test)]
mod screenshots;
#[cfg(target_os = "macos")]
mod native_menu;

use base_core::geom::Xf;
use base_core::scene::{Kind, Scene};
use base_core::units::Units;
use base_ui::dialogs::AboutInfo;
use base_ui::editor::{EditorCanvas, EditorCommands, EditorHost, EditorState, Tool};
use base_ui::i18n::{self, tr, Lang};
use base_ui::menu::{MenuHost, Menus};
use base_ui::theme::{self, Scheme};
use base_ui::widgets::icon_button_ink;
use base_ui::window::{NEW_BLUE, OPEN_ORANGE};
use eframe::egui::{self, Color32, RichText};
use menu::{menus, Act, MAX_RECENT};
use std::path::PathBuf;

const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
/// The size of the work area in millimetres until the machine is configurable.
const WORK_AREA: (f64, f64) = (300.0, 200.0);
/// How many steps of undo are kept.
const UNDO_STEPS: usize = 100;
const ICON_256: &[u8] = include_bytes!("../../../assets/icons/icon-256.png");
const ICON_512: &[u8] = include_bytes!("../../../assets/icons/icon-512.png");

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

/// The icon of each action in the menus.
pub fn act_icon(a: Act) -> Option<&'static str> {
    match a {
        Act::Std(s) => Some(s.icon()),
        Act::New => Some("file-plus"),
        Act::Open => Some("folder-open"),
        Act::OpenRecent(_) | Act::ClearRecent => None,
        Act::Quit => Some("x"),
        Act::ViewOptions => Some("sliders-horizontal"),
        Act::OpenGithub => Some("github"),
        Act::About => Some("info"),
        Act::SetLang(_) | Act::SetScheme(_) => None,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Screen {
    Start,
    Editor,
}

struct App {
    scene: Scene,
    ed: EditorState,
    history: History,
    scheme: Scheme,
    lang: Lang,
    status: String,
    screen: Screen,
    path: Option<PathBuf>,
    recent: Vec<PathBuf>,
    logo: Option<egui::TextureHandle>,
    show_options: bool,
    show_about: bool,
    #[cfg(target_os = "macos")]
    native_menu: Option<native_menu::NativeMenu>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> App {
        App::build(&cc.egui_ctx, cc.storage, cfg!(target_os = "macos"))
    }

    /// Build the app without a window: used by `new` and by the tests.
    fn build(ctx: &egui::Context, storage: Option<&dyn eframe::Storage>, native_menu: bool) -> App {
        static TEXTS: std::sync::Once = std::sync::Once::new();
        TEXTS.call_once(|| i18n::register(&base_ui::texts::TEXTS));
        let saved = |k: &str| storage.and_then(|st| st.get_string(k));
        let lang = saved("lang").and_then(|c| Lang::from_code(&c)).unwrap_or(Lang::En);
        let scheme = saved("scheme").and_then(|c| Scheme::from_id(&c)).unwrap_or(Scheme::LightDark);
        i18n::set_lang(lang);
        base_ui::fonts::install(ctx);
        base_ui::icons::install(ctx);
        theme::apply(ctx, scheme);
        let on = |k: &str, default: bool| saved(k).map(|v| v == "1").unwrap_or(default);
        let ed = EditorState { show_grid: on("grid_main_on", true), snap: on("snap", false), snap_objects: on("snap_objects", false), ..EditorState::new() };
        let recent = saved("recent_files").map(|s| s.lines().map(PathBuf::from).take(MAX_RECENT).collect()).unwrap_or_default();
        #[cfg(not(target_os = "macos"))]
        let _ = native_menu;
        App {
            scene: Scene::default(),
            ed,
            history: History::default(),
            scheme,
            lang,
            status: tr("Ready").into(),
            screen: Screen::Start,
            path: None,
            recent,
            logo: base_ui::start::load_logo(ctx, ICON_256, "millcreator-logo"),
            show_options: false,
            show_about: false,
            #[cfg(target_os = "macos")]
            native_menu: native_menu.then(|| native_menu::install(ctx)),
        }
    }

    /// What is kept between runs.
    fn settings_pairs(&self) -> Vec<(&'static str, String)> {
        let flag = |b: bool| if b { "1" } else { "0" }.to_string();
        vec![
            ("lang", self.lang.code().to_owned()),
            ("scheme", self.scheme.id().to_owned()),
            ("grid_main_on", flag(self.ed.show_grid)),
            ("snap", flag(self.ed.snap)),
            ("snap_objects", flag(self.ed.snap_objects)),
            ("recent_files", self.recent.iter().map(|p| p.to_string_lossy().into_owned()).collect::<Vec<_>>().join("\n")),
        ]
    }

    fn add_recent(&mut self, p: &std::path::Path) {
        let p = std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
        self.recent.retain(|r| *r != p);
        self.recent.insert(0, p);
        self.recent.truncate(MAX_RECENT);
    }

    /// A new, empty design.
    fn new_design(&mut self) {
        self.scene = Scene::default();
        self.history = History::default();
        self.path = None;
        self.ed.sel.clear();
        self.ed.node_sel.clear();
        self.ed.view.need_fit = true;
        self.screen = Screen::Editor;
        self.status = tr("Ready").into();
    }

    fn open(&mut self, path: PathBuf) {
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        match std::fs::read(&path).map_err(|e| e.to_string()).and_then(|d| read_scene(&d)) {
            Ok(scene) => {
                self.status = format!("Opened {name}: {} shapes", scene.shapes.len());
                self.scene = scene;
                self.history = History::default();
                self.ed.sel.clear();
                self.ed.node_sel.clear();
                self.ed.view.need_fit = true;
                self.screen = Screen::Editor;
                self.add_recent(&path);
                self.path = Some(path);
            }
            Err(e) => self.status = format!("Could not open {name}: {e}"),
        }
    }

    fn open_dialog(&mut self) {
        if let Some(p) = rfd::FileDialog::new().add_filter("SVG / PDF / Illustrator", &["svg", "pdf", "ai"]).pick_file() {
            self.open(p);
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
            #[cfg(target_os = "macos")]
            let native = self.native_menu.is_some();
            #[cfg(not(target_os = "macos"))]
            let native = false;
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
            } else if !native && pressed(egui::Key::N) {
                self.new_design();
            } else if !native && pressed(egui::Key::O) {
                self.open_dialog();
            } else if !native && pressed(egui::Key::Q) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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

    #[cfg(target_os = "macos")]
    fn native_menu_frame(&mut self, ctx: &egui::Context) {
        let Some(mut nm) = self.native_menu.take() else { return };
        for a in nm.take_actions() {
            self.run_act(ctx, a);
        }
        nm.sync(|a| self.act_checked(a), |a| self.act_enabled(a), &self.recent);
        self.native_menu = Some(nm);
    }

    /// A file dropped on the window is opened.
    fn handle_drops(&mut self, ctx: &egui::Context) {
        let files: Vec<PathBuf> = ctx.input(|i| i.raw.dropped_files.iter().map(|f| f.path().to_path_buf()).collect());
        if let Some(p) = files.into_iter().next() {
            self.open(p);
        }
    }

    /// The bar across the top of the window.
    fn title_bar(&mut self, ui: &mut egui::Ui) {
        let mut title = format!("MillCreator {VERSION}");
        if let Some(name) = self.path.as_ref().and_then(|p| p.file_name()) {
            title.push_str(&format!("  —  {}", name.to_string_lossy()));
        }
        if base_ui::window::title_bar(ui, &title, self.logo.as_ref()) {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    /// The bar under the menu: the tool, new and open, the grid, and the size of the selection.
    fn control_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            base_ui::editor::control_name(self, ui);
            ui.separator();
            let ctx = ui.ctx().clone();
            for (act, icon, tip, ink) in [(Act::New, "file-plus", "New", NEW_BLUE), (Act::Open, "folder-open", "Open…", OPEN_ORANGE)] {
                if icon_button_ink(ui, icon, &tr(tip), true, 28.0, ink) {
                    self.run_act(&ctx, act);
                }
            }
            ui.separator();
            base_ui::editor::grid_toggles(self, ui);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| base_ui::editor::selection_size(self, ui));
        });
    }

    fn status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.label(&self.status);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                base_ui::editor::zoom_and_cursor(self, ui);
                ui.separator();
                ui.label(RichText::new(tr("Early skeleton: no milling features yet")).color(theme::text_dim()));
            });
        });
    }

    /// The tabs of the right panel. Only the properties of the selection so far.
    fn side_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.add(egui::Button::selectable(true, RichText::new(tr("Properties")).strong()));
        });
        ui.separator();
        egui::ScrollArea::vertical().id_salt("tab_props").auto_shrink([false, false]).show(ui, |ui| {
            if !base_ui::editor::properties_sections(self, ui) {
                ui.label(RichText::new(tr("Nothing selected")).color(theme::text_dim()));
            }
        });
    }

    fn start_screen(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                base_ui::start::start_header(ui, self.logo.as_ref(), "MillCreator", VERSION);
                ui.add_space(18.0);
                ui.label(RichText::new(tr("Start")).size(16.0).strong());
                ui.label(RichText::new(tr("Open a drawing or start an empty design.")).color(theme::text_dim()));
                ui.add_space(8.0);
                let frame = egui::Frame::group(ui.style()).inner_margin(egui::Margin::same(12));
                frame.show(ui, |ui| {
                    ui.set_width(460.0);
                    let new = egui::Button::image_and_text(base_ui::icons::slot(Some("file-plus"), Color32::WHITE), RichText::new(tr("New")).color(Color32::WHITE).strong()).fill(theme::accent());
                    if ui.add_sized([ui.available_width(), 30.0], new).clicked() {
                        self.run_act(&ctx, Act::New);
                    }
                    ui.add_space(4.0);
                    let open = egui::Button::image_and_text(base_ui::icons::slot(Some("folder-open"), theme::text()), tr("Open SVG / PDF…"));
                    if ui.add_sized([ui.available_width(), 30.0], open).clicked() {
                        self.run_act(&ctx, Act::Open);
                    }
                    if !self.recent.is_empty() {
                        ui.add_space(8.0);
                        ui.label(RichText::new(tr("Open recent")).color(theme::text_dim()));
                        let mut chosen = None;
                        for (i, p) in self.recent.iter().enumerate().take(5) {
                            let name = p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                            if ui.add_sized([ui.available_width(), 24.0], egui::Button::new(name)).on_hover_text(p.display().to_string()).clicked() {
                                chosen = Some(i as u8);
                            }
                        }
                        if let Some(i) = chosen {
                            self.run_act(&ctx, Act::OpenRecent(i));
                        }
                    }
                });
                ui.add_space(14.0);
                let (l, s) = base_ui::start::start_prefs(ui, self.lang, self.scheme);
                if let Some(l) = l {
                    self.run_act(&ctx, Act::SetLang(l));
                }
                if let Some(s) = s {
                    self.run_act(&ctx, Act::SetScheme(s));
                }
            });
        });
    }

    fn dialogs(&mut self, ctx: &egui::Context) {
        let mut show = self.show_options;
        base_ui::editor::view_options_window(self, ctx, &mut show);
        self.show_options = show;
        let about = AboutInfo {
            title: "About MillCreator",
            name: "MillCreator",
            version: VERSION,
            lines: vec!["Open-source CNC mill design and control software (early skeleton).".into(), "Written in Rust with egui; shares its core and interface with LightCreator through baseCreator.".into()],
            url: "https://github.com/michalsarna/MillCreator",
            licence: "MIT OR Apache-2.0",
        };
        base_ui::dialogs::about_window(ctx, &mut self.show_about, &about, self.logo.as_ref());
    }

    fn draw(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        #[cfg(target_os = "macos")]
        self.native_menu_frame(&ctx);
        self.title_bar(ui);
        base_ui::window::window_edges(&ctx);
        #[cfg(target_os = "macos")]
        base_ui::window::window_edge_drag(&ctx);
        self.handle_drops(&ctx);
        if self.screen == Screen::Start {
            self.start_screen(ui);
            self.dialogs(&ctx);
            return;
        }
        self.shortcuts(&ctx);
        let chrome = egui::Frame::new().fill(theme::panel()).stroke(egui::Stroke::new(1.0, theme::border())).inner_margin(egui::Margin::symmetric(8, 4));
        #[cfg(target_os = "macos")]
        let in_window_menu = self.native_menu.is_none();
        #[cfg(not(target_os = "macos"))]
        let in_window_menu = true;
        if in_window_menu {
            egui::Panel::top("menu").frame(chrome).show(ui, |ui| base_ui::menu::menu_bar(self, ui));
        }
        egui::Panel::top("controls").frame(chrome).exact_size(36.0).show(ui, |ui| self.control_bar(ui));
        egui::Panel::bottom("status").frame(chrome).exact_size(26.0).show(ui, |ui| self.status_bar(ui));
        egui::Panel::left("tools").frame(chrome).exact_size(52.0).resizable(false).show(ui, |ui| base_ui::editor::tool_palette(self, ui, None));
        egui::Panel::right("side").frame(chrome.inner_margin(egui::Margin::same(8))).default_size(410.0).show(ui, |ui| self.side_panel(ui));
        egui::Panel::right("arrange_bar").frame(chrome.inner_margin(egui::Margin::symmetric(2, 4))).exact_size(40.0).resizable(false).show(ui, |ui| base_ui::editor::arrange_bar(self, ui));
        egui::CentralPanel::no_frame().show(ui, |ui| self.canvas(ui));
        self.dialogs(&ctx);
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
            Act::Std(s) => self.screen == Screen::Editor && s.enabled(self),
            _ => true,
        }
    }
    fn act_checked(&self, a: Act) -> Option<bool> {
        match a {
            Act::Std(s) => s.checked(self),
            Act::SetLang(l) => Some(self.lang == l),
            Act::SetScheme(s) => Some(self.scheme == s),
            _ => None,
        }
    }
    fn act_icon(&self, a: Act) -> Option<&'static str> {
        act_icon(a)
    }
    fn recent_files(&self) -> Vec<PathBuf> {
        self.recent.clone()
    }
    fn open_recent_act(&self, i: usize) -> Option<Act> {
        (i < self.recent.len()).then_some(Act::OpenRecent(i as u8))
    }
    fn clear_recent_act(&self) -> Option<Act> {
        Some(Act::ClearRecent)
    }
    fn run_act(&mut self, ctx: &egui::Context, a: Act) {
        match a {
            Act::New => self.new_design(),
            Act::Open => self.open_dialog(),
            Act::OpenRecent(i) => {
                if let Some(p) = self.recent.get(i as usize).cloned() {
                    self.open(p);
                }
            }
            Act::ClearRecent => self.recent.clear(),
            Act::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            Act::Std(s) => {
                if self.screen == Screen::Editor {
                    s.run(self)
                }
            }
            Act::ViewOptions => self.show_options = true,
            Act::SetLang(l) => {
                self.lang = l;
                i18n::set_lang(l);
            }
            Act::SetScheme(s) => {
                self.scheme = s;
                theme::apply(ctx, s);
            }
            Act::OpenGithub => ctx.open_url(egui::OpenUrl::new_tab("https://github.com/michalsarna/MillCreator")),
            Act::About => self.show_about = true,
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        for (k, v) in self.settings_pairs() {
            storage.set_string(k, v);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }
}

fn main() -> eframe::Result {
    let mut viewport = egui::ViewportBuilder::default()
        .with_title(format!("MillCreator {VERSION}"))
        .with_app_id("millcreator")
        .with_inner_size([1360.0, 860.0])
        .with_min_inner_size([900.0, 560.0])
        // The application draws its own flat, square frame, as LightCreator does.
        .with_decorations(false);
    if let Some(icon) = base_ui::start::window_icon(ICON_512) {
        viewport = viewport.with_icon(icon);
    }
    let opts = eframe::NativeOptions { viewport, ..Default::default() };
    eframe::run_native(
        "MillCreator",
        opts,
        Box::new(|cc| {
            #[cfg(target_os = "macos")]
            native_menu::set_dock_icon(ICON_512);
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
        let mut a = App::build(&egui::Context::default(), None, false);
        a.screen = Screen::Editor;
        a
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
