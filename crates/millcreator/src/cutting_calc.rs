//! The numbers of cutting that go together: the cutting speed Vc (m/min), the spindle speed n (rpm), the feed per tooth fz (mm)
//! and the feed F (mm/min) of a bit with the diameter D (mm) and z flutes. Only the spindle speed and the feed are kept in
//! files: the others are worked out from them and from the bit.
//!
//! Vc = π · D · n / 1000, fz = F / (n · z).
use eframe::egui;

/// The cutting speed in m/min of a bit of diameter `d` mm at `rpm`.
pub fn vc(d: f64, rpm: f64) -> f64 {
    std::f64::consts::PI * d * rpm / 1000.0
}

/// The spindle speed that gives the cutting speed `vc` on a bit of diameter `d` (0 when the bit has no diameter).
pub fn rpm_for(d: f64, vc: f64) -> f64 {
    if d > 0.0 {
        vc * 1000.0 / (std::f64::consts::PI * d)
    } else {
        0.0
    }
}

/// The feed per tooth in mm of a feed `feed` mm/min at `rpm` on a bit with `z` flutes (0 when it cannot be told).
pub fn fz(feed: f64, rpm: f64, z: u32) -> f64 {
    let t = rpm * z as f64;
    if t > 0.0 {
        feed / t
    } else {
        0.0
    }
}

/// The feed in mm/min that gives the feed per tooth `fz`.
pub fn feed_for(fz: f64, rpm: f64, z: u32) -> f64 {
    fz * rpm * z as f64
}

/// The rows under the speed and the feeds: Vc beside the spindle speed, fz beside the feed and the plunge feed. Typing in one of
/// them changes the spindle speed or the feed, the other follows. The rows are dimmed when the layer has no bit (there is no
/// diameter and no number of flutes then). Returns whether anything changed.
pub fn rows(ui: &mut egui::Ui, d: f64, z: u32, rpm: &mut f64, feed: &mut f64, plunge: &mut f64) -> bool {
    use crate::i18n::tr;
    let mut changed = false;
    let known = d > 0.0 && z >= 1;
    let hint = tr("From the bit: the diameter and the number of flutes");
    ui.add_enabled_ui(known, |ui| {
        let mut v = vc(d, *rpm);
        ui.label(tr("Cutting speed (Vc)")).on_disabled_hover_text(hint.clone());
        if ui.add(egui::DragValue::new(&mut v).range(0.0..=5000.0).speed(0.5).max_decimals(1).suffix(" m/min")).changed() {
            *rpm = rpm_for(d, v).clamp(0.0, 1_000_000.0);
            changed = true;
        }
        ui.end_row();
        let mut f = fz(*feed, *rpm, z);
        ui.label(tr("Feed per tooth (fz)"));
        if ui.add(egui::DragValue::new(&mut f).range(0.0..=10.0).speed(0.001).max_decimals(4).suffix(" mm")).changed() && *rpm > 0.0 {
            *feed = feed_for(f, *rpm, z).clamp(1.0, 100_000.0);
            changed = true;
        }
        ui.end_row();
        let mut p = fz(*plunge, *rpm, z);
        ui.label(tr("Plunge per tooth"));
        if ui.add(egui::DragValue::new(&mut p).range(0.0..=10.0).speed(0.001).max_decimals(4).suffix(" mm")).changed() && *rpm > 0.0 {
            *plunge = feed_for(p, *rpm, z).clamp(1.0, 100_000.0);
            changed = true;
        }
        ui.end_row();
    });
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cutting_speed_and_the_feed_per_tooth_follow_the_formulas() {
        // 6 mm at 18000 rpm: π · 6 · 18000 / 1000 = 339.29 m/min.
        assert!((vc(6.0, 18000.0) - 339.292).abs() < 0.01);
        assert!((rpm_for(6.0, vc(6.0, 18000.0)) - 18000.0).abs() < 1e-6, "the way back");
        assert_eq!(rpm_for(0.0, 100.0), 0.0);
        // 900 mm/min at 18000 rpm with 2 flutes: 0.025 mm per tooth.
        assert!((fz(900.0, 18000.0, 2) - 0.025).abs() < 1e-12);
        assert!((feed_for(0.025, 18000.0, 2) - 900.0).abs() < 1e-9);
        assert_eq!(fz(900.0, 0.0, 2), 0.0, "a stopped spindle has no feed per tooth");
        assert_eq!(fz(900.0, 18000.0, 0), 0.0);
    }

    #[test]
    fn typing_one_value_changes_the_other_and_nothing_is_saved_but_rpm_and_feed() {
        let ctx = egui::Context::default();
        let (mut rpm, mut feed, mut plunge) = (12000.0, 800.0, 300.0);
        let mut changed = true;
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::Grid::new("t").show(ui, |ui| changed = rows(ui, 6.0, 2, &mut rpm, &mut feed, &mut plunge));
        });
        out.textures_delta.clear();
        assert!(!changed && (rpm, feed, plunge) == (12000.0, 800.0, 300.0), "drawing changes nothing");
        // What a typed Vc does to the spindle speed, and a typed fz to the feed.
        rpm = rpm_for(6.0, 200.0);
        assert!((vc(6.0, rpm) - 200.0).abs() < 1e-9);
        feed = feed_for(0.05, rpm, 2);
        assert!((fz(feed, rpm, 2) - 0.05).abs() < 1e-12);
    }
}
