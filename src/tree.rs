use crate::fx::draw_diamond;
use crate::icons::{draw_icon, IconKind};
use crate::story::{ending_tone_label, BookData, Language, Protagonist, SaveState};
use crate::theme::{lerp_color, with_alpha, ThemePalette};
use crate::widgets::{font_serif_bold, font_serif_italic, font_ui};
use eframe::egui::{self, Color32, Pos2, Rect, ScrollArea, Sense, Shape, Stroke, Vec2};
use std::collections::{HashMap, HashSet, VecDeque};

struct LayoutNode {
    id: String,
    depth: usize,
    y_slot: f32,
    is_ending: bool,
}

struct TreeLayout {
    nodes: HashMap<String, LayoutNode>,
    edges: Vec<(String, String, String, bool)>, // (from, to, trait_name, is_primary)
    max_depth: usize,
    total_rows: f32,
}

/// Builds a tree layout containing ONLY nodes that have been visited/unlocked by the player.
fn build_discovered_layout(
    book: &BookData,
    save: &SaveState,
    protagonist: Protagonist,
) -> TreeLayout {
    let start_id = protagonist.start_id().to_string();
    let mut depth_map: HashMap<String, usize> = HashMap::new();
    let mut primary_children: HashMap<String, Vec<String>> = HashMap::new();
    let mut all_edges: Vec<(String, String, String, bool)> = Vec::new();
    let mut queue = VecDeque::new();

    let is_unlocked = |id: &str| -> bool {
        save.visited_nodes.contains(id) || save.discovered_endings.contains(id)
    };

    if book.nodes.contains_key(&start_id) && is_unlocked(&start_id) {
        depth_map.insert(start_id.clone(), 0);
        queue.push_back(start_id.clone());
    }

    while let Some(u) = queue.pop_front() {
        let d = depth_map[&u];
        if let Some(node) = book.nodes.get(&u) {
            for choice in &node.choices {
                let v = choice.target.clone();
                if !book.nodes.contains_key(&v) || !is_unlocked(&v) {
                    continue;
                }
                if !depth_map.contains_key(&v) {
                    depth_map.insert(v.clone(), d + 1);
                    primary_children.entry(u.clone()).or_default().push(v.clone());
                    all_edges.push((u.clone(), v.clone(), choice.trait_name.clone(), true));
                    queue.push_back(v);
                } else {
                    all_edges.push((u.clone(), v.clone(), choice.trait_name.clone(), false));
                }
            }
        }
    }

    let mut y_slots: HashMap<String, f32> = HashMap::new();
    let mut next_leaf_row = 0.0_f32;
    let mut visited_dfs: HashSet<String> = HashSet::new();

    fn assign_y(
        u: &str,
        primary_children: &HashMap<String, Vec<String>>,
        y_slots: &mut HashMap<String, f32>,
        next_leaf_row: &mut f32,
        visited: &mut HashSet<String>,
    ) -> f32 {
        if let Some(&y) = y_slots.get(u) {
            return y;
        }
        if !visited.insert(u.to_string()) {
            return *next_leaf_row;
        }
        let children = primary_children.get(u).cloned().unwrap_or_default();
        if children.is_empty() {
            let y = *next_leaf_row;
            *next_leaf_row += 1.0;
            y_slots.insert(u.to_string(), y);
            y
        } else {
            let mut min_y = f32::MAX;
            let mut max_y = f32::MIN;
            for ch in &children {
                let cy = assign_y(ch, primary_children, y_slots, next_leaf_row, visited);
                min_y = min_y.min(cy);
                max_y = max_y.max(cy);
            }
            let mid = (min_y + max_y) * 0.5;
            y_slots.insert(u.to_string(), mid);
            mid
        }
    }

    if depth_map.contains_key(&start_id) {
        assign_y(
            &start_id,
            &primary_children,
            &mut y_slots,
            &mut next_leaf_row,
            &mut visited_dfs,
        );
    }

    let max_depth = depth_map.values().copied().max().unwrap_or(0);
    let mut nodes = HashMap::new();
    for (id, depth) in depth_map {
        let y_slot = y_slots.get(&id).copied().unwrap_or(0.0);
        let is_ending = book.nodes.get(&id).is_some_and(|n| n.is_ending);
        nodes.insert(
            id.clone(),
            LayoutNode {
                id,
                depth,
                y_slot,
                is_ending,
            },
        );
    }

    TreeLayout {
        nodes,
        edges: all_edges,
        max_depth,
        total_rows: next_leaf_row.max(1.0),
    }
}

