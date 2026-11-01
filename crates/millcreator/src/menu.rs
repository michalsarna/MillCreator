//! What the menus of MillCreator can do and how they are laid out. The standard editing commands come from base-ui.
use base_ui::i18n::Lang;
use base_ui::menu::{key, standard_menus, Entry, MenuParts, Menus, StdAct};
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

/// The menus: the layout of baseCreator (`standard_menus`) with the entries of MillCreator in their places. The macOS app menu (About,
/// Quit) is added by `native_menu`.
pub fn menus() -> Menus<Act> {
    use Entry::{Item, Sep};
    standard_menus(MenuParts {
        new: Act::New,
        open: Act::Open,
        save: Act::Save,
        save_as: Act::SaveAs,
        quit: Act::Quit,
        about: Act::About,
        view_gcode: Act::ViewGcode,
        generate: Act::GenerateShape,
        import_plugin: Act::ImportPlugin,
        online_clipart: Act::OnlineClipart,
        plugins: Act::Plugins,
        view_options: Act::ViewOptions,
        program_options: Act::ProgramOptions,
        set_lang: Act::SetLang,
        set_scheme: Act::SetScheme,
        github: Act::OpenGithub,
        download_backup: Act::DownloadBackup,
        restore_backup: Act::RestoreBackup,
        check_updates: Act::CheckUpdates,
        file_extra: vec![Item(Act::ProjectSetup, "Project setup…", None)],
        import: vec![Item(Act::ImportDxf, "Import DXF…", None), Item(Act::ImportLightburn, "Import LightBurn project…", None), Item(Act::ImportUgsd, "Import UGS design…", None), Item(Act::ImportModel, "Import 3D model as relief…", None)],
        export: vec![Item(Act::ExportGcode, "Export G-code…", None), Item(Act::ExportRelief, "Export relief as STL…", None)],
        output_extra: vec![],
        view_extra: vec![],
        background: true,
        machine: vec![Item(Act::DeviceSettings, "Device settings…", None), Item(Act::Firmware, "Firmware settings ($$)…", None), Item(Act::Calibration, "Calibration…", None), Item(Act::ToolSensor, "Tool length sensor…", None), Item(Act::Diagnostics, "Diagnostics and statistics…", None), Sep, Item(Act::StartFrom, "Start from line…", None), Item(Act::CheckProgram, "Check the program", None)],
        process: ("Milling", vec![Item(Act::ToolLibrary, "Bit library…", None), Sep, Item(Act::Preview3d, "3D view of the job…", key("CmdOrCtrl+Shift+P", "Ctrl+Shift+P")), Item(Act::ExportGcode, "Export G-code…", key("CmdOrCtrl+E", "Ctrl+E")), Item(Act::ExportRelief, "Export relief as STL…", None)]),
        settings_extra: vec![],
        help_first: vec![],
        help_extra: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every key of the menus belongs to one entry only (Ctrl+Shift+G was Ungroup and Export G-code at once), and the menus have the
    /// layout of all the programs.
    #[test]
    fn no_two_entries_of_the_menus_share_a_key_and_the_layout_is_the_common_one() {
        let m = menus();
        assert!(base_ui::menu::key_clashes(&m).is_empty(), "{:?}", base_ui::menu::key_clashes(&m));
        let titles: Vec<&str> = m.iter().map(|(t, _)| *t).collect();
        assert_eq!(&titles[..6], ["File", "Edit", "Insert", "Arrange", "View", "Machine"]);
        assert_eq!(&titles[7..], ["Settings", "Help"]);
    }
}
