//! The layers of a milling project. A layer says what is done with the shapes that lie on it: cut outside or inside
//! the line, follow the line, clear a pocket or drill, with which bit, how deep and how fast.
use crate::tools::Tool;
use serde::{Deserialize, Serialize};

/// How many layers a project has.
/// How many layers there are, and their colours: the same in all programs (`base_core::layers`).
pub use base_core::layers::{LAYER_COUNT, PALETTE};

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
    /// A picture on the layer is engraved (see `PictureMethod`).
    Picture,
    /// The top of the material is planed over the box of the shapes of the layer.
    Face,
    /// A hole bigger than the bit is cut by going down round it in a spiral.
    Bore,
    /// The edge of a shape is chamfered with a V-bit.
    Chamfer,
    /// The inside of a slot or a hole, with a notch at every sharp corner so that a round bit can reach into it.
    DogboneSlot,
    /// A V-bit goes deeper the farther it is from the edge of the shape: sharp, sloping walls (lettering, signs, inlays).
    VCarve,
}

/// Which way the bit goes round a profile or a pocket (for a spindle that turns clockwise).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CutDir {
    /// As the rings come out of the program (what it always did).
    #[default]
    AsDrawn,
    /// The bit turns with the feed: the material is on the left of the way of the bit.
    Climb,
    /// The bit turns against the feed: the material is on the right.
    Conventional,
}

impl CutDir {
    pub const ALL: [CutDir; 3] = [CutDir::AsDrawn, CutDir::Climb, CutDir::Conventional];
    pub fn label(self) -> &'static str {
        match self {
            CutDir::AsDrawn => "As drawn",
            CutDir::Climb => "Climb",
            CutDir::Conventional => "Conventional",
        }
    }
}

/// How the bit goes down into the material.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Entry {
    /// Straight down, at the plunge feed.
    #[default]
    Plunge,
    /// Down in a zigzag along the first part of the way, at the ramp angle.
    RampZigzag,
    /// Down along the contour itself, at the ramp angle.
    RampContour,
    /// Down in a helix (a pocket, where it fits; otherwise the ramp along the contour).
    Helix,
}

impl Entry {
    pub const ALL: [Entry; 4] = [Entry::Plunge, Entry::RampZigzag, Entry::RampContour, Entry::Helix];
    pub fn label(self) -> &'static str {
        match self {
            Entry::Plunge => "Plunge",
            Entry::RampZigzag => "Zigzag ramp",
            Entry::RampContour => "Ramp along the contour",
            Entry::Helix => "Helix",
        }
    }
}

/// How a pocket is cleared.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PocketPattern {
    /// Ring after ring, from the outside in.
    #[default]
    Rings,
    /// Parallel lines at an angle, back and forth.
    Raster,
}

impl PocketPattern {
    pub const ALL: [PocketPattern; 2] = [PocketPattern::Rings, PocketPattern::Raster];
    pub fn label(self) -> &'static str {
        match self {
            PocketPattern::Rings => "Rings",
            PocketPattern::Raster => "Raster",
        }
    }
}

/// A pass along the wall of a pocket.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutlinePass {
    #[default]
    None,
    Before,
    After,
}

impl OutlinePass {
    pub const ALL: [OutlinePass; 3] = [OutlinePass::None, OutlinePass::Before, OutlinePass::After];
    pub fn label(self) -> &'static str {
        match self {
            OutlinePass::None => "None",
            OutlinePass::Before => "Before",
            OutlinePass::After => "After",
        }
    }
}

/// Trochoidal milling: the bit goes along the line in loops, so that it cuts a slot wider than itself with a light load.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trochoid {
    pub enabled: bool,
    /// The diameter of the loops (mm): the slot is this plus the diameter of the bit wide.
    pub diameter: f64,
    /// How far the loops advance along the line with every turn (mm).
    pub step: f64,
    /// Half circles swinging to and fro instead of full circles.
    pub half: bool,
}

impl Default for Trochoid {
    fn default() -> Trochoid {
        Trochoid { enabled: false, diameter: 3.0, step: 1.0, half: false }
    }
}

/// What a tab (bridge) of a profile is like.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabKind {
    /// The bit rises above the material: the tab is as high as the material.
    Full,
    /// The bit rises to the height of the tab.
    #[default]
    Partial,
    /// As partial, with a slope up and down.
    Ramp,
}

