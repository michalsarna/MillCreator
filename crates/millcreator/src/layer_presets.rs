//! The way of cutting of a layer (the operation, the bit, depths, speeds, tabs, ramps…) saved under a name and put on any layer later.
//! The list, the window and the file are in `base_ui::layer_presets_ui`; this says what a preset holds. The file is in the shared
//! data folder.
use crate::layers::MillLayer;

/// The file in the shared data folder.
pub const FILE: &str = "mill_layer_presets.json";

/// What of a layer a preset keeps: everything but what belongs to the shapes of the layer.
pub fn of(l: &MillLayer) -> MillLayer {
    let mut layer = l.clone();
    layer.tabs.manual.clear();
    layer.rest_of = None;
    layer
}

/// A preset whose numbers are out of range is left out of the list.
pub fn valid(l: &MillLayer) -> bool {
    l.check().is_ok()
}

/// Put the way of cutting on a layer; its name, colour, visibility, rank, the layer it takes over from and the tabs placed by hand stay.
/// A bit that is not in the library any more is left off the layer; the answer says so.
pub fn apply(preset: &MillLayer, to: &mut MillLayer, bit_exists: &dyn Fn(&str) -> bool) -> bool {
    let keep = (to.name.clone(), to.color, to.visible, to.enabled, to.rank, to.rest_of, std::mem::take(&mut to.tabs.manual));
    *to = preset.clone();
    (to.name, to.color, to.visible, to.enabled, to.rank, to.rest_of, to.tabs.manual) = keep;
    match to.tool.as_deref() {
        Some(id) if !bit_exists(id) => {
            to.tool = None;
            false
        }
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layers::{default_layers, Operation};
    use crate::tools::Library;
    use base_ui::layer_presets_ui::Library as Presets;

    #[test]
    fn a_preset_carries_the_way_of_cutting_and_leaves_the_layer_its_name_and_its_hand_made_tabs() {
        let lib = Library::new();
        let mut layers = default_layers(&lib);
        layers[0].op = Operation::Pocket;
        layers[0].depth = 4.0;
        layers[0].leave = 0.2;
        layers[0].tabs.enabled = true;
        layers[0].tabs.manual.push(crate::layers::ManualTab { shape: 7, at: 0.5 });
        let p = of(&layers[0]);
        assert!(p.tabs.manual.is_empty(), "tabs placed on shapes are not a way of cutting");
        let mut other = layers[1].clone();
        other.name = "My layer".into();
        other.color = [1, 2, 3];
        other.rank = 9;
        other.tabs.manual.push(crate::layers::ManualTab { shape: 3, at: 0.1 });
        assert!(apply(&p, &mut other, &|id| lib.get(id).is_some()));
        assert_eq!((other.op, other.depth, other.leave, other.tabs.enabled), (Operation::Pocket, 4.0, 0.2, true));
        assert_eq!((other.name.as_str(), other.color, other.rank), ("My layer", [1, 2, 3], 9));
        assert_eq!(other.tabs.manual.len(), 1, "its own hand-placed tab stays");
    }

    #[test]
    fn a_preset_with_a_bit_that_is_not_in_the_library_leaves_the_layer_without_a_bit() {
        let lib = Library::new();
        let layers = default_layers(&lib);
        let mut p = of(&layers[0]);
        p.tool = Some("gone-bit".into());
        let mut other = layers[1].clone();
        assert!(!apply(&p, &mut other, &|id| lib.get(id).is_some()), "the caller is told");
        assert!(other.tool.is_none());
        p.tool = Some("std-flat-3".into());
        assert!(apply(&p, &mut other, &|id| lib.get(id).is_some()));
        assert_eq!(other.tool.as_deref(), Some("std-flat-3"));
    }

    #[test]
    fn the_old_file_of_presets_is_read_and_a_bad_number_is_left_out() {
        let lib = Library::new();
        let layers = default_layers(&lib);
        let mut bad = of(&layers[0]);
        bad.depth = -5.0;
        let text = serde_json::to_string(&vec![base_ui::layer_presets_ui::Named { name: "good".to_string(), layer: of(&layers[0]) }, base_ui::layer_presets_ui::Named { name: "bad".to_string(), layer: bad }]).unwrap();
        let back = base_ui::layer_presets_ui::parse::<MillLayer>(&text, &valid);
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].name, "good");
        let mut list: Presets<MillLayer> = Presets::from_path(None, &valid);
        assert!(list.put("A", of(&layers[0])));
    }
}
