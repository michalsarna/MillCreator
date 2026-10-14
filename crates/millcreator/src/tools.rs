//! The library of milling bits: the kinds of bits and the dimensions each kind has, a set of ready-made bits, the bits
//! you add (kept for every device), and reading and writing a file with bits.
use serde::{Deserialize, Serialize};

/// The most bits a library or an imported file may hold.
pub const MAX_TOOLS: usize = 2000;
/// The largest bit file that is read.
pub const MAX_FILE: u64 = 4_000_000;
/// The version of the bit file format.
pub const FILE_FORMAT: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolKind {
    FlatEnd,
    BallNose,
    BullNose,
    VBit,
    Engraver,
    TaperedBall,
    Drill,
}

/// A dimension of a bit. The letters are the ones drawn in the picture of the bit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dim {
    /// D: the cutting diameter (the widest cutting part).
    Diameter,
    /// L: the length of the cutting part.
    CutLength,
    /// O: the overall length.
    Overall,
    /// S: the shank diameter.
    Shank,
    /// R: the corner radius.
    Corner,
    /// A: the angle (V angle, taper angle or drill point angle).
    Angle,
    /// T: the diameter of the tip.
    Tip,
}

impl Dim {
    pub fn letter(self) -> &'static str {
        match self {
            Dim::Diameter => "D",
            Dim::CutLength => "L",
            Dim::Overall => "O",
            Dim::Shank => "S",
            Dim::Corner => "R",
            Dim::Angle => "A",
            Dim::Tip => "T",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Dim::Diameter => "Cutting diameter",
            Dim::CutLength => "Cutting length",
            Dim::Overall => "Overall length",
            Dim::Shank => "Shank diameter",
            Dim::Corner => "Corner radius",
            Dim::Angle => "Angle",
            Dim::Tip => "Tip diameter",
        }
    }
    pub fn unit(self) -> &'static str {
        if self == Dim::Angle {
            "°"
        } else {
            "mm"
        }
    }
}

impl ToolKind {
    pub const ALL: [ToolKind; 7] = [ToolKind::FlatEnd, ToolKind::BallNose, ToolKind::BullNose, ToolKind::VBit, ToolKind::Engraver, ToolKind::TaperedBall, ToolKind::Drill];

    pub fn label(self) -> &'static str {
        match self {
            ToolKind::FlatEnd => "Flat end mill",
            ToolKind::BallNose => "Ball nose",
            ToolKind::BullNose => "Bull nose (corner radius)",
            ToolKind::VBit => "V-bit",
            ToolKind::Engraver => "Engraving bit",
            ToolKind::TaperedBall => "Tapered ball nose",
            ToolKind::Drill => "Drill",
        }
    }

    /// The dimensions this kind of bit has, in the order they are entered.
    pub fn dims(self) -> &'static [Dim] {
        use Dim::*;
        match self {
            ToolKind::FlatEnd | ToolKind::BallNose => &[Diameter, CutLength, Shank, Overall],
            ToolKind::BullNose => &[Diameter, Corner, CutLength, Shank, Overall],
            ToolKind::VBit => &[Diameter, Angle, CutLength, Shank, Overall],
            ToolKind::Engraver => &[Diameter, Tip, Angle, CutLength, Shank, Overall],
            ToolKind::TaperedBall => &[Diameter, Tip, Angle, CutLength, Shank, Overall],
            ToolKind::Drill => &[Diameter, Angle, CutLength, Shank, Overall],
        }
    }

    /// The limits of a dimension, for the entry fields.
    pub fn range(dim: Dim) -> (f64, f64) {
        match dim {
            Dim::Angle => (1.0, 180.0),
            Dim::Corner | Dim::Tip => (0.0, 50.0),
            _ => (0.05, 200.0),
        }
    }
}

/// The measures of a bit; the ones a kind does not have are ignored.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dims {
    pub diameter: f64,
    pub cut_length: f64,
    pub overall: f64,
    pub shank: f64,
    pub corner: f64,
    pub angle: f64,
    pub tip: f64,
}

