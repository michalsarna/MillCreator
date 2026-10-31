//! A library of layer settings: the way of cutting of a layer (the operation, the bit, depths, speeds, tabs, ramps…) saved under a name and
//! put on any layer later. Kept in a file of the shared data folder.
use crate::layers::MillLayer;
use serde::{Deserialize, Serialize};

pub const MAX_PRESETS: usize = 60;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayerPreset {
    pub name: String,
    pub layer: MillLayer,
}

impl LayerPreset {
    pub fn of(l: &MillLayer, name: &str) -> LayerPreset {
        let mut layer = l.clone();
        // What belongs to the shapes of the layer is not part of a way of cutting.
        layer.tabs.manual.clear();
        layer.rest_of = None;
        LayerPreset { name: name.trim().chars().take(60).collect(), layer }
    }

    /// Put the way of cutting on a layer; its name, colour, visibility, rank, the layer it takes over from and the tabs placed by hand stay.
    pub fn apply(&self, to: &mut MillLayer) {
        let keep = (to.name.clone(), to.color, to.visible, to.enabled, to.rank, to.rest_of, std::mem::take(&mut to.tabs.manual));
        *to = self.layer.clone();
        (to.name, to.color, to.visible, to.enabled, to.rank, to.rest_of, to.tabs.manual) = keep;
    }
}

fn file() -> Option<std::path::PathBuf> {
    base_core::paths::base_dir().map(|d| d.join("mill_layer_presets.json"))
}

/// What the file holds; a preset whose numbers are out of range is left out.
pub fn parse(text: &str) -> Vec<LayerPreset> {
    let mut v: Vec<LayerPreset> = serde_json::from_str(text).unwrap_or_default();
    v.retain(|p| !p.name.trim().is_empty() && p.name.len() <= 120 && p.layer.check().is_ok());
    v.truncate(MAX_PRESETS);
    v
}

pub fn load() -> Vec<LayerPreset> {
    file().and_then(|f| std::fs::read_to_string(f).ok()).map(|t| parse(&t)).unwrap_or_default()
}

pub fn store(v: &[LayerPreset]) {
    if let (Some(f), Ok(t)) = (file(), serde_json::to_string_pretty(v)) {
        let _ = std::fs::write(f, t);
    }
}

/// Add a preset (or replace the one of the same name).
pub fn put(list: &mut Vec<LayerPreset>, p: LayerPreset) -> bool {
    if p.name.is_empty() {
        return false;
    }
    if let Some(old) = list.iter_mut().find(|q| q.name == p.name) {
        *old = p;
        return true;
    }
    if list.len() >= MAX_PRESETS {
        return false;
    }
    list.push(p);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layers::{default_layers, Operation};
    use crate::tools::Library;

    #[test]
    fn a_preset_carries_the_way_of_cutting_and_leaves_the_layer_its_name_and_its_hand_made_tabs() {
        let lib = Library::new();
        let mut layers = default_layers(&lib);
        layers[0].op = Operation::Pocket;
        layers[0].depth = 4.0;
        layers[0].leave = 0.2;
        layers[0].tabs.enabled = true;
        layers[0].tabs.manual.push(crate::layers::ManualTab { shape: 7, at: 0.5 });
        let p = LayerPreset::of(&layers[0], "  Pocket 4 mm ");
        assert_eq!(p.name, "Pocket 4 mm");
        assert!(p.layer.tabs.manual.is_empty(), "tabs placed on shapes are not a way of cutting");
        let mut other = layers[1].clone();
        other.name = "My layer".into();
        other.color = [1, 2, 3];
        other.rank = 9;
        other.tabs.manual.push(crate::layers::ManualTab { shape: 3, at: 0.1 });
        p.apply(&mut other);
        assert_eq!((other.op, other.depth, other.leave, other.tabs.enabled), (Operation::Pocket, 4.0, 0.2, true));
        assert_eq!((other.name.as_str(), other.color, other.rank), ("My layer", [1, 2, 3], 9));
        assert_eq!(other.tabs.manual.len(), 1, "its own hand-placed tab stays");
    }

    #[test]
    fn the_list_replaces_by_name_is_limited_and_reads_back_without_bad_entries() {
        let lib = Library::new();
        let layers = default_layers(&lib);
        let mut list = vec![];
        assert!(put(&mut list, LayerPreset::of(&layers[0], "A")));
        assert!(put(&mut list, LayerPreset::of(&layers[1], "A")));
        assert_eq!(list.len(), 1, "same name: replaced");
        assert!(!put(&mut list, LayerPreset::of(&layers[0], "  ")), "no name, no preset");
        for i in 0..MAX_PRESETS + 5 {
            put(&mut list, LayerPreset::of(&layers[0], &format!("P{i}")));
        }
        assert_eq!(list.len(), MAX_PRESETS);
        // A bad number is dropped when the file is read, broken text gives nothing.
        let mut bad = LayerPreset::of(&layers[0], "bad");
        bad.layer.depth = -5.0;
        let text = serde_json::to_string(&vec![LayerPreset::of(&layers[0], "good"), bad]).unwrap();
        let back = parse(&text);
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].name, "good");
        assert!(parse("nonsense").is_empty());
    }
}
