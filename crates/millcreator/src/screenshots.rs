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
    save(&mut h, "01-start.png");
    let badge = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/badge.svg").canonicalize().expect("example svg");
    {
        let a = h.state_mut().as_mut().expect("app built");
        a.open(badge);
        assert_eq!(a.screen, Screen::Editor);
        a.select_all();
        a.status = "Opened badge.svg".into();
    }
    save(&mut h, "02-editor-properties.png");
    h.state_mut().as_mut().expect("app built").side_tab = crate::SideTab::Clipart;
    save(&mut h, "03-clipart.png");
}