impl TabKind {
    pub const ALL: [TabKind; 3] = [TabKind::Full, TabKind::Partial, TabKind::Ramp];
    pub fn label(self) -> &'static str {
        match self {
            TabKind::Full => "Full",
            TabKind::Partial => "Partial",
            TabKind::Ramp => "Ramp",
        }
    }
}

/// A tab placed by hand: on the outline of a shape, `at` of the way round it (0 … 1).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualTab {
    pub shape: u64,
    pub at: f64,
}

/// Tabs (bridges) that hold a part in the material while it is cut out.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tabs {
    pub enabled: bool,
    pub kind: TabKind,
    /// The width of a tab at the part, along the profile (mm).
    pub length: f64,
    /// The height of a tab above the bottom of the cut (mm); a full tab is as high as the material.
    pub height: f64,
    /// The slope of a ramp tab (degrees from the horizontal).
    pub slope: f64,
    /// The tabs are spread evenly, `spacing` mm apart, at least `min` and at most `max` on a ring.
    pub spacing: f64,
    pub min: u32,
    pub max: u32,
    /// Rings that are holes (of a part) get no tabs.
    pub skip_inner: bool,
    /// Tabs placed by hand on the work area. A ring that has some of them near it gets those and no others.
    #[serde(default)]
    pub manual: Vec<ManualTab>,
}

impl Default for Tabs {
    fn default() -> Tabs {
        Tabs { enabled: false, kind: TabKind::Partial, length: 5.0, height: 2.0, slope: 45.0, spacing: 60.0, min: 2, max: 8, skip_inner: true, manual: vec![] }
    }
}

/// The coolant a layer works with.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Coolant {
    #[default]
    Off,
    /// M7.
    Mist,
    /// M8 (also what a dust collector is often wired to).
    Flood,
}

impl Coolant {
    pub const ALL: [Coolant; 3] = [Coolant::Off, Coolant::Mist, Coolant::Flood];
    pub fn label(self) -> &'static str {
        match self {
            Coolant::Off => "Off",
            Coolant::Mist => "Mist",
            Coolant::Flood => "Flood",
        }
    }
    /// The word of the program that switches it on.
    pub fn word(self) -> Option<&'static str> {
        match self {
            Coolant::Off => None,
            Coolant::Mist => Some("M7"),
            Coolant::Flood => Some("M8"),
        }
    }
}

/// What a V-carve makes when it is one half of an inlay.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Inlay {
    /// A plain V-carve inside the shapes.
    #[default]
    None,
    /// The socket: carved inside the shape.
    Socket,
    /// The plug: the shape is made smaller by the gap and the V-walls are carved round it (outside the line), so that it
    /// fits the socket. The plug is cut on the back of a piece: mirror the shapes of the layer first.
    Plug,
}

impl Inlay {
    pub const ALL: [Inlay; 3] = [Inlay::None, Inlay::Socket, Inlay::Plug];
    pub fn label(self) -> &'static str {
        match self {
            Inlay::None => "None",
            Inlay::Socket => "Socket",
            Inlay::Plug => "Plug",
        }
    }
}

/// Which part of a relief the layer cuts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReliefPart {
    /// Passes down to the relief itself (what it always did).
    #[default]
    Both,
    /// Roughing: passes down to the relief with `stock to leave` on it (use a flat bit).
    Rough,
    /// Finishing: one go over the relief itself, row after row (use a ball nose).
    Finish,
}

impl ReliefPart {
    pub const ALL: [ReliefPart; 3] = [ReliefPart::Both, ReliefPart::Rough, ReliefPart::Finish];
    pub fn label(self) -> &'static str {
        match self {
            ReliefPart::Both => "Rough and finish",
            ReliefPart::Rough => "Rough",
            ReliefPart::Finish => "Finish",
        }
    }
}

fn one() -> f64 {
    1.0
}

fn three() -> f64 {
    3.0
}

fn yes() -> bool {
    true
}

