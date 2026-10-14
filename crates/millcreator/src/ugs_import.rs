//! The layers for a design of UGS Designer: one for each way the shapes are cut.
use crate::layers::{default_layers, MillLayer, Operation};
use crate::tools::{Dims, Library, Tool, ToolKind};
use base_core::ugsd::{Cut, CutType, Import};

fn operation(c: CutType) -> (Operation, &'static str) {
    match c {
        CutType::OutsidePath => (Operation::ProfileOutside, "Cut out"),
        CutType::InsidePath => (Operation::ProfileInside, "Holes and windows"),
        CutType::Pocket => (Operation::Pocket, "Pocket"),
        CutType::CenterDrill => (Operation::Drill, "Drill"),
        _ => (Operation::OnPath, "Engrave"),
    }
}

/// The flat bit of the design's diameter: one the library has, or a new one of the user's own.
fn flat_bit(tools: &mut Library, diameter: f64) -> Option<Tool> {
    if let Some(t) = tools.all().find(|t| t.kind == ToolKind::FlatEnd && (t.dims.diameter - diameter).abs() < 1e-6) {
        return Some(t.clone());
    }
    let mut dims = Dims::default();
    dims.diameter = diameter;
    dims.cut_length = (diameter * 4.0).max(3.0);
    dims.shank = diameter.max(3.175);
    dims.overall = dims.cut_length + 25.0;
    let t = Tool::new("new", &format!("Flat {diameter} mm"), ToolKind::FlatEnd, dims);
    let id = tools.add(t).ok()?;
    tools.get(&id).cloned()
}

/// The layers of the project for the cuts of the design. Layers the design does not use stay as in a new project.
/// Returns the layers and the name of the bit added to the library, if any.
pub fn layers(imp: &Import, tools: &mut Library, max_rpm: f64) -> (Vec<MillLayer>, Option<String>) {
    let mut layers = default_layers(tools);
    let bit = imp.settings.tool_diameter.and_then(|d| flat_bit(tools, d));
    for c in &imp.cuts {
        let Some(slot) = layers.get_mut(c.index) else { continue };
        *slot = layer_for(c, slot.clone(), bit.as_ref(), &imp.settings, max_rpm);
    }
    let added = bit.filter(|b| tools.user.iter().any(|u| u.id == b.id)).map(|b| b.name);
    (layers, added)
}

fn layer_for(c: &Cut, mut l: MillLayer, bit: Option<&Tool>, s: &base_core::ugsd::Settings, max_rpm: f64) -> MillLayer {
    let (op, name) = operation(c.cut);
    l.op = op;
    l.name = format!("{name} {} mm", fmt(c.cut_depth));
    l.enabled = c.cut != CutType::None;
    if let Some(t) = bit {
        l.use_tool(t);
    }
    l.depth = c.cut_depth.clamp(0.1, 1000.0);
    // The Designer cuts a millimetre per pass unless the file says otherwise.
    l.step_down = s.depth_per_pass.unwrap_or(1.0).clamp(0.1, l.depth);
    if c.feed > 0.0 {
        l.feed = c.feed;
    }
    if let Some(p) = s.plunge_speed {
        l.plunge = p;
    }
    if c.spindle_pct > 0.0 && max_rpm > 0.0 {
        l.rpm = (c.spindle_pct.min(100.0) / 100.0 * max_rpm).round();
    }
    if l.check().is_err() {
        let d = MillLayer { op, ..l.clone() };
        return if d.check().is_ok() { d } else { l };
    }
    l
}

fn fmt(v: f64) -> String {
    if (v - v.round()).abs() < 1e-9 {
        format!("{}", v as i64)
    } else {
        format!("{v:.1}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_way_of_cutting_gets_its_layer() {
        let d = r#"{"version":"1","settings":{"toolDiameter":1.0,"plungeSpeed":120},"entities":[
          {"type":"RECTANGLE","cutType":"OUTSIDE_PATH","cutDepth":10,"feedRate":600,"spindleSpeed":50,"transform":{"m00":20,"m10":0,"m01":0,"m11":10,"m02":0,"m12":0}},
          {"type":"RECTANGLE","cutType":"POCKET","cutDepth":3,"feedRate":900,"spindleSpeed":100,"transform":{"m00":5,"m10":0,"m01":0,"m11":5,"m02":2,"m12":2}}]}"#;
        let imp = base_core::ugsd::import(d, 100.0, 16).unwrap();
        let mut lib = Library::new();
        let (l, added) = layers(&imp, &mut lib, 20000.0);
        assert_eq!((l[0].op, l[0].depth, l[0].feed, l[0].rpm), (Operation::ProfileOutside, 10.0, 600.0, 10000.0));
        assert_eq!((l[1].op, l[1].depth, l[1].plunge), (Operation::Pocket, 3.0, 120.0));
        assert_eq!(l[0].tool, l[1].tool);
        assert_eq!(l[0].step_down, 1.0);
        assert!(l.iter().all(|x| x.check().is_ok()));
        // A 1 mm flat bit is among the ready-made ones.
        assert_eq!(added, None);
        assert_eq!(lib.get(l[0].tool.as_deref().unwrap()).unwrap().dims.diameter, 1.0);
    }

    #[test]
    fn a_bit_the_library_lacks_is_added() {
        let d = r#"{"version":"1","settings":{"toolDiameter":2.5},"entities":[{"type":"RECTANGLE","cutType":"ON_PATH","cutDepth":1,"transform":{"m00":5,"m10":0,"m01":0,"m11":5,"m02":0,"m12":0}}]}"#;
        let imp = base_core::ugsd::import(d, 100.0, 16).unwrap();
        let mut lib = Library::new();
        let (l, added) = layers(&imp, &mut lib, 20000.0);
        assert_eq!(added.as_deref(), Some("Flat 2.5 mm"));
        assert_eq!(l[0].op, Operation::OnPath);
    }
}
