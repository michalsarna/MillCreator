//! Thread milling: a thread mill goes round a circle in a helix that climbs one pitch per turn. Inside a hole (a nut: the hole is
//! drilled or bored to the small diameter first) or round a boss (a screw: the boss is cut to the outer diameter first). The circle
//! that is drawn is the outer (major) diameter of the thread in both cases. The bit always climbs (it goes anticlockwise inside a
//! hole, clockwise round a boss, with a clockwise spindle); a right-hand thread goes up inside a hole and down round a boss, a
//! left-hand one the other way.
use crate::toolpath::{Builder, MoveKind};
use base_core::geom::{Polyline, Pt};
use serde::{Deserialize, Serialize};

/// Points of the helix per turn.
const STEPS: usize = 36;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Thread {
    /// The rise of one turn (mm).
    #[serde(default = "one")]
    pub pitch: f64,
    /// A thread inside a hole (a nut); otherwise round a boss (a screw).
    #[serde(default = "yes")]
    pub internal: bool,
    #[serde(default)]
    pub left: bool,
    /// The depth of the thread is cut in this many goes, each one a little farther out (in for a screw).
    #[serde(default = "one_pass")]
    pub passes: u32,
}

fn one() -> f64 {
    1.0
}
fn yes() -> bool {
    true
}
fn one_pass() -> u32 {
    1
}

impl Default for Thread {
    fn default() -> Thread {
        Thread { pitch: 1.0, internal: true, left: false, passes: 1 }
    }
}

/// Usual screws: the name, the outer diameter and the pitch (mm).
pub const STANDARDS: &[(&str, f64, f64)] = &[
    ("M3", 3.0, 0.5),
    ("M4", 4.0, 0.7),
    ("M5", 5.0, 0.8),
    ("M6", 6.0, 1.0),
    ("M8", 8.0, 1.25),
    ("M10", 10.0, 1.5),
    ("M12", 12.0, 1.75),
    ("M14", 14.0, 2.0),
    ("M16", 16.0, 2.0),
    ("M20", 20.0, 2.5),
    ("M24", 24.0, 3.0),
    ("M30", 30.0, 3.5),
    ("1/4\"-20 UNC", 6.35, 25.4 / 20.0),
    ("5/16\"-18 UNC", 7.9375, 25.4 / 18.0),
    ("3/8\"-16 UNC", 9.525, 25.4 / 16.0),
    ("1/2\"-13 UNC", 12.7, 25.4 / 13.0),
    ("5/8\"-11 UNC", 15.875, 25.4 / 11.0),
    ("3/4\"-10 UNC", 19.05, 25.4 / 10.0),
    ("1/4\"-28 UNF", 6.35, 25.4 / 28.0),
    ("3/8\"-24 UNF", 9.525, 25.4 / 24.0),
    ("1/2\"-20 UNF", 12.7, 25.4 / 20.0),
];

impl Thread {
    pub fn is_default(&self) -> bool {
        *self == Thread::default()
    }

    pub fn check(&self) -> Result<(), String> {
        if !(self.pitch.is_finite() && (0.1..=20.0).contains(&self.pitch)) || !(1..=20).contains(&self.passes) {
            return Err("a number of the thread is out of range".into());
        }
        Ok(())
    }

    /// How deep the thread is cut into the wall, radially (mm): 5/8 of the sharp-V height for a nut, 17/24 of it for a screw (ISO).
    pub fn depth(&self) -> f64 {
        let h = 3f64.sqrt() / 2.0 * self.pitch;
        if self.internal {
            h * 5.0 / 8.0
        } else {
            h * 17.0 / 24.0
        }
    }

    /// The standard size whose pitch this is, if it is one.
    pub fn standard_of(diameter: f64) -> Option<&'static (&'static str, f64, f64)> {
        STANDARDS.iter().find(|s| (s.1 - diameter).abs() < 0.05)
    }
}

/// The middle and the outer radius of a closed shape drawn for a thread, and whether it is round (within 2 %).
pub fn circle_of(ring: &Polyline) -> Option<(Pt, f64, bool)> {
    if !ring.closed || ring.pts.len() < 3 {
        return None;
    }
    let b = ring.bounds()?;
    let c = b.center();
    let r = ((b.max.x - b.min.x) + (b.max.y - b.min.y)) / 4.0;
    if !(r > 0.0) {
        return None;
    }
    let round = ring.pts.iter().all(|p| (p.dist(c) - r).abs() <= 0.02 * r + 0.01);
    Some((c, r, round))
}

/// What went wrong with one thread.
#[derive(Debug, PartialEq, Eq)]
pub enum Problem {
    /// The bit does not go into the hole drilled for the thread (or is as wide as the thread).
    TooBig,
}

