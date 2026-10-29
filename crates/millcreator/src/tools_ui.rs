//! The window of the bit library and the picture of a bit with its dimensions marked where they are measured.
use crate::tools::{Cutting, Dim, Dims, Library, Tool, ToolKind};
use base_ui::i18n::tr;
use base_ui::theme;
use base_ui::window::{delete_button_enabled, import_button, popup, save_button_enabled};
use egui::{Align2, Color32, FontId, Pos2, Rect, Stroke, Vec2};

// ───────────────────────── the picture of a bit ─────────────────────────

/// A point of the picture in mm: X from the axis of the bit, Y up from the tip.
type P = (f64, f64);

/// The right half of the outline of the bit from the tip up to the top of the shank. The left half is the mirror image.
pub fn profile(kind: ToolKind, d: &Dims) -> Vec<P> {
    let r = d.diameter / 2.0;
    let sr = d.shank / 2.0;
    let overall = d.overall.max(d.cut_length);
    let mut v: Vec<P> = vec![];
    let arc = |v: &mut Vec<P>, cx: f64, cy: f64, rad: f64, a0: f64, a1: f64| {
        let n = 16;
        for i in 0..=n {
            let a = a0 + (a1 - a0) * i as f64 / n as f64;
            v.push((cx + rad * a.cos(), cy + rad * a.sin()));
        }
    };
    let half_angle = (d.angle.clamp(1.0, 179.0) / 2.0).to_radians();
    let cut_top;
    match kind {
        ToolKind::FlatEnd => {
            v.push((0.0, 0.0));
            v.push((r, 0.0));
            cut_top = d.cut_length;
        }
        ToolKind::BallNose => {
            arc(&mut v, 0.0, r, r, -std::f64::consts::FRAC_PI_2, 0.0);
            cut_top = d.cut_length.max(r);
        }
        ToolKind::BullNose => {
            let c = d.corner.min(r);
            v.push((0.0, 0.0));
            v.push((r - c, 0.0));
            arc(&mut v, r - c, c, c, -std::f64::consts::FRAC_PI_2, 0.0);
            cut_top = d.cut_length.max(c);
        }
        ToolKind::VBit | ToolKind::Drill => {
            let h = r / half_angle.tan();
            v.push((0.0, 0.0));
            v.push((r, h));
            cut_top = d.cut_length.max(h);
        }
        ToolKind::Engraver => {
            let t = (d.tip / 2.0).min(r);
            let h = (r - t) / half_angle.tan();
            v.push((0.0, 0.0));
            v.push((t, 0.0));
            v.push((r, h));
            cut_top = d.cut_length.max(h);
        }
        ToolKind::TaperedBall => {
            // The ball of the tip, then the flank that is tangent to it.
            let t = (d.tip / 2.0).min(r).max(0.01);
            let (sa, ca) = (half_angle.sin(), half_angle.cos());
            arc(&mut v, 0.0, t, t, -std::f64::consts::FRAC_PI_2, -half_angle);
            let run = ((r - t * ca) / sa).max(0.0);
            let h = t - t * sa + run * ca;
            v.push((r, h));
            cut_top = d.cut_length.max(h);
        }
    }
    v.push((r, cut_top));
    if (sr - r).abs() > 1e-9 {
        v.push((sr, cut_top));
    }
    v.push((sr, overall));
    v
}

/// A mark of a dimension: the letter, where it is measured (two points in mm) and which way the label sits.
#[derive(Clone, Debug, PartialEq)]
pub struct Mark {
    pub dim: Dim,
    pub from: P,
    pub to: P,
}

