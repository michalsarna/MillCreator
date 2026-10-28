//! MillCreator: design and prepare CNC milling jobs. It looks like LightCreator (the window frame, the menus, the
//! control and status bars, the tool palette, the Properties tab, the colour schemes and the languages come from
//! baseCreator). It adds devices (with cameras), milling layers, a library of bits, a simple material, toolpaths with a
//! 3D view and G-code.
mod autosave_glue;
mod backup_ui;
mod device;
mod device_ui;
mod layers;
mod layers_ui;
mod machine_ui;
mod materials;
mod gcode_view_glue;
#[cfg(test)]
mod grbl_sim_tests;
mod menu;
mod plugins_glue;
mod preview_ui;
mod relief;
mod section;
mod toolpath;
mod ugs_import;
mod update_glue;
mod view3d;
mod workspaces;
mod texts;
mod warn;
mod tools;
mod tools_ui;
#[cfg(test)]
mod screenshots;
#[cfg(target_os = "macos")]
mod native_menu;

use base_core::geom::Xf;
use base_core::project::{self, Project};
use base_core::scene::{Kind, Scene};
use base_core::units::Units;
use base_ui::clipart_ui::{self, ClipGallery, ClipHost, ClipState};
use base_ui::control_ui::{self, ControlCtx, ControlUi};
use base_ui::dialogs::AboutInfo;
use base_ui::job_ui::StartFrom;
use base_ui::tls_ui::TlsUi;
use base_ui::tools_ui::{CalUi, DiagUi};
use base_ui::editor::{EditorCanvas, EditorCommands, EditorHost, EditorImages, EditorState, Tool};
use base_ui::i18n::{self, tr, Lang};
use base_ui::menu::{MenuHost, Menus};
use base_ui::theme::{self, Scheme};
use base_ui::widgets::icon_button_ink;
use device::Device;
use device_ui::{CamState, DeviceEdit};
use layers::MillLayer;
use machine_ui::MachineLink;
use materials::Stock;
use toolpath::Program;
use tools::Library;
use tools_ui::ToolsWindow;
use view3d::View3d;
use base_ui::window::{NEW_BLUE, OPEN_ORANGE, SAVE_GREEN};
use eframe::egui::{self, Color32, RichText};
use menu::{menus, Act, MAX_RECENT};
use std::path::PathBuf;

const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
/// How many steps of undo are kept.
const UNDO_STEPS: usize = 100;
const ICON_256: &[u8] = include_bytes!("../../../assets/icons/icon-256.png");
const ICON_512: &[u8] = include_bytes!("../../../assets/icons/icon-512.png");

// Easter egg: around a holiday the program wears a festive icon (see `base_ui::seasonal`).
const FESTIVE: base_ui::seasonal::Variants = base_ui::seasonal::Variants {
    christmas: (include_bytes!("../../../assets/icons/seasonal/christmas-256.png"), include_bytes!("../../../assets/icons/seasonal/christmas-512.png")),
    new_years_eve: (include_bytes!("../../../assets/icons/seasonal/newyear-256.png"), include_bytes!("../../../assets/icons/seasonal/newyear-512.png")),
    halloween: (include_bytes!("../../../assets/icons/seasonal/halloween-256.png"), include_bytes!("../../../assets/icons/seasonal/halloween-512.png")),
    easter: (include_bytes!("../../../assets/icons/seasonal/easter-256.png"), include_bytes!("../../../assets/icons/seasonal/easter-512.png")),
};

/// The 256 px and 512 px logo of the day; the tests (and so the screenshots) always get the normal one.
fn app_icon() -> base_ui::seasonal::Icon {
    let normal = (ICON_256, ICON_512);
    if cfg!(test) { normal } else { base_ui::seasonal::icon_today(&FESTIVE, normal) }
}

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
        Act::Save | Act::SaveAs => Some("save"),
        Act::ImportLightburn | Act::ImportUgsd | Act::ImportPlugin => Some("file-input"),
        Act::ViewGcode => Some("eye"),
        Act::Plugins => Some("package-open"),
        Act::GenerateShape => Some("shapes"),
        Act::ToolLibrary => Some("package"),
        Act::DeviceSettings => Some("settings"),
        Act::Firmware => Some("settings"),
        Act::ToolSensor => Some("target"),
        Act::Calibration => Some("sliders-horizontal"),
        Act::Diagnostics => Some("zap"),
        Act::StartFrom => Some("play"),
        Act::CheckProgram => Some("check"),
        Act::Preview3d => Some("eye"),
        Act::ExportGcode => Some("file-code"),
        Act::ExportRelief => Some("file-output"),
        Act::OpenRecent(_) | Act::ClearRecent => None,
        Act::Quit => Some("x"),
        Act::ViewOptions => Some("sliders-horizontal"),
        Act::ProgramOptions => Some("settings"),
        Act::CheckUpdates => Some("download"),
        Act::DownloadBackup => Some("package"),
        Act::RestoreBackup => Some("package-open"),
        Act::OpenGithub => Some("github"),
        Act::About => Some("info"),
        Act::SetLang(_) | Act::SetScheme(_) => None,
    }
}

/// The tabs of the side panel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SideTab {
    Properties,
    Layers,
    Material,
    Device,
    Control,
    Console,
    Clipart,
    Description,
    Camera,
}

impl SideTab {
    /// The tabs that are always there, in their first order (the camera comes last and only when the device has one).
    const IDS: [&'static str; 8] = ["properties", "layers", "material", "device", "control", "console", "clipart", "description"];

    fn id(self) -> &'static str {
        match self {
            SideTab::Properties => "properties",
            SideTab::Layers => "layers",
            SideTab::Material => "material",
            SideTab::Device => "device",
            SideTab::Control => "control",
            SideTab::Console => "console",
            SideTab::Clipart => "clipart",
            SideTab::Description => "description",
            SideTab::Camera => "camera",
        }
    }

    fn from_id(id: &str) -> Option<SideTab> {
        [SideTab::Properties, SideTab::Layers, SideTab::Material, SideTab::Device, SideTab::Control, SideTab::Console, SideTab::Clipart, SideTab::Description, SideTab::Camera].into_iter().find(|t| t.id() == id)
    }

