//! Looking for a newer release on GitHub: once when the program starts (can be switched off in the program options) and on
//! demand from Help > Check for updates…. The check and its window are in base-ui; this says which program they are about.
use crate::{App, VERSION};
use base_ui::dialogs::UpdateConfig;
use base_ui::i18n::tr;
use base_ui::window::popup;
use eframe::egui;

pub static CONFIG: UpdateConfig = UpdateConfig { app_name: "MillCreator", repo: "michalsarna/MillCreator", version: VERSION };

impl App {
    /// Settings → Program options…: what the program does by itself.
    pub(crate) fn program_options_window(&mut self, ctx: &egui::Context) {
        let mut open = self.show_program_options;
        popup(ctx, "program_options", &tr("Program options"), [460.0, 360.0], &mut open, |ui| {
            egui::Grid::new("program_options_grid").num_columns(2).spacing([12.0, 8.0]).show(ui, |ui| {
                ui.label(tr("Updates"));
                ui.checkbox(&mut self.check_updates, tr("Look for a newer version when the program starts"));
                ui.end_row();
                base_ui::units_ui::display_units_row(ui, &mut self.ed.grid_prefs.units);
                base_ui::view3d::quality_row(ui, &mut self.ed.grid_prefs.render_quality);
                self.web.options_ui(ui, true);
                ui.end_row();
            });
        });
        self.show_program_options = open;
    }
}