/// Where each dimension of the kind is measured.
pub fn marks(kind: ToolKind, d: &Dims) -> Vec<Mark> {
    let r = d.diameter / 2.0;
    let sr = d.shank / 2.0;
    let overall = d.overall.max(d.cut_length);
    let half_angle = (d.angle.clamp(1.0, 179.0) / 2.0).to_radians();
    let widest = r.max(sr);
    let mut out = vec![];
    for dim in kind.dims() {
        let m = match dim {
            Dim::Diameter => {
                // Where the bit is as wide as it gets: at the top of a cone, in the middle of a straight part.
                let y = match kind {
                    ToolKind::VBit | ToolKind::Drill => (r / half_angle.tan()).min(d.cut_length),
                    ToolKind::Engraver | ToolKind::TaperedBall => ((r - d.tip / 2.0).max(0.0) / half_angle.tan()).min(d.cut_length),
                    _ => d.cut_length * 0.6,
                };
                Mark { dim: *dim, from: (-r, y), to: (r, y) }
            }
            Dim::CutLength => Mark { dim: *dim, from: (-widest - 1.5, 0.0), to: (-widest - 1.5, d.cut_length) },
            Dim::Overall => Mark { dim: *dim, from: (widest + 1.5, 0.0), to: (widest + 1.5, overall) },
            Dim::Shank => {
                let y = d.cut_length + (overall - d.cut_length) * 0.5;
                Mark { dim: *dim, from: (-sr, y), to: (sr, y) }
            }
            Dim::Corner => {
                let c = d.corner.min(r);
                // From the centre of the corner to the arc, at 45 degrees.
                let a = std::f64::consts::FRAC_PI_4;
                Mark { dim: *dim, from: (r - c, c), to: (r - c + c * a.cos(), c - c * a.sin()) }
            }
            Dim::Angle => {
                // The two sides of the angle are drawn from the apex of the cone.
                let apex_y = match kind {
                    ToolKind::Engraver => -(d.tip / 2.0) / half_angle.tan(),
                    ToolKind::TaperedBall => {
                        let t = (d.tip / 2.0).max(0.01);
                        t - t / half_angle.sin()
                    }
                    _ => 0.0,
                };
                Mark { dim: *dim, from: (0.0, apex_y), to: (0.0, apex_y + r.max(1.0)) }
            }
            Dim::Tip => {
                let t = d.tip / 2.0;
                Mark { dim: *dim, from: (-t, if kind == ToolKind::TaperedBall { t } else { 0.0 }), to: (t, if kind == ToolKind::TaperedBall { t } else { 0.0 }) }
            }
        };
        out.push(m);
    }
    out
}

fn arrow_line(painter: &egui::Painter, a: Pos2, b: Pos2, color: Color32) {
    painter.line_segment([a, b], Stroke::new(1.2, color));
    let dir = (b - a).normalized();
    if dir.length() > 0.0 {
        let n = Vec2::new(-dir.y, dir.x) * 3.0;
        for (p, s) in [(a, 1.0), (b, -1.0)] {
            let back = p + dir * (6.0 * s);
            painter.line_segment([p, back + n], Stroke::new(1.2, color));
            painter.line_segment([p, back - n], Stroke::new(1.2, color));
        }
    }
}

