//! Automatic saving (see `base_core::autosave`): a project with a file is written to it, one without a name to a recovery
//! file, which the start screen offers to recover; saving under a name removes it.
use crate::{data_paths, App, Screen};
use base_core::autosave::{self, Found};
use base_ui::autosave_ui::Recovery;

const APP: &str = "millcreator";

impl App {
    /// Save what changed, every `autosave::INTERVAL` (and at once with `force`, when the program closes).
    pub(crate) fn autosave_tick(&mut self, ctx: &egui::Context, force: bool) {
        if self.screen != Screen::Editor || !self.autosave.changed(self.rev) {
            return;
        }
        if !force && !self.autosave.due(self.rev) {
            // Come back to look again, also when nothing else makes the window repaint.
            ctx.request_repaint_after(std::time::Duration::from_secs(5));
            return;
        }
        let text = self.to_project().to_json();
        let r = match &self.path {
            Some(p) => autosave::write_atomic(p, &text),
            None if self.scene.shapes.is_empty() => Ok(()),
            None => autosave::save_recovery(&data_paths().base, APP, &text),
        };
        if let Err(e) = r {
            self.status = format!("Automatic saving failed: {e}");
        }
        self.autosave.saved(self.rev);
    }

    /// The person saved the project under a name: the recovery file is not needed any more.
    pub(crate) fn autosave_saved_by_hand(&mut self) {
        autosave::clear(&data_paths().base, APP);
        self.recovery = None;
        self.autosave.in_sync(self.rev);
    }

    /// The project on the screen is what is on disk (just opened, or new).
    pub(crate) fn autosave_in_sync(&mut self) {
        self.autosave.in_sync(self.rev);
    }

    /// The choice on the start screen: open the unsaved project (as one without a name), or throw it away.
    pub(crate) fn recovery_choice(&mut self, choice: Recovery) {
        let Some(Found { path, .. }) = self.recovery.take() else { return };
        let recent = self.recent.clone();
        match choice {
            Recovery::Discard => autosave::clear(&data_paths().base, APP),
            Recovery::Recover => {
                self.open(path.clone());
                // It is still unsaved work: no name, not a recent file, and the recovery file stays until it is saved.
                self.path = None;
                self.recent = recent;
                self.autosave_in_sync();
                if !self.status.starts_with("Could not") {
                    self.status = "Recovered the project that was not saved".into();
                }
            }
        }
    }
}
