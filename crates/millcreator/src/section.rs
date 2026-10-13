//! The part of the project file that belongs to MillCreator: the layers, the material, a snapshot of the machine and the
//! bits the layers use. Other programs keep it untouched, and MillCreator keeps theirs.
use crate::device::Device;
use crate::layers::{default_layers, MillLayer, LAYER_COUNT};
use crate::materials::Stock;
use crate::tools::{Library, Tool};
use serde::{Deserialize, Serialize};

pub const SECTION: &str = "mill";
/// The version of this part. A project with a higher one is refused.
pub const SECTION_FORMAT: u32 = 1;

/// The machine the project was made for: enough to warn when it is opened for another one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Machine {
    pub fn of(d: &Device) -> Machine {
        Machine { name: d.name.clone(), x: d.area.x, y: d.area.y, z: d.area.z }
    }

    /// What differs from `d`, in words; empty when the work area is the same.
    pub fn differences(&self, d: &Device) -> Option<String> {
        let same = |a: f64, b: f64| (a - b).abs() < 0.5;
        if same(self.x, d.area.x) && same(self.y, d.area.y) && same(self.z, d.area.z) {
            None
        } else {
            Some(format!("{} ({} × {} × {} mm)", self.name, self.x, self.y, self.z))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MillSection {
    pub format: u32,
    pub layers: Vec<MillLayer>,
    pub stock: Stock,
    pub machine: Machine,
    /// The bits the layers use, so that the project can be opened where they are not in the library.
    #[serde(default)]
    pub tools: Vec<Tool>,
}

impl MillSection {
    pub fn new(layers: &[MillLayer], stock: &Stock, device: &Device, lib: &Library) -> MillSection {
        let used: Vec<Tool> = lib.all().filter(|t| layers.iter().any(|l| l.tool.as_deref() == Some(t.id.as_str()))).cloned().collect();
        MillSection { format: SECTION_FORMAT, layers: layers.to_vec(), stock: stock.clone(), machine: Machine::of(device), tools: used }
    }
}

/// What a project brings in.
pub struct Loaded {
    pub layers: Vec<MillLayer>,
    pub stock: Stock,
    pub machine: Option<Machine>,
    /// The bits of the project that were not in the library and were added to it.
    pub added_tools: Vec<String>,
}

/// Read the section of a project. A project without one (a drawing, or a project of a laser program) gets the default
/// layers. Bits the project uses that the library lacks are added to the library (as the user's own).
pub fn load(p: &base_core::project::Project, lib: &mut Library) -> Result<Loaded, String> {
    let Some(s) = p.section::<MillSection>(SECTION)? else {
        return Ok(Loaded { layers: default_layers(lib), stock: Stock::default(), machine: None, added_tools: vec![] });
    };
    if s.format > SECTION_FORMAT {
        return Err(format!("the milling part of the project has format {}, this program reads up to {SECTION_FORMAT}; update the program", s.format));
    }
    let mut added = vec![];
    for t in s.tools {
        if lib.get(&t.id).is_none() && t.check().is_ok() && lib.user.len() < crate::tools::MAX_TOOLS {
            added.push(t.name.clone());
            lib.user.push(Tool { builtin: false, ..t });
        }
    }
    let mut layers: Vec<MillLayer> = s.layers.into_iter().take(LAYER_COUNT).collect();
    let defaults = default_layers(lib);
    for (i, l) in layers.iter_mut().enumerate() {
        if l.check().is_err() {
            *l = defaults[i].clone();
        }
    }
    while layers.len() < LAYER_COUNT {
        layers.push(defaults[layers.len()].clone());
    }
    Ok(Loaded { layers, stock: s.stock.sane(), machine: Some(s.machine), added_tools: added })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::materials::MaterialKind;
    use base_core::project::Project;
    use base_core::scene::Scene;

    #[test]
    fn the_section_goes_into_the_project_and_comes_back_with_the_bits_it_uses() {
        let mut lib = Library::new();
        let id = lib.add(Tool::new("x", "My cutter", crate::tools::ToolKind::FlatEnd, crate::tools::Dims::default())).unwrap();
        let mut layers = default_layers(&lib);
        layers[0].use_tool(&lib.get(&id).unwrap().clone());
        layers[0].depth = 7.0;
        let mut stock = Stock::of(MaterialKind::Plastic);
        stock.color = [1, 2, 3];
        stock.thickness = 12.0;
        let dev = Device::dummy();
        let mut p = Project::new(Scene::default());
        p.set_section(SECTION, &MillSection::new(&layers, &stock, &dev, &lib));
        let text = p.to_json();
        // In another library the bit is not there: it comes with the project.
        let mut other = Library::new();
        let back = load(&Project::from_json(&text).unwrap(), &mut other).unwrap();
        assert_eq!(back.layers[0].depth, 7.0);
        assert_eq!(back.stock, stock);
        assert_eq!(back.machine.unwrap().name, "Dummy");
        assert_eq!(back.added_tools, vec!["My cutter".to_string()]);
        assert!(other.get(&back.layers[0].tool.clone().unwrap()).is_some());
        // The ready-made bits are not written into the project.
        let section: MillSection = p.section(SECTION).unwrap().unwrap();
        assert_eq!(section.tools.len(), layers.iter().filter_map(|l| l.tool.clone()).collect::<std::collections::HashSet<_>>().len());
        assert!(section.tools.iter().any(|t| t.name == "My cutter"));
    }

    #[test]
    fn a_project_without_the_section_gets_the_default_layers_and_a_newer_one_is_refused() {
        let mut lib = Library::new();
        let p = Project::new(Scene::default());
        let l = load(&p, &mut lib).unwrap();
        assert_eq!(l.layers.len(), LAYER_COUNT);
        assert!(l.machine.is_none());
        let mut newer = Project::new(Scene::default());
        newer.set_section(SECTION, &serde_json::json!({ "format": 99, "layers": [], "stock": Stock::default(), "machine": { "name": "x", "x": 1, "y": 1, "z": 1 } }));
        assert!(load(&newer, &mut lib).is_err());
    }

    #[test]
    fn wrong_layers_are_replaced_and_missing_ones_added() {
        let mut lib = Library::new();
        let mut layers = default_layers(&lib);
        layers.truncate(3);
        layers[1].feed = f64::NAN;
        let mut p = Project::new(Scene::default());
        let mut s = MillSection::new(&layers, &Stock::default(), &Device::dummy(), &lib);
        s.stock.thickness = -4.0;
        p.set_section(SECTION, &s);
        // NaN is written as null by JSON, which a number does not read: the whole section is then refused or repaired.
        match load(&Project::from_json(&p.to_json()).unwrap(), &mut lib) {
            Ok(l) => {
                assert_eq!(l.layers.len(), LAYER_COUNT);
                assert!(l.layers.iter().all(|x| x.check().is_ok()));
                assert_eq!(l.stock.thickness, 10.0);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn the_project_of_another_machine_is_noticed() {
        let m = Machine::of(&Device::dummy());
        assert!(m.differences(&Device::dummy()).is_none());
        let mut d = Device::dummy();
        d.area.x = 600.0;
        assert!(m.differences(&d).unwrap().starts_with("Dummy"));
    }

    #[test]
    fn the_sections_of_other_programs_are_kept_next_to_this_one() {
        let lib = Library::new();
        let mut p = Project::new(Scene::default());
        p.set_section("laser", &serde_json::json!({ "format": 1, "material": "Plywood" }));
        let kept = p.other_sections(SECTION);
        let mut out = Project::new(Scene::default());
        out.set_section(SECTION, &MillSection::new(&default_layers(&lib), &Stock::default(), &Device::dummy(), &lib));
        out.restore_sections(&kept);
        let mut names = out.section_names();
        names.sort();
        assert_eq!(names, vec!["laser", "mill"]);
    }
}
