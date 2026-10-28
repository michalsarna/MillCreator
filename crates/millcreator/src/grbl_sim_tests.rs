//! End-to-end tests of what MillCreator generates, against the real GRBL 1.1h firmware compiled for the PC (grbl-sim).
//!
//! The harness and the tests of the serial link itself (streaming, feed hold, abort, settings, `M0` pauses, spindle and arcs)
//! are in baseCreator (`base_ui::sim_test`, docs/grbl-sim.md there); these check the jobs of the toolpaths: that GRBL accepts
//! every line, that the bit change stops the job where it must, and that the machine ends at zero with the spindle off.
//! They are `#[ignore]`d because they need the simulator binary: build it with `tools/grbl-sim/build.sh` of baseCreator into
//! `target/grbl-sim` (or point `GRBL_SIM_EXE` at it) and run
//!
//! ```sh
//! cargo test -p millcreator grbl_sim -- --ignored --test-threads=1
//! ```
use crate::device::Device;
use crate::layers::default_layers;
use crate::materials::Stock;
use crate::tools::Library;
use crate::toolpath::{generate, to_gcode};
use base_core::geom::{Polyline, Pt, Xf};
use base_core::scene::{Kind, Scene};
use base_ui::link::{clean_gcode, Cmd};
use base_ui::sim_test::Session;

/// The G-code the program sends for `scene` (the first layer 2 mm deep in one pass, the engraving layer 1 mm).
fn job_of(scene: &Scene) -> Vec<String> {
    let (lib, dev, stock) = (Library::new(), Device::dummy(), Stock::default());
    let mut layers = default_layers(&lib);
    layers[0].depth = 2.0;
    layers[0].step_down = 2.0;
    layers[3].depth = 1.0;
    let prog = generate(scene, &layers, &lib, &dev, &stock);
    clean_gcode(&to_gcode(&prog, &dev, &layers, &lib, "sim"))
}

fn square(scene: &mut Scene, layer: usize, x: f64, y: f64, s: f64) {
    scene.add(layer, Kind::Rect { w: s, h: s }, Xf::translate(x, y));
}

#[test]
#[ignore = "needs grbl-sim: tools/grbl-sim/build.sh of baseCreator"]
fn grbl_sim_a_milled_square_is_accepted_and_ends_at_zero_with_the_spindle_off() {
    let mut s = Session::connect();
    s.wait_banner();
    s.fast_machine(&[]);
    let mut scene = Scene::default();
    square(&mut scene, 0, 50.0, 50.0, 40.0);
    let job = job_of(&scene);
    assert!(job.iter().any(|l| l.starts_with("M3 S")) && job.iter().any(|l| l.starts_with("G1 Z-")), "{job:?}");
    s.run_job(job, 120.0);
    s.wait_idle_at(0.0, 0.0, 60.0);
    // The program ends with M30: GRBL reports the spindle stopped.
    assert!(s.parser_state().contains("M5"), "the spindle is off");
    assert!(s.errors().is_empty(), "GRBL rejected lines: {:?}", s.errors_described());
}

#[test]
#[ignore = "needs grbl-sim: tools/grbl-sim/build.sh of baseCreator"]
fn grbl_sim_the_bit_change_stops_the_job_and_it_goes_on_when_told() {
    let mut s = Session::connect();
    s.wait_banner();
    s.fast_machine(&[]);
    // As the program starts a job: the pauses (M0) are the program's to hold.
    s.link.send(Cmd::Barriers(true));
    let mut scene = Scene::default();
    square(&mut scene, 0, 50.0, 50.0, 40.0);
    scene.add(3, Kind::Path(vec![Polyline::new(vec![Pt::new(10.0, 10.0), Pt::new(30.0, 10.0)], false)]), Xf::IDENTITY);
    let job = job_of(&scene);
    assert_eq!(job.iter().filter(|l| l.starts_with("M0 (Change the bit")).count(), 1, "{job:?}");
    let n = job.len();
    s.link.send(Cmd::Job(job));
    s.wait("the bit change", 120.0, |v| !v.barriers.is_empty());
    assert!(s.seen.barriers[0].0.contains("Change the bit to: V-bit"), "{:?}", s.seen.barriers);
    // Everything before it was done: the machine stands still and the job has not gone on.
    s.wait("idle at the pause", 60.0, |v| v.state == "Idle");
    assert!(s.seen.progress.0 < n, "{:?}", s.seen.progress);
    s.link.send(Cmd::Continue);
    s.wait("job finished", 120.0, |v| v.progress == (n, n));
    s.wait_idle_at(0.0, 0.0, 60.0);
    assert!(s.errors().is_empty(), "GRBL rejected lines: {:?}", s.errors_described());
}