fn draw_cubic_bezier(
    painter: &egui::Painter,
    p0: Pos2,
    p1: Pos2,
    p2: Pos2,
    p3: Pos2,
    stroke: Stroke,
) {
    let steps = 20;
    let mut pts = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let u = 1.0 - t;
        let x = u * u * u * p0.x
            + 3.0 * u * u * t * p1.x
            + 3.0 * u * t * t * p2.x
            + t * t * t * p3.x;
        let y = u * u * u * p0.y
            + 3.0 * u * u * t * p1.y
            + 3.0 * u * t * t * p2.y
            + t * t * t * p3.y;
        pts.push(Pos2::new(x, y));
    }
    painter.add(Shape::line(pts, stroke));
}

fn truncate_chars(s: &str, max_chars: usize) -> String {
    let count = s.chars().count();
    if count <= max_chars {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max_chars.saturating_sub(1)).collect();
        out.push_str("...");
        out
    }
}

pub fn render_endings_tree(
    ui: &mut egui::Ui,
    book: &BookData,
    save: &SaveState,
    protagonist: Protagonist,
    lang: Language,
    palette: &ThemePalette,
    time: f32,
) {
    let layout = build_discovered_layout(book, save, protagonist);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 14.0;
        legend_item(
            ui,
            palette.gold,
            lang.t("Cena percorrida", "Visited scene"),
            palette,
        );
        legend_item(
            ui,
            palette.emerald,
            lang.t("Final concluído", "Completed ending"),
            palette,
        );
    });
    ui.add_space(10.0);

    egui::Frame::NONE
        .fill(palette.bg_paper)
        .corner_radius(10.0)
        .stroke(Stroke::new(1.0_f32, palette.border))
        .inner_margin(egui::Margin::same(6))
        .show(ui, |ui| {
            let avail_h = (ui.available_height() - 14.0).max(380.0);

            if layout.nodes.is_empty() {
                let (rect, _) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), avail_h),
                    Sense::hover(),
                );
                let painter = ui.painter_at(rect);
                draw_icon(
                    &painter,
                    Pos2::new(rect.center().x, rect.center().y - 30.0),
                    28.0,
                    IconKind::Compass,
                    with_alpha(palette.gold, 0.7),
                );
                let empty_title = match lang {
                    Language::PtBr => {
                        format!("Nenhum caminho percorrido ainda em {}", protagonist.name())
                    }
                    Language::En => format!("No paths explored yet for {}", protagonist.name()),
                };
                painter.text(
                    Pos2::new(rect.center().x, rect.center().y + 6.0),
                    egui::Align2::CENTER_CENTER,
                    empty_title,
                    font_serif_bold(17.0),
                    palette.ink,
                );
                painter.text(
                    Pos2::new(rect.center().x, rect.center().y + 32.0),
                    egui::Align2::CENTER_CENTER,
                    lang.t(
                        "A árvore de escolhas e desfechos cresce automaticamente conforme você avança na história.",
                        "The tree of choices and endings grows automatically as you progress through the story.",
                    ),
                    font_serif_italic(14.0),
                    palette.ink_muted,
                );
                return;
            }

            let col_w = 240.0_f32;
            let row_h = 78.0_f32;
            let node_w = 186.0_f32;
            let node_h = 54.0_f32;
            let pad_x = 44.0_f32;
            let pad_y = 44.0_f32;

            let canvas_w = (layout.max_depth as f32 + 1.0) * col_w + pad_x * 2.0;
            let canvas_h = layout.total_rows * row_h + pad_y * 2.0;

            ScrollArea::both()
                .max_height(avail_h)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let (canvas_rect, _resp) = ui.allocate_exact_size(
                        Vec2::new(canvas_w.max(ui.available_width()), canvas_h.max(avail_h - 12.0)),
                        Sense::hover(),
                    );
                    let painter = ui.painter_at(canvas_rect);

                    // Subtle Art Deco dot grid in background
                    let grid_col = with_alpha(palette.gold, 0.08);
                    let mut gx = canvas_rect.left() + 24.0;
                    while gx < canvas_rect.right() {
                        let mut gy = canvas_rect.top() + 24.0;
                        while gy < canvas_rect.bottom() {
                            painter.circle_filled(Pos2::new(gx, gy), 1.0, grid_col);
                            gy += 32.0;
                        }
                        gx += 32.0;
                    }

                    // Center vertically if tree is shorter than viewport
                    let tree_actual_h = layout.total_rows * row_h;
                    let extra_y = ((canvas_rect.height() - tree_actual_h - pad_y * 2.0) * 0.5).max(0.0);

                    let node_rect = |ln: &LayoutNode| -> Rect {
                        let cx = canvas_rect.left() + pad_x + (ln.depth as f32) * col_w + node_w * 0.5;
                        let cy = canvas_rect.top() + pad_y + extra_y + ln.y_slot * row_h + node_h * 0.5;
                        Rect::from_center_size(Pos2::new(cx, cy), Vec2::new(node_w, node_h))
                    };

                    // 1. Draw all connecting Bézier paths between unlocked nodes
                    for (from_id, to_id, trait_name, is_primary) in &layout.edges {
                        let (Some(u), Some(v)) = (layout.nodes.get(from_id), layout.nodes.get(to_id))
                        else {
                            continue;
                        };
                        let r_u = node_rect(u);
                        let r_v = node_rect(v);

                        let p0 = Pos2::new(r_u.right(), r_u.center().y);
                        let p3 = Pos2::new(r_v.left(), r_v.center().y);
                        let dx = (p3.x - p0.x).abs().max(48.0) * 0.52;
                        let p1 = Pos2::new(p0.x + dx, p0.y);
                        let p2 = Pos2::new(p3.x - dx, p3.y);

                        let base = if trait_name.is_empty() {
                            palette.gold
                        } else {
                            lerp_color(palette.gold, palette.trait_color(trait_name), 0.45)
                        };
                        let col = with_alpha(base, if *is_primary { 0.92 } else { 0.58 });
                        let width = if *is_primary { 2.3_f32 } else { 1.5_f32 };

                        draw_cubic_bezier(&painter, p0, p1, p2, p3, Stroke::new(width, col));
                        painter.circle_filled(p3, 3.0, col);
                    }

                    // 2. Draw only unlocked nodes
                    let pointer_pos = ui.input(|i| i.pointer.hover_pos());
                    let mut hovered_tooltip: Option<(String, String, Option<String>)> = None;

                    for ln in layout.nodes.values() {
                        let rect = node_rect(ln);
                        let Some(story_node) = book.nodes.get(&ln.id) else {
                            continue;
                        };

                        let is_hovered =
                            pointer_pos.is_some_and(|p| canvas_rect.contains(p) && rect.contains(p));
                        let is_current = save.current_node_id.as_deref() == Some(ln.id.as_str());

                        let tone = story_node.ending_tone.as_deref().unwrap_or("sereno");
                        let tone_col = palette.ending_tone_color(tone);

                        if is_current {
                            let pulse = 0.5 + 0.5 * (time * 3.2).sin();
                            painter.rect_filled(
                                rect.expand(4.0 + 3.0 * pulse),
                                9.0,
                                with_alpha(palette.gold, 0.16 + 0.12 * pulse),
                            );
                        } else if is_hovered {
                            let glow = if ln.is_ending { tone_col } else { palette.gold };
                            painter.rect_filled(rect.expand(3.0), 9.0, with_alpha(glow, 0.16));
                        }

                        if ln.is_ending {
                            painter.rect_filled(rect, 7.0, palette.bg_card_hover);
                            painter.rect_stroke(
                                rect,
                                7.0,
                                Stroke::new(if is_hovered { 2.2_f32 } else { 1.6_f32 }, tone_col),
                                egui::StrokeKind::Inside,
                            );
                            let bar = Rect::from_min_max(
                                Pos2::new(rect.left(), rect.top() + 5.0),
                                Pos2::new(rect.left() + 4.5, rect.bottom() - 5.0),
                            );
                            painter.rect_filled(bar, 2.0, tone_col);

                            draw_icon(
                                &painter,
                                Pos2::new(rect.left() + 18.0, rect.center().y),
                                13.0,
                                IconKind::Starburst,
                                tone_col,
                            );

                            let end_title = story_node
                                .ending_title
                                .as_deref()
                                .unwrap_or(&story_node.title);
                            painter.text(
                                Pos2::new(rect.left() + 30.0, rect.top() + 10.0),
                                egui::Align2::LEFT_TOP,
                                ending_tone_label(tone, lang),
                                font_ui(9.0),
                                tone_col,
                            );
                            painter.text(
                                Pos2::new(rect.left() + 30.0, rect.top() + 25.0),
                                egui::Align2::LEFT_TOP,
                                truncate_chars(end_title, 21),
                                font_serif_bold(12.0),
                                palette.ink,
                            );

                            if is_hovered {
                                hovered_tooltip = Some((
                                    format!("{} - {}", story_node.id, end_title),
                                    format!(
                                        "{} {}",
                                        lang.t("Desfecho", "Ending"),
                                        ending_tone_label(tone, lang)
                                    ),
                                    story_node
                                        .paragraphs
                                        .first()
                                        .map(|p| truncate_chars(p, 160)),
                                ));
                            }
                        } else {
                            painter.rect_filled(rect, 7.0, palette.bg_card);
                            painter.rect_stroke(
                                rect,
                                7.0,
                                Stroke::new(
                                    if is_current || is_hovered { 1.8_f32 } else { 1.2_f32 },
                                    palette.gold,
                                ),
                                egui::StrokeKind::Inside,
                            );
                            draw_diamond(
                                &painter,
                                Pos2::new(rect.left() + 16.0, rect.center().y),
                                5.0,
                                palette.gold,
                            );
                            painter.text(
                                Pos2::new(rect.left() + 28.0, rect.top() + 10.0),
                                egui::Align2::LEFT_TOP,
                                &story_node.id,
                                font_ui(9.0),
                                palette.gold,
                            );
                            painter.text(
                                Pos2::new(rect.left() + 28.0, rect.top() + 25.0),
                                egui::Align2::LEFT_TOP,
                                truncate_chars(&story_node.title, 21),
                                font_serif_bold(12.0),
                                palette.ink,
                            );

                            if is_hovered {
                                hovered_tooltip = Some((
                                    format!("{} - {}", story_node.id, story_node.title),
                                    if is_current {
                                        lang.t("Você está aqui agora.", "You are here now.")
                                            .to_string()
                                    } else {
                                        match lang {
                                            Language::PtBr => format!(
                                                "{} escolhas a partir desta cena",
                                                story_node.choices.len()
                                            ),
                                            Language::En => format!(
                                                "{} choices from this scene",
                                                story_node.choices.len()
                                            ),
                                        }
                                    },
                                    story_node.prompt.clone(),
                                ));
                            }
                        }
                    }

                    if let Some((title, subtitle, extra)) = hovered_tooltip {
                        egui::show_tooltip_at_pointer(
                            ui.ctx(),
                            ui.layer_id(),
                            egui::Id::new("tree_node_tooltip"),
                            |ui| {
                                ui.set_max_width(280.0);
                                ui.label(
                                    egui::RichText::new(title)
                                        .font(font_serif_bold(13.5))
                                        .color(palette.ink),
                                );
                                ui.label(
                                    egui::RichText::new(subtitle)
                                        .font(font_ui(11.0))
                                        .color(palette.gold),
                                );
                                if let Some(ex) = extra {
                                    ui.add_space(4.0);
                                    ui.label(
                                        egui::RichText::new(ex)
                                            .font(font_serif_italic(12.0))
                                            .color(palette.ink_soft),
                                    );
                                }
                            },
                        );
                    }
                });
        });
}

fn legend_item(ui: &mut egui::Ui, color: Color32, text: &str, palette: &ThemePalette) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let (r, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
        draw_diamond(ui.painter(), r.center(), 4.5, color);
        ui.label(
            egui::RichText::new(text)
                .font(font_ui(11.5))
                .color(palette.ink_soft),
        );
    });
}
