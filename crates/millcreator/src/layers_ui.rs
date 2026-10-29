//! The Layers tab (what is done with the shapes of each layer) and the Material tab.
use crate::layers::{CutDir, Entry, MillLayer, Operation, PictureMethod, TabKind};
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

    /// The layers that hold pictures, from a drawing that was just opened or started: they were set up by the person who made it.
    pub(crate) fn reset_picture_layers(&mut self) {
        self.picture_layers = self.picture_layer_set();
    }

    fn picture_layer_set(&self) -> std::collections::HashSet<usize> {
        self.scene.shapes.iter().filter(|s| matches!(s.kind, base_core::scene::Kind::Image(_))).map(|s| s.layer).collect()
    }

    /// A picture put on a layer switches the layer to the Picture operation (once: afterwards the person decides).
    pub(crate) fn sync_picture_layers(&mut self) {
        let now = self.picture_layer_set();
        for &l in &now {
            if self.picture_layers.insert(l) {
                if let Some(layer) = self.layers.get_mut(l) {
                    layer.op = Operation::Picture;
                }
            }
        }
        self.picture_layers.retain(|l| now.contains(l));
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
        let show_all = self.show_all_layers;
        let mut move_req: Option<(usize, bool)> = None;
        let order = crate::layers::layer_order(&self.layers);
        base_ui::widgets::list_with_splitter(ui, "layer_list", 190.0, |ui| {
            for i in order {
                // Of 64 layers show the ones with shapes and the one being edited (the colour bar has all of them).
                if !(show_all || counts[i] > 0 || i == active) {
                    continue;
                }
                let l = &mut self.layers[i];
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 3.0;
                    // The layers are done in the order of this list: from the top down.
                    if ui.add(egui::Button::image(base_ui::icons::slot(Some("chevron-up"), theme::text())).small()).on_hover_text(tr("Earlier in the job")).clicked() {
                        move_req = Some((i, true));
                    }
                    if ui.add(egui::Button::image(base_ui::icons::slot(Some("chevron-down"), theme::text())).small()).on_hover_text(tr("Later in the job")).clicked() {
                        move_req = Some((i, false));
                    }
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, 3.0, l.color32());
                    changed |= ui.checkbox(&mut l.enabled, "").on_hover_text(tr("Milled: taken into the toolpaths and the G-code")).changed();
                    if base_ui::widgets::toggle_icon(ui, "eye", &tr("Shown in the work area"), l.visible) {
                        l.visible = !l.visible;
                        changed = true;
                    }
                    let text = format!("{}  ·  {}", l.name, tr(l.op.label()));
                    let r = ui.add_sized([ui.available_width() - 44.0, 22.0], egui::Button::selectable(i == active, text));
                    if r.clicked() {
                        pick = Some(i);
                    }
                    ui.label(RichText::new(counts[i].to_string()).color(theme::text_dim())).on_hover_text(tr("Shapes on this layer"));
                });
            }
        });
        if let Some((i, up)) = move_req {
            crate::layers::move_layer(&mut self.layers, i, up);
            changed = true;
        }
        ui.checkbox(&mut self.show_all_layers, trf("Show all {} layers", &[&self.layers.len()]));
        if let Some(i) = pick {
            self.ed.active_layer = i;
        }
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
                    if l.op == Operation::Pocket || l.op == Operation::Picture {
                        ui.label(tr("Step over"));
                        changed |= ui.add(egui::DragValue::new(&mut l.step_over_pct).range(1.0..=100.0).suffix(" %")).changed();
                        ui.end_row();
                    }
                    if l.op == Operation::Picture {
                        ui.label(tr("Picture method"));
                        egui::ComboBox::from_id_salt("layer_picture").selected_text(tr(l.picture.label())).width(200.0).show_ui(ui, |ui| {
                            for m in PictureMethod::ALL {
                                changed |= ui.selectable_value(&mut l.picture, m, tr(m.label())).on_hover_text(tr(m.hint())).changed();
                            }
                        });
                        ui.end_row();
                        if l.picture.dithers() {
                            ui.label(tr("Dithering"));
                            egui::ComboBox::from_id_salt("layer_dither").selected_text(tr(l.dither.label())).width(200.0).show_ui(ui, |ui| {
                                for d in base_core::raster::Dither::ALL {
                                    changed |= ui.selectable_value(&mut l.dither, d, tr(d.label())).on_hover_text(tr(d.hint())).changed();
                                }
                            });
                            ui.end_row();
                        }
                        ui.label(tr("Scan angle"));
                        changed |= ui.add(egui::DragValue::new(&mut l.scan_angle).range(-360.0..=360.0).suffix(" °")).on_hover_text(tr("The angle of the rows of the engraving")).changed();
                        ui.end_row();
                    }
                    let walls = matches!(l.op, Operation::ProfileOutside | Operation::ProfileInside | Operation::Pocket);
                    if walls {
                        ui.label(tr("Direction"));
                        egui::ComboBox::from_id_salt("layer_dir").selected_text(tr(l.direction.label())).width(200.0).show_ui(ui, |ui| {
                            for dir in CutDir::ALL {
                                changed |= ui.selectable_value(&mut l.direction, dir, tr(dir.label())).changed();
                            }
                        });
                        ui.end_row();
                        ui.label(tr("Stock to leave"));
                        ui.horizontal(|ui| {
                            changed |= drag_len(ui, units, &mut l.leave, 0.05, Some((0.0, 100.0))).changed();
                            changed |= ui.checkbox(&mut l.finish, tr("Finishing pass")).changed();
                        });
                        ui.end_row();
                    }
                    if l.op != Operation::Drill {
                        ui.label(tr("Entry"));
                        egui::ComboBox::from_id_salt("layer_entry").selected_text(tr(l.entry.label())).width(200.0).show_ui(ui, |ui| {
                            for e in Entry::ALL {
                                changed |= ui.selectable_value(&mut l.entry, e, tr(e.label())).changed();
                            }
                        });
                        ui.end_row();
                        if l.entry != Entry::Plunge {
                            ui.label(tr("Ramp angle"));
                            changed |= ui.add(egui::DragValue::new(&mut l.ramp_angle).range(0.1..=89.0).speed(0.1).suffix(" °")).changed();
                            ui.end_row();
                            ui.label(tr("Ramp feed"));
                            changed |= ui.add(egui::DragValue::new(&mut l.ramp_feed).range(0.0..=100000.0).suffix(" mm/min")).on_hover_text(tr("0 = the plunge feed")).changed();
                            ui.end_row();
                        }
                    }
                    if matches!(l.op, Operation::ProfileOutside | Operation::ProfileInside | Operation::OnPath) {
                        ui.label(tr("Tabs"));
                        changed |= ui.checkbox(&mut l.tabs.enabled, "").changed();
                        ui.end_row();
                        if l.tabs.enabled {
                            ui.label(tr("Tab type"));
                            egui::ComboBox::from_id_salt("layer_tab_kind").selected_text(tr(l.tabs.kind.label())).width(200.0).show_ui(ui, |ui| {
                                for k in TabKind::ALL {
                                    changed |= ui.selectable_value(&mut l.tabs.kind, k, tr(k.label())).changed();
                                }
                            });
                            ui.end_row();
                            ui.label(tr("Tab width"));
                            changed |= drag_len(ui, units, &mut l.tabs.length, 0.1, Some((0.1, 1000.0))).changed();
                            ui.end_row();
                            if l.tabs.kind != TabKind::Full {
                                ui.label(tr("Tab height"));
                                changed |= drag_len(ui, units, &mut l.tabs.height, 0.1, Some((0.0, 1000.0))).changed();
                                ui.end_row();
                            }
                            if l.tabs.kind == TabKind::Ramp {
                                ui.label(tr("Tab slope"));
                                changed |= ui.add(egui::DragValue::new(&mut l.tabs.slope).range(5.0..=89.0).suffix(" °")).changed();
                                ui.end_row();
                            }
                            ui.label(tr("Tab spacing"));
                            changed |= drag_len(ui, units, &mut l.tabs.spacing, 1.0, Some((1.0, 100000.0))).changed();
                            ui.end_row();
                            ui.label(tr("Fewest and most tabs"));
                            ui.horizontal(|ui| {
                                changed |= ui.add(egui::DragValue::new(&mut l.tabs.min).range(0..=100)).changed();
                                changed |= ui.add(egui::DragValue::new(&mut l.tabs.max).range(0..=100)).changed();
                                l.tabs.max = l.tabs.max.max(l.tabs.min);
                            });
                            ui.end_row();
                            ui.label("");
                            changed |= ui.checkbox(&mut l.tabs.skip_inner, tr("No tabs on holes")).changed();
                            ui.end_row();
                        }
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
        ui.add_space(8.0);
        ui.label(RichText::new(tr("First pass")).strong());
        changed |= ui.checkbox(&mut self.stock.first_pass, tr("Start with a shallow pass along the edges")).on_hover_text(tr("Every layer first goes once along the edges it will cut, only this deep, which breaks the top of the material cleanly. Pockets get it along their edge only; drilling has none. It works with every bit.")).changed();
        ui.add_enabled_ui(self.stock.first_pass, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr("Depth of the first pass"));
                changed |= drag_len(ui, units, &mut self.stock.first_pass_depth, 0.05, Some((0.01, MAX_THICKNESS))).changed();
            });
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
