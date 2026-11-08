//! Help > Download backup… and Restore from backup…: the settings and the shared gallery in one zip (see `base_core::backup`).
use crate::{data_paths, theme, App, Lang, Scheme, Screen, i18n, device};
use base_ui::backup_ui;
use eframe::egui;
use std::collections::BTreeMap;
use std::path::PathBuf;

impl App {
    /// Ask where to put the backup and write it.
    pub(crate) fn download_backup(&mut self) {
        let _ = data_paths();
        if let Some(line) = backup_ui::download("MillCreator", env!("CARGO_PKG_VERSION"), &self.backup_pairs(), self.clip.lib.dir()) {
            self.status = line;
        }
    }

    /// What goes into the settings of a backup: the settings of the program and the layer presets, which have a file of their own.
    fn backup_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = self.settings_pairs();
        pairs.push(base_ui::layer_presets_ui::backup_pair(self.presets.as_ref(), crate::layer_presets::FILE, &crate::layer_presets::valid));
        pairs.push(self.control_devices.backup_pair());
        pairs
    }

    /// Choose a backup file, ask, and restore from it.
    pub(crate) fn restore_backup_dialog(&mut self, ctx: &egui::Context) {
        let Some(src) = backup_ui::pick_restore(true) else { return };
        let result = base_core::backup::restore(&src, "MillCreator", self.clip.lib.dir());
        if let Ok(r) = &result {
            self.apply_settings(ctx, &r.settings);
            self.clip.lib.refresh();
            self.clip.reload();
        }
        self.status = backup_ui::restore_status(&src, &result);
    }

    /// Take the settings of a backup (the strings of `settings_pairs`) into the running program.
    pub(crate) fn apply_settings(&mut self, ctx: &egui::Context, m: &BTreeMap<String, String>) {
        let get = |k: &str| m.get(k).map(String::as_str);
        if let Some(l) = get("lang").and_then(Lang::from_code) {
            self.lang = l;
            i18n::set_lang(l);
        }
        if let Some(s) = get("scheme").and_then(Scheme::from_id) {
            self.scheme = s;
            theme::apply(ctx, s);
        }
        if let Some(v) = get("grid_main_on") {
            self.ed.show_grid = v == "1";
        }
        if let Some(g) = get("grid_prefs").and_then(|j| serde_json::from_str(j).ok()) {
            self.ed.grid_prefs = g;
        }
        if let Some(g) = get("grid_main_mm").and_then(|v| v.parse::<f64>().ok()).filter(|g| *g >= 0.1) {
            self.ed.grid = g;
        }
        if let Some(v) = get("snap") {
            self.ed.snap = v == "1";
        }
        if let Some(v) = get("snap_objects") {
            self.ed.snap_objects = v == "1";
        }
        if let Some(t) = get("devices") {
            self.devices = device::load_all(t);
            // Keep working on the device that is open if the restored list has it; otherwise the first one.
            self.active = self.devices.iter().position(|d| d.name == self.device.name);
            self.start_sel = self.active.unwrap_or(0);
            if self.active.is_none() && self.screen == Screen::Start {
                if let Some(d) = self.devices.first() {
                    self.device = d.clone();
                    self.active = Some(0);
                }
            }
            self.dev_edit = None;
            self.confirm_delete = false;
        }
        if let Some(t) = get("tools") {
            let mut lib = crate::Library::new();
            lib.load_text(t);
            self.tools = lib;
        }
        if let Some(t) = get("knives") {
            self.knives.load_text(t);
        }
        // A backup from before the presets were in it leaves them alone.
        if let Some(text) = get(base_ui::layer_presets_ui::BACKUP_KEY) {
            let valid = &crate::layer_presets::valid;
            self.presets.get_or_insert_with(|| base_ui::layer_presets_ui::Library::load(crate::layer_presets::FILE, valid)).replace_text(text, valid);
        }
        // A backup from before the devices were in it leaves them alone.
        if let Some(text) = get(base_core::control_devices::BACKUP_KEY) {
            self.control_devices.restore(text);
        }
        self.tabs.restore(get("tab_order").unwrap_or(""), get("side_collapsed").is_some_and(|v| v == "1"));
        if let Some(r) = get("recent_files") {
            self.recent = r.lines().map(PathBuf::from).take(crate::MAX_RECENT).collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_backup_brings_the_settings_back_and_keeps_newer_gallery_pictures() {
        let ctx = egui::Context::default();
        let dir = std::env::temp_dir().join(format!("millcreator-backup-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let svg = "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 10 10'><rect width='10' height='10'/></svg>";
        let mut a = App::build(&ctx, None, false);
        a.clip = base_ui::clipart_ui::ClipState::new(base_core::clipart::Library::open(dir.join("gallery")));
        a.clip.lib.add(svg, "star", "Mine", "file", "", "").unwrap();
        a.lang = Lang::Pl;
        let zip = dir.join("b.zip");
        std::fs::create_dir_all(&dir).unwrap();
        let n = base_core::backup::write(&zip, "MillCreator", "test", &a.settings_pairs(), a.clip.lib.dir()).unwrap();
        assert!(n >= 4);
        // A fresh program with an empty gallery; another program has saved a picture in the meantime.
        let mut b = App::build(&ctx, None, false);
        b.clip = base_ui::clipart_ui::ClipState::new(base_core::clipart::Library::open(dir.join("gallery2")));
        b.clip.lib.add(&svg.replace("<rect", "<circle r='4'"), "newer", "Mine", "file", "", "").unwrap();
        let r = base_core::backup::restore(&zip, "MillCreator", b.clip.lib.dir()).unwrap();
        b.apply_settings(&ctx, &r.settings);
        assert_eq!(b.lang, Lang::Pl);
        i18n::set_lang(Lang::En);
        b.clip.lib.refresh();
        let names: Vec<_> = b.clip.lib.items.iter().map(|c| c.name.clone()).collect();
        assert!(names.contains(&"star".to_string()) && names.contains(&"newer".to_string()), "{names:?}");
        // A backup of another program is refused.
        assert!(base_core::backup::restore(&zip, "LightCreator", b.clip.lib.dir()).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_copies_of_the_controller_settings_and_the_sensor_are_in_the_backup() {
        let ctx = egui::Context::default();
        let dir = std::env::temp_dir().join(format!("mill-backup-copies-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let mut a = App::build(&ctx, None, false);
        a.clip = base_ui::clipart_ui::ClipState::new(base_core::clipart::Library::open(dir.join("gallery")));
        let mut d = crate::device::Device::dummy();
        d.control.fw_copies.make(1_700_000_000, "Grbl 1.1h", &[(110u32, "3000.000".to_string()), (130, "300.000".to_string())].into_iter().collect());
        d.control.tls = base_core::control::Tls { set: true, x: -10.0, y: -20.0, ref_wz: Some(-31.3), ..Default::default() };
        a.devices = vec![d];
        let zip = dir.join("b.zip");
        base_core::backup::write(&zip, "MillCreator", "test", &a.settings_pairs(), a.clip.lib.dir()).unwrap();
        let mut b = App::build(&ctx, None, false);
        b.clip = base_ui::clipart_ui::ClipState::new(base_core::clipart::Library::open(dir.join("gallery2")));
        let r = base_core::backup::restore(&zip, "MillCreator", b.clip.lib.dir()).unwrap();
        b.apply_settings(&ctx, &r.settings);
        let c = &b.devices[0].control;
        assert_eq!(c.fw_copies.0.len(), 1);
        assert_eq!(c.fw_copies.0[0].values.get(&110).map(String::as_str), Some("3000.000"));
        assert_eq!(c.tls.ref_wz, Some(-31.3));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_layer_presets_are_in_the_backup_and_an_older_backup_leaves_them_alone() {
        let ctx = egui::Context::default();
        let dir = std::env::temp_dir().join(format!("mill-backup-presets-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let mut a = App::build(&ctx, None, false);
        let mut lib = base_ui::layer_presets_ui::Library::from_path(None, &crate::layer_presets::valid);
        lib.put("birch 3 mm", crate::layers::default_layers(&crate::tools::Library::new())[0].clone());
        a.presets = Some(lib);
        let gallery = dir.join("gallery");
        let zip = dir.join("b.zip");
        base_core::backup::write(&zip, "MillCreator", "test", &a.backup_pairs(), &gallery).unwrap();
        assert!(!a.settings_pairs().iter().any(|(k, _)| *k == base_ui::layer_presets_ui::BACKUP_KEY), "the presets stay out of the saved settings");
        // A program with a preset of its own: the list of the backup takes its place, as the other settings do.
        let mut b = App::build(&ctx, None, false);
        b.presets.as_mut().unwrap().put("mine", crate::layers::default_layers(&crate::tools::Library::new())[0].clone());
        let r = base_core::backup::restore(&zip, "MillCreator", &dir.join("gallery2")).unwrap();
        b.apply_settings(&ctx, &r.settings);
        let names: Vec<_> = b.presets.as_ref().unwrap().items.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["birch 3 mm"]);
        // Without the entry nothing happens to them.
        let mut older = r.settings.clone();
        older.remove(base_ui::layer_presets_ui::BACKUP_KEY);
        b.apply_settings(&ctx, &older);
        assert_eq!(b.presets.as_ref().unwrap().items.len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_control_devices_are_in_the_backup_and_an_older_backup_leaves_them_alone() {
        use base_core::control_devices::{ControlDevice, KEYBOARD_ID};
        let ctx = egui::Context::default();
        let dir = std::env::temp_dir().join(format!("mill-backup-devices-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let mut a = App::build(&ctx, None, false);
        a.control_devices.lib.add(ControlDevice::keyboard("Keyboard"));
        let zip = dir.join("b.zip");
        base_core::backup::write(&zip, "MillCreator", "test", &a.backup_pairs(), &dir.join("gallery")).unwrap();
        assert!(!a.settings_pairs().iter().any(|(k, _)| *k == base_core::control_devices::BACKUP_KEY), "they have a file of their own");
        let mut b = App::build(&ctx, None, false);
        let r = base_core::backup::restore(&zip, "MillCreator", &dir.join("gallery2")).unwrap();
        b.apply_settings(&ctx, &r.settings);
        assert_eq!(b.control_devices.lib.default.as_deref(), Some(KEYBOARD_ID));
        let mut older = r.settings.clone();
        older.remove(base_core::control_devices::BACKUP_KEY);
        b.control_devices.lib.remove(KEYBOARD_ID);
        b.control_devices.lib.add(ControlDevice::pad("abc", "Pad"));
        b.apply_settings(&ctx, &older);
        assert_eq!(b.control_devices.lib.default.as_deref(), Some("pad-abc"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_order_of_the_tabs_and_the_collapsed_panel_are_in_the_backup() {
        let ctx = egui::Context::default();
        let mut a = App::build(&ctx, None, false);
        let mut order = a.tabs.order().to_vec();
        order.swap(0, 3);
        a.tabs.restore(&order.join(","), true);
        let pairs: std::collections::BTreeMap<String, String> = a.settings_pairs().into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        let mut b = App::build(&ctx, None, false);
        b.apply_settings(&ctx, &pairs);
        assert_eq!(b.tabs.order(), a.tabs.order());
        assert!(b.tabs.collapsed);
    }
}
