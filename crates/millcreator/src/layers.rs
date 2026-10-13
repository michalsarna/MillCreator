//! The layers of a milling project. A layer says what is done with the shapes that lie on it: cut outside or inside
//! the line, follow the line, clear a pocket or drill, with which bit, how deep and how fast.
use crate::tools::Tool;
use serde::{Deserialize, Serialize};

/// How many layers a project has.
pub const LAYER_COUNT: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
    /// The bit goes round the outside of the line: the shape stays (a part is cut out).
    ProfileOutside,
    /// The bit goes round the inside of the line: the shape falls out (a hole or a window).
    ProfileInside,
    /// The centre of the bit follows the line (engraving, grooves).
    OnPath,
    /// All of the area inside the line is cleared.
    Pocket,
    /// A hole at the centre of every closed shape.
    Drill,
}

impl Operation {
    pub const ALL: [Operation; 5] = [Operation::ProfileOutside, Operation::ProfileInside, Operation::OnPath, Operation::Pocket, Operation::Drill];

    pub fn label(self) -> &'static str {
        match self {
            Operation::ProfileOutside => "Cut outside the line",
            Operation::ProfileInside => "Cut inside the line",
            Operation::OnPath => "Follow the line",
            Operation::Pocket => "Pocket",
            Operation::Drill => "Drill",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Operation::ProfileOutside => "The bit goes round the outside of the line, so the shape stays: a part is cut out of the material.",
            Operation::ProfileInside => "The bit goes round the inside of the line, so the shape falls out: a hole or a window.",
            Operation::OnPath => "The centre of the bit follows the line: engraving and grooves. Open lines work too.",
            Operation::Pocket => "All of the area inside the line is cleared, ring after ring, to the depth of the layer.",
            Operation::Drill => "A hole at the centre of every closed shape of the layer, in steps of the pass depth.",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MillLayer {
    pub name: String,
    pub color: [u8; 3],
    /// Shown in the work area.
    pub visible: bool,
    /// Taken into the toolpaths and the G-code.
    pub enabled: bool,
    pub op: Operation,
    /// The id of the bit in the library.
    pub tool: Option<String>,
    /// The total depth below the top of the material (mm).
    pub depth: f64,
    /// How deep one pass goes (mm).
    pub step_down: f64,
    /// How far the next ring of a pocket is, as a part of the diameter (percent).
    pub step_over_pct: f64,
    /// Feed in the material (mm/min).
    pub feed: f64,
    /// Feed of the plunge (mm/min).
    pub plunge: f64,
    pub rpm: f64,
}

pub const PALETTE: [[u8; 3]; LAYER_COUNT] = [
    [0x1f, 0x77, 0xb4],
    [0xd6, 0x27, 0x28],
    [0x2c, 0xa0, 0x2c],
    [0xff, 0x7f, 0x0e],
    [0x94, 0x67, 0xbd],
    [0x8c, 0x56, 0x4b],
    [0xe3, 0x77, 0xc2],
    [0x17, 0xbe, 0xcf],
    [0xbc, 0xbd, 0x22],
    [0x7f, 0x7f, 0x7f],
    [0x00, 0x00, 0x8b],
    [0x8b, 0x00, 0x00],
    [0x00, 0x64, 0x00],
    [0xb8, 0x86, 0x0b],
    [0x4b, 0x00, 0x82],
    [0x20, 0x20, 0x30],
];

impl MillLayer {
    /// A layer set up from a bit: its speeds and a pass depth that suits its diameter.
    pub fn with_tool(mut self, t: &Tool) -> MillLayer {
        self.use_tool(t);
        self
    }

    /// Take the bit and its starting speeds.
    pub fn use_tool(&mut self, t: &Tool) {
        self.tool = Some(t.id.clone());
        self.feed = t.cutting.feed;
        self.plunge = t.cutting.plunge;
        self.rpm = t.cutting.rpm;
        self.step_down = (t.dims.diameter * t.cutting.step_down_pct / 100.0).max(0.1);
        self.step_over_pct = t.cutting.step_over_pct;
    }

    pub fn color32(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.color[0], self.color[1], self.color[2])
    }

    /// The depths of the passes, from the first to the last (positive, below the surface). The last one is the full
    /// depth.
    pub fn passes(&self) -> Vec<f64> {
        let depth = self.depth.max(0.0);
        let step = self.step_down.max(0.01);
        if depth <= 0.0 {
            return vec![];
        }
        let n = ((depth / step) - 1e-9).ceil().max(1.0) as usize;
        let n = n.min(500);
        let mut v: Vec<f64> = (1..=n).map(|i| (step * i as f64).min(depth)).collect();
        if let Some(last) = v.last_mut() {
            *last = depth;
        }
        v
    }

    /// Is every number of the layer usable?
    pub fn check(&self) -> Result<(), String> {
        let ok = |v: f64, lo: f64, hi: f64| v.is_finite() && v >= lo && v <= hi;
        if !ok(self.depth, 0.0, 1000.0) || !ok(self.step_down, 0.01, 1000.0) || !ok(self.step_over_pct, 1.0, 100.0) || !ok(self.feed, 1.0, 100_000.0) || !ok(self.plunge, 1.0, 100_000.0) || !ok(self.rpm, 0.0, 1_000_000.0) {
            return Err(format!("layer {}: a number is out of range", self.name));
        }
        if self.name.len() > 120 {
            return Err("a layer name is too long".into());
        }
        Ok(())
    }
}

/// The layers of a new project: the first ones set up for the usual jobs, the others waiting.
pub fn default_layers(tools: &crate::tools::Library) -> Vec<MillLayer> {
    let bit = |id: &str| tools.get(id).or_else(|| tools.builtin.first()).cloned();
    (0..LAYER_COUNT)
        .map(|i| {
            let (name, op, depth, tool) = match i {
                0 => ("Cut out", Operation::ProfileOutside, 10.0, "std-flat-3"),
                1 => ("Holes and windows", Operation::ProfileInside, 10.0, "std-flat-3"),
                2 => ("Pocket", Operation::Pocket, 3.0, "std-flat-3"),
                3 => ("Engrave", Operation::OnPath, 1.0, "std-v-60"),
                4 => ("Drill", Operation::Drill, 10.0, "std-drill-3"),
                _ => ("", Operation::OnPath, 3.0, "std-flat-3"),
            };
            let layer = MillLayer {
                name: if name.is_empty() { format!("Layer {}", i + 1) } else { name.to_string() },
                color: PALETTE[i],
                visible: true,
                enabled: true,
                op,
                tool: None,
                depth,
                step_down: 1.0,
                step_over_pct: 40.0,
                feed: 800.0,
                plunge: 300.0,
                rpm: 12000.0,
            };
            match bit(tool) {
                Some(t) => layer.with_tool(&t),
                None => layer,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::Library;

    #[test]
    fn a_new_project_has_sixteen_layers_set_up_with_bits() {
        let lib = Library::new();
        let l = default_layers(&lib);
        assert_eq!(l.len(), LAYER_COUNT);
        assert!(l.iter().all(|x| x.check().is_ok()));
        assert!(l.iter().all(|x| lib.get(x.tool.as_deref().unwrap()).is_some()));
        assert_eq!(l[0].op, Operation::ProfileOutside);
        assert_eq!(l[4].op, Operation::Drill);
        let colours: std::collections::HashSet<_> = l.iter().map(|x| x.color).collect();
        assert_eq!(colours.len(), LAYER_COUNT);
    }

    #[test]
    fn passes_end_at_the_full_depth() {
        let mut l = default_layers(&Library::new()).remove(0);
        l.depth = 10.0;
        l.step_down = 3.0;
        assert_eq!(l.passes(), vec![3.0, 6.0, 9.0, 10.0]);
        l.step_down = 5.0;
        assert_eq!(l.passes(), vec![5.0, 10.0]);
        l.step_down = 50.0;
        assert_eq!(l.passes(), vec![10.0]);
        l.depth = 0.0;
        assert!(l.passes().is_empty());
    }

    #[test]
    fn taking_a_bit_takes_its_speeds() {
        let lib = Library::new();
        let mut l = default_layers(&lib).remove(2);
        let t = lib.get("std-flat-6").unwrap();
        l.use_tool(t);
        assert_eq!((l.feed, l.rpm), (t.cutting.feed, t.cutting.rpm));
        assert!((l.step_down - 3.0).abs() < 1e-9, "half of 6 mm");
        assert_eq!(l.tool.as_deref(), Some("std-flat-6"));
    }
}
