//! Renders the README screenshots headlessly (no window, no display needed).
//!
//! `cargo test -p millcreator render_screenshots -- --ignored --nocapture` writes PNG files into `docs/screenshots/`.
use crate::{App, Screen};
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
fn demo_job(a: &mut App) {
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
    let mut h: Shot = Harness::builder().with_size([1280.0, 800.0]).build_ui_state(
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
}