/// Draw the bit into `rect`, with the letters of its dimensions.
pub fn draw_tool(painter: &egui::Painter, rect: Rect, kind: ToolKind, real: &Dims) {
    // The shank is shown short, with a break in it, so the cutting end is large enough to see.
    let shank_shown = (real.diameter * 1.5).max(8.0);
    let broken = real.overall - real.cut_length > shank_shown * 1.3;
    let mut shown = real.clone();
    if broken {
        shown.overall = real.cut_length + shank_shown;
    }
    let d = &shown;
    let ink = theme::text();
    let body = if theme::text().r() > 128 { Color32::from_rgb(0x5a, 0x66, 0x7a) } else { Color32::from_rgb(0xb8, 0xc2, 0xd0) };
    let mark_color = Color32::from_rgb(0xd0, 0x50, 0x20);
    let right = profile(kind, d);
    let overall = d.overall.max(d.cut_length).max(1.0);
    let widest = (d.diameter / 2.0).max(d.shank / 2.0).max(0.5);
    // The picture needs room at the sides for the marks (L on the left, O on the right).
    let side = widest + 4.0;
    let s = ((rect.height() as f64 - 30.0) / (overall + 2.0)).min((rect.width() as f64 * 0.5 - 8.0) / (side + 1.0)).max(1.0) as f32;
    let cx = rect.center().x;
    let bottom = rect.bottom() - 16.0;
    let to_screen = |p: P| Pos2::new(cx + p.0 as f32 * s, bottom - p.1 as f32 * s);
    let mut outline: Vec<Pos2> = right.iter().map(|p| to_screen(*p)).collect();
    outline.extend(right.iter().rev().map(|p| to_screen((-p.0, p.1))));
    painter.add(egui::Shape::convex_polygon(outline.clone(), body.gamma_multiply(0.55), Stroke::new(1.5, ink)));
    // The cutting part is a little darker than the shank.
    let cut_top = d.cut_length;
    painter.line_segment([to_screen((-widest.max(d.shank / 2.0) - 0.3, cut_top)), to_screen((widest.max(d.shank / 2.0) + 0.3, cut_top))], Stroke::new(0.8, ink.gamma_multiply(0.35)));
    if broken {
        // The sign of a break on the shank.
        let y = d.cut_length + shank_shown * 0.82;
        let half = d.shank as f32 / 2.0 * s + 3.0;
        for off in [-3.0f32, 3.0] {
            let c = to_screen((0.0, y));
            painter.line_segment([Pos2::new(c.x - half, c.y + off + 2.0), Pos2::new(c.x + half, c.y + off - 2.0)], Stroke::new(1.4, theme::workspace()));
        }
    }
    for m in marks(kind, d) {
        let (a, b) = (to_screen(m.from), to_screen(m.to));
        match m.dim {
            Dim::Angle => {
                // Two lines from the apex along the flanks and an arc between them.
                let half = (d.angle.clamp(1.0, 179.0) / 2.0).to_radians() as f32;
                // From the apex of the cone: the flanks go a little past the cone, the arc sits inside it.
                let cone = (d.diameter / 2.0 / (half as f64).tan()) as f32;
                let len = (cone * 1.25 * s).max(30.0);
                let up = Vec2::new(0.0, -1.0);
                let rot = |a: f32| Vec2::new(up.x * a.cos() - up.y * a.sin(), up.x * a.sin() + up.y * a.cos());
                let apex = a;
                painter.line_segment([apex, apex + rot(-half) * len], Stroke::new(1.0, mark_color));
                painter.line_segment([apex, apex + rot(half) * len], Stroke::new(1.0, mark_color));
                let rad = (cone * 0.55 * s).clamp(14.0, len * 0.9);
                let pts: Vec<Pos2> = (0..=16).map(|i| apex + rot(-half + 2.0 * half * i as f32 / 16.0) * rad).collect();
                painter.add(egui::Shape::line(pts, Stroke::new(1.2, mark_color)));
                painter.text(apex + rot(0.0) * (rad + 9.0), Align2::CENTER_CENTER, m.dim.letter(), FontId::proportional(13.0), mark_color);
            }
            Dim::Corner => {
                painter.line_segment([a, b], Stroke::new(1.5, mark_color));
                painter.circle_filled(a, 2.0, mark_color);
                painter.text(b + Vec2::new(10.0, 2.0), Align2::CENTER_CENTER, m.dim.letter(), FontId::proportional(13.0), mark_color);
            }
            Dim::Tip => {
                // Below the tip, with short lines up to its two sides.
                let y = a.y + 9.0;
                painter.line_segment([Pos2::new(a.x, a.y), Pos2::new(a.x, y + 3.0)], Stroke::new(0.8, mark_color));
                painter.line_segment([Pos2::new(b.x, b.y), Pos2::new(b.x, y + 3.0)], Stroke::new(0.8, mark_color));
                arrow_line(painter, Pos2::new(a.x, y), Pos2::new(b.x, y), mark_color);
                painter.text(Pos2::new((a.x + b.x) / 2.0 + 14.0, y + 5.0), Align2::LEFT_CENTER, m.dim.letter(), FontId::proportional(13.0), mark_color);
            }
            _ => {
                arrow_line(painter, a, b, mark_color);
                let horizontal = (a.y - b.y).abs() < 0.5;
                let mid = a.lerp(b, 0.5);
                let at = if horizontal { mid + Vec2::new(0.0, -9.0) } else if a.x < cx { mid + Vec2::new(-10.0, 0.0) } else { mid + Vec2::new(10.0, 0.0) };
                painter.text(at, Align2::CENTER_CENTER, m.dim.letter(), FontId::proportional(13.0), mark_color);
                // Lines out from the bit to the vertical marks, so it is clear where they start and stop.
                if !horizontal {
                    for p in [a, b] {
                        painter.line_segment([Pos2::new(cx + (p.x - cx).signum() * (widest as f32 * s), p.y), p], Stroke::new(0.6, mark_color.gamma_multiply(0.6)));
                    }
                }
            }
        }
    }
}

