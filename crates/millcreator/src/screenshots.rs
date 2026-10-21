//! Renders the README screenshots headlessly (no window, no display needed).
//!
//! `cargo test -p millcreator render_screenshots -- --ignored --nocapture` writes PNG files into `docs/screenshots/`.
use crate::App;
use base_ui::editor::EditorCommands;
use egui_kittest::Harness;
use std::path::PathBuf;

type Shot = Harness<'static, Option<App>>;

fn save(h: &mut Shot, name: &str) {
    h.run_steps(5);
    let img = h.render().expect("render");
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/screenshots");
    std::fs::create_dir_all(&dir).expect("create screenshot dir");
    let p = dir.join(name);
    img.save(&p).expect("write png");
    println!("wrote {}", p.display());
}

/// A small plaque: the outline cut out, two holes drilled, a pocket, and a star engraved.
pub(crate) fn demo_job(a: &mut App) {
    use base_core::bezier::Contour;
    use base_core::geom::Xf;
    use base_core::scene::Kind;
    a.new_design();
    a.scene.add(0, Kind::Rect { w: 150.0, h: 90.0 }, Xf::translate(75.0, 55.0));
    a.scene.add(4, Kind::Ellipse { w: 6.0, h: 6.0 }, Xf::translate(82.0, 62.0));
    a.scene.add(4, Kind::Ellipse { w: 6.0, h: 6.0 }, Xf::translate(212.0, 62.0));
    a.scene.add(4, Kind::Ellipse { w: 6.0, h: 6.0 }, Xf::translate(82.0, 128.0));
    a.scene.add(4, Kind::Ellipse { w: 6.0, h: 6.0 }, Xf::translate(212.0, 128.0));
    a.scene.add(2, Kind::Rect { w: 56.0, h: 36.0 }, Xf::translate(95.0, 82.0));
    a.scene.add(3, Kind::Bezier(vec![Contour::star(5, 0.45, 44.0, 44.0)]), Xf::translate(165.0, 78.0));
    a.description = "Plaque, birch plywood 10 mm.\nCut out last, so the part is held until then.".into();
    a.layers[0].name = "Cut out".into();
    a.touch_rev();
}

fn app(h: &mut Shot) -> &mut App {
    h.state_mut().as_mut().expect("app built")
}

