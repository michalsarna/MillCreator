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
    ProjectSetup,
    ImportLightburn,
    ImportDxf,
    ImportUgsd,
    ImportModel,
    ImportPlugin,
    ViewGcode,
    Plugins,
    GenerateShape,
    OnlineClipart,
    ToolLibrary,
    Firmware,
    DeviceSettings,
    ToolSensor,
    Calibration,
    Diagnostics,
    StartFrom,
    CheckProgram,
    Preview3d,
    ExportGcode,
    ExportRelief,
    OpenRecent(u8),
    ClearRecent,
    Quit,
    Std(StdAct),
    ViewOptions,
    ProgramOptions,
    CheckUpdates,
    SetLang(Lang),
    SetScheme(Scheme),
    OpenGithub,
    DownloadBackup,
    RestoreBackup,
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
    let mut file = vec![Item(Act::New, "New", key("CmdOrCtrl+N", "Ctrl+N")), Item(Act::Open, "Open…", key("CmdOrCtrl+O", "Ctrl+O")), Recent, Sep, Item(Act::Save, "Save", key("CmdOrCtrl+S", "Ctrl+S")), Item(Act::SaveAs, "Save as…", key("CmdOrCtrl+Shift+S", "Ctrl+Shift+S")), Item(Act::ProjectSetup, "Project setup…", None), Sep, Sub("Import", [std(std_entries::import_image()), vec![Item(Act::ImportDxf, "Import DXF…", None), Item(Act::ImportLightburn, "Import LightBurn project…", None), Item(Act::ImportUgsd, "Import UGS design…", None), Item(Act::ImportModel, "Import 3D model as relief…", None), Item(Act::ImportPlugin, base_ui::plugins_ui::MENU_IMPORT, None), Item(Act::GenerateShape, base_ui::plugins_ui::MENU_GENERATE, None)]].concat()), Sub("Export", std(std_entries::export_selection())), Item(Act::ViewGcode, base_ui::gcode_view_ui::MENU_VIEW, None)];
    let mut help = vec![Item(Act::OpenGithub, "Project page on GitHub", None), Sep, Item(Act::DownloadBackup, base_ui::backup_ui::DOWNLOAD, None), Item(Act::RestoreBackup, base_ui::backup_ui::RESTORE, None), Sep, Item(Act::CheckUpdates, "Check for updates…", None)];
    if !cfg!(target_os = "macos") {
        file.push(Sep);
        file.push(Item(Act::Quit, "Quit", key("CmdOrCtrl+Q", "Ctrl+Q")));
        help.push(Sep);
        help.push(Item(Act::About, "About", None));
    }
    vec![
        ("File", file),
        ("Edit", std(std_entries::edit())),
        // The tools of the left palette that insert objects, and the generators of plugins.
        ("Insert", [std(std_entries::insert_tools()), vec![Sep, Item(Act::GenerateShape, base_ui::plugins_ui::MENU_GENERATE, None), Item(Act::OnlineClipart, "Online clipart…", None)]].concat()),
        ("Machine", vec![Item(Act::DeviceSettings, "Device settings…", None), Item(Act::Firmware, "Firmware settings ($$)…", None), Item(Act::Calibration, "Calibration…", None), Item(Act::ToolSensor, "Tool length sensor…", None), Item(Act::Diagnostics, "Diagnostics and statistics…", None), Sep, Item(Act::StartFrom, "Start from line…", None), Item(Act::CheckProgram, "Check the program", None)]),
        ("Milling", vec![Item(Act::ToolLibrary, "Bit library…", None), Sep, Item(Act::Preview3d, "3D view of the job…", key("CmdOrCtrl+Shift+3", "Ctrl+Shift+3")), Item(Act::ExportGcode, "Export G-code…", key("CmdOrCtrl+E", "Ctrl+E")), Item(Act::ExportRelief, "Export relief as STL…", None)]),
        (
            "Arrange",
            [std(std_entries::align()), vec![Sep], std(std_entries::place()), vec![Sep], std(std_entries::group()), vec![Sep], std(std_entries::order()), vec![Sep], std(std_entries::nodes()), vec![Sep], std(std_entries::shape_ops()), vec![Sep], std(std_entries::design_tools()), vec![Sep], std(std_entries::image_tools())].concat(),
        ),
        ("View", [std(std_entries::grid()), std(std_entries::zoom()), vec![Sep], std(std_entries::split_view()), vec![Sep], std(std_entries::background())].concat()),
        (
            "Settings",
            vec![
                Item(Act::ViewOptions, "View options…", None),
                Item(Act::ProgramOptions, "Program options…", None),
                Sep,
                Item(Act::Plugins, base_ui::plugins_ui::MENU_PLUGINS, None),
                Sep,
                Sub("Language", Lang::ALL.iter().map(|l| Item(Act::SetLang(*l), l.name(), None)).collect()),
                Sub("Colour scheme", Scheme::available().iter().map(|s| Item(Act::SetScheme(*s), s.label(), None)).collect()),
            ],
        ),
        ("Help", help),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every key of the menus belongs to one entry only (Ctrl+Shift+G was Ungroup and Export G-code at once).
    #[test]
    fn no_two_entries_of_the_menus_share_a_key() {
        fn keys(entries: &[Entry<Act>], out: &mut Vec<(&'static str, &'static str)>) {
            for e in entries {
                match e {
                    Entry::Item(_, label, Some((accel, _))) => out.push((accel, label)),
                    Entry::Sub(_, children) => keys(children, out),
                    _ => {}
                }
            }
        }
        let mut all = vec![];
        for (_, entries) in menus() {
            keys(&entries, &mut all);
        }
        for (i, (k, label)) in all.iter().enumerate() {
            // The same entry twice (Generate shape is in File and in Insert) is the same command, not a clash.
            if let Some((_, other)) = all[i + 1..].iter().find(|(k2, l2)| k2 == k && l2 != label) {
                panic!("{k} is the key of {label} and of {other}");
            }
        }
        assert!(all.iter().any(|(k, l)| *k == "CmdOrCtrl+Shift+G" && *l == "Ungroup"));
    }
}