// ───────────────────────── the window ─────────────────────────

/// What the window of the library is doing.
#[derive(Default)]
pub struct ToolsWindow {
    pub open: bool,
    /// The id of the bit that is shown.
    pub selected: Option<String>,
    /// The bit as it is being edited.
    pub draft: Option<Tool>,
    pub message: String,
    /// Set when the library changed, so that it is kept.
    pub changed: bool,
}

impl ToolsWindow {
    pub fn select(&mut self, lib: &Library, id: &str) {
        self.selected = Some(id.to_string());
        self.draft = lib.get(id).cloned();
        self.message.clear();
    }
}

fn dim_row(ui: &mut egui::Ui, d: &mut Dims, dim: Dim, enabled: bool) -> bool {
    let (lo, hi) = ToolKind::range(dim);
    let mut v = d.get(dim);
    let mut changed = false;
    ui.label(egui::RichText::new(dim.letter()).strong().color(Color32::from_rgb(0xd0, 0x50, 0x20)));
    ui.label(tr(dim.label()));
    ui.add_enabled_ui(enabled, |ui| {
        if ui.add(egui::DragValue::new(&mut v).range(lo..=hi).speed(0.05).max_decimals(3).suffix(format!(" {}", dim.unit()))).changed() {
            d.set(dim, v);
            changed = true;
        }
    });
    ui.end_row();
    changed
}

