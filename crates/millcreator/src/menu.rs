//! What the menus of MillCreator can do and how they are laid out. The standard editing commands come from base-ui.
use base_ui::i18n::Lang;
use base_ui::menu::{key, std_entries, Entry, Menus, StdAct};
use base_ui::theme::Scheme;

/// How many recent files the File menu keeps.
pub const MAX_RECENT: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Act {
    New,
    Open,
    Save,
    SaveAs,
    ImportLightburn,
    ToolLibrary,
    Preview3d,
    ExportGcode,
    OpenRecent(u8),
    ClearRecent,
    Quit,
    Std(StdAct),
    ViewOptions,
    SetLang(Lang),
    SetScheme(Scheme),
    OpenGithub,
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

/// Top-level menus: (English title, entries). The macOS app menu (About, Quit) is added by `native_menu`.
pub fn menus() -> Menus<Act> {
    use Entry::{Item, Recent, Sep, Sub};
    let mut file = vec![Item(Act::New, "New", key("CmdOrCtrl+N", "Ctrl+N")), Item(Act::Open, "Open…", key("CmdOrCtrl+O", "Ctrl+O")), Recent, Sep, Item(Act::Save, "Save", key("CmdOrCtrl+S", "Ctrl+S")), Item(Act::SaveAs, "Save as…", key("CmdOrCtrl+Shift+S", "Ctrl+Shift+S")), Sep, Sub("Import", vec![Item(Act::ImportLightburn, "Import LightBurn project…", None)])];
    let mut help = vec![Item(Act::OpenGithub, "Project page on GitHub", None)];
    if !cfg!(target_os = "macos") {
        file.push(Sep);
        file.push(Item(Act::Quit, "Quit", key("CmdOrCtrl+Q", "Ctrl+Q")));
        help.push(Sep);
        help.push(Item(Act::About, "About", None));
    }
    vec![
        ("File", file),
        ("Edit", std(std_entries::edit())),
        ("Milling", vec![Item(Act::ToolLibrary, "Bit library…", None), Sep, Item(Act::Preview3d, "3D view of the job…", key("CmdOrCtrl+Shift+3", "Ctrl+Shift+3")), Item(Act::ExportGcode, "Export G-code…", key("CmdOrCtrl+Shift+G", "Ctrl+Shift+G"))]),
        (
            "Arrange",
            [std(std_entries::align()), vec![Sep], std(std_entries::place()), vec![Sep], std(std_entries::group()), vec![Sep], std(std_entries::order()), vec![Sep], std(std_entries::nodes())].concat(),
        ),
        ("View", [std(std_entries::grid()), std(std_entries::zoom())].concat()),
        (
            "Settings",
            vec![
                Item(Act::ViewOptions, "View options…", None),
                Sep,
                Sub("Language", Lang::ALL.iter().map(|l| Item(Act::SetLang(*l), l.name(), None)).collect()),
                Sub("Colour scheme", Scheme::ALL.iter().map(|s| Item(Act::SetScheme(*s), s.label(), None)).collect()),
            ],
        ),
        ("Help", help),
    ]
}