#[test]
#[ignore = "writes docs/screenshots; run explicitly"]
fn render_screenshots() {
    let mut h: Shot = Harness::builder().with_accessibility_check(false).with_size([1280.0, 800.0]).build_ui_state(
        |ui, state: &mut Option<App>| {
            let a = state.get_or_insert_with(|| App::build(ui.ctx(), None, false));
            a.draw(ui);
        },
        None,
    );
    h.run_steps(2);
    {
        let a = app(&mut h);
        a.devices.clear();
        let mut router = crate::device::Device::dummy();
        router.name = "Desktop router 300 × 200".into();
        a.devices.push(router);
        a.use_dummy_device();
        a.start_sel = 0;
    }
    save(&mut h, "01-start.png");
    {
        let a = app(&mut h);
        a.start_add_device();
        let e = a.dev_edit.as_mut().expect("window");
        e.draft.name = "Big router".into();
        e.draft.area.x = 600.0;
        e.draft.area.y = 400.0;
    }
    save(&mut h, "02-add-device.png");
    {
        let a = app(&mut h);
        a.dev_edit.as_mut().expect("window").page = crate::device_ui::Page::Camera;
        a.dev_edit.as_mut().expect("window").draft.cameras.push(base_core::camimg::CameraSource::url("Over the table", "http://raspberrypi.local:8080/stream.mjpg", 0));
    }
    save(&mut h, "03-device-camera.png");
    {
        let a = app(&mut h);
        a.dev_edit = None;
        a.start_sel = 0;
        demo_job(a);
        a.select_all();
        a.status = "Plaque: five layers in use".into();
    }
    save(&mut h, "04-editor-properties.png");
    app(&mut h).side_tab = crate::SideTab::Layers;
    app(&mut h).ed.active_layer = 2;
    save(&mut h, "05-layers.png");
    app(&mut h).side_tab = crate::SideTab::Material;
    save(&mut h, "06-material.png");
    app(&mut h).side_tab = crate::SideTab::Device;
    save(&mut h, "07-device-tab.png");
    app(&mut h).side_tab = crate::SideTab::Clipart;
    save(&mut h, "08-clipart.png");
    app(&mut h).side_tab = crate::SideTab::Description;
    save(&mut h, "09-description.png");
    // The bit library with a few kinds of bits.
    app(&mut h).side_tab = crate::SideTab::Layers;
    for (id, name) in [("std-flat-6", "10-bits-flat.png"), ("std-v-60", "11-bits-vbit.png"), ("std-bull-6", "12-bits-bull.png"), ("std-tball-10", "13-bits-tapered.png"), ("std-eng-20", "14-bits-engraver.png"), ("std-drill-3", "15-bits-drill.png")] {
        let a = app(&mut h);
        a.tools_win.open = true;
        let lib = a.tools.clone();
        a.tools_win.select(&lib, id);
        save(&mut h, name);
    }
    app(&mut h).tools_win.open = false;
    {
        let a = app(&mut h);
        a.open_preview();
    }
    save(&mut h, "16-3d-view.png");
    {
        let a = app(&mut h);
        a.view.rotate(-60.0, 40.0);
        a.view.zoom_by(1.4);
        a.view.progress = 0.55;
    }
    save(&mut h, "17-3d-view-turned.png");
    {
        let a = app(&mut h);
        a.show_3d = false;
        a.stock.set_kind(crate::materials::MaterialKind::Plastic);
        a.stock.color = [0x3a, 0x7f, 0xd0];
        a.view.reset();
        a.view.progress = 1.0;
        a.open_preview();
    }
    save(&mut h, "18-3d-view-plastic.png");
    // The plan from above, with the animation controls, and the device pages of the connection and the rotary.
    {
        let a = app(&mut h);
        a.view.top = true;
        a.view.progress = 0.6;
    }
    save(&mut h, "19-preview-from-above.png");
    {
        let a = app(&mut h);
        a.show_3d = false;
        a.view.top = false;
        a.view.progress = 1.0;
        a.toolpaths_on = true;
        a.side_tab = crate::SideTab::Layers;
    }
    save(&mut h, "20-toolpaths-on-the-work-area.png");
    {
        let a = app(&mut h);
        a.toolpaths_on = false;
        a.start_edit_device(0);
        a.dev_edit.as_mut().expect("window").page = crate::device_ui::Page::Connection;
    }
    save(&mut h, "21-device-connection.png");
    {
        let a = app(&mut h);
        let e = a.dev_edit.as_mut().expect("window");
        e.page = crate::device_ui::Page::Rotary;
        e.draft.rotary.enabled = true;
    }
    save(&mut h, "22-device-rotary.png");
    app(&mut h).dev_edit = None;
    app(&mut h).side_tab = crate::SideTab::Device;
    save(&mut h, "23-device-tab-machine.png");
    {
        use base_ui::link::{ConsoleLine, Dir};
        let a = app(&mut h);
        let line = |dir, text: &str| ConsoleLine { dir, text: text.into(), poll: false };
        a.machine.console.clear();
        a.machine.console.push(ConsoleLine::info("Connected to /dev/ttyUSB0 @ 115200"));
        a.machine.console.push(line(Dir::Rx, "Grbl 1.1h ['$' for help]"));
        for (t, r) in [("$H", "ok"), ("G21 G90", "ok"), ("G0 Z5", "ok"), ("G0 X10 Y10", "ok")] {
            a.machine.console.push(line(Dir::Tx, t));
            a.machine.console.push(line(Dir::Rx, r));
        }
        a.show_3d = false;
        a.side_tab = crate::SideTab::Console;
    }
    save(&mut h, "24-console.png");
    // The Control tab with a machine that is connected, and the windows of the machine.
    {
        let a = app(&mut h);
        a.machine.connected = true;
        a.machine.state = "Idle".into();
        a.machine.mpos = [-120.5, -42.25, -5.0];
        a.machine.wpos = [29.5, 107.75, 12.0];
        a.machine.pos = a.machine.wpos;
        a.machine.wcs = 2;
        a.machine.fw.info = base_core::firmware::FirmwareInfo { name: "Grbl".into(), version: "1.1h".into(), options: "V".into(), raw: vec!["[VER:1.1h.20190825:]".into(), "[OPT:V,15,128]".into()] };
        a.device.control.macros = vec![base_core::control::Macro { name: "Park".into(), code: "G0 Z[safez]\nG0 X0 Y0".into() }, base_core::control::Macro { name: "Spindle test".into(), code: "S8000 M3\nG4 P3\nM5".into() }];
        a.side_tab = crate::SideTab::Control;
    }
    save(&mut h, "25-control-tab.png");
    {
        let a = app(&mut h);
        a.machine.fw.values = [(30, "24000.000"), (31, "1000.000"), (32, "0"), (100, "800.000"), (101, "800.000"), (102, "800.000"), (110, "3000.000"), (111, "3000.000"), (112, "500.000"), (130, "300.000"), (131, "200.000"), (132, "60.000")].into_iter().map(|(k, v)| (k, v.to_string())).collect();
        a.fw_open = true;
    }
    save(&mut h, "26-firmware.png");
    {
        let a = app(&mut h);
        a.fw_open = false;
        a.cal.open = true;
    }
    save(&mut h, "27-calibration.png");
    {
        let a = app(&mut h);
        a.cal.open = false;
        a.machine.pins = "P".into();
        a.device.control.stats.job_done(7200);
        a.device.control.stats.job_done(3600);
        a.device.control.stats.alarm(1);
        a.device.control.stats.error(20);
        a.device.control.reminders.push(base_core::control::Reminder { name: "Oil the rails".into(), every_hours: 2.0, done_at_hours: 0.0 });
        a.diag.open = true;
    }
    save(&mut h, "28-diagnostics.png");
    {
        let a = app(&mut h);
        a.diag.open = false;
        a.machine.pins.clear();
        a.start_lines = "G21 G90 G54\nM5\nG0 Z5\nG0 X10 Y10\nS12000 M3\nG4 P2\nF300\nG1 Z-1\nG1 X50 Y10\nG1 X50 Y40\nG1 X10 Y40\nG0 Z5\nM5\nM30".lines().map(String::from).collect();
        a.start_from.line = 10;
        a.start_from.open = true;
    }
    save(&mut h, "29-start-from-line.png");
    {
        let a = app(&mut h);
        a.start_from.open = false;
        a.machine.barrier = Some(base_ui::machine_ctl::Barrier { text: "M0 (Change the bit to: V-bit 60°)".into(), line: 120 });
    }
    save(&mut h, "30-job-paused.png");
    {
        let a = app(&mut h);
        a.machine.barrier = None;
        a.start_edit_device(0);
        a.dev_edit.as_mut().expect("window").page = crate::device_ui::Page::Probe;
    }
    save(&mut h, "31-device-probe.png");
    {
        let a = app(&mut h);
        let e = a.dev_edit.as_mut().expect("window");
        e.page = crate::device_ui::Page::Macros;
        e.draft.control.macros = vec![base_core::control::Macro { name: "Park".into(), code: "G0 Z[safez]\nG0 X0 Y0".into() }];
        e.draft.control.hooks.after_job = "M5\nG0 Z[safez]".into();
    }
    save(&mut h, "32-device-macros.png");
    {
        let a = app(&mut h);
        a.dev_edit = None;
        a.lang = base_ui::i18n::Lang::Pl;
        base_ui::i18n::set_lang(base_ui::i18n::Lang::Pl);
        a.side_tab = crate::SideTab::Control;
    }
    save(&mut h, "33-control-tab-polish.png");
    {
        let a = app(&mut h);
        a.cal.open = true;
        a.machine.fw.values = [(100, "800.000".to_string())].into_iter().collect();
    }
    save(&mut h, "34-calibration-polish.png");
    {
        let a = app(&mut h);
        a.cal.open = false;
        a.lang = base_ui::i18n::Lang::En;
        base_ui::i18n::set_lang(base_ui::i18n::Lang::En);
        a.device.control.tls = base_core::control::Tls { set: true, x: -12.5, y: -8.0, start_z: -25.0, ref_wz: Some(-31.3), ..Default::default() };
        a.tls_ui.open = true;
    }
    save(&mut h, "35-tool-length-sensor.png");
    {
        let a = app(&mut h);
        a.tls_ui.open = false;
        let v = |l: &[(u32, &str)]| -> std::collections::BTreeMap<u32, String> { l.iter().map(|(k, x)| (*k, x.to_string())).collect() };
        a.device.control.fw_copies.0.clear();
        a.device.control.fw_copies.make(1_791_800_000, "Grbl 1.1h", &v(&[(110, "3000.000"), (130, "300.000")]));
        a.device.control.fw_copies.make(1_791_900_000, "Grbl 1.1h", &v(&[(110, "5000.000"), (130, "300.000")]));
        a.machine.fw.changes = vec![base_core::control::Change { id: 110, old: Some("3000.000".into()), new: Some("5000.000".into()) }];
        a.fw_open = true;
    }
    save(&mut h, "36-firmware-copies.png");
    {
        let a = app(&mut h);
        a.fw_open = false;
        a.machine.fw.changes.clear();
    }
}