/// How a picture is engraved.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PictureMethod {
    /// The picture is dithered; every dark cell is a hole (a plunge to the depth of the layer).
    Dots,
    /// The picture is dithered; every row is cut at the depth of the layer where the picture is dark.
    #[default]
    Rows,
    /// Rows again, but the depth follows the tone: the darker, the deeper (down to the depth of the layer).
    Depth,
    /// A 3D relief: roughing passes down to the full depth, with the shape of the bit taken into account (a drop cutter).
    Relief,
}

impl PictureMethod {
    pub const ALL: [PictureMethod; 4] = [PictureMethod::Rows, PictureMethod::Dots, PictureMethod::Depth, PictureMethod::Relief];

    pub fn label(self) -> &'static str {
        match self {
            PictureMethod::Rows => "Dithered rows",
            PictureMethod::Dots => "Dots",
            PictureMethod::Depth => "Depth by tone",
            PictureMethod::Relief => "3D relief",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            PictureMethod::Rows => "The picture is dithered and each row is cut at the depth of the layer where it is dark. The dithering method is chosen below.",
            PictureMethod::Dots => "A hole to the depth of the layer for every dark cell of the dithered picture.",
            PictureMethod::Depth => "The rows are cut with a depth that follows the tone of the picture: the darker, the deeper, down to the depth of the layer.",
            PictureMethod::Relief => "A relief: black is the deepest, white is left uncut. Passes of the pass depth down to the full depth, and the shape of the bit is taken into account. The model can be exported as STL.",
        }
    }

    /// Does the method dither the picture?
    pub fn dithers(self) -> bool {
        matches!(self, PictureMethod::Rows | PictureMethod::Dots)
    }
}

impl Operation {
    pub const ALL: [Operation; 11] = [Operation::ProfileOutside, Operation::ProfileInside, Operation::OnPath, Operation::Pocket, Operation::Drill, Operation::Picture, Operation::Face, Operation::Bore, Operation::Chamfer, Operation::DogboneSlot, Operation::VCarve];