impl Dims {
    pub fn get(&self, d: Dim) -> f64 {
        match d {
            Dim::Diameter => self.diameter,
            Dim::CutLength => self.cut_length,
            Dim::Overall => self.overall,
            Dim::Shank => self.shank,
            Dim::Corner => self.corner,
            Dim::Angle => self.angle,
            Dim::Tip => self.tip,
        }
    }
    pub fn set(&mut self, d: Dim, v: f64) {
        match d {
            Dim::Diameter => self.diameter = v,
            Dim::CutLength => self.cut_length = v,
            Dim::Overall => self.overall = v,
            Dim::Shank => self.shank = v,
            Dim::Corner => self.corner = v,
            Dim::Angle => self.angle = v,
            Dim::Tip => self.tip = v,
        }
    }
}

impl Default for Dims {
    fn default() -> Dims {
        Dims { diameter: 3.0, cut_length: 12.0, overall: 38.0, shank: 3.175, corner: 0.5, angle: 60.0, tip: 0.2 }
    }
}

/// How a bit is run, as a starting point for a layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cutting {
    /// Feed in the material, mm/min.
    pub feed: f64,
    /// Feed of the plunge, mm/min.
    pub plunge: f64,
    /// Spindle speed, rpm.
    pub rpm: f64,
    /// How deep one pass goes, as a part of the diameter (percent).
    pub step_down_pct: f64,
    /// How far the next ring or line is, as a part of the diameter (percent).
    pub step_over_pct: f64,
    pub flutes: u32,
}

