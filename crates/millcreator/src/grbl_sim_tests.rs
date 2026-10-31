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
use crate::toolpath::{generate_with, to_gcode, JobOptions};
use base_core::geom::{Polyline, Pt, Xf};
use base_core::scene::{Kind, Scene};
use base_ui::link::{clean_gcode, Cmd};
use base_ui::sim_test::Session;

/// The G-code the program sends for `scene` (the first layer 2 mm deep in one pass, the engraving layer 1 mm).
fn job_of(scene: &Scene) -> Vec<String> {
    job_with(scene, JobOptions::default())
}

/// The same with the options of the Control tab.
fn job_with(scene: &Scene, opts: JobOptions) -> Vec<String> {
    let (lib, dev, stock) = (Library::new(), Device::dummy(), Stock::default());
    let mut layers = default_layers(&lib);
    layers[0].depth = 2.0;
    layers[0].step_down = 2.0;
    layers[1].depth = 2.0;
    layers[1].step_down = 2.0;
    layers[3].depth = 1.0;
    let prog = generate_with(scene, &layers, &lib, &dev, &stock, opts);
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

#[test]
#[ignore = "needs grbl-sim: tools/grbl-sim/build.sh of baseCreator"]
fn grbl_sim_one_element_at_a_time_runs_with_the_same_bit() {
    let mut s = Session::connect();
    s.wait_banner();
    s.fast_machine(&[]);
    // Two groups, each a square cut out (layer 0) and a smaller one inside it (layer 1, the same bit as layer 0).
    let mut scene = Scene::default();
    for x in [20.0, 100.0] {
        let outer = scene.add(0, Kind::Rect { w: 40.0, h: 40.0 }, Xf::translate(x, 20.0));
        let inner = scene.add(1, Kind::Rect { w: 20.0, h: 20.0 }, Xf::translate(x + 10.0, 30.0));
        let g = scene.new_group_id();
        for sh in scene.shapes.iter_mut().filter(|sh| sh.id == outer || sh.id == inner) {
            sh.group = Some(g);
        }
    }
    let job = job_with(&scene, JobOptions { one_by_one: true });
    assert!(!job.iter().any(|l| l.starts_with("M0")), "the same bit all through: no stop for a bit change");
    s.run_job(job, 240.0);
    s.wait_idle_at(0.0, 0.0, 60.0);
    assert!(s.errors().is_empty(), "GRBL rejected lines: {:?}", s.errors_described());
}

/// A job with ramps, a helix, tabs, climb milling and a finishing pass: layer 0 (cut out) with tabs and a ramp, layer 2 (pocket) with a helix.
fn job_with_entries(entry: crate::layers::Entry) -> Vec<String> {
    use crate::layers::{CutDir, TabKind, Tabs};
    let (lib, dev, stock) = (Library::new(), Device::dummy(), Stock::default());
    let mut layers = default_layers(&lib);
    layers[0].depth = 4.0;
    layers[0].step_down = 2.0;
    layers[0].direction = CutDir::Climb;
    layers[0].leave = 0.3;
    layers[0].entry = entry;
    layers[0].ramp_angle = 4.0;
    layers[0].tabs = Tabs { enabled: true, kind: TabKind::Ramp, length: 5.0, height: 1.5, spacing: 60.0, ..Tabs::default() };
    layers[2].depth = 2.0;
    layers[2].step_down = 1.0;
    layers[2].direction = CutDir::Conventional;
    layers[2].entry = crate::layers::Entry::Helix;
    layers[2].ramp_angle = 5.0;
    let mut scene = Scene::default();
    square(&mut scene, 0, 60.0, 60.0, 50.0);
    square(&mut scene, 2, 10.0, 10.0, 30.0);
    let prog = generate_with(&scene, &layers, &lib, &dev, &stock, JobOptions::default());
    clean_gcode(&to_gcode(&prog, &dev, &layers, &lib, "sim"))
}

#[test]
#[ignore = "needs grbl-sim: tools/grbl-sim/build.sh of baseCreator"]
fn grbl_sim_ramps_a_helix_tabs_and_a_finishing_pass_are_accepted_and_end_at_zero() {
    for entry in [crate::layers::Entry::RampContour, crate::layers::Entry::RampZigzag] {
        let mut s = Session::connect();
        s.wait_banner();
        s.fast_machine(&[]);
        let job = job_with_entries(entry);
        assert!(job.iter().filter(|l| l.starts_with("G1 X") && l.contains(" Z")).count() > 20, "ramps are moves in X, Y and Z: {job:?}");
        s.run_job(job, 240.0);
        s.wait_idle_at(0.0, 0.0, 60.0);
        assert!(s.parser_state().contains("M5"), "the spindle is off");
        assert!(s.errors().is_empty(), "{entry:?}: GRBL rejected lines: {:?}", s.errors_described());
    }
}

/// A job with the operations of stage 3: a raster pocket with an outline pass, a face, a bore, a chamfer, a V-carve, a dogbone slot and a trochoid slot.
fn job_with_operations(chamfer_only: bool) -> Vec<String> {
    use crate::layers::{OutlinePass, Operation, PocketPattern, Trochoid};
    let (lib, dev, stock) = (Library::new(), Device::dummy(), Stock::default());
    let mut layers = default_layers(&lib);
    let tool = |id: &str| lib.get(id).unwrap().clone();
    // 0: a face; 1: a bore; 2: a raster pocket with an outline pass; 3: a chamfer; 5: a dogbone slot; 6: a trochoid slot. All with one bit
    // (a bit change stops the job at an M0), so the chamfer, which needs a V-bit, is a job of its own (`chamfer_only`).
    for l in layers.iter_mut() {
        l.use_tool(&tool("std-flat-3"));
    }
    layers[3].use_tool(&tool("std-v-60"));
    layers[4].use_tool(&tool("std-v-60"));
    layers[4].op = Operation::VCarve;
    layers[4].depth = 2.0;
    layers[0].op = Operation::Face;
    layers[0].depth = 0.5;
    layers[0].step_down = 0.5;
    layers[0].overhang = 1.0;
    layers[1].op = Operation::Bore;
    layers[1].depth = 2.0;
    layers[1].step_down = 2.0;
    layers[2].depth = 1.5;
    layers[2].step_down = 1.5;
    layers[2].pattern = PocketPattern::Raster;
    layers[2].raster_angle = 30.0;
    layers[2].outline = OutlinePass::After;
    layers[3].op = Operation::Chamfer;
    layers[3].chamfer = 1.0;
    layers[5].op = Operation::DogboneSlot;
    layers[5].depth = 2.0;
    layers[5].step_down = 2.0;
    layers[5].use_tool(&tool("std-flat-3"));
    layers[6].op = Operation::OnPath;
    layers[6].depth = 1.0;
    layers[6].step_down = 1.0;
    layers[6].use_tool(&tool("std-flat-3"));
    layers[6].trochoid = Trochoid { enabled: true, diameter: 4.0, step: 2.0, half: false };
    let mut scene = Scene::default();
    square(&mut scene, 0, 10.0, 10.0, 40.0);
    scene.add(1, Kind::Ellipse { w: 14.0, h: 14.0 }, Xf::translate(70.0, 20.0));
    square(&mut scene, 2, 10.0, 70.0, 30.0);
    square(&mut scene, 3, 60.0, 60.0, 25.0);
    scene.add(4, Kind::Rect { w: 20.0, h: 6.0 }, Xf::translate(60.0, 100.0));
    scene.add(5, Kind::Rect { w: 30.0, h: 8.0 }, Xf::translate(100.0, 10.0));
    scene.add(6, Kind::Path(vec![Polyline::new(vec![Pt::new(100.0, 40.0), Pt::new(140.0, 40.0)], false)]), Xf::IDENTITY);
    for (i, l) in layers.iter_mut().enumerate() {
        l.enabled = (i == 3 || i == 4) == chamfer_only;
    }
    let prog = generate_with(&scene, &layers, &lib, &dev, &stock, JobOptions::default());
    clean_gcode(&to_gcode(&prog, &dev, &layers, &lib, "sim"))
}

#[test]
#[ignore = "needs grbl-sim: tools/grbl-sim/build.sh of baseCreator"]
fn grbl_sim_the_new_operations_are_accepted_and_end_at_zero_with_the_spindle_off() {
    let mut s = Session::connect();
    s.wait_banner();
    s.fast_machine(&[]);
    for chamfer_only in [true, false] {
        let job = job_with_operations(chamfer_only);
        assert!(job.len() > if chamfer_only { 10 } else { 500 }, "{} lines", job.len());
        assert!(!job.iter().any(|l| l.starts_with("M0")), "one bit: no pause");
        s.run_job(job, 900.0);
        s.wait_idle_at(0.0, 0.0, 120.0);
        assert!(s.parser_state().contains("M5"), "the spindle is off");
        assert!(s.errors().is_empty(), "GRBL rejected lines: {:?}", s.errors_described());
    }
}