    pub fn label(self) -> &'static str {
        match self {
            Operation::ProfileOutside => "Cut outside the line",
            Operation::ProfileInside => "Cut inside the line",
            Operation::OnPath => "Follow the line",
            Operation::Pocket => "Pocket",
            Operation::Drill => "Drill",
            Operation::Picture => "Picture",
            Operation::Face => "Face",
            Operation::Bore => "Bore",
            Operation::Chamfer => "Chamfer",
            Operation::DogboneSlot => "Dogbone slot",
            Operation::VCarve => "V-carve",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Operation::ProfileOutside => "The bit goes round the outside of the line, so the shape stays: a part is cut out of the material.",
            Operation::ProfileInside => "The bit goes round the inside of the line, so the shape falls out: a hole or a window.",
            Operation::OnPath => "The centre of the bit follows the line: engraving and grooves. Open lines work too.",
            Operation::Pocket => "All of the area inside the line is cleared, ring after ring, to the depth of the layer.",
            Operation::Drill => "A hole at the centre of every closed shape of the layer, in steps of the pass depth.",
            Operation::Picture => "A picture on the layer is engraved: rows or dots by the dithered picture, or rows whose depth follows the tone.",
            Operation::Face => "The top of the material is planed over the box of the shapes of the layer; the bit goes a little over the edge.",
            Operation::Bore => "A round hole bigger than the bit: the bit goes down round it in a spiral and then once round at the full depth.",
            Operation::Chamfer => "The edge of the shapes is chamfered: a V-bit follows the line at the depth that makes the chamfer as wide as asked.",
            Operation::DogboneSlot => "The bit goes round the inside of the line and into every sharp corner, so that a round bit leaves room for a square part.",
            Operation::VCarve => "A V-bit goes deeper the farther it is from the edge of the shape, so the walls slope and the corners are sharp. The depth is the deepest the bit goes; a wider shape gets a flat bottom.",
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
    /// A picture on the layer (`Operation::Picture`): how it is engraved, with which dithering, and the angle of the rows.
    /// The distance of the rows is the step over of the bit.
    #[serde(default)]
    pub picture: PictureMethod,
    #[serde(default)]
    pub dither: base_core::raster::Dither,
    #[serde(default)]
    pub scan_angle: f64,
    /// Where the layer is in the order the layers are done in (the lower first; equal numbers go by layer). Moving layers
    /// in the list renumbers them all.
    #[serde(default)]
    pub rank: u32,
    /// Which way a profile or a pocket is milled (climb or conventional).
    #[serde(default)]
    pub direction: CutDir,
    /// The material left on the walls by the passes down to the depth (mm); a finishing pass takes it off.
    #[serde(default)]
    pub leave: f64,
    /// With material left, one more pass at the full depth along the real line.
    #[serde(default = "yes")]
    pub finish: bool,
    /// How the bit goes down, the slope of the ramp or the helix (degrees) and its feed (0 = the plunge feed).
    #[serde(default)]
    pub entry: Entry,
    #[serde(default = "three")]
    pub ramp_angle: f64,
    #[serde(default)]
    pub ramp_feed: f64,
    /// Tabs of a profile.
    #[serde(default)]
    pub tabs: Tabs,
    /// How a pocket (and a face) is cleared, the angle of the lines of a raster, and a pass along the wall.
    #[serde(default)]
    pub pattern: PocketPattern,
    #[serde(default)]
    pub raster_angle: f64,
    #[serde(default)]
    pub outline: OutlinePass,
    /// A rest pocket: only what the bit of this layer (a pocket with a bigger bit) could not reach is cleared.
    #[serde(default)]
    pub rest_of: Option<usize>,
    /// How far over the edge of the box the edge of the bit goes when a surface is planed (mm).
    #[serde(default)]
    pub overhang: f64,
    /// The width of a chamfer on the surface (mm).
    #[serde(default = "one")]
    pub chamfer: f64,
    #[serde(default)]
    pub trochoid: Trochoid,
    /// V-carve: nothing is cut shallower than this (mm), the inlay half and the gap of an inlay (mm).
    #[serde(default)]
    pub vc_start: f64,
    #[serde(default)]
    pub inlay: Inlay,
    #[serde(default)]
    pub inlay_gap: f64,
    /// A relief: which part of it this layer cuts. The stock to leave of a rough part is `leave`.
    #[serde(default)]
    pub relief_part: ReliefPart,
    /// Coolant (M7 / M8) while the layer is cut, and the dust collector (the device's own lines for it).
    #[serde(default)]
    pub coolant: Coolant,
    #[serde(default)]
    pub dust: bool,
    /// A pocket by rings: every depth goes round all the rings, and the way from one ring to the next is a cut in the material
    /// (when the whole way is inside the pocket) instead of a lift. Off: every ring goes through all depths, with a lift between.
    #[serde(default, skip_serializing_if = "is_false")]
    pub stay_down: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}


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
        // 0 % is the full depth in one pass.
        self.step_down = if t.cutting.step_down_pct <= 0.0 { 0.0 } else { (t.dims.diameter * t.cutting.step_down_pct / 100.0).max(0.1) };
        self.step_over_pct = t.cutting.step_over_pct;
    }

    pub fn color32(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.color[0], self.color[1], self.color[2])
    }

    /// The depths of the passes, from the first to the last (positive, below the surface). The last one is the full
    /// depth. A pass depth of 0 means the full depth in one pass.
    pub fn passes(&self) -> Vec<f64> {
        let depth = self.depth.max(0.0);
        let step = if self.step_down <= 0.0 { depth } else { self.step_down.max(0.01) };
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
        if !ok(self.depth, 0.0, 1000.0) || !ok(self.step_down, 0.0, 1000.0) || !ok(self.step_over_pct, 1.0, 100.0) || !ok(self.feed, 1.0, 100_000.0) || !ok(self.plunge, 1.0, 100_000.0) || !ok(self.rpm, 0.0, 1_000_000.0) || !ok(self.scan_angle, -360.0, 360.0) || !ok(self.leave, 0.0, 100.0) || !ok(self.ramp_angle, 0.1, 89.0) || !ok(self.ramp_feed, 0.0, 100_000.0) || !ok(self.raster_angle, -360.0, 360.0) || !ok(self.overhang, 0.0, 1000.0) || !ok(self.chamfer, 0.0, 1000.0) || !ok(self.trochoid.diameter, 0.1, 1000.0) || !ok(self.trochoid.step, 0.05, 1000.0) || !ok(self.vc_start, 0.0, 1000.0) || !ok(self.inlay_gap, 0.0, 100.0) {
            return Err(format!("layer {}: a number is out of range", self.name));
        }
        let t = &self.tabs;
        if !ok(t.length, 0.1, 1000.0) || !ok(t.height, 0.0, 1000.0) || !ok(t.slope, 5.0, 89.0) || !ok(t.spacing, 1.0, 100_000.0) || t.min > 100 || t.max > 100 || t.min > t.max {
            return Err(format!("layer {}: a number of the tabs is out of range", self.name));
        }
        if self.name.len() > 120 {
            return Err("a layer name is too long".into());
        }
        Ok(())
    }
}