/// The window of the bit library. `lib` is changed in place; the caller keeps it when `win.changed` is set.
pub fn tools_window(ctx: &egui::Context, win: &mut ToolsWindow, lib: &mut Library) {
    if !win.open {
        return;
    }
    let mut open = true;
    popup(ctx, "tool_library", &tr("Bit library"), [860.0, 600.0], &mut open, |ui| {
        if win.selected.is_none() {
            if let Some(t) = lib.builtin.first() {
                let id = t.id.clone();
                win.select(lib, &id);
            }
        }
        // Everything in a scroll area of the height that is left: a window whose content fills it exactly (and then has the spacing
        // of the layout after it) grows by that on every frame, up to the height of the screen.
        let total = base_ui::window::fill_size(ui).y;
        egui::ScrollArea::vertical().id_salt("tool_window").max_height(total).auto_shrink([false, true]).show(ui, |ui| {
        ui.horizontal_top(|ui| {
            // The list.
            ui.vertical(|ui| {
                ui.set_width(250.0);
                // The list and the right part fill the window; the buttons under the list stay at its foot.
                let list_h = base_ui::window::height_above(ui, "tool_left", total);
                egui::ScrollArea::vertical().id_salt("tool_list").max_height(list_h).min_scrolled_height(list_h).auto_shrink([false, false]).show(ui, |ui| {
                    let mut pick = None;
                    for (title, user) in [(tr("Ready-made bits"), false), (tr("My bits"), true)] {
                        ui.label(egui::RichText::new(title).strong().color(theme::text_dim()));
                        let list: Vec<&Tool> = if user { lib.user.iter().collect() } else { lib.builtin.iter().collect() };
                        if list.is_empty() {
                            ui.label(egui::RichText::new(tr("Nothing here yet")).color(theme::text_dim()));
                        }
                        for t in list {
                            let on = win.selected.as_deref() == Some(t.id.as_str());
                            if ui.add_sized([ui.available_width(), 22.0], egui::Button::selectable(on, &t.name)).clicked() {
                                pick = Some(t.id.clone());
                            }
                        }
                        ui.add_space(6.0);
                    }
                    if let Some(id) = pick {
                        win.select(lib, &id);
                    }
                });
                base_ui::window::bottom(ui, "tool_left", |ui| {
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        if ui.button(tr("New bit")).clicked() {
                            let t = Tool::new("", &lib.free_name(&tr("New bit")), ToolKind::FlatEnd, Dims::default());
                            if let Ok(id) = lib.add(t) {
                                win.changed = true;
                                win.select(lib, &id);
                            }
                        }
                        let can_copy = win.selected.is_some();
                        if ui.add_enabled(can_copy, egui::Button::new(tr("Copy"))).on_hover_text(tr("A copy among your own bits")).clicked() {
                            if let Some(t) = win.selected.as_deref().and_then(|id| lib.get(id)).cloned() {
                                let mut c = t;
                                c.name = lib.free_name(&c.name);
                                if let Ok(id) = lib.add(c) {
                                    win.changed = true;
                                    win.select(lib, &id);
                                }
                            }
                        }
                        let mine = win.selected.as_deref().is_some_and(|id| lib.user.iter().any(|t| t.id == id));
                        if delete_button_enabled(ui, mine, &tr("Delete")).clicked() {
                            if let Some(id) = win.selected.take() {
                                lib.remove(&id);
                                win.draft = None;
                                win.changed = true;
                            }
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        if import_button(ui, &tr("Import…")).on_hover_text(tr("Read bits from a file")).clicked() {
                            if let Some(p) = rfd::FileDialog::new().add_filter("MillCreator bits", &["json"]).pick_file() {
                                win.message = match std::fs::metadata(&p) {
                                    Ok(m) if m.len() > crate::tools::MAX_FILE => tr("The file is too large").to_string(),
                                    _ => match std::fs::read_to_string(&p).map_err(|e| e.to_string()).and_then(|t| lib.import(&t)) {
                                        Ok((n, problems)) => {
                                            win.changed |= n > 0;
                                            let mut m = format!("{} {}", n, tr("bits added"));
                                            if !problems.is_empty() {
                                                m.push_str(&format!(". {}: {}", tr("Left out"), problems.join("; ")));
                                            }
                                            m
                                        }
                                        Err(e) => format!("{}: {e}", tr("Cannot use the file")),
                                    },
                                };
                            }
                        }
                        let any = !lib.user.is_empty();
                        if save_button_enabled(ui, any, &tr("Export…")).on_hover_text(tr("Save your bits to a file")).clicked() {
                            if let Some(p) = rfd::FileDialog::new().add_filter("MillCreator bits", &["json"]).set_file_name("bits.json").save_file() {
                                win.message = match std::fs::write(&p, lib.export(None)) {
                                    Ok(()) => format!("{} {}", tr("Saved"), p.display()),
                                    Err(e) => format!("{}: {e}", tr("Could not save")),
                                };
                            }
                        }
                        if ui.add_enabled(win.selected.is_some(), egui::Button::new(tr("Export this bit…"))).clicked() {
                            if let (Some(id), Some(p)) = (win.selected.clone(), rfd::FileDialog::new().add_filter("MillCreator bits", &["json"]).set_file_name("bit.json").save_file()) {
                                win.message = match std::fs::write(&p, lib.export(Some(&[id]))) {
                                    Ok(()) => format!("{} {}", tr("Saved"), p.display()),
                                    Err(e) => format!("{}: {e}", tr("Could not save")),
                                };
                            }
                        }
                    });
                });
            });
            ui.separator();
            // The bit that is shown.
            ui.vertical(|ui| {
                let Some(draft) = win.draft.as_mut() else {
                    ui.label(egui::RichText::new(tr("Choose a bit")).color(theme::text_dim()));
                    return;
                };
                let editable = !draft.builtin;
                let mut changed = false;
                let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width().min(520.0), 230.0), egui::Sense::hover());
                let painter = ui.painter_at(rect);
                painter.rect_filled(rect, 4.0, theme::workspace());
                draw_tool(&painter, rect, draft.kind, &draft.dims);
                ui.add_space(4.0);
                if !editable {
                    ui.label(egui::RichText::new(tr("A ready-made bit cannot be changed. Make a copy to change it.")).color(theme::text_dim()));
                }
                let edit_h = (total - ui.min_rect().height() - ui.spacing().item_spacing.y).max(120.0);
                egui::ScrollArea::vertical().id_salt("tool_edit").max_height(edit_h).min_scrolled_height(edit_h).auto_shrink([false, false]).show(ui, |ui| {
                    egui::Grid::new("tool_grid").num_columns(3).spacing([10.0, 5.0]).show(ui, |ui| {
                        ui.label("");
                        ui.label(tr("Name"));
                        ui.add_enabled_ui(editable, |ui| {
                            changed |= ui.text_edit_singleline(&mut draft.name).changed();
                        });
                        ui.end_row();
                        ui.label("");
                        ui.label(tr("Type"));
                        ui.add_enabled_ui(editable, |ui| {
                            egui::ComboBox::from_id_salt("tool_kind").selected_text(tr(draft.kind.label())).width(220.0).show_ui(ui, |ui| {
                                for k in ToolKind::ALL {
                                    changed |= ui.selectable_value(&mut draft.kind, k, tr(k.label())).changed();
                                }
                            });
                        });
                        ui.end_row();
                        for dim in draft.kind.dims() {
                            changed |= dim_row(ui, &mut draft.dims, *dim, editable);
                        }
                    });
                    ui.add_space(6.0);
                    ui.label(egui::RichText::new(tr("Starting speeds for a layer")).strong());
                    egui::Grid::new("tool_cut").num_columns(2).spacing([10.0, 5.0]).show(ui, |ui| {
                        let c: &mut Cutting = &mut draft.cutting;
                        let rows: [(&str, &mut f64, &str, f64, f64); 6] = [("Feed", &mut c.feed, " mm/min", 1.0, 100000.0), ("Plunge feed", &mut c.plunge, " mm/min", 1.0, 100000.0), ("Ramp feed", &mut c.ramp, " mm/min", 0.0, 100000.0), ("Spindle speed", &mut c.rpm, " rpm", 1.0, 1000000.0), ("Pass depth (of the diameter)", &mut c.step_down_pct, " %", 1.0, 400.0), ("Step over (of the diameter)", &mut c.step_over_pct, " %", 1.0, 100.0)];
                        for (label, v, suffix, lo, hi) in rows {
                            ui.label(tr(label));
                            ui.add_enabled_ui(editable, |ui| {
                                changed |= ui.add(egui::DragValue::new(v).range(lo..=hi).suffix(suffix)).changed();
                            });
                            ui.end_row();
                        }
                        ui.label(tr("Flutes"));
                        ui.add_enabled_ui(editable, |ui| {
                            changed |= ui.add(egui::DragValue::new(&mut c.flutes).range(1..=12)).changed();
                        });
                        ui.end_row();
                    });
                    ui.add_space(6.0);
                    ui.label(tr("Notes"));
                    ui.add_enabled_ui(editable, |ui| {
                        changed |= ui.add(egui::TextEdit::multiline(&mut draft.notes).desired_rows(3).desired_width(f32::INFINITY)).changed();
                    });
                });
                let _ = changed;
            });
        });
        // Save the edited bit when it is valid; say what is wrong when it is not.
        if let Some(d) = win.draft.clone() {
            if !d.builtin {
                match d.check() {
                    Ok(()) => {
                        if lib.user.iter().find(|t| t.id == d.id).is_some_and(|t| *t != d) {
                            if lib.update(d).is_ok() {
                                win.changed = true;
                            }
                        }
                    }
                    Err(e) => win.message = e,
                }
            }
        }
        if !win.message.is_empty() {
            ui.label(egui::RichText::new(&win.message).color(Color32::from_rgb(0xd0, 0x50, 0x20)));
        }
        });
    });
    win.open = open;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::builtin_tools;

    #[test]
    fn the_outline_of_every_bit_starts_at_the_tip_and_ends_at_the_top_and_stays_inside_the_widest_part() {
        for t in builtin_tools() {
            let p = profile(t.kind, &t.dims);
            let first = p.first().unwrap();
            assert!(first.1 >= -1e-9 && first.1 <= t.dims.diameter, "{}: the tip {first:?}", t.name);
            assert!((p.last().unwrap().1 - t.dims.overall.max(t.dims.cut_length)).abs() < 1e-9, "{}", t.name);
            let widest = t.dims.diameter.max(t.dims.shank) / 2.0 + 1e-9;
            assert!(p.iter().all(|q| q.0 >= -1e-9 && q.0 <= widest && q.1 >= -1e-9), "{}: {p:?}", t.name);
            // The outline goes up: never down.
            assert!(p.windows(2).all(|w| w[1].1 >= w[0].1 - t.dims.diameter), "{}", t.name);
        }
    }

    #[test]
    fn every_dimension_of_a_kind_has_a_mark_with_its_letter_and_the_marks_are_where_they_are_measured() {
        for t in builtin_tools() {
            let ms = marks(t.kind, &t.dims);
            assert_eq!(ms.len(), t.kind.dims().len(), "{}", t.name);
            for m in &ms {
                let len = ((m.to.0 - m.from.0).powi(2) + (m.to.1 - m.from.1).powi(2)).sqrt();
                match m.dim {
                    Dim::Diameter => assert!((len - t.dims.diameter).abs() < 1e-9, "{}", t.name),
                    Dim::CutLength => assert!((len - t.dims.cut_length).abs() < 1e-9),
                    Dim::Overall => assert!((len - t.dims.overall).abs() < 1e-9),
                    Dim::Shank => assert!((len - t.dims.shank).abs() < 1e-9),
                    Dim::Tip => assert!((len - t.dims.tip).abs() < 1e-9),
                    _ => assert!(len > 0.0),
                }
            }
        }
        let letters: std::collections::HashSet<_> = [Dim::Diameter, Dim::CutLength, Dim::Overall, Dim::Shank, Dim::Corner, Dim::Angle, Dim::Tip].iter().map(|d| d.letter()).collect();
        assert_eq!(letters.len(), 7);
    }

    #[test]
    fn the_window_draws_selects_and_a_bit_of_the_user_is_saved_when_it_is_changed() {
        let ctx = egui::Context::default();
        let mut lib = Library::new();
        let mut win = ToolsWindow { open: true, ..ToolsWindow::default() };
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| tools_window(ui.ctx(), &mut win, &mut lib));
        out.textures_delta.clear();
        assert!(win.draft.as_ref().is_some_and(|d| d.builtin), "a ready-made bit is shown first");
        let id = lib.add(Tool::new("x", "Mine", ToolKind::FlatEnd, Dims::default())).unwrap();
        win.select(&lib, &id);
        win.draft.as_mut().unwrap().dims.diameter = 5.0;
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| tools_window(ui.ctx(), &mut win, &mut lib));
        out.textures_delta.clear();
        assert_eq!(lib.get(&id).unwrap().dims.diameter, 5.0);
        assert!(win.changed);
        // A wrong number is not saved and is explained.
        win.draft.as_mut().unwrap().dims.cut_length = 500.0;
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| tools_window(ui.ctx(), &mut win, &mut lib));
        out.textures_delta.clear();
        assert_eq!(lib.get(&id).unwrap().dims.cut_length, Dims::default().cut_length);
        assert!(!win.message.is_empty());
    }
}