#[cfg(test)]
mod multi_select_tests {
    use super::*;
    use base_ui::editor::EditorHost;
    use egui::{Event, Modifiers, PointerButton};

    fn click(h: &mut Shot, pos: egui::Pos2, m: Modifiers) {
        h.event(Event::ModifiersChanged(m));
        h.event(Event::PointerMoved(pos));
        h.run_steps(2);
        h.event(Event::PointerButton { pos, button: PointerButton::Primary, pressed: true, modifiers: m });
        h.run_steps(2);
        h.event(Event::PointerButton { pos, button: PointerButton::Primary, pressed: false, modifiers: m });
        h.run_steps(3);
        h.event(Event::ModifiersChanged(Modifiers::NONE));
        h.run_steps(1);
    }

    #[test]
    fn shift_and_ctrl_click_add_to_the_selection_and_take_out_again() {
        let mut h: Shot = Harness::builder().with_accessibility_check(false).with_size([1280.0, 800.0]).build_ui_state(
            |ui, state: &mut Option<App>| {
                let a = state.get_or_insert_with(|| App::build(ui.ctx(), None, false));
                a.draw(ui);
            },
            None,
        );
        h.run_steps(2);
        {
            let a = app(&mut h);
            a.use_dummy_device();
            a.screen = crate::Screen::Editor;
            a.new_design();
            // Two rectangles far apart.
            a.scene.add(0, base_core::scene::Kind::Rect { w: 30.0, h: 30.0 }, base_core::geom::Xf::translate(40.0, 40.0));
            a.scene.add(0, base_core::scene::Kind::Rect { w: 30.0, h: 30.0 }, base_core::geom::Xf::translate(160.0, 40.0));
            a.ed.view.need_fit = false;
        }
        h.run_steps(3);
        let (p1, p2) = {
            let a = app(&mut h);
            let o = a.ed.canvas_origin;
            (a.w2s(o, base_core::geom::Pt::new(55.0, 55.0)), a.w2s(o, base_core::geom::Pt::new(175.0, 55.0)))
        };
        // A plain click selects one; a Ctrl / Cmd click adds the other; the same click again takes it out.
        let add = Modifiers::COMMAND;
        click(&mut h, p1, Modifiers::NONE);
        assert_eq!(app(&mut h).ed.sel.len(), 1);
        click(&mut h, p2, add);
        assert_eq!(app(&mut h).ed.sel.len(), 2, "Ctrl / Cmd click adds");
        click(&mut h, p2, add);
        assert_eq!(app(&mut h).ed.sel.len(), 1, "and takes out again");
        click(&mut h, p2, Modifiers::SHIFT);
        assert_eq!(app(&mut h).ed.sel.len(), 2, "Shift still works");
        click(&mut h, p2, Modifiers::NONE);
        assert_eq!(app(&mut h).ed.sel.len(), 1, "a plain click replaces the selection");
    }
}