    fn label(self) -> String {
        match self {
            SideTab::Properties => tr("Properties").to_string(),
            SideTab::Layers => tr("Layers").to_string(),
            SideTab::Material => tr("Material").to_string(),
            SideTab::Device => tr("Device").to_string(),
            SideTab::Control => tr("Control").to_string(),
            SideTab::Console => tr("Console").to_string(),
            SideTab::Clipart => clipart_ui::tab_label().to_string(),
            SideTab::Description => base_ui::description_ui::tab_label().to_string(),
            SideTab::Camera => tr("Camera").to_string(),
        }
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
    side_tab: SideTab,
    /// The order of the tabs of the side panel and whether it is collapsed.
    tabs: base_ui::side_tabs::SideTabs,
    clip: ClipState,
    /// The free text of the project, kept for the next save.
    description: String,
    /// The parts of the project file that are not ours (laser, pen...), kept as they were read and written back.
    kept: Project,
    /// The devices that are kept between runs, the one the project is made for, and which of the list it is.
    devices: Vec<Device>,
    device: Device,
    active: Option<usize>,
    start_sel: usize,
    dev_edit: Option<DeviceEdit>,
    confirm_delete: bool,
    /// The bits (ready-made and the user's own, which are kept for every device) and the window of the library.
    tools: Library,
    tools_win: ToolsWindow,
    layers: Vec<MillLayer>,
    stock: Stock,
    view: View3d,
    show_3d: bool,
    prog: Option<Program>,
    /// Counts the changes of the project; the program is made again when `job_stamp` differs from `prog_rev`.
    rev: u64,
    /// The layers that hold pictures and were switched to the Picture operation already (so that a person who changes the
    /// operation of such a layer is not overruled).
    picture_layers: std::collections::HashSet<usize>,
    /// The list of layers shows all of them (not only those with shapes).
    show_all_layers: bool,
    autosave: base_core::autosave::Timer,
    /// The revision at the last save by hand, open or new; `rev` differing from it is unsaved work (see `is_dirty`).
    saved_rev: u64,
    /// The tabs of open drawings (the shown one is the live fields above), and the number of the recovery file of the shown one.
    tabs_state: workspaces::Tabs,
    slot: usize,
    /// The program is being closed: every tab with unsaved work is asked about in turn.
    quitting: bool,
    /// The last tab was closed: the program closes too.
    close_program: bool,
    allow_close: bool,
    /// When the tabs that are not shown were last written.
    parked_saved_at: std::time::Instant,
    /// An unsaved project left by an earlier run (offered on the start screen).
    recovery: Vec<base_core::autosave::Found>,
    prog_rev: u64,
    /// Options of a job, in the Control tab (this session only): cut the elements one after the other, and cut only what is
    /// selected when the G-code is saved or the job started.
    one_by_one: bool,
    selection_only: bool,
    cam: Option<CamState>,
    /// Was the camera picture drawn in the last frame? A stream nobody looks at is stopped (its frames would pile up).
    cam_drawn: bool,
    /// The camera window shrinks to the picture.
    cam_fit: base_ui::window::FitWindow,
    machine: MachineLink,
    control_ui: ControlUi,
    fw_open: bool,
    tls_ui: TlsUi,
    cal: CalUi,
    diag: DiagUi,
    diag_gcode: Option<String>,
    plugins: base_ui::plugins_ui::PluginsUi,
    /// What the postprocessor of the last export said, for the status bar.
    post_note: Option<String>,
    /// The G-code file shown in place of the work area (File > View G-code file…).
    gcode_view: Option<base_ui::gcode_view_ui::GcodeViewer>,
    start_from: StartFrom,
    start_lines: Vec<String>,
    /// A job that waits for the check run to find nothing wrong.
    after_check: Option<machine_ui::AfterCheck>,
    playback: base_ui::playback::Playback,
    /// The toolpaths are drawn over the work area.
    toolpaths_on: bool,
    show_camera: bool,
    /// The camera view is a tab of the work area, and that tab is the one shown (in place of the drawing).
    cam_ws_shown: bool,
    show_options: bool,
    /// When the last Escape was pressed (a second one within half a second selects the select tool).
    esc_time: f64,
    /// Look for a newer release when the program starts.
    check_updates: bool,
    update: base_ui::dialogs::UpdateState,
    show_program_options: bool,
    show_about: bool,
    #[cfg(target_os = "macos")]
    native_menu: Option<native_menu::NativeMenu>,
}

/// A file that is imported into the drawing: a LightBurn project or a UGS design.
fn is_import(p: &std::path::Path) -> bool {
    p.extension().is_some_and(|e| ["lbrn2", "lbrn", "ugsd"].iter().any(|x| e.eq_ignore_ascii_case(x)))
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> App {
        let mut app = App::build(&cc.egui_ctx, cc.storage, cfg!(target_os = "macos"));
        if app.check_updates {
            app.update.start(&cc.egui_ctx, &update_glue::CONFIG, false);
        }
        app
    }

    /// Build the app without a window: used by `new` and by the tests.
    fn build(ctx: &egui::Context, storage: Option<&dyn eframe::Storage>, native_menu: bool) -> App {
        static TEXTS: std::sync::Once = std::sync::Once::new();
        TEXTS.call_once(|| i18n::register(&base_ui::texts::TEXTS));
        texts::register();
        let saved = |k: &str| storage.and_then(|st| st.get_string(k));
        let lang = saved("lang").and_then(|c| Lang::from_code(&c)).unwrap_or(Lang::En);
        let scheme = Scheme::from_saved(saved("scheme").as_deref());
        i18n::set_lang(lang);
        base_ui::fonts::install(ctx);
        base_ui::icons::install(ctx);
        theme::apply(ctx, scheme);
        let on = |k: &str, default: bool| saved(k).map(|v| v == "1").unwrap_or(default);
        let ed = EditorState { show_grid: on("grid_main_on", true), snap: on("snap", false), snap_objects: on("snap_objects", false), ..EditorState::new() };
        let mut tools = Library::new();
        if let Some(t) = saved("tools") {
            tools.load_text(&t);
        }
        let devices = saved("devices").map(|t| device::load_all(&t)).unwrap_or_default();
        let recent = saved("recent_files").map(|s| s.lines().map(PathBuf::from).take(MAX_RECENT).collect()).unwrap_or_default();
        #[cfg(not(target_os = "macos"))]
        let _ = native_menu;
        let mut app = App {
            scene: Scene::default(),
            ed,
            history: History::default(),
            scheme,
            lang,
            status: tr("Ready").into(),
            screen: Screen::Start,
            path: None,
            recent,
            logo: base_ui::start::load_logo(ctx, app_icon().0, "millcreator-logo"),
            side_tab: SideTab::Properties,
            tabs: base_ui::side_tabs::SideTabs::new(&SideTab::IDS, &saved("tab_order").unwrap_or_default(), saved("side_collapsed").is_some_and(|v| v == "1")),
            clip: ClipState::new(base_core::clipart::Library::open(data_paths().gallery.clone())),
            description: String::new(),
            kept: Project::default(),
            device: devices.first().cloned().unwrap_or_else(Device::dummy),
            active: (!devices.is_empty()).then_some(0),
            devices,
            start_sel: 0,
            dev_edit: None,
            confirm_delete: false,
            layers: layers::default_layers(&tools),
            tools,
            tools_win: ToolsWindow::default(),
            stock: Stock::default(),
            view: View3d::default(),
            show_3d: false,
            prog: None,
            rev: 0,
            picture_layers: Default::default(),
            show_all_layers: false,
            autosave: Default::default(),
            saved_rev: 0,
            tabs_state: Default::default(),
            slot: 0,
            quitting: false,
            close_program: false,
            allow_close: false,
            parked_saved_at: std::time::Instant::now(),
            recovery: base_core::autosave::find_all(&data_paths().base, "millcreator"),
            prog_rev: 0,
            one_by_one: false,
            selection_only: false,
            cam: None,
            cam_drawn: false,
            cam_fit: Default::default(),
            machine: {
                let mut m = MachineLink::new(ctx);
                m.fw.backup_dir = data_paths().settings.parent().map(|p| p.join("firmware-backups"));
                m
            },
            control_ui: ControlUi::default(),
            fw_open: false,
            tls_ui: TlsUi::default(),
            cal: CalUi::default(),
            diag: DiagUi::default(),
            diag_gcode: None,
            plugins: base_ui::plugins_ui::PluginsUi::new("millcreator", &data_paths().base),
            post_note: None,
            gcode_view: None,
            start_from: StartFrom::default(),
            start_lines: vec![],
            after_check: None,
            playback: base_ui::playback::Playback::default(),
            toolpaths_on: false,
            show_camera: false,
            cam_ws_shown: false,
            show_options: false,
            esc_time: -10.0,
            check_updates: on("check_updates", true),
            update: Default::default(),
            show_program_options: false,
            show_about: false,
            #[cfg(target_os = "macos")]
            native_menu: native_menu.then(|| native_menu::install(ctx)),
        };
        app.after_plugins_changed();
        app
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
            ("devices", self.devices_text()),
            ("tools", self.tools.save_text()),
            ("check_updates", flag(self.check_updates)),
            ("tab_order", self.tabs.order_string()),
            ("side_collapsed", flag(self.tabs.collapsed)),
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
        if self.screen == Screen::Start {
            self.take_selected_device();
        }
        self.scene = Scene::default();
        self.history = History::default();
        self.path = None;
        self.description.clear();
        self.kept = Project::default();
        self.layers = layers::default_layers(&self.tools);
        self.stock = Stock::default();
        self.touch_rev();
        self.reset_picture_layers();
        self.autosave_in_sync();
        self.ed.sel.clear();
        self.ed.node_sel.clear();
        self.ed.view.need_fit = true;
        self.screen = Screen::Editor;
        self.status = tr("Ready").into();
    }

    fn open(&mut self, path: PathBuf) {
        // A picture goes on the page of the drawing (the import dialog adjusts it, and it can be traced into outlines).
        if base_ui::editor::is_image_file(&path) {
            if self.screen == Screen::Start {
                self.new_design();
            }
            return self.open_image_dialog(path);
        }
        // A LightBurn project or a UGS design is imported into the drawing, not opened.
        if is_import(&path) {
            return self.import_file(path);
        }
        // A G-code file is shown, not opened (unless a plugin reads that extension).
        if base_ui::gcode_view_ui::is_gcode(&path) && !self.plugin_reads(&path) {
            return self.open_gcode(path);
        }
        if self.try_plugin_import(&path) {
            return;
        }
        if self.screen == Screen::Start {
            self.take_selected_device();
        }
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let is_project = path.extension().is_some_and(|e| e.eq_ignore_ascii_case(project::EXTENSION));
        let read = std::fs::read(&path).map_err(|e| e.to_string()).and_then(|d| {
            if is_project {
                let text = String::from_utf8(d).map_err(|_| "this is not a project file".to_string())?;
                Project::from_json(&text).map(|p| (p.scene.clone(), Some(p)))
            } else {
                read_scene(&d).map(|s| (s, None))
            }
        });
        match read {
            Ok((scene, file)) => {
                // A drawing goes into a tab of its own (a blank tab is used as it is); a project that is open is shown.
                if !self.tab_for_file(&path, is_project) {
                    return;
                }
                // The milling part of the project: layers, material and the bits it uses (they join the library).
                let loaded = match &file {
                    Some(p) => section::load(p, &mut self.tools),
                    None => section::load(&Project::default(), &mut self.tools),
                };
                let loaded = match loaded {
                    Ok(l) => l,
                    Err(e) => {
                        self.status = format!("Could not open {name}: {e}");
                        return;
                    }
                };
                self.layers = loaded.layers;
                self.stock = loaded.stock;
                self.touch_rev();
                // Sections of other programs wait here until the next save; a drawing from SVG or PDF has none.
                self.description = file.as_ref().map(|p| p.description.clone()).unwrap_or_default();
                self.kept = file.unwrap_or_default();
                self.kept.scene = Scene::default();
                self.status = format!("Opened {name}: {} shapes", scene.shapes.len());
                if let Some(m) = loaded.machine.as_ref().and_then(|m| m.differences(&self.device)) {
                    self.status.push_str(&format!(" — {}", tr("this project was made for a different machine: {}").replace("{}", &m)));
                }
                if !loaded.added_tools.is_empty() {
                    self.status.push_str(&format!(" — {}: {}", tr("bits added to the library"), loaded.added_tools.join(", ")));
                    self.tools_win.changed = true;
                }
                self.scene = scene;
                self.reset_picture_layers();
                self.history = History::default();
                self.ed.sel.clear();
                self.ed.node_sel.clear();
                self.ed.view.need_fit = true;
                self.screen = Screen::Editor;
                self.add_recent(&path);
                // An SVG or PDF is not saved over: the next save asks for a project name.
                self.path = is_project.then_some(path);
                self.autosave_in_sync();
            }
            Err(e) => self.status = format!("Could not open {name}: {e}"),
        }
    }

    /// Import a LightBurn project or a UGS design into the drawing: its shapes are added, selected, and undoable. The ways
    /// of cutting of a UGS design become the settings of the layers that have no shapes yet; the other layers keep theirs.
    fn import_file(&mut self, path: PathBuf) {
        if self.screen == Screen::Start {
            self.new_design();
        }
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let is_ugsd = path.extension().is_some_and(|e| e.eq_ignore_ascii_case("ugsd"));
        let area_h = self.work_size().1;
        let max_rpm = self.device.spindle.max_rpm;
        let last = layers::LAYER_COUNT - 1;
        let read = std::fs::read(&path).map_err(|e| e.to_string()).and_then(|d| {
            let text = String::from_utf8(d).map_err(|_| "this is not a text file".to_string())?;
            if is_ugsd {
                let imp = base_core::ugsd::import(&text, area_h, layers::LAYER_COUNT)?;
                let ls = ugs_import::layers(&imp, &mut self.tools, max_rpm);
                Ok((imp.scene, imp.warnings, Some((ls, imp.cuts.iter().map(|c| c.index).collect::<Vec<_>>()))))
            } else {
                let imp = base_core::lightburn::import(&text, area_h)?;
                Ok((imp.scene, imp.warnings, None))
            }
        });
        match read {
            Ok((scene, warnings, ugs)) => {
                self.checkpoint();
                let used: std::collections::HashSet<usize> = self.scene.shapes.iter().map(|s| s.layer).collect();
                if let Some(((new_layers, added), cut_layers)) = ugs.map(|(l, c)| (l, c)) {
                    for i in cut_layers.into_iter().filter(|i| !used.contains(i)) {
                        self.layers[i] = new_layers[i].clone();
                    }
                    if let Some(a) = added {
                        self.status = format!("{}: {a}", tr("bits added to the library"));
                        self.tools_win.changed = true;
                    }
                }
                let n = scene.shapes.len();
                let ids = self.scene.append(scene, last);
                self.ed.sel = ids.into_iter().collect();
                self.ed.node_sel.clear();
                self.touch_rev();
                self.ed.view.need_fit = true;
                self.status = format!("Imported {n} shapes from {name}");
                if !warnings.is_empty() {
                    self.status.push_str(&format!(" (some parts could not be used: {})", warnings.join("; ")));
                }
            }
            Err(e) => self.status = format!("Could not import {name}: {e}"),
        }
    }

    /// The project as it is saved: the drawing, the description and the milling part, with the sections of other
    /// programs as they were read.
    fn to_project(&self) -> Project {
        let mut p = Project::new(self.scene.clone());
        p.description = self.description.clone();
        p.set_section(section::SECTION, &section::MillSection::new(&self.layers, &self.stock, &self.device, &self.tools));
        p.restore_sections(&self.kept.other_sections(section::SECTION));
        p.restore_extra(self.kept.extra());
        p
    }

    fn save(&mut self, as_new: bool) {
        let path = match (&self.path, as_new) {
            (Some(p), false) => Some(p.clone()),
            _ => rfd::FileDialog::new().add_filter("Project", &[project::EXTENSION]).set_file_name("untitled.cre").save_file(),
        };
        let Some(path) = path else { return };
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        match std::fs::write(&path, self.to_project().to_json()) {
            Ok(()) => {
                self.status = format!("Saved {name}");
                self.add_recent(&path);
                self.path = Some(path);
                // The project has a name now: what was kept for it while it had none is not needed.
                self.autosave_saved_by_hand();
            }
            Err(e) => self.status = format!("Could not save {name}: {e}"),
        }
    }

    fn open_dialog(&mut self) {
        if let Some(p) = rfd::FileDialog::new().add_filter("Project, SVG, PDF, Illustrator", &[project::EXTENSION, "svg", "pdf", "ai", "ugsd"]).pick_file() {
            self.open(p);
        }
    }

    /// File > Import > Import LightBurn project…: the shapes of the project come over as a new, unsaved drawing.
    fn import_lightburn_dialog(&mut self) {
        if let Some(p) = rfd::FileDialog::new().add_filter("LightBurn", &["lbrn2", "lbrn"]).pick_file() {
            self.open(p);
        }
    }

    /// File > Import > Import UGS design…: the shapes and the ways of cutting of a UGS Designer file come over as a new,
    /// unsaved drawing.
    fn import_ugsd_dialog(&mut self) {
        if let Some(p) = rfd::FileDialog::new().add_filter("UGS Designer", &["ugsd"]).pick_file() {
            self.open(p);
        }
    }

    /// The size of the work area of the device in mm.
    fn work_size_mm(&self) -> (f64, f64) {
        (self.device.area.x, self.device.area.y)
    }

    /// Something in the project changed: the toolpaths are out of date.
    fn touch_rev(&mut self) {
        self.rev += 1;
    }

    fn undo(&mut self) {
        if let Some(s) = self.history.back(&self.scene) {
            self.scene = s;
            self.forget_missing();
            self.touch_rev();
        }
    }

    fn redo(&mut self) {
        if let Some(s) = self.history.forward(&self.scene) {
            self.scene = s;
            self.forget_missing();
            self.touch_rev();
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
        // Ctrl+1 … Ctrl+0 show a tab, Ctrl+W closes the shown one.
        match base_ui::doc_tabs::shortcut(ctx) {
            Some(base_ui::doc_tabs::TabKey::Select(i)) => self.switch_to(i),
            Some(base_ui::doc_tabs::TabKey::Close) => self.close_tab_request(),
            None => {}
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
                self.new_tab();
            } else if !native && pressed(egui::Key::S) {
                self.save(shift);
            } else if !native && pressed(egui::Key::O) {
                self.open_dialog();
            } else if !native && pressed(egui::Key::Q) {
                self.request_close(ctx);
            }
        } else if pressed(egui::Key::Delete) || pressed(egui::Key::Backspace) {
            if self.ed.tool == Tool::Node && !self.ed.node_sel.is_empty() {
                self.delete_nodes();
            } else {
                self.delete_selection();
            }
        } else if pressed(egui::Key::Escape) {
            let now = ctx.input(|i| i.time);
            self.note_escape(now);
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

    /// A second Escape within half a second goes back to the select tool.
    pub(crate) fn note_escape(&mut self, now: f64) {
        if now - self.esc_time < 0.5 {
            self.ed.tool = Tool::Select;
            self.esc_time = -10.0;
        } else {
            self.esc_time = now;
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
            let ctx = ui.ctx().clone();
            self.request_close(&ctx);
        }
    }

    /// The bar under the menu: the tool, new and open, the grid, and the size of the selection.
    fn control_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            base_ui::editor::control_name(self, ui);
            ui.separator();
            let ctx = ui.ctx().clone();
            for (act, icon, tip, ink) in [(Act::New, "file-plus", "New", NEW_BLUE), (Act::Open, "folder-open", "Open…", OPEN_ORANGE), (Act::Save, "save", "Save", SAVE_GREEN)] {
                if icon_button_ink(ui, icon, &tr(tip), true, 28.0, ink) {
                    self.run_act(&ctx, act);
                }
            }
            ui.separator();
            base_ui::editor::grid_toggles(self, ui);
            ui.separator();
            self.view_buttons(ui);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if base_ui::window::save_button(ui, &tr("Save G-code")).clicked() {
                    self.export_gcode();
                }
                base_ui::editor::selection_size(self, ui);
            });
        });
    }

    fn status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.label(&self.status);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                base_ui::editor::zoom_and_cursor(self, ui);
                ui.separator();
                ui.label(RichText::new(&self.device.name).color(theme::text_dim())).on_hover_text(tr("The device of this project"));
                ui.separator();
                let (text, color) = if self.machine.connected {
                    let state = if self.machine.state.is_empty() { tr("Connected").to_string() } else { self.machine.state.clone() };
                    (state, Color32::from_rgb(0x2e, 0xa0, 0x4f))
                } else {
                    (tr("Disconnected").to_string(), theme::text_dim())
                };
                ui.label(RichText::new(text).color(color));
            });
        });
    }

    /// The collapsed side panel: a narrow strip with the names of the tabs; a click on one opens the panel on it.
    fn side_strip(&mut self, ui: &mut egui::Ui) {
        let camera = self.device.camera().is_some();
        let extra: &[&'static str] = if camera { &["camera"] } else { &[] };
        let label = |id: &str| SideTab::from_id(id).map(SideTab::label).unwrap_or_default();
        if let Some(t) = self.tabs.strip(ui, self.side_tab.id(), extra, &label).and_then(SideTab::from_id) {
            self.side_tab = t;
        }
    }

    /// The tabs of the right panel.
    fn side_panel(&mut self, ui: &mut egui::Ui) {
        let camera = self.device.camera().is_some();
        if self.side_tab == SideTab::Camera && !camera {
            self.side_tab = SideTab::Properties;
        }
        let extra: &[&'static str] = if camera { &["camera"] } else { &[] };
        let label = |id: &str| SideTab::from_id(id).map(SideTab::label).unwrap_or_default();
        if let Some(t) = self.tabs.bar(ui, self.side_tab.id(), extra, &label).and_then(SideTab::from_id) {
            self.side_tab = t;
        }
        ui.separator();
        match self.side_tab {
            SideTab::Properties => {
                egui::ScrollArea::vertical().id_salt("tab_props").auto_shrink([false, false]).show(ui, |ui| {
                    if !base_ui::editor::properties_sections(self, ui) {
                        ui.label(RichText::new(tr("Nothing selected")).color(theme::text_dim()));
                    }
                });
            }
            SideTab::Layers => self.layers_panel(ui),
            SideTab::Material => self.material_panel(ui),
            SideTab::Device => {
                egui::ScrollArea::vertical().id_salt("tab_device").auto_shrink([false, false]).show(ui, |ui| {
                    self.device_tab(ui);
                    ui.separator();
                    self.machine_section(ui);
                });
            }
            SideTab::Control => {
                let td = self.tool_diameter();
                let job_box = self.job_box();
                let travel = [self.device.area.x, self.device.area.y, self.device.area.z];
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    self.quick_row(ui);
                    ui.add_space(4.0);
                    self.start_button(ui);
                    ui.add_space(4.0);
                    // STOP is live only while the machine works (a job, a jog, homing, a held move).
                    if control_ui::stop_button(ui, self.machine.working()) {
                        self.abort_job();
                    }
                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);
                    self.job_buttons(ui);
                    let c = ControlCtx { control: &self.device.control, safe_z: self.device.safe_z, tool_diameter: td, job_box, probe: true, travel, spindle: Some(control_ui::SpindleCtx { min_rpm: self.device.spindle.min_rpm, max_rpm: self.device.spindle.max_rpm }), zeroing: true, z_axis: true, hold_to_move: self.device.connection.grbl_version == base_core::machine::GrblVersion::Grbl11 };
                    if let Some(s) = control_ui::panel(ui, &mut self.machine, &mut self.control_ui, &c) {
                        self.status = s;
                    }
                });
            }
            SideTab::Console => {
                if let Some(line) = self.machine.console.show(ui, self.machine.connected) {
                    self.machine_line(&line);
                }
            }
            SideTab::Clipart => self.clipart_panel(ui),
            SideTab::Description => {
                if base_ui::description_ui::panel(ui, &mut self.description) {
                    self.touch_rev();
                }
            }
            SideTab::Camera => {
                egui::ScrollArea::vertical().id_salt("tab_camera").auto_shrink([false, false]).show(ui, |ui| self.camera_tab(ui));
            }
        }
    }

    fn start_screen(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                base_ui::start::start_header(ui, self.logo.as_ref(), "MillCreator", VERSION);
                ui.add_space(18.0);
                if !self.recovery.is_empty() {
                    if let Some(choice) = base_ui::autosave_ui::banner_all(ui, &self.recovery.clone(), 460.0) {
                        self.recovery_choice(choice);
                    }
                    ui.add_space(12.0);
                }
                let frame = egui::Frame::group(ui.style()).inner_margin(egui::Margin::same(12));
                frame.show(ui, |ui| {
                    ui.set_width(460.0);
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| self.device_section(ui));
                });
                ui.add_space(12.0);
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
                    let open = egui::Button::image_and_text(base_ui::icons::slot(Some("folder-open"), theme::text()), tr("Open…"));
                    if ui.add_sized([ui.available_width(), 30.0], open).clicked() {
                        self.run_act(&ctx, Act::Open);
                    }
                    ui.add_space(4.0);
                    let view = egui::Button::image_and_text(base_ui::icons::slot(Some("eye"), theme::text()), tr(base_ui::gcode_view_ui::MENU_VIEW));
                    if ui.add_sized([ui.available_width(), 30.0], view).clicked() {
                        self.run_act(&ctx, Act::ViewGcode);
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
        self.close_dialogs(ctx);
        self.image_dialogs(ctx);
        let mut show = self.show_options;
        base_ui::editor::view_options_window(self, ctx, &mut show);
        self.clip_dialogs(ctx);
        self.online_clipart_window(ctx);
        self.plugin_windows(ctx);
        tools_ui::tools_window(ctx, &mut self.tools_win, &mut self.tools);
        self.device_window(ctx);
        self.machine_windows(ctx);
        self.preview_window(ctx);
        self.show_options = show;
        self.update.tick(ctx, &update_glue::CONFIG);
        base_ui::alert_ui::window(ctx, &mut self.machine);
        self.program_options_window(ctx);
        let about = AboutInfo {
            title: "About MillCreator",
            name: "MillCreator",
            version: VERSION,
            lines: vec!["Open-source CNC mill design and preparation software: layers, bits, toolpaths, 3D view and G-code.".into(), "Written in Rust with egui; shares its core and interface with LightCreator through baseCreator.".into()],
            url: "https://github.com/michalsarna/MillCreator",
            licence: "MIT OR Apache-2.0",
        };
        base_ui::dialogs::about_window(ctx, &mut self.show_about, &about, self.logo.as_ref());
    }

    fn draw(&mut self, ui: &mut egui::Ui) {
        self.drop_idle_camera();
        let ctx = ui.ctx().clone();
        self.autosave_tick(&ctx, false);
        self.sync_picture_layers();
        self.poll_machine();
        if self.machine.wants_repaint() {
            ctx.request_repaint_after(std::time::Duration::from_millis(250));
        }
        #[cfg(target_os = "macos")]
        self.native_menu_frame(&ctx);
        self.title_bar(ui);
        base_ui::window::window_edges(&ctx);
        #[cfg(target_os = "macos")]
        base_ui::window::window_edge_drag(&ctx);
        self.handle_drops(&ctx);
        self.close_requested_tick(&ctx);
        if self.screen == Screen::Start {
            // A G-code file opened from the start screen takes the whole window; closing it returns here.
            if self.gcode_view.is_some() {
                egui::CentralPanel::no_frame().show(ui, |ui| {
                    self.gcode_view_panel(ui);
                });
            } else {
                self.start_screen(ui);
            }
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
        egui::Panel::top("doc_tabs").frame(chrome.inner_margin(egui::Margin::symmetric(8, 2))).exact_size(30.0).show(ui, |ui| self.doc_tab_bar(ui));
        egui::Panel::bottom("status").frame(chrome).exact_size(26.0).show(ui, |ui| self.status_bar(ui));
        egui::Panel::left("tools").frame(chrome).exact_size(52.0).resizable(false).show(ui, |ui| base_ui::editor::tool_palette(self, ui, None));
        let side = if self.tabs.collapsed {
            egui::Panel::right("side_strip").frame(chrome.inner_margin(egui::Margin::same(3))).exact_size(base_ui::side_tabs::STRIP_W).resizable(false).show(ui, |ui| self.side_strip(ui))
        } else {
            egui::Panel::right("side").frame(chrome.inner_margin(egui::Margin::same(8))).default_size(410.0).resizable(!base_ui::window::pointer_over_window(ui.ctx())).show(ui, |ui| self.side_panel(ui))
        };
        base_ui::widgets::panel_edge_highlight(ui.ctx(), side.response.rect, base_ui::widgets::Edge::Left);
        egui::Panel::right("arrange_bar").frame(chrome.inner_margin(egui::Margin::symmetric(2, 4))).exact_size(40.0).resizable(false).show(ui, |ui| base_ui::editor::arrange_bar(self, ui));
        egui::Panel::bottom("layer_bar").frame(chrome).exact_size(36.0).show(ui, |ui| base_ui::layer_bar::layer_bar(self, ui));
        egui::CentralPanel::no_frame().show(ui, |ui| {
            if self.camera_work_tab() && self.cam_ws_shown {
                self.camera_work_panel(ui);
            } else if !self.gcode_view_panel(ui) {
                self.canvas(ui)
            }
        });
        self.dialogs(&ctx);
    }
}