/// The moves of one thread round `c` of the outer radius `r` to the depth `depth` (positive) with a bit of the radius `bit_r`.
pub(crate) fn mill(b: &mut Builder, c: Pt, r: f64, depth: f64, bit_r: f64, t: &Thread) -> Result<(), Problem> {
    let h = t.depth();
    let n = t.passes.clamp(1, 20) as usize;
    let small = r - h;
    if t.internal && bit_r >= small - 0.05 {
        return Err(Problem::TooBig);
    }
    // The radius of the centre of the bit at every pass: inside, from the drilled hole out to the full depth; round a boss, from
    // its outside in.
    let radii: Vec<f64> = (1..=n)
        .map(|k| {
            let f = k as f64 / n as f64;
            if t.internal {
                small + h * f - bit_r
            } else {
                r - h * f + bit_r
            }
        })
        .collect();
    // Anticlockwise inside a hole, clockwise round a boss; up inside for a right-hand thread, down round a boss.
    let turn = if t.internal { 1.0 } else { -1.0 };
    let up = t.internal != t.left;
    let (z0, z1) = if up { (-depth, 0.0) } else { (0.0, -depth) };
    let turns = depth / t.pitch;
    let steps = ((turns * STEPS as f64).ceil() as usize).max(1);
    // Where the bit waits between passes: the middle of the hole, or outside the boss.
    let park = |rr: f64| if t.internal { c } else { Pt::new(c.x + rr + bit_r + 1.0, c.y) };
    let outer = radii.iter().copied().fold(0.0, f64::max);
    let wait = park(outer);
    b.dive(wait.x, wait.y, z0);
    for (k, rr) in radii.iter().enumerate() {
        if k > 0 {
            b.push(MoveKind::Cut, [wait.x, wait.y, z0], b.feed);
        }
        // Onto the circle at the angle 0, then the helix.
        b.push(MoveKind::Cut, [c.x + rr, c.y, z0], b.feed / 2.0);
        for s in 1..=steps {
            let f = s as f64 / steps as f64;
            let a = turn * f * turns * std::f64::consts::TAU;
            b.push(MoveKind::Cut, [c.x + rr * a.cos(), c.y + rr * a.sin(), z0 + (z1 - z0) * f], b.feed);
        }
        // Off the wall, the same way it came in.
        let end = turn * turns * std::f64::consts::TAU;
        let away = if t.internal { c } else { Pt::new(c.x + (outer + bit_r + 1.0) * end.cos(), c.y + (outer + bit_r + 1.0) * end.sin()) };
        b.push(MoveKind::Cut, [away.x, away.y, z1], b.feed / 2.0);
        if !t.internal {
            b.push(MoveKind::Cut, [wait.x, wait.y, z1], b.feed);
        }
    }
    // Up out of the hole at the plunge feed to just above the material (the thread may end at its top), then away.
    if b.at[2] < 1.0 {
        let [x, y, _] = b.at;
        b.push(MoveKind::Plunge, [x, y, 1.0], b.plunge);
    }
    b.retract();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_depth_of_a_thread_follows_iso() {
        let nut = Thread { pitch: 1.0, ..Thread::default() };
        assert!((nut.depth() - 0.5413).abs() < 1e-3);
        let screw = Thread { internal: false, ..nut };
        assert!((screw.depth() - 0.6134).abs() < 1e-3);
        assert_eq!(Thread::standard_of(8.0).map(|s| s.2), Some(1.25));
        assert!(Thread::standard_of(7.0).is_none());
        assert!(Thread::default().check().is_ok());
        assert!(Thread { pitch: 0.0, ..Thread::default() }.check().is_err());
        assert!(Thread { passes: 0, ..Thread::default() }.check().is_err());
        let t: Thread = serde_json::from_str("{}").unwrap();
        assert!(t.is_default());
    }

    #[test]
    fn a_circle_is_found_and_a_square_is_not_round() {
        let circle = Polyline::new((0..64).map(|i| {
            let a = i as f64 / 64.0 * std::f64::consts::TAU;
            Pt::new(10.0 + 4.0 * a.cos(), 20.0 + 4.0 * a.sin())
        }).collect(), true);
        let (c, r, round) = circle_of(&circle).unwrap();
        assert!(c.dist(Pt::new(10.0, 20.0)) < 1e-6 && (r - 4.0).abs() < 0.01 && round);
        let square = Polyline::new(vec![Pt::new(0.0, 0.0), Pt::new(8.0, 0.0), Pt::new(8.0, 8.0), Pt::new(0.0, 8.0)], true);
        assert!(!circle_of(&square).unwrap().2);
    }
}

/// The rows of the layer's settings for a thread (a grid of two columns). True when something changed.
pub(crate) fn rows(ui: &mut egui::Ui, units: base_core::units::Units, t: &mut Thread) -> bool {
    use base_ui::i18n::tr;
    let mut changed = false;
    ui.label(tr("Thread"));
    ui.horizontal(|ui| {
        changed |= ui.selectable_value(&mut t.internal, true, tr("Inside (a nut)")).changed();
        changed |= ui.selectable_value(&mut t.internal, false, tr("Outside (a screw)")).changed();
    });
    ui.end_row();
    ui.label(tr("Usual size"));
    let current = STANDARDS.iter().find(|s| (s.2 - t.pitch).abs() < 1e-6).map_or("—".to_string(), |s| s.0.to_string());
    egui::ComboBox::from_id_salt("thread_std").selected_text(current).width(160.0).show_ui(ui, |ui| {
        for s in STANDARDS {
            if ui.selectable_label((s.2 - t.pitch).abs() < 1e-6, format!("{}  Ø {} · {} mm", s.0, crate::toolpath::num(s.1), crate::toolpath::num(s.2))).clicked() {
                t.pitch = s.2;
                changed = true;
            }
        }
    })
    .response
    .on_hover_text(tr("Takes the pitch of a usual screw. Draw circles of its outer diameter (Ø)."));
    ui.end_row();
    ui.label(tr("Pitch"));
    changed |= base_ui::units_ui::drag_len(ui, units, &mut t.pitch, 0.01, Some((0.1, 20.0))).on_hover_text(tr("The rise of one turn of the thread")).changed();
    ui.end_row();
    ui.label(tr("Radial passes"));
    changed |= ui.add(egui::DragValue::new(&mut t.passes).range(1..=20)).on_hover_text(tr("The depth of the thread is cut in this many goes, each one a little deeper into the wall")).changed();
    ui.end_row();
    ui.label("");
    changed |= ui.checkbox(&mut t.left, tr("Left-hand thread")).changed();
    ui.end_row();
    changed
}
