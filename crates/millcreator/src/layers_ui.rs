//! The Layers tab (what is done with the shapes of each layer) and the Material tab.
use crate::layers::{MillLayer, Operation};
use crate::materials::{MaterialKind, MAX_THICKNESS};
use crate::App;
use base_ui::editor::EditorHost;
use base_ui::i18n::{tr, trf};
use base_ui::theme;
use base_ui::units_ui::drag_len;
use eframe::egui::{self, Color32, RichText};

impl App {
    /// How many shapes lie on each layer.
    pub(crate) fn shapes_per_layer(&self) -> Vec<usize> {
        let mut n = vec![0; self.layers.len()];
        for s in &self.scene.shapes {
            if let Some(c) = n.get_mut(s.layer) {
                *c += 1;
            }
        }
        n
    }

    /// Put the selected shapes on a layer.
    pub(crate) fn assign_selection_to_layer(&mut self, layer: usize) {
        let ids = self.scene.unlocked_ids(&self.ed.sel.iter().copied().collect::<Vec<_>>());
        if ids.is_empty() {
            return;
        }
        self.checkpoint();
        for id in ids {
            if let Some(s) = self.scene.shape_mut(id) {
                s.layer = layer;
            }
        }
        self.touch();
    }

    pub(crate) fn layers_panel(&mut self, ui: &mut egui::Ui) {
        let counts = self.shapes_per_layer();
        let active = self.ed.active_layer.min(self.layers.len().saturating_sub(1));
        let mut pick = None;
        let mut changed = false;
        egui::ScrollArea::vertical().id_salt("layer_list").max_height(250.0).auto_shrink([false, true]).show(ui, |ui| {
            for (i, l) in self.layers.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 3.0, l.color32());
                    changed |= ui.checkbox(&mut l.enabled, "").on_hover_text(tr("Milled: taken into the toolpaths and the G-code")).changed();
                    changed |= ui.toggle_value(&mut l.visible, "👁").on_hover_text(tr("Shown in the work area")).changed();
                    let text = format!("{}  ·  {}", l.name, tr(l.op.label()));
                    let r = ui.add_sized([ui.available_width() - 44.0, 22.0], egui::Button::selectable(i == active, text));
                    if r.clicked() {
                        pick = Some(i);
                    }
                    ui.label(RichText::new(counts[i].to_string()).color(theme::text_dim())).on_hover_text(tr("Shapes on this layer"));
                });
            }
        });
        if let Some(i) = pick {
            self.ed.active_layer = i;
        }
        ui.separator();
        let units = self.device.units;
        let lib = &self.tools;
        let li = self.ed.active_layer.min(self.layers.len().saturating_sub(1));
        let mut assign = false;
        let mut open_library = false;
        {
            let l = &mut self.layers[li];
            egui::ScrollArea::vertical().id_salt("layer_edit").auto_shrink([false, false]).show(ui, |ui| {
                egui::Grid::new("layer_grid").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
                    ui.label(tr("Name"));
                    changed |= ui.text_edit_singleline(&mut l.name).changed();
                    ui.end_row();
                    ui.label(tr("Colour"));
                    changed |= ui.color_edit_button_srgb(&mut l.color).changed();
                    ui.end_row();
                    ui.label(tr("Operation"));
                    egui::ComboBox::from_id_salt("layer_op").selected_text(tr(l.op.label())).width(200.0).show_ui(ui, |ui| {
                        for op in Operation::ALL {
                            changed |= ui.selectable_value(&mut l.op, op, tr(op.label())).on_hover_text(tr(op.hint())).changed();
                        }
                    });
                    ui.end_row();
                    ui.label(tr("Bit"));
                    ui.horizontal(|ui| {
                        let current = l.tool.as_deref().and_then(|id| lib.get(id)).map(|t| t.name.clone()).unwrap_or_else(|| tr("No bit").to_string());
                        let mut chosen = None;
                        egui::ComboBox::from_id_salt("layer_tool").selected_text(current).width(200.0).show_ui(ui, |ui| {
                            for t in lib.all() {
                                if ui.selectable_label(l.tool.as_deref() == Some(t.id.as_str()), &t.name).clicked() {
                                    chosen = Some(t.clone());
                                }
                            }
                        });
                        if let Some(t) = chosen {
                            l.use_tool(&t);
                            changed = true;
                        }
                        if ui.button("…").on_hover_text(tr("Bit library")).clicked() {
                            open_library = true;
                        }
                    });
                    ui.end_row();
                    ui.label(tr("Depth"));
                    changed |= drag_len(ui, units, &mut l.depth, 0.1, Some((0.0, 1000.0))).changed();
                    ui.end_row();
                    ui.label(tr("Pass depth"));
                    changed |= drag_len(ui, units, &mut l.step_down, 0.05, Some((0.01, 1000.0))).changed();
                    ui.end_row();
                    if l.op == Operation::Pocket {
                        ui.label(tr("Step over"));
                        changed |= ui.add(egui::DragValue::new(&mut l.step_over_pct).range(1.0..=100.0).suffix(" %")).changed();
                        ui.end_row();
                    }
                    ui.label(tr("Feed"));
                    changed |= ui.add(egui::DragValue::new(&mut l.feed).range(1.0..=100000.0).suffix(" mm/min")).changed();
                    ui.end_row();
                    ui.label(tr("Plunge feed"));
                    changed |= ui.add(egui::DragValue::new(&mut l.plunge).range(1.0..=100000.0).suffix(" mm/min")).changed();
                    ui.end_row();
                    ui.label(tr("Spindle speed"));
                    changed |= ui.add(egui::DragValue::new(&mut l.rpm).range(0.0..=1000000.0).suffix(" rpm")).changed();
                    ui.end_row();
                });
                ui.add_space(4.0);
                ui.label(RichText::new(tr(l.op.hint())).color(theme::text_dim()));
                ui.add_space(6.0);
                if ui.button(trf("Put the selected shapes on this layer", &[])).clicked() {
                    assign = true;
                }
                let passes = l.passes().len();
                ui.label(RichText::new(trf("{} passes to {} mm", &[&passes, &l.depth])).color(theme::text_dim()));
            });
        }
        if open_library {
            self.tools_win.open = true;
        }
        if assign {
            self.assign_selection_to_layer(li);
        }
        if changed {
            self.touch();
        }
    }

    pub(crate) fn material_panel(&mut self, ui: &mut egui::Ui) {
        let mut changed = false;
        ui.label(RichText::new(tr("Material of the workpiece")).strong());
        ui.add_space(4.0);
        let units = self.device.units;
        for kind in MaterialKind::ALL {
            let on = self.stock.kind == kind;
            ui.horizontal(|ui| {
                let c = if on { self.stock.color } else { kind.color() };
                let (rect, _) = ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::hover());
                ui.painter().rect_filled(rect, 4.0, Color32::from_rgb(c[0], c[1], c[2]));
                ui.painter().rect_stroke(rect, 4.0, egui::Stroke::new(1.0, theme::border()), egui::StrokeKind::Inside);
                if ui.add_sized([ui.available_width(), 24.0], egui::Button::selectable(on, tr(kind.label()))).clicked() && !on {
                    self.stock.set_kind(kind);
                    changed = true;
                }
            });
        }
        ui.add_space(8.0);
        egui::Grid::new("material_grid").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
            ui.label(tr("Colour"));
            ui.add_enabled_ui(self.stock.kind.colour_is_free(), |ui| {
                changed |= ui.color_edit_button_srgb(&mut self.stock.color).changed();
            });
            ui.end_row();
            ui.label(tr("Thickness"));
            changed |= drag_len(ui, units, &mut self.stock.thickness, 0.5, Some((0.1, MAX_THICKNESS))).changed();
            ui.end_row();
        });
        ui.add_space(4.0);
        ui.label(RichText::new(tr("Only the colour of a plastic can be changed. The material and its thickness are shown in the 3D view and saved in the project.")).color(theme::text_dim()));
        if changed {
            self.touch();
        }
    }
}

/// A layer's one-line summary, for lists.
#[allow(dead_code)]
pub fn summary(l: &MillLayer) -> String {
    format!("{} · {} · {} mm", l.name, tr(l.op.label()), l.depth)
}