impl ClipHost for App {
    fn clip(&self) -> &ClipState {
        &self.clip
    }
    fn clip_mut(&mut self) -> &mut ClipState {
        &mut self.clip
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
    fn layer_visible(&self, layer: usize) -> bool {
        self.layers.get(layer).is_none_or(|l| l.visible)
    }
    fn checkpoint(&mut self) {
        let before = self.scene.clone();
        self.history.push(before);
    }
    fn touch(&mut self) {
        self.touch_rev();
    }
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
    fn open_shape_generators(&mut self) {
        self.plugins.gen_open = true;
    }
    fn open_online_clipart_search(&mut self, ctx: &egui::Context) {
        self.side_tab = SideTab::Clipart;
        base_ui::clipart_ui::ClipGallery::open_online_clipart(self, ctx);
    }
    fn say(&mut self, msg: String) {
        self.status = msg;
    }
    fn said(&self) -> &str {
        &self.status
    }
    fn work_size(&self) -> (f64, f64) {
        self.work_size_mm()
    }
    fn units(&self) -> Units {
        self.device.units
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
        self.layer_visible(layer).then(|| self.layer_color(layer))
    }
    fn layer_color(&self, layer: usize) -> egui::Color32 {
        self.layers.get(layer).map_or(egui::Color32::from_rgb(0x20, 0x20, 0x30), |l| l.color32())
    }
    fn paint_over_shapes(&mut self, v: &base_ui::editor::CanvasView) {
        self.paint_toolpaths(v);
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
            Act::Save | Act::SaveAs | Act::Preview3d | Act::ExportGcode => self.screen == Screen::Editor,
            Act::ExportRelief => self.screen == Screen::Editor && self.relief_picture().is_some(),
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
            Act::New => {
                self.new_tab();
            }
            Act::Open => self.open_dialog(),
            Act::Save => self.save(false),
            Act::SaveAs => self.save(true),
            Act::ImportLightburn => self.import_lightburn_dialog(),
            Act::ImportUgsd => self.import_ugsd_dialog(),
            Act::ViewGcode => self.view_gcode_dialog(),
            Act::ImportPlugin => self.import_with_plugin_dialog(),
            Act::Plugins => self.plugins.open = true,
            Act::GenerateShape => self.plugins.gen_open = true,
            Act::ToolLibrary => self.tools_win.open = true,
            Act::DeviceSettings => self.open_device_settings(),
            Act::Firmware => self.fw_open = true,
            Act::ToolSensor => self.tls_ui.open = true,
            Act::Calibration => self.cal.open = true,
            Act::Diagnostics => {
                self.diag_gcode = self.job_lines().map(|(l, _)| l.join("\n"));
                self.diag.open = true;
            }
            Act::StartFrom => {
                if let Some((lines, _)) = self.job_lines() {
                    self.start_lines = lines;
                    self.start_from.open = true;
                }
            }
            Act::CheckProgram => self.check_program(),
            Act::Preview3d => self.open_preview(),
            Act::ExportGcode => self.export_gcode(),
            Act::ExportRelief => self.export_relief(),
            Act::OpenRecent(i) => {
                if let Some(p) = self.recent.get(i as usize).cloned() {
                    self.open(p);
                }
            }
            Act::ClearRecent => self.recent.clear(),
            Act::Quit => self.request_close(ctx),
            Act::Std(s) => {
                if self.screen == Screen::Editor {
                    s.run(self)
                }
            }
            Act::ViewOptions => self.show_options = true,
            Act::ProgramOptions => self.show_program_options = true,
            Act::CheckUpdates => self.update.start(ctx, &update_glue::CONFIG, true),
            Act::SetLang(l) => {
                self.lang = l;
                i18n::set_lang(l);
            }
            Act::SetScheme(s) => {
                self.scheme = s;
                theme::apply(ctx, s);
            }
            Act::DownloadBackup => self.download_backup(),
            Act::RestoreBackup => self.restore_backup_dialog(ctx),
            Act::OpenGithub => ctx.open_url(egui::OpenUrl::new_tab("https://github.com/michalsarna/MillCreator")),
            Act::About => self.show_about = true,
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Called every half minute and when the program closes: a last chance to keep the work.
        self.autosave_tick(&egui::Context::default(), true);
        for (k, v) in self.settings_pairs() {
            storage.set_string(k, v);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.draw(ui);
    }
}

/// Where this program keeps its data: the settings in its own folder, the gallery shared with the other programs
/// (see `base_core::paths`).
pub fn data_paths() -> &'static base_core::paths::DataPaths {
    static PATHS: std::sync::OnceLock<base_core::paths::DataPaths> = std::sync::OnceLock::new();
    PATHS.get_or_init(|| {
        if cfg!(test) {
            // Tests never touch the real folders.
            return base_core::paths::prepare_in(&std::env::temp_dir().join(format!("millcreator-test-data-{}", std::process::id())), "millcreator");
        }
        base_core::paths::prepare("millcreator")
    })
}

fn main() -> eframe::Result {
    let mut viewport = egui::ViewportBuilder::default()
        .with_title(format!("MillCreator {VERSION}"))
        .with_app_id("millcreator")
        .with_inner_size([1360.0, 860.0])
        .with_min_inner_size([900.0, 560.0])
        // The application draws its own flat, square frame, as LightCreator does.
        .with_decorations(false);
    if let Some(icon) = base_ui::start::window_icon(app_icon().1) {
        viewport = viewport.with_icon(icon);
    }
    let opts = eframe::NativeOptions { viewport, persistence_path: Some(data_paths().settings.clone()), ..Default::default() };
    eframe::run_native(
        "MillCreator",
        opts,
        Box::new(|cc| {
            #[cfg(target_os = "macos")]
            native_menu::set_dock_icon(app_icon().1);
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
    /// Tests that save or look for the recovery file share one data folder: one at a time.
    static DATA: std::sync::Mutex<()> = std::sync::Mutex::new(());

    use super::*;

    #[test]
    fn a_picture_put_on_a_layer_switches_the_layer_to_the_picture_operation_once() {
        let mut a = app();
        a.layers[2].op = layers::Operation::Pocket;
        a.reset_picture_layers();
        let im = base_core::image::ImageData { name: "t".into(), px_w: 2, px_h: 2, gray: vec![0; 4], w: 10.0, h: 10.0, invert: false };
        let id = a.scene.add(2, base_core::scene::Kind::Image(im), base_core::geom::Xf::IDENTITY);
        a.sync_picture_layers();
        assert_eq!(a.layers[2].op, layers::Operation::Picture, "the layer switched");
        // The person chooses another operation: it stays so while the picture stays.
        a.layers[2].op = layers::Operation::OnPath;
        a.sync_picture_layers();
        assert_eq!(a.layers[2].op, layers::Operation::OnPath);
        // Without the picture, and then with it again, the layer switches again.
        a.scene.shapes.retain(|s| s.id != id);
        a.sync_picture_layers();
        let im = base_core::image::ImageData { name: "t".into(), px_w: 2, px_h: 2, gray: vec![0; 4], w: 10.0, h: 10.0, invert: false };
        a.scene.add(2, base_core::scene::Kind::Image(im), base_core::geom::Xf::IDENTITY);
        a.sync_picture_layers();
        assert_eq!(a.layers[2].op, layers::Operation::Picture);
        // A drawing that is opened keeps what its layers say.
        a.layers[2].op = layers::Operation::Pocket;
        a.reset_picture_layers();
        a.sync_picture_layers();
        assert_eq!(a.layers[2].op, layers::Operation::Pocket);
    }

    #[test]
    fn the_relief_of_a_picture_is_exported_as_a_closed_stl_model_only_when_there_is_one() {
        let mut a = app();
        assert!(a.relief_picture().is_none() && a.relief_model().is_none(), "nothing to export");
        let gray: Vec<u8> = (0..64).map(|i| ((i % 8) * 255 / 7) as u8).collect();
        let im = base_core::image::ImageData { name: "t".into(), px_w: 8, px_h: 8, gray, w: 30.0, h: 20.0, invert: false };
        let id = a.scene.add(3, base_core::scene::Kind::Image(im), base_core::geom::Xf::translate(10.0, 10.0));
        a.sync_picture_layers();
        assert_eq!(a.layers[3].op, layers::Operation::Picture);
        assert!(a.relief_picture().is_none(), "rows are not a relief");
        a.layers[3].picture = layers::PictureMethod::Relief;
        a.layers[3].step_over_pct = 20.0;
        assert_eq!(a.relief_picture().map(|s| s.id), Some(id));
        let stl = a.relief_model().expect("a model");
        let n = u32::from_le_bytes([stl[80], stl[81], stl[82], stl[83]]) as usize;
        assert!(n > 100 && stl.len() == 84 + 50 * n, "{n} triangles in {} bytes", stl.len());
        assert!(a.act_enabled(menu::Act::ExportRelief));
        a.layers[3].picture = layers::PictureMethod::Rows;
        assert!(!a.act_enabled(menu::Act::ExportRelief));
    }

    /// Without the "links" feature of eframe, open_url silently does nothing: no link of the program (the download page of an
    /// update, the project page, the list of plugins) would open.
    #[test]
    fn eframe_has_the_links_feature_so_that_links_open() {
        let manifest = include_str!("../Cargo.toml");
        let eframe = manifest.lines().find(|l| l.starts_with("eframe")).unwrap();
        assert!(eframe.contains("\"links\""), "{eframe}");
    }
    use base_core::geom::Pt;

    const SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><rect x="10" y="10" width="40" height="20" stroke="#ff0000" fill="none"/><circle cx="70" cy="70" r="10"/></svg>"##;

    fn app() -> App {
        let mut a = App::build(&egui::Context::default(), None, false);
        a.screen = Screen::Editor;
        a
    }

    #[test]
    fn the_shape_operations_of_the_shared_editor_work_here_too() {
        use base_core::scene::Kind;
        use base_ui::editor::EditorShapeOps;
        use base_ui::menu::StdAct;
        let mut a = app();
        let r1 = a.scene.add(0, Kind::Rect { w: 20.0, h: 20.0 }, base_core::geom::Xf::translate(10.0, 10.0));
        let r2 = a.scene.add(0, Kind::Rect { w: 20.0, h: 20.0 }, base_core::geom::Xf::translate(20.0, 10.0));
        a.ed.sel = vec![r1, r2];
        a.bool_op(base_core::ops::BoolOp::Union);
        assert_eq!(a.scene.shapes.len(), 1, "two rectangles became one shape");
        assert!(matches!(a.scene.shapes[0].kind, Kind::Path(_)));
        // Offset keeps the original and adds a bigger outline.
        a.ed.sel = vec![a.scene.shapes[0].id];
        a.offset_selected(2.0, true);
        assert_eq!(a.scene.shapes.len(), 2);
        let width = |i: usize| { let b = a.scene.shapes[i].polys()[0].bounds().unwrap(); b.max.x - b.min.x };
        let (w0, w1) = (width(0), width(1));
        assert!(w1 > w0 + 3.0, "{w0} {w1}");
        // Rounding the corners of a rectangle gives curves, and a shape can be turned into curves.
        let r = a.scene.add(0, Kind::Rect { w: 30.0, h: 30.0 }, base_core::geom::Xf::translate(100.0, 100.0));
        a.ed.sel = vec![r];
        a.round_corners(4.0);
        assert!(matches!(a.scene.shape(r).unwrap().kind, Kind::Bezier(_)));
        let e = a.scene.add(0, Kind::Ellipse { w: 10.0, h: 10.0 }, base_core::geom::Xf::translate(0.0, 0.0));
        a.ed.sel = vec![e];
        a.to_curves();
        assert!(matches!(a.scene.shape(e).unwrap().kind, Kind::Bezier(_)));
        // The windows open from the menu commands.
        StdAct::OffsetShape.run(&mut a);
        StdAct::RoundCorners.run(&mut a);
        assert!(a.ed.show_offset && a.ed.show_round);
    }

    /// A 4 x 4 picture (a black square on white) as a BMP file, made by hand.
    fn bmp(path: &std::path::Path) {
        let (w, h) = (4u32, 4u32);
        let row = (w * 3).div_ceil(4) * 4;
        let mut b: Vec<u8> = vec![b'B', b'M'];
        b.extend((54 + row * h).to_le_bytes());
        b.extend([0u8; 4]);
        b.extend(54u32.to_le_bytes());
        b.extend(40u32.to_le_bytes());
        b.extend(w.to_le_bytes());
        b.extend(h.to_le_bytes());
        b.extend(1u16.to_le_bytes());
        b.extend(24u16.to_le_bytes());
        b.extend([0u8; 24]);
        for y in 0..h {
            let mut line = vec![];
            for x in 0..w {
                let dark = (1..3).contains(&x) && (1..3).contains(&y);
                line.extend(if dark { [0u8, 0, 0] } else { [255u8, 255, 255] });
            }
            line.resize(row as usize, 0);
            b.extend(line);
        }
        std::fs::write(path, b).unwrap();
    }

    #[test]
    fn a_picture_opens_the_import_dialog_and_can_be_adjusted_and_traced_but_makes_no_toolpath() {
        use base_ui::editor::EditorImages;
        use base_ui::menu::StdAct;
        let dir = std::env::temp_dir().join(format!("mill-image-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("square.bmp");
        bmp(&file);
        let mut a = app();
        a.open(file.clone());
        assert!(a.ed.img_dlg.is_some(), "the picture is offered for import");
        // Without a picture on the page there is nothing to adjust or trace.
        assert!(!StdAct::AdjustImage.enabled(&a) && !StdAct::TraceImage.enabled(&a));
        // The picture on the page (what the Import button of the dialog does).
        let im = base_core::image::ImageData::from_bytes(&std::fs::read(&file).unwrap(), "square.bmp", 40.0).unwrap();
        a.ed.img_dlg = None;
        let id = a.scene.add(0, base_core::scene::Kind::Image(im), base_core::geom::Xf::translate(10.0, 10.0));
        a.ed.sel = vec![id];
        assert!(StdAct::AdjustImage.enabled(&a) && StdAct::TraceImage.enabled(&a));
        a.open_trace();
        assert!(a.ed.trace_dlg.is_some());
        a.open_adjust();
        assert!(a.ed.img_dlg.is_some());
        // A picture is for looking at and tracing: it is not machined.
        let _ = std::fs::remove_dir_all(&dir);
        let prog = toolpath::generate(&a.scene, &a.layers, &a.tools, &a.device, &a.stock);
        assert!(prog.moves.is_empty(), "no cutting for a picture: {} moves", prog.moves.len());
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
    fn a_project_is_saved_and_opened_and_keeps_the_sections_of_other_programs() {
        let _data = DATA.lock().unwrap_or_else(|e| e.into_inner());
        let dir = std::env::temp_dir().join(format!("mc-project-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        // A project made by a laser program: a drawing and a laser section.
        let mut p = Project::new(read_scene(SVG).unwrap());
        p.description = "from the laser".into();
        p.set_section("laser", &serde_json::json!({ "format": 1, "material": "Plywood" }));
        let file = dir.join("job.cre");
        std::fs::write(&file, p.to_json()).unwrap();
        let mut a = app();
        a.open(file.clone());
        assert_eq!((a.scene.shapes.len(), a.path.as_ref(), a.screen), (2, Some(&file), Screen::Editor));
        // Edit and save in place.
        a.scene.add(0, Kind::Rect { w: 5.0, h: 5.0 }, Xf::IDENTITY);
        a.save(false);
        let saved = Project::from_json(&std::fs::read_to_string(&file).unwrap()).unwrap();
        assert_eq!(saved.scene.shapes.len(), 3);
        assert_eq!(saved.description, "from the laser");
        assert_eq!(saved.section::<serde_json::Value>("laser").unwrap().unwrap()["material"], "Plywood");
        // A new design starts without the sections of the other programs.
        a.new_design();
        assert_eq!(a.to_project().section_names(), vec!["mill"], "only its own part");
        // An SVG is opened without becoming the file that is saved over.
        let svg = dir.join("badge.svg");
        std::fs::write(&svg, SVG).unwrap();
        a.open(svg);
        assert!(a.path.is_none());
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn a_lightburn_project_is_imported_into_the_drawing() {
        let dir = std::env::temp_dir().join(format!("mc-lb-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("job.lbrn2");
        std::fs::write(&file, r#"<LightBurnProject><Shape Type="Rect" CutIndex="0" W="30" H="10" Cr="0"><XForm>1 0 0 1 50 20</XForm></Shape></LightBurnProject>"#).unwrap();
        let mut a = app();
        a.scene = read_scene(SVG).unwrap();
        let before = a.scene.shapes.len();
        let h = a.work_size().1;
        let recent = a.recent.clone();
        a.open(file);
        assert_eq!(a.scene.shapes.len(), before + 1, "added to the drawing");
        assert_eq!(a.ed.sel.len(), 1);
        assert_eq!(a.recent, recent, "an import is not a project to reopen");
        // LightBurn's centre is 20 mm above the bottom; here that is 20 mm above the bottom of the work area too.
        let b = a.scene.shapes.last().unwrap().bounds().unwrap();
        assert!((b.min.y - (h - 25.0)).abs() < 1e-9 && (b.min.x - 35.0).abs() < 1e-9, "{b:?}");
        assert!(a.status.starts_with("Imported"), "{}", a.status);
        // One undo takes the import back.
        let now = a.scene.clone();
        a.scene = a.history.back(&now).unwrap();
        assert_eq!(a.scene.shapes.len(), before);
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn a_ugs_design_is_imported_with_its_cuts_on_free_layers_only() {
        let dir = std::env::temp_dir().join(format!("mc-ugsd-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("job.ugsd");
        std::fs::write(&file, r#"{"version":"1","entities":[{"type":"RECTANGLE","cutType":"POCKET","cutDepth":4,"feedRate":700,"spindleSpeed":100,"transform":{"m00":30,"m10":0,"m01":0,"m11":10,"m02":50,"m12":20}},{"type":"RECTANGLE","isHidden":true,"cutType":"OUTSIDE_PATH","cutDepth":6,"feedRate":500,"transform":{"m00":5,"m10":0,"m01":0,"m11":5,"m02":0,"m12":0}}]}"#).unwrap();
        let mut a = app();
        a.scene.add(1, base_core::scene::Kind::Rect { w: 5.0, h: 5.0 }, base_core::geom::Xf::IDENTITY);
        let own = a.layers[1].clone();
        let recent = a.recent.clone();
        a.open(file);
        assert_eq!(a.scene.shapes.len(), 3, "the hidden shape is drawn too");
        assert!(a.path.is_none());
        assert_eq!(a.recent, recent);
        // The pocket is the first way of cutting: layer 0 was free and takes it; layer 1 holds a shape and stays.
        assert_eq!((a.layers[0].op, a.layers[0].depth, a.layers[0].feed), (layers::Operation::Pocket, 4.0, 700.0));
        assert_eq!(a.layers[1], own);
        std::fs::remove_dir_all(dir).ok();
    }

    /// A folder of plugins with a small importer and a postprocessor, and a pack with a bit.
    fn scratch_plugins(name: &str) -> PathBuf {
        let base = std::env::temp_dir().join(format!("mc-plugins-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let p = base.join("plugins/my-tools");
        std::fs::create_dir_all(&p).unwrap();
        std::fs::write(p.join("plugin.json"), r#"{"id":"my-tools","name":"My tools","importers":[{"id":"xy","name":"XY","extensions":["xy"],"script":"read.rhai"}],"postprocessors":[{"id":"n","name":"Numbers","script":"post.rhai"}],"data":[{"kind":"mill-tools","file":"tools.json"}]}"#).unwrap();
        std::fs::write(p.join("read.rhai"), "fn read(text, ctx) { [rect(5, 5, 10, 20), circle(30, 30, 4)] }").unwrap();
        std::fs::write(p.join("post.rhai"), r#"fn post(lines, ctx) { let out = []; let n = 1; for l in lines { out.push("N" + n + " " + l); n += 1; } out }"#).unwrap();
        let mut lib = Library::new();
        let t = tools::Tool::new("x", "Plugin bit 2.5 mm", tools::ToolKind::FlatEnd, tools::Dims { diameter: 2.5, ..tools::Dims::default() });
        lib.add(t).unwrap();
        std::fs::write(p.join("tools.json"), lib.export(None)).unwrap();
        base
    }

    #[test]
    fn a_plugin_reads_its_files_adds_its_bits_and_postprocesses_the_gcode() {
        let base = scratch_plugins("all");
        let mut a = app();
        a.plugins = base_ui::plugins_ui::PluginsUi::new("millcreator", &base);
        a.after_plugins_changed();
        a.after_plugins_changed();
        assert_eq!(a.tools.all().filter(|t| t.name == "Plugin bit 2.5 mm").count(), 1, "read once, not at every start");
        let file = base.join("shapes.xy");
        std::fs::write(&file, "anything").unwrap();
        a.scene = read_scene(SVG).unwrap();
        let before = a.scene.shapes.len();
        let recent = a.recent.clone();
        a.open(file);
        assert_eq!(a.scene.shapes.len(), before + 2, "{}", a.status);
        assert_eq!(a.recent, recent);
        a.plugins.post = Some("my-tools/n".into());
        assert_eq!(a.postprocessed("G21\nM5\n".into()).unwrap(), "N1 G21\nN2 M5\n");
        a.plugins.post = None;
        assert_eq!(a.postprocessed("G21\n".into()).unwrap(), "G21\n");
        std::fs::remove_dir_all(base).ok();
    }

    #[test]
    fn a_postprocessor_that_sends_controller_commands_saves_nothing() {
        let base = scratch_plugins("bad");
        std::fs::write(base.join("plugins/my-tools/post.rhai"), r#"fn post(lines, ctx) { ["$RST=*"] }"#).unwrap();
        let mut a = app();
        a.plugins = base_ui::plugins_ui::PluginsUi::new("millcreator", &base);
        a.plugins.post = Some("my-tools/n".into());
        assert!(a.postprocessed("G21\n".into()).is_none());
        assert!(a.status.contains("nothing was saved"), "{}", a.status);
        std::fs::remove_dir_all(base).ok();
    }

    #[test]
    fn an_unnamed_project_is_kept_in_a_recovery_file_and_recovered_after_a_restart() {
        let _data = DATA.lock().unwrap_or_else(|e| e.into_inner());
        let base = &data_paths().base;
        base_core::autosave::clear(base, "millcreator");
        let mut a = app();
        a.scene = read_scene(SVG).unwrap();
        let n = a.scene.shapes.len();
        a.touch_rev();
        a.autosave_tick(&egui::Context::default(), true);
        assert_eq!(base_core::autosave::find(base, "millcreator").map(|f| f.shapes), Some(n), "kept while it has no name");
        // The program is started again: the start screen has the offer; Recover opens it as a project without a name.
        let mut b = App::build(&egui::Context::default(), None, false);
        assert_eq!(b.recovery.first().map(|f| f.shapes), Some(n));
        let recent = b.recent.clone();
        b.recovery_choice(base_ui::autosave_ui::Recovery::Recover);
        assert_eq!((b.scene.shapes.len(), b.screen), (n, Screen::Editor));
        assert!(b.path.is_none() && b.recent == recent);
        assert!(base_core::autosave::find(base, "millcreator").is_some(), "kept until it is saved");
        // Saved by hand: the recovery file is gone.
        b.autosave_saved_by_hand();
        assert!(base_core::autosave::find(base, "millcreator").is_none());
        // Discard removes it too.
        a.touch_rev();
        a.autosave_tick(&egui::Context::default(), true);
        let mut c = App::build(&egui::Context::default(), None, false);
        c.recovery_choice(base_ui::autosave_ui::Recovery::Discard);
        assert!(base_core::autosave::find(base, "millcreator").is_none() && c.recovery.is_empty());
        a_project_with_a_file_is_saved_to_it_and_not_to_the_recovery_file();
    }

    fn a_project_with_a_file_is_saved_to_it_and_not_to_the_recovery_file() {
        let dir = std::env::temp_dir().join(format!("millcreator-autosave-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("work.cre");
        base_core::autosave::clear(&data_paths().base, "millcreator");
        let mut a = app();
        a.scene = read_scene(SVG).unwrap();
        a.path = Some(file.clone());
        a.touch_rev();
        a.autosave_tick(&egui::Context::default(), true);
        let text = std::fs::read_to_string(&file).unwrap();
        assert_eq!(Project::from_json(&text).unwrap().scene.shapes.len(), a.scene.shapes.len());
        assert!(base_core::autosave::find(&data_paths().base, "millcreator").is_none());
        // Nothing changed since: nothing is written again.
        std::fs::remove_file(&file).unwrap();
        a.autosave_tick(&egui::Context::default(), true);
        assert!(!file.exists());
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn a_click_on_a_colour_of_the_bar_moves_the_selected_shapes_to_that_layer() {
        use base_ui::layer_bar::LayerBarHost;
        let mut a = app();
        let first = a.scene.add(0, base_core::scene::Kind::Rect { w: 5.0, h: 5.0 }, base_core::geom::Xf::IDENTITY);
        let second = a.scene.add(0, base_core::scene::Kind::Rect { w: 5.0, h: 5.0 }, base_core::geom::Xf::translate(20.0, 0.0));
        let locked = a.scene.add(0, base_core::scene::Kind::Rect { w: 5.0, h: 5.0 }, base_core::geom::Xf::translate(40.0, 0.0));
        a.scene.shape_mut(locked).unwrap().locked = true;
        // Nothing selected: the layer only becomes the one new shapes go to.
        a.click_layer(7);
        assert_eq!(a.ed.active_layer, 7);
        assert!(a.scene.shapes.iter().all(|s| s.layer == 0));
        // With a selection the shapes move (not the locked one), and one undo takes it back.
        a.ed.sel = vec![first, second, locked];
        a.click_layer(60);
        let layer_of = |a: &App, id| a.scene.shape(id).unwrap().layer;
        assert_eq!((layer_of(&a, first), layer_of(&a, second), layer_of(&a, locked)), (60, 60, 0));
        assert_eq!(a.layer_total(), 64);
        let now = a.scene.clone();
        a.scene = a.history.back(&now).unwrap();
        assert_eq!(layer_of(&a, first), 0);
    }

    #[test]
    fn the_look_for_updates_switch_is_on_by_default_and_is_remembered() {
        let a = app();
        assert!(a.check_updates);
        let on = a.settings_pairs().into_iter().find(|(k, _)| *k == "check_updates").map(|(_, v)| v);
        assert_eq!(on.as_deref(), Some("1"));
        assert_eq!(update_glue::CONFIG.repo, concat!("michalsarna/", "MillCreator"));
        assert!(base_ui::dialogs::is_newer("v99.0.0", update_glue::CONFIG.version) && !base_ui::dialogs::is_newer(update_glue::CONFIG.version, update_glue::CONFIG.version));
    }

    #[test]
    fn a_second_escape_within_half_a_second_selects_the_select_tool() {
        let mut a = app();
        a.ed.tool = Tool::Rect;
        a.note_escape(10.0);
        assert_eq!(a.ed.tool, Tool::Rect, "one Escape alone keeps the tool");
        a.note_escape(10.3);
        assert_eq!(a.ed.tool, Tool::Select);
        a.ed.tool = Tool::Star;
        a.note_escape(10.4);
        assert_eq!(a.ed.tool, Tool::Star, "the pair was used up");
        a.note_escape(11.5);
        assert_eq!(a.ed.tool, Tool::Star, "too late is a new first Escape");
    }

    #[test]
    fn the_spindle_speed_is_spread_over_the_range_of_the_controller() {
        let mut a = app();
        a.device.spindle.max_rpm = 24000.0;
        let lines = vec!["G21".to_string(), "M3 S12000".to_string(), "G1 X1".to_string()];
        // The range of the controller is not known: the speed is the S value.
        assert_eq!(a.scale_spindle(lines.clone()), lines);
        // $30 = 1000: half of the top speed is S500.
        a.machine.fw.values.insert(30, "1000".into());
        assert_eq!(a.scale_spindle(lines.clone()), vec!["G21", "M3 S500", "G1 X1"]);
        // $30 equal to the top speed: unchanged.
        a.machine.fw.values.insert(30, "24000".into());
        assert_eq!(a.scale_spindle(lines.clone()), lines);
    }

    #[test]
    fn a_broken_project_is_an_error_message_and_changes_nothing() {
        let dir = std::env::temp_dir().join(format!("mc-broken-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("broken.cre");
        std::fs::write(&file, "{ not json").unwrap();
        let mut a = app();
        a.scene = read_scene(SVG).unwrap();
        a.open(file);
        assert_eq!(a.scene.shapes.len(), 2);
        assert!(a.status.starts_with("Could not open"), "{}", a.status);
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn a_gallery_picture_is_placed_and_undone() {
        let mut a = app();
        let (_, name, svg) = clipart_ui::BUILTIN_CLIPART[0];
        a.place_clip(svg, name);
        assert!(!a.scene.shapes.is_empty() && !a.ed.sel.is_empty());
        a.undo();
        assert!(a.scene.shapes.is_empty());
    }

    #[test]
    fn the_clipart_tab_draws() {
        let mut h = egui_kittest::Harness::builder().with_accessibility_check(false).with_context_setup(|ctx| {
            // The app was built on another context: this one needs the fonts and the icons (menu images) too.
            base_ui::fonts::install(ctx);
            base_ui::icons::install(ctx);
        }).with_size([1280.0, 800.0]).build_ui_state(
            |ui, a: &mut App| {
                a.screen = Screen::Editor;
                a.side_tab = SideTab::Clipart;
                a.draw(ui);
            },
            App::build(&egui::Context::default(), None, false),
        );
        h.run_steps(3);
    }

    #[test]
    fn the_description_tab_draws_the_text_of_the_project_and_it_is_saved() {
        let mut app = App::build(&egui::Context::default(), None, false);
        app.description = "Birch plywood 3 mm".into();
        let mut h = egui_kittest::Harness::builder().with_accessibility_check(false).with_context_setup(|ctx| {
            // The app was built on another context: this one needs the fonts and the icons (menu images) too.
            base_ui::fonts::install(ctx);
            base_ui::icons::install(ctx);
        }).with_size([1280.0, 800.0]).build_ui_state(
            |ui, a: &mut App| {
                a.screen = Screen::Editor;
                a.side_tab = SideTab::Description;
                a.draw(ui);
            },
            app,
        );
        h.run_steps(3);
        assert_eq!(h.state().description, "Birch plywood 3 mm");
        assert_eq!(h.state().to_project().description, "Birch plywood 3 mm");
    }

    #[test]
    fn the_job_options_are_in_the_control_tab_and_change_what_is_made() {
        use egui_kittest::kittest::Queryable;
        let mut app = App::build(&egui::Context::default(), None, false);
        for x in [100.0, 10.0] {
            app.scene.add(0, Kind::Rect { w: 30.0, h: 30.0 }, Xf::translate(x, 20.0));
        }
        app.touch_rev();
        let mut h = egui_kittest::Harness::builder().with_accessibility_check(false).with_context_setup(|ctx| {
            base_ui::fonts::install(ctx);
            base_ui::icons::install(ctx);
        }).with_size([420.0, 1000.0]).build_ui_state(
            |ui, a: &mut App| {
                a.screen = Screen::Editor;
                a.side_tab = SideTab::Control;
                a.draw(ui);
            },
            app,
        );
        h.run_steps(3);
        for w in ["One element at a time", "Selection only"] {
            assert!(h.query_by_label(w).is_some(), "{w} is in the Control tab");
        }
        let a = h.state_mut();
        let all = a.make_program().moves.len();
        let stamp = a.job_stamp();
        // Selection only: just the shape that is selected; nothing selected gives nothing, and says why.
        a.selection_only = true;
        assert_ne!(stamp, a.job_stamp(), "the program kept is made again");
        assert!(a.make_program().is_empty());
        assert_eq!(a.nothing_to_cut(), "There is nothing to cut: nothing is selected");
        a.ed.sel = vec![a.scene.shapes[0].id];
        let one = a.make_program().moves.len();
        assert!(one > 0 && one < all, "{one} of {all}");
        // Both together work as well, and the options are not part of the project.
        a.one_by_one = true;
        assert_eq!(a.make_program().moves.len(), one);
        assert!(!a.to_project().description.contains("one_by_one"));
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

    fn drawn(a: &mut App, svg_times: usize) {
        for _ in 0..svg_times {
            let more = read_scene(SVG).unwrap();
            let last = layers::LAYER_COUNT - 1;
            a.scene.append(more, last);
        }
        a.touch_rev();
    }

    #[test]
    fn a_new_tab_is_a_new_drawing_and_each_tab_keeps_its_own() {
        let _data = DATA.lock().unwrap_or_else(|e| e.into_inner());
        let mut a = app();
        drawn(&mut a, 1);
        let one = a.scene.shapes.len();
        assert!(a.new_tab());
        assert_eq!(a.tab_count(), 2);
        assert_eq!(a.scene.shapes.len(), 0, "the new tab is empty");
        drawn(&mut a, 2);
        assert!(a.new_tab());
        drawn(&mut a, 3);
        a.switch_to(0);
        assert_eq!(a.scene.shapes.len(), one);
        a.switch_to(1);
        assert_eq!(a.scene.shapes.len(), 2 * one);
        a.switch_to(2);
        assert_eq!(a.scene.shapes.len(), 3 * one);
        assert_eq!(a.tabs_state.active, 2);
    }

    #[test]
    fn there_are_at_most_ten_tabs() {
        let _data = DATA.lock().unwrap_or_else(|e| e.into_inner());
        let mut a = app();
        for _ in 1..doc_tabs_max() {
            assert!(a.new_tab());
        }
        assert_eq!(a.tab_count(), doc_tabs_max());
        assert!(!a.new_tab(), "the eleventh is refused");
        assert_eq!(a.tab_count(), doc_tabs_max());
        assert!(a.status.contains("10"), "{}", a.status);
    }

    fn doc_tabs_max() -> usize {
        base_ui::doc_tabs::MAX_TABS
    }

    #[test]
    fn what_was_made_for_one_tab_is_not_taken_for_another() {
        let _data = DATA.lock().unwrap_or_else(|e| e.into_inner());
        let mut a = app();
        drawn(&mut a, 1);
        a.new_tab();
        drawn(&mut a, 2);
        let r = a.rev;
        a.switch_to(0);
        assert!(a.prog.is_none(), "the toolpaths of the other tab are dropped");
        assert_ne!(a.rev, r, "the revision is a new number in every tab shown");
    }

    #[test]
    fn unsaved_work_is_per_tab_and_closing_asks_about_each() {
        use base_ui::doc_tabs::{quit_answer, quit_prepare, Answer, Quit, TabHost};
        let _data = DATA.lock().unwrap_or_else(|e| e.into_inner());
        let mut a = app();
        a.autosave_in_sync();
        assert!(!a.any_dirty(), "an empty drawing is nothing to save");
        drawn(&mut a, 1);
        a.new_tab();
        a.new_tab();
        drawn(&mut a, 2);
        a.switch_to(0);
        assert!(a.is_dirty() && a.any_dirty());
        assert!(a.tab_dirty(0) && !a.tab_dirty(1) && a.tab_dirty(2));
        let ctx = egui::Context::default();
        a.request_close(&ctx);
        assert!(a.quitting && !a.allow_close);
        assert_eq!(quit_prepare(&mut a), Quit::Asking);
        assert_eq!(a.tabs_state.active, 0);
        // Discard it: it is closed and the next one with work is shown.
        assert_eq!(quit_answer(&mut a, Answer::Discard), Quit::Asking);
        assert_eq!(a.tab_count(), 2);
        assert!(a.is_dirty());
        // Cancel: the program stays, and the tab keeps its work.
        assert_eq!(quit_answer(&mut a, Answer::Cancel), Quit::Stopped);
        assert_eq!(a.tab_count(), 2);
        assert!(a.any_dirty());
    }

    #[test]
    fn a_project_that_is_open_is_shown_not_opened_again() {
        let _data = DATA.lock().unwrap_or_else(|e| e.into_inner());
        let mut a = app();
        let dir = std::env::temp_dir().join(format!("millcreator-tabs-files-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mk = |n: &str| {
            let f = dir.join(n);
            let mut p = Project::new(read_scene(SVG).unwrap());
            p.description = n.into();
            std::fs::write(&f, p.to_json()).unwrap();
            f
        };
        let (f1, f2) = (mk("one.cre"), mk("two.cre"));
        a.open(f1.clone());
        assert_eq!(a.tab_count(), 1, "a blank tab is used for the first file");
        a.open(f2.clone());
        assert_eq!(a.tab_count(), 2);
        a.open(f1.clone());
        assert_eq!(a.tab_count(), 2, "one.cre is open already");
        assert_eq!(a.tabs_state.active, 0);
        assert_eq!(a.description, "one.cre");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ctrl_digits_show_the_tabs_and_ctrl_w_closes_the_shown_one() {
        let _data = DATA.lock().unwrap_or_else(|e| e.into_inner());
        let mut a = app();
        a.new_tab();
        a.new_tab();
        assert_eq!((a.tab_count(), a.tabs_state.active), (3, 2));
        let ctx = egui::Context::default();
        let press = |a: &mut App, key: egui::Key| {
            let ev = egui::Event::Key { key, physical_key: Some(key), pressed: true, repeat: false, modifiers: egui::Modifiers::COMMAND };
            let mut out = ctx.run_ui(egui::RawInput { events: vec![ev], ..Default::default() }, |ui| a.shortcuts(ui.ctx()));
            out.textures_delta.clear();
        };
        press(&mut a, egui::Key::Num1);
        assert_eq!(a.tabs_state.active, 0);
        press(&mut a, egui::Key::Num2);
        assert_eq!(a.tabs_state.active, 1);
        press(&mut a, egui::Key::Num9);
        assert_eq!(a.tabs_state.active, 1, "there is no ninth tab");
        press(&mut a, egui::Key::W);
        assert_eq!(a.tab_count(), 2, "Ctrl+W closes the shown tab (it has no unsaved work)");
        drawn(&mut a, 1);
        press(&mut a, egui::Key::W);
        assert!(a.tabs_state.closing && a.tab_count() == 2, "a tab with unsaved work is asked about first");
    }

    #[test]
    fn closing_the_last_tab_closes_the_program_and_leaves_nothing_to_recover() {
        use base_ui::doc_tabs::TabHost;
        let _data = DATA.lock().unwrap_or_else(|e| e.into_inner());
        let base = &data_paths().base;
        for s in 0..base_core::autosave::SLOTS {
            base_core::autosave::clear_slot(base, "millcreator", s);
        }
        let mut a = app();
        // Two tabs: closing the first leaves one, the program stays.
        a.new_tab();
        a.close_shown_tab();
        assert_eq!((a.tab_count(), a.close_program), (1, false));
        // The last tab holds work that was written to its recovery file; it is given up.
        drawn(&mut a, 1);
        let ctx = egui::Context::default();
        a.autosave_tick(&ctx, true);
        assert_eq!(base_core::autosave::find_all(base, "millcreator").len(), 1);
        a.discard_active();
        a.close_shown_tab();
        assert!(a.close_program, "the last tab closed: the program closes");
        // The program asks the window to close, and what it does on the way out saves nothing.
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| a.close_dialogs(ui.ctx()));
        out.textures_delta.clear();
        let closing = out.viewport_output.values().any(|v| v.commands.iter().any(|c| matches!(c, egui::ViewportCommand::Close)));
        assert!(closing && a.allow_close);
        a.autosave_tick(&ctx, true);
        assert!(!a.any_dirty());
        assert!(base_core::autosave::find_all(base, "millcreator").is_empty(), "nothing the person gave up comes back");
    }

    #[test]
    fn tabs_that_are_not_shown_are_kept_in_their_own_recovery_files_and_recovered_each_in_a_tab() {
        let _data = DATA.lock().unwrap_or_else(|e| e.into_inner());
        let base = &data_paths().base;
        for s in 0..base_core::autosave::SLOTS {
            base_core::autosave::clear_slot(base, "millcreator", s);
        }
        let mut a = app();
        drawn(&mut a, 1);
        let one = a.scene.shapes.len();
        a.new_tab();
        drawn(&mut a, 2);
        assert!(a.parked_need_autosave());
        a.autosave_tick(&egui::Context::default(), true);
        assert!(!a.parked_need_autosave());
        assert_eq!(base_core::autosave::find_all(base, "millcreator").len(), 2, "every unsaved tab has its file");
        // The program is started again: Recover opens each of them in a tab.
        let mut b = App::build(&egui::Context::default(), None, false);
        assert_eq!(b.recovery.len(), 2);
        b.recovery_choice(base_ui::autosave_ui::Recovery::Recover);
        assert_eq!((b.tab_count(), b.screen), (2, Screen::Editor));
        let mut shapes = vec![b.scene.shapes.len()];
        b.switch_to(0);
        shapes.push(b.scene.shapes.len());
        shapes.sort();
        assert_eq!(shapes, vec![one, 2 * one]);
        assert!(b.any_dirty(), "still unsaved work");
        assert_eq!(base_core::autosave::find_all(base, "millcreator").len(), 2, "the files stay until the work is saved");
        // Closing a tab removes its file.
        b.close_shown_tab();
        assert_eq!(base_core::autosave::find_all(base, "millcreator").len(), 1);
        for s in 0..base_core::autosave::SLOTS {
            base_core::autosave::clear_slot(base, "millcreator", s);
        }
    }
}

impl base_ui::layer_bar::LayerBarHost for App {
    fn layer_label(&self, layer: usize) -> String {
        self.layers.get(layer).map(|l| l.name.clone()).unwrap_or_default()
    }
    fn layer_total(&self) -> usize {
        self.layers.len()
    }
    fn open_layer_options(&mut self, layer: usize) {
        self.ed.active_layer = layer;
        self.side_tab = SideTab::Layers;
    }
}

#[cfg(test)]
mod festive_icon_tests {
    use super::*;

    #[test]
    fn every_festive_icon_decodes_at_its_size() {
        let v = &FESTIVE;
        for (name, (small, big)) in [("christmas", v.christmas), ("new year", v.new_years_eve), ("halloween", v.halloween), ("easter", v.easter)] {
            let small = base_ui::start::window_icon(small).unwrap_or_else(|| panic!("{name} 256 does not decode"));
            let big = base_ui::start::window_icon(big).unwrap_or_else(|| panic!("{name} 512 does not decode"));
            assert_eq!((small.width, small.height), (256, 256), "{name}");
            assert_eq!((big.width, big.height), (512, 512), "{name}");
        }
        assert_eq!(app_icon(), (ICON_256, ICON_512), "tests always get the normal icon");
    }
}
