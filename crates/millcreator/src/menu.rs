//! What the menus of MillCreator can do and how they are laid out. The standard editing commands come from base-ui.
use base_ui::i18n::Lang;
use base_ui::menu::{key, machine_menu, overlay_entries, standard_menus, Entry, MachineMenu, MenuParts, Menus, StdAct};
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
    ImportSvg,
    ImportAi,
    ImportDxf,
    ImportUgsd,
    ImportModel,
    ImportPlugin,
    ViewGcode,
    Plugins,
    GenerateShape,
    OnlineClipart,
    ToolLibrary,
    KnifeLibrary,
    Firmware,
    DeviceSettings,
    ToolSensor,
    Calibration,
    Diagnostics,
    ControlDevices,
    ControlDeviceOfMachine,
    StartFrom,
    CheckProgram,
    Preview3d,
    TechnicalDrawing,
    ExportGcode,
    ExportSvg,
    ExportCam,
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
    Print,
    QuickGuide,
    ExportDiagnostics,
    SwitchDevice,
    ShareSer2net,
    Frame,
    StartJob,
    ToggleOverlay,
    /// Draw the no-go zones on the work area.
    ToggleZones,
    /// The device settings, on their page of the no-go zones.
    ZoneSettings,
    PreviewGcode,
    LayerPresets,
    /// Place tabs on the outlines by hand (the switch of the Layers tab).
    TabsByHand,
    AlignmentHoles,
    /// The operation of the active layer.
    SetOp(crate::layers::Operation),
    /// The side of a two-sided job that is cut.
    SetSide(crate::layers::Side),
    PickZero,
    FindCentre,
    SavedPositions,
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
        import_svg: Act::ImportSvg,
        import_ai: Act::ImportAi,
        import_dxf: Act::ImportDxf,
        import_lightburn: Act::ImportLightburn,
        import_ugsd: Act::ImportUgsd,
        export_svg: Act::ExportSvg,
        export_cam: Act::ExportCam,
        import_extra: vec![Item(Act::ImportModel, "Import 3D model as relief…", None)],
        export_first: vec![Item(Act::ExportGcode, "Export G-code…", None)],
        export_extra: vec![Item(Act::ExportRelief, "Export relief as STL…", None)],
        output_extra: vec![Item(Act::Print, "Print…", key("CmdOrCtrl+P", "Ctrl+P"))],
        view_extra: [overlay_entries(Act::ToggleOverlay, Act::Std(StdAct::BackgroundPicture)), vec![Item(Act::ToggleZones, "Show no-go zones", None)]].concat(),
        background: false,
        machine: machine_menu(MachineMenu {
            switch_device: Act::SwitchDevice,
            device_settings: Act::DeviceSettings,
            firmware: Act::Firmware,
            calibration: Act::Calibration,
            own: vec![
                Item(Act::ToolSensor, "Tool length sensor…", None),
                Item(Act::ZoneSettings, "No-go zones…", None),
                Item(Act::PickZero, "Pick the zero of the program", None),
                Item(Act::FindCentre, "Find the centre", None),
                Item(Act::SavedPositions, "Saved positions…", None),
                Entry::Sub("Side to cut", crate::layers::Side::ALL.iter().map(|s| Item(Act::SetSide(*s), s.label(), None)).collect()),
            ],
            control_devices: Act::ControlDevices,
            control_device_of_machine: Act::ControlDeviceOfMachine,
            diagnostics: Act::Diagnostics,
            share: Act::ShareSer2net,
            frame: Act::Frame,
            start_job: Act::StartJob,
            start_from: Act::StartFrom,
            check_program: Act::CheckProgram,
        }),
        process: ("Milling", vec![Item(Act::ToolLibrary, "Bit library…", None), Item(Act::KnifeLibrary, "Knife library…", None), Item(Act::LayerPresets, "Layer presets…", None), Sep, Entry::Sub("Operation of the active layer", crate::layers::Operation::ALL.iter().map(|o| Item(Act::SetOp(*o), o.label(), None)).collect()), Item(Act::TabsByHand, "Place tabs by hand", None), Item(Act::AlignmentHoles, "Add alignment holes", None), Sep, Item(Act::PreviewGcode, "Preview G-code", None), Item(Act::Preview3d, "3D view of the job…", key("CmdOrCtrl+Shift+P", "Ctrl+Shift+P")), Item(Act::TechnicalDrawing, "Show technical drawing…", None), Item(Act::ExportGcode, "Export G-code…", key("CmdOrCtrl+E", "Ctrl+E")), Item(Act::ExportRelief, "Export relief as STL…", None)]),
        settings_extra: vec![],
        help_first: vec![Item(Act::QuickGuide, "Quick guide", key("F1", "F1"))],
        help_extra: vec![Item(Act::ExportDiagnostics, "Export diagnostics (zip)…", None)],
        zones: true,
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