#[cfg(test)]
mod context_menu_tests {
    use super::*;
    use egui::{Event, Modifiers, PointerButton};
    use base_ui::editor::EditorHost;
    use egui_kittest::kittest::Queryable;

    #[test]
    fn a_right_click_on_a_shape_opens_the_menu_and_its_commands_work() {
        let mut h: Shot = Harness::builder().with_accessibility_check(false).with_size([1280.0, 800.0]).build_ui_state(
            |ui, state: &mut Option<App>| {
                let a = state.get_or_insert_with(|| App::build(ui.ctx(), None, false));
                a.draw(ui);
            },
            None,
        );
        h.run_steps(2);
        {
            let a = app(&mut h);
            a.use_dummy_device();
            a.screen = crate::Screen::Editor;
            a.new_design();
            a.scene.add(0, base_core::scene::Kind::Rect { w: 30.0, h: 30.0 }, base_core::geom::Xf::translate(40.0, 40.0));
            a.ed.view.need_fit = false;
        }
        h.run_steps(3);
        let at = {
            let a = app(&mut h);
            let o = a.ed.canvas_origin;
            a.w2s(o, base_core::geom::Pt::new(55.0, 55.0))
        };
        let button = |pressed| Event::PointerButton { pos: at, button: PointerButton::Secondary, pressed, modifiers: Modifiers::NONE };
        h.event(Event::PointerMoved(at));
        h.run_steps(2);
        h.event(button(true));
        h.run_steps(2);
        h.event(button(false));
        h.run_steps(4);
        assert_eq!(app(&mut h).ed.sel.len(), 1, "the shape under the pointer is selected");
        if let Ok(p) = std::env::var("SHOT") {
            h.render().unwrap().save(p).unwrap();
        }
        // (the submenus are looked at on the picture; their labels are not found by the test harness)
        // Duplicate is in the menu and works.
        h.get_by_label("Duplicate").click();
        h.run_steps(4);
        assert_eq!(app(&mut h).scene.shapes.len(), 2, "Duplicate from the context menu");
    }
}

