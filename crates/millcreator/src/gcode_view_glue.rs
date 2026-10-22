//! The G-code viewer itself lives in baseCreator (`base_ui::gcode_view_ui`); this is the glue to the application: the menu command,
//! a file that is opened or dropped, and the view in the place of the work area.
use crate::App;
use std::path::{Path, PathBuf};

impl App {
    /// Does a plugin read files with this extension? Then it wins over the viewer.
    pub(crate) fn plugin_reads(&self, path: &Path) -> bool {
        let ext = path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
        self.plugins.import_extensions().iter().any(|e| e.eq_ignore_ascii_case(&ext))
    }

    pub(crate) fn view_gcode_dialog(&mut self) {
        if let Some(p) = base_ui::gcode_view_ui::pick_file() {
            self.open(p);
        }
    }

    pub(crate) fn open_gcode(&mut self, p: PathBuf) {
        if let Err(e) = base_ui::gcode_view_ui::open(&mut self.gcode_view, p) {
            self.status = e;
        }
    }

    /// The viewer in the place of the work area; false when there is none (the editor is drawn then).
    pub(crate) fn gcode_view_panel(&mut self, ui: &mut egui::Ui) -> bool {
        base_ui::gcode_view_ui::panel(&mut self.gcode_view, ui)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_g_code_file_is_shown_in_the_viewer_not_opened_as_a_drawing() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        let dir = std::env::temp_dir().join(format!("gview-glue-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("job.nc");
        std::fs::write(&p, "G21 G90\nG0 X0 Y0\nG1 X10 Y0 F500\nG1 X10 Y10\n").unwrap();
        let shapes = a.scene.shapes.len();
        assert_eq!(a.screen, crate::Screen::Start);
        a.open(p);
        assert!(a.gcode_view.is_some(), "the viewer opened");
        assert_eq!(a.screen, crate::Screen::Start, "from the start screen the editor is not entered");
        assert_eq!(a.scene.shapes.len(), shapes, "the drawing is not touched");
        // A file without moves is refused with a line for the status bar.
        let q = dir.join("empty.nc");
        std::fs::write(&q, "; nothing\n").unwrap();
        a.gcode_view = None;
        a.open(q);
        assert!(a.gcode_view.is_none());
        assert!(!a.status.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