/// The layers in the order they are done in: by their rank, then by their number.
pub fn layer_order(layers: &[MillLayer]) -> Vec<usize> {
    let mut o: Vec<usize> = (0..layers.len()).collect();
    o.sort_by_key(|&i| (layers[i].rank, i));
    o
}

/// Move a layer one place earlier (`up`) or later in the order the layers are done in; all the layers get a new rank.
pub fn move_layer(layers: &mut [MillLayer], layer: usize, up: bool) {
    let mut o = layer_order(layers);
    let Some(at) = o.iter().position(|&i| i == layer) else { return };
    let to = if up { at.checked_sub(1) } else { (at + 1 < o.len()).then_some(at + 1) };
    if let Some(to) = to {
        o.swap(at, to);
    }
    for (rank, &i) in o.iter().enumerate() {
        layers[i].rank = rank as u32;
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
                picture: PictureMethod::default(),
                dither: base_core::raster::Dither::default(),
                scan_angle: 0.0,
                rank: 0,
                direction: CutDir::default(),
                leave: 0.0,
                finish: true,
                stay_down: false,
                entry: Entry::default(),
                ramp_angle: 3.0,
                ramp_feed: 0.0,
                tabs: Tabs::default(),
                pattern: PocketPattern::default(),
                raster_angle: 0.0,
                outline: OutlinePass::default(),
                rest_of: None,
                overhang: 0.0,
                chamfer: 1.0,
                trochoid: Trochoid::default(),
                vc_start: 0.0,
                inlay: Inlay::None,
                inlay_gap: 0.0,
                relief_part: ReliefPart::Both,
                coolant: Coolant::Off,
                dust: false,
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
    fn a_new_project_has_all_its_layers_set_up_with_bits() {
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
    fn a_pass_depth_of_zero_is_the_full_depth_in_one_pass() {
        let mut l = default_layers(&Library::new()).remove(0);
        l.depth = 7.5;
        l.step_down = 0.0;
        assert_eq!(l.passes(), vec![7.5]);
        assert!(l.check().is_ok(), "0 is allowed in a layer");
        l.step_down = -1.0;
        assert!(l.check().is_err(), "a negative pass depth is not");
        assert_eq!(crate::relief::pass_limits(6.0, 0.0), vec![6.0]);
        // A bit with 0 % gives a layer with 0.
        let lib = Library::new();
        let mut t = lib.all().next().expect("a bit in the library").clone();
        t.cutting.step_down_pct = 0.0;
        assert!(t.check().is_ok());
        l.use_tool(&t);
        assert_eq!(l.step_down, 0.0);
        // The old files, with a pass depth, are read as they were.
        t.cutting.step_down_pct = 50.0;
        l.use_tool(&t);
        assert!(l.step_down >= 0.1);
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

    #[test]
    fn layers_can_be_moved_in_the_order_they_are_done_in() {
        let lib = crate::tools::Library::new();
        let mut l = default_layers(&lib);
        assert_eq!(&layer_order(&l)[..3], &[0, 1, 2]);
        move_layer(&mut l, 2, true);
        assert_eq!(&layer_order(&l)[..3], &[0, 2, 1]);
        move_layer(&mut l, 0, false);
        assert_eq!(&layer_order(&l)[..3], &[2, 0, 1]);
        move_layer(&mut l, 2, true);
        assert_eq!(layer_order(&l)[0], 2, "the first cannot go earlier");
        let last = layer_order(&l)[l.len() - 1];
        move_layer(&mut l, last, false);
        assert_eq!(layer_order(&l)[l.len() - 1], last, "the last cannot go later");
        // Older projects have no ranks: the order is the numbers.
        assert_eq!(layer_order(&default_layers(&lib)), (0..l.len()).collect::<Vec<_>>());
    }
}