#[cfg(test)]
mod window_size_tests {
    use super::*;

    fn rect_of(h: &Shot, id: &'static str) -> Option<egui::Rect> {
        h.ctx.memory(|m| m.area_rect(egui::Id::unique(Some(id))))
    }

    #[test]
    fn the_preview_and_the_bit_library_keep_their_size_and_do_not_grow_to_the_screen() {
        let mut h: Shot = Harness::builder().with_accessibility_check(false).with_size([1600.0, 1000.0]).build_ui_state(
            |ui, state: &mut Option<App>| {
                let a = state.get_or_insert_with(|| App::build(ui.ctx(), None, false));
                a.draw(ui);
            },
            None,
        );
        h.run_steps(2);
        {
            let a = app(&mut h);
            a.use_dummy_device();
            a.screen = crate::Screen::Editor;
            demo_job(a);
            a.show_3d = true;
            a.tools_win.open = true;
        }
        let mut seen = vec![];
        for _ in 0..14 {
            h.run_steps(1);
            seen.push((rect_of(&h, "preview_3d").map(|r| (r.width(), r.height())), rect_of(&h, "tool_library").map(|r| (r.width(), r.height()))));
        }
        let (first, last) = (seen[2], *seen.last().unwrap());
        assert_eq!(first, last, "the windows keep their size: {seen:?}");
        assert!(last.0.is_some() && last.1.is_some(), "both windows are shown");
        assert!(last.0.unwrap().1 < 800.0 && last.1.unwrap().1 < 800.0, "{seen:?}");
    }
}

#[cfg(test)]
mod plugin_import_window_tests {
    use super::*;
    use egui_kittest::kittest::Queryable;

    /// An importer with a form and a preview: a window opens, the preview is made in the background, and Import puts the shapes
    /// of the form as it was left into the drawing.
    #[test]
    fn an_importer_with_a_form_opens_its_window_and_import_adds_the_shapes() {
        let base = std::env::temp_dir().join(format!("mill-import-window-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let p = base.join("plugins/form-demo");
        std::fs::create_dir_all(&p).unwrap();
        std::fs::write(p.join("plugin.json"), r#"{"id":"form-demo","name":"Form demo","importers":[{"id":"frm","name":"Form","extensions":["frm"],"script":"read.rhai","preview":true,"params":[{"id":"w","label":"Width","kind":"number","default":10}]}]}"#).unwrap();
        std::fs::write(p.join("read.rhai"), "fn read(text, ctx) { rect(0.0, 0.0, ctx.p.w, 5.0) }").unwrap();
        let file = base.join("a.frm");
        std::fs::write(&file, "x").unwrap();
        let mut h: Shot = Harness::builder().with_accessibility_check(false).with_size([1280.0, 800.0]).build_ui_state(
            |ui, state: &mut Option<App>| {
                let a = state.get_or_insert_with(|| App::build(ui.ctx(), None, false));
                a.draw(ui);
            },
            None,
        );
        h.run_steps(2);
        {
            let a = app(&mut h);
            a.use_dummy_device();
            a.screen = crate::Screen::Editor;
            a.new_design();
            a.plugins = base_ui::plugins_ui::PluginsUi::new("millcreator", &base);
            a.open(file);
            assert!(a.scene.shapes.is_empty(), "nothing is added until Import is pressed");
        }
        // The preview comes from another thread.
        for _ in 0..200 {
            h.run_steps(2);
            std::thread::sleep(std::time::Duration::from_millis(10));
            if h.query_by_label_contains("10.0 × 5.0").is_some() {
                break;
            }
        }
        h.get_by_label("Import").click();
        h.run_steps(4);
        let a = app(&mut h);
        assert_eq!(a.scene.shapes.len(), 1, "{}", a.status);
        let b = a.scene.shapes[0].bounds().unwrap();
        assert!((b.max.x - b.min.x - 10.0).abs() < 1e-6, "the width of the form: {b:?}");
        let _ = std::fs::remove_dir_all(&base);
    }
}