impl Default for Cutting {
    fn default() -> Cutting {
        Cutting { feed: 800.0, plunge: 300.0, rpm: 12000.0, step_down_pct: 50.0, step_over_pct: 40.0, flutes: 2 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tool {
    /// Identifies the bit in the projects; stays the same when the bit is renamed.
    pub id: String,
    pub name: String,
    pub kind: ToolKind,
    pub dims: Dims,
    #[serde(default)]
    pub cutting: Cutting,
    #[serde(default)]
    pub notes: String,
    /// A ready-made bit of the program: it cannot be changed or deleted. Never written to a file.
    #[serde(skip)]
    pub builtin: bool,
}

impl Tool {
    pub fn new(id: &str, name: &str, kind: ToolKind, dims: Dims) -> Tool {
        Tool { id: id.into(), name: name.into(), kind, dims, cutting: Cutting::default(), notes: String::new(), builtin: false }
    }

    /// The width of the cut at `depth` below the surface, in mm: the width of the groove this bit makes when it goes
    /// that deep. A flat or ball bit cuts its diameter (a ball nose less near the surface of a shallow cut); a V-bit
    /// and the pointed bits get wider with depth up to their diameter.
    pub fn cut_width(&self, depth: f64) -> f64 {
        let d = self.dims.diameter.max(0.0);
        let depth = depth.max(0.0);
        match self.kind {
            ToolKind::FlatEnd | ToolKind::BullNose | ToolKind::Drill => d,
            ToolKind::BallNose => {
                let r = d / 2.0;
                if depth >= r {
                    d
                } else {
                    2.0 * (r * r - (r - depth) * (r - depth)).max(0.0).sqrt()
                }
            }
            ToolKind::VBit | ToolKind::Engraver | ToolKind::TaperedBall => {
                let half = (self.dims.angle.clamp(1.0, 180.0) / 2.0).to_radians().tan();
                let tip = if self.kind == ToolKind::VBit { 0.0 } else { self.dims.tip.max(0.0) };
                (tip + 2.0 * depth * half).min(d)
            }
        }
    }

    /// The radius the path of a cut is moved away from the line: half of the diameter.
    pub fn radius(&self) -> f64 {
        self.dims.diameter.max(0.0) / 2.0
    }

    /// Is every number of the bit usable?
    pub fn check(&self) -> Result<(), String> {
        for d in self.kind.dims() {
            let v = self.dims.get(*d);
            let (lo, hi) = ToolKind::range(*d);
            if !v.is_finite() || v < lo || v > hi {
                return Err(format!("{}: {} is outside {}…{}", d.label(), v, lo, hi));
            }
        }
        let d = &self.dims;
        if self.kind.dims().contains(&Dim::CutLength) && self.kind.dims().contains(&Dim::Overall) && d.cut_length > d.overall {
            return Err("the cutting length is longer than the bit".into());
        }
        if self.kind == ToolKind::BullNose && d.corner > d.diameter / 2.0 {
            return Err("the corner radius is larger than half of the diameter".into());
        }
        if matches!(self.kind, ToolKind::Engraver | ToolKind::TaperedBall) && d.tip > d.diameter {
            return Err("the tip is wider than the bit".into());
        }
        let c = &self.cutting;
        for v in [c.feed, c.plunge, c.rpm, c.step_down_pct, c.step_over_pct] {
            if !v.is_finite() || v <= 0.0 || v > 1_000_000.0 {
                return Err("a feed, a speed or a step is out of range".into());
            }
        }
        if self.name.trim().is_empty() || self.name.len() > 120 || self.notes.len() > 4000 || self.id.is_empty() || self.id.len() > 80 {
            return Err("the name or the notes are empty or too long".into());
        }
        Ok(())
    }
}

fn dims(diameter: f64, cut_length: f64, shank: f64, overall: f64) -> Dims {
    Dims { diameter, cut_length, shank, overall, ..Dims::default() }
}

/// The ready-made bits.
pub fn builtin_tools() -> Vec<Tool> {
    let mut v = vec![];
    let mut add = |id: &str, name: &str, kind: ToolKind, d: Dims, feed: f64, plunge: f64, rpm: f64, flutes: u32| {
        let mut t = Tool::new(id, name, kind, d);
        t.cutting = Cutting { feed, plunge, rpm, flutes, ..Cutting::default() };
        t.builtin = true;
        v.push(t);
    };
    add("std-flat-1", "Flat 1 mm", ToolKind::FlatEnd, dims(1.0, 3.0, 3.175, 38.0), 300.0, 100.0, 18000.0, 2);
    add("std-flat-2", "Flat 2 mm", ToolKind::FlatEnd, dims(2.0, 8.0, 3.175, 38.0), 600.0, 200.0, 18000.0, 2);
    add("std-flat-3", "Flat 3.175 mm (1/8\")", ToolKind::FlatEnd, dims(3.175, 12.0, 3.175, 38.0), 900.0, 300.0, 16000.0, 2);
    add("std-flat-6", "Flat 6 mm", ToolKind::FlatEnd, dims(6.0, 22.0, 6.0, 50.0), 1400.0, 500.0, 14000.0, 2);
    add("std-flat-8", "Flat 8 mm", ToolKind::FlatEnd, dims(8.0, 25.0, 8.0, 60.0), 1600.0, 600.0, 12000.0, 2);
    add("std-ball-3", "Ball nose 3.175 mm", ToolKind::BallNose, dims(3.175, 12.0, 3.175, 38.0), 800.0, 300.0, 16000.0, 2);
    add("std-ball-6", "Ball nose 6 mm", ToolKind::BallNose, dims(6.0, 22.0, 6.0, 50.0), 1200.0, 400.0, 14000.0, 2);
    let mut bull = dims(6.0, 22.0, 6.0, 50.0);
    bull.corner = 1.0;
    add("std-bull-6", "Bull nose 6 mm, R1", ToolKind::BullNose, bull, 1200.0, 400.0, 14000.0, 2);
    let mut v60 = dims(12.0, 12.0, 6.0, 40.0);
    v60.angle = 60.0;
    add("std-v-60", "V-bit 60°", ToolKind::VBit, v60, 700.0, 250.0, 16000.0, 2);
    let mut v90 = dims(12.7, 12.7, 6.35, 40.0);
    v90.angle = 90.0;
    add("std-v-90", "V-bit 90°", ToolKind::VBit, v90, 700.0, 250.0, 16000.0, 2);
    let mut eng = dims(3.175, 6.0, 3.175, 38.0);
    eng.angle = 20.0;
    eng.tip = 0.1;
    add("std-eng-20", "Engraver 20°, tip 0.1 mm", ToolKind::Engraver, eng, 500.0, 150.0, 18000.0, 1);
    let mut tb = dims(6.0, 20.0, 6.0, 50.0);
    tb.angle = 10.0;
    tb.tip = 1.0;
    add("std-tball-10", "Tapered ball 10°, tip 1 mm", ToolKind::TaperedBall, tb, 700.0, 250.0, 16000.0, 2);
    let mut dr = dims(3.0, 20.0, 3.0, 50.0);
    dr.angle = 118.0;
    add("std-drill-3", "Drill 3 mm", ToolKind::Drill, dr, 200.0, 120.0, 8000.0, 2);
    v
}

/// The bits of the program and the bits added by the user. The user's bits are kept in one file for every device.
#[derive(Clone, Debug, Default)]
pub struct Library {
    pub builtin: Vec<Tool>,
    pub user: Vec<Tool>,
}

#[derive(Serialize, Deserialize)]
struct FileData {
    format: u32,
    tools: Vec<Tool>,
}

impl Library {
    pub fn new() -> Library {
        Library { builtin: builtin_tools(), user: vec![] }
    }

    pub fn all(&self) -> impl Iterator<Item = &Tool> {
        self.builtin.iter().chain(self.user.iter())
    }

    pub fn get(&self, id: &str) -> Option<&Tool> {
        self.all().find(|t| t.id == id)
    }

    /// A free id for a new bit.
    pub fn new_id(&self) -> String {
        let mut n = self.user.len() + 1;
        loop {
            let id = format!("user-{n}");
            if self.get(&id).is_none() {
                return id;
            }
            n += 1;
        }
    }

    /// A free name: the wanted one, or with a number after it.
    pub fn free_name(&self, wanted: &str) -> String {
        if !self.all().any(|t| t.name == wanted) {
            return wanted.to_string();
        }
        (2..).map(|n| format!("{wanted} ({n})")).find(|n| !self.all().any(|t| &t.name == n)).unwrap_or_else(|| wanted.to_string())
    }

    /// Add a bit of the user's own. Returns its id.
    pub fn add(&mut self, mut t: Tool) -> Result<String, String> {
        t.check()?;
        if self.user.len() >= MAX_TOOLS {
            return Err("the library is full".into());
        }
        t.builtin = false;
        t.id = self.new_id();
        t.name = self.free_name(t.name.trim());
        let id = t.id.clone();
        self.user.push(t);
        Ok(id)
    }

    /// Replace a bit of the user's own (not a ready-made one).
    pub fn update(&mut self, t: Tool) -> Result<(), String> {
        t.check()?;
        match self.user.iter_mut().find(|u| u.id == t.id) {
            Some(u) => {
                *u = Tool { builtin: false, ..t };
                Ok(())
            }
            None => Err("only the bits you added can be changed".into()),
        }
    }

    pub fn remove(&mut self, id: &str) -> bool {
        let n = self.user.len();
        self.user.retain(|t| t.id != id);
        self.user.len() != n
    }

    /// The bits as a file: all of the user's, or the listed ones (ready-made bits too).
    pub fn export(&self, ids: Option<&[String]>) -> String {
        let tools: Vec<Tool> = match ids {
            Some(ids) => self.all().filter(|t| ids.contains(&t.id)).cloned().collect(),
            None => self.user.clone(),
        };
        serde_json::to_string_pretty(&FileData { format: FILE_FORMAT, tools }).unwrap_or_default()
    }

    /// Read a file of bits into the library. Every bit is checked first and given an id and a name of its own; a bit
    /// that does not pass the check is left out and named in the answer. Returns how many were added and the problems.
    pub fn import(&mut self, text: &str) -> Result<(usize, Vec<String>), String> {
        if text.len() as u64 > MAX_FILE {
            return Err("the file is too large".into());
        }
        let data: FileData = serde_json::from_str(text).map_err(|e| format!("this is not a file of bits: {e}"))?;
        if data.format > FILE_FORMAT {
            return Err("the file is from a newer version of the program".into());
        }
        let mut added = 0;
        let mut problems = vec![];
        for t in data.tools.into_iter().take(MAX_TOOLS) {
            let name = t.name.clone();
            match self.add(t) {
                Ok(_) => added += 1,
                Err(e) => problems.push(format!("{name}: {e}")),
            }
        }
        Ok((added, problems))
    }

    /// Read a file of bits (plugins bring them) and add the ones whose name the library does not have yet; the others
    /// are left alone, so reading the same file at every start adds nothing. Returns how many were added.
    pub fn import_new(&mut self, text: &str) -> usize {
        let Ok(data) = serde_json::from_str::<FileData>(text) else { return 0 };
        if data.format > FILE_FORMAT {
            return 0;
        }
        let mut added = 0;
        for t in data.tools.into_iter().take(MAX_TOOLS) {
            if self.all().any(|o| o.name == t.name) {
                continue;
            }
            if self.add(t).is_ok() {
                added += 1;
            }
        }
        added
    }

    /// The user's bits as they are kept between runs.
    pub fn save_text(&self) -> String {
        self.export(None)
    }

    /// Load the user's bits (what `save_text` wrote). A damaged file gives an empty set of bits.
    pub fn load_text(&mut self, text: &str) {
        self.user.clear();
        if let Ok(data) = serde_json::from_str::<FileData>(text) {
            for mut t in data.tools.into_iter().take(MAX_TOOLS) {
                t.builtin = false;
                if t.check().is_ok() && self.get(&t.id).is_none() {
                    self.user.push(t);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_ready_made_bit_is_valid_and_has_its_dimensions() {
        for t in builtin_tools() {
            assert!(t.check().is_ok(), "{}: {:?}", t.name, t.check());
            assert!(t.builtin);
        }
        // Every kind is there, and every kind lists the cutting diameter first.
        for k in ToolKind::ALL {
            assert_eq!(k.dims()[0], Dim::Diameter);
            assert!(builtin_tools().iter().any(|t| t.kind == k), "{k:?}");
        }
        let ids: std::collections::HashSet<_> = builtin_tools().into_iter().map(|t| t.id).collect();
        assert_eq!(ids.len(), builtin_tools().len(), "ids are unique");
    }

    #[test]
    fn the_width_of_a_cut_follows_the_shape_of_the_bit() {
        let lib = Library::new();
        let flat = lib.get("std-flat-3").unwrap();
        assert_eq!(flat.cut_width(0.1), 3.175);
        let v = lib.get("std-v-90").unwrap();
        assert!((v.cut_width(1.0) - 2.0).abs() < 1e-9, "a 90° V-bit cuts 2 mm wide at 1 mm");
        assert_eq!(v.cut_width(100.0), v.dims.diameter, "never wider than the bit");
        let ball = lib.get("std-ball-6").unwrap();
        assert!(ball.cut_width(0.5) < 6.0 && (ball.cut_width(3.0) - 6.0).abs() < 1e-9);
        let eng = lib.get("std-eng-20").unwrap();
        assert!(eng.cut_width(0.0) >= 0.1 - 1e-9);
    }

    #[test]
    fn bits_are_added_changed_and_removed_but_the_ready_made_ones_stay() {
        let mut lib = Library::new();
        let n = lib.builtin.len();
        let id = lib.add(Tool::new("x", "My bit", ToolKind::FlatEnd, Dims::default())).unwrap();
        assert!(id.starts_with("user-"));
        // The same name gets a number.
        let id2 = lib.add(Tool::new("x", "My bit", ToolKind::FlatEnd, Dims::default())).unwrap();
        assert_eq!(lib.get(&id2).unwrap().name, "My bit (2)");
        let mut t = lib.get(&id).unwrap().clone();
        t.dims.diameter = 4.0;
        lib.update(t).unwrap();
        assert_eq!(lib.get(&id).unwrap().dims.diameter, 4.0);
        let mut ready = lib.builtin[0].clone();
        ready.dims.diameter = 9.0;
        assert!(lib.update(ready).is_err(), "a ready-made bit cannot be changed");
        assert!(!lib.remove("std-flat-1"));
        assert!(lib.remove(&id));
        assert_eq!(lib.builtin.len(), n);
        assert_eq!(lib.user.len(), 1);
    }

    #[test]
    fn a_bit_with_wrong_numbers_is_refused() {
        let mut t = Tool::new("a", "Bad", ToolKind::FlatEnd, Dims::default());
        t.dims.diameter = f64::NAN;
        assert!(t.check().is_err());
        t.dims.diameter = 3.0;
        t.dims.cut_length = 100.0;
        assert!(t.check().is_err(), "longer than the bit");
        let mut bull = Tool::new("b", "Bull", ToolKind::BullNose, Dims::default());
        bull.dims.corner = 5.0;
        assert!(bull.check().is_err());
        let mut lib = Library::new();
        assert!(lib.add(Tool::new("c", "  ", ToolKind::FlatEnd, Dims::default())).is_err());
    }

    #[test]
    fn a_file_of_bits_goes_out_and_comes_back_and_a_bad_one_is_refused() {
        let mut a = Library::new();
        a.add(Tool::new("x", "Mine", ToolKind::VBit, Dims { angle: 45.0, ..Dims::default() })).unwrap();
        let text = a.export(None);
        let mut b = Library::new();
        let (added, problems) = b.import(&text).unwrap();
        assert_eq!((added, problems.len()), (1, 0));
        assert_eq!(b.user[0].name, "Mine");
        assert_eq!(b.user[0].dims.angle, 45.0);
        // A ready-made bit can be exported by its id; on the way in it becomes the user's own.
        let text = a.export(Some(&["std-flat-2".to_string()]));
        let mut c = Library::new();
        assert_eq!(c.import(&text).unwrap().0, 1);
        assert!(!c.user[0].builtin && c.user[0].name.starts_with("Flat 2 mm"));
        assert!(b.import("not json").is_err());
        assert!(b.import(r#"{"format": 99, "tools": []}"#).is_err());
        // A bit with a wrong number does not come in, the others do.
        let mixed = r#"{"format":1,"tools":[
            {"id":"a","name":"Good","kind":"FlatEnd","dims":{"diameter":3,"cut_length":10,"overall":30,"shank":3,"corner":0,"angle":60,"tip":0.1}},
            {"id":"b","name":"Bad","kind":"FlatEnd","dims":{"diameter":-3,"cut_length":10,"overall":30,"shank":3,"corner":0,"angle":60,"tip":0.1}}]}"#;
        let mut d = Library::new();
        let (added, problems) = d.import(mixed).unwrap();
        assert_eq!(added, 1);
        assert_eq!(problems.len(), 1);
        assert!(problems[0].starts_with("Bad"));
    }

    #[test]
    fn the_saved_library_is_read_back_and_a_damaged_one_is_empty() {
        let mut a = Library::new();
        a.add(Tool::new("x", "Mine", ToolKind::Drill, Dims::default())).unwrap();
        let mut b = Library::new();
        b.load_text(&a.save_text());
        assert_eq!(b.user.len(), 1);
        b.load_text("{ broken");
        assert!(b.user.is_empty());
    }
}
