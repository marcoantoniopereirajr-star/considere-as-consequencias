use crate::art::{ArtGallery, ArtKind};
use crate::fx::{draw_art_deco_divider, draw_art_deco_frame, ease_out_cubic, FxEngine};
use crate::icons::{icon_for_trait, paint_icon_inline, IconKind};
use crate::story::{
    ending_tone_label, trait_label, BookData, HistoryEntry, Language, Protagonist, SaveState,
    ALL_TRAITS,
};
use crate::theme::{with_alpha, ThemeKind, ThemePalette};
use crate::tree::render_endings_tree;
use crate::widgets::{
    animated_choice_card, animated_deco_button, font_serif, font_serif_bold, font_serif_italic,
    font_ui, protagonist_card, ButtonStyle,
};
use eframe::egui::{self, Key, Pos2, Rect, RichText, ScrollArea, Stroke, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    Playing,
    Endings,
}

struct PendingTransition {
    target_id: String,
    chosen_label: String,
    chosen_trait: String,
    choice_index: usize,
    timer: f32,
}

pub struct ConsequencesApp {
    book_pt: BookData,
    book_en: BookData,
    save: SaveState,
    screen: Screen,

    // Theme smooth transition
    theme_kind: ThemeKind,
    current_palette: ThemePalette,
    target_palette: ThemePalette,

    // Visual effects & scene animation
    art: ArtGallery,
    fx: FxEngine,
    scene_age: f32,
    pending: Option<PendingTransition>,
    show_compass: bool,
    show_foreword: bool,
    confirm_reset: bool,
    endings_filter: Protagonist,
    scroll_to_top: bool,
    last_celebrated_node: Option<String>,
}

impl ConsequencesApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let book_pt = BookData::load_for(Language::PtBr);
        let book_en = BookData::load_for(Language::En);
        let save = SaveState::load_or_default();
        let theme_kind = ThemeKind::from_index(save.theme_index);
        let palette = theme_kind.palette();

        Self {
            book_pt,
            book_en,
            save,
            screen: Screen::Home,
            theme_kind,
            current_palette: palette,
            target_palette: palette,
            art: ArtGallery::default(),
            fx: FxEngine::new(),
            scene_age: 0.0,
            pending: None,
            show_compass: true,
            show_foreword: false,
            confirm_reset: false,
            endings_filter: Protagonist::Helen,
            scroll_to_top: false,
            last_celebrated_node: None,
        }
    }

    fn lang(&self) -> Language {
        self.save.language
    }

    fn active_book(&self) -> &BookData {
        match self.save.language {
            Language::PtBr => &self.book_pt,
            Language::En => &self.book_en,
        }
    }

    fn toggle_language(&mut self, click_pos: Pos2) {
        self.save.language = self.save.language.toggle();
        self.save.save();
        self.fx.spawn_choice_burst(
            click_pos,
            self.current_palette.gold,
            self.current_palette.emerald,
        );
    }

    fn set_theme(&mut self, kind: ThemeKind) {
        self.theme_kind = kind;
        self.target_palette = kind.palette();
        self.save.theme_index = kind.to_index();
        self.save.save();
    }

    fn reset_all_progress(&mut self, click_pos: Pos2) {
        self.save.reset_progress();
        self.pending = None;
        self.confirm_reset = false;
        self.last_celebrated_node = None;
        self.scene_age = 0.0;
        self.fx.spawn_choice_burst(
            click_pos,
            self.current_palette.accent,
            self.current_palette.gold,
        );
    }

    fn start_protagonist(&mut self, protagonist: Protagonist, click_pos: Pos2) {
        let start_id = protagonist.start_id().to_string();
        self.fx
            .spawn_choice_burst(click_pos, self.current_palette.gold, self.current_palette.accent);
        self.save.current_node_id = Some(start_id.clone());
        self.save.history.clear();
        self.save.traits.clear();
        self.save.visited_nodes.insert(start_id);
        self.save.save();
        self.screen = Screen::Playing;
        self.confirm_reset = false;
        self.scene_age = 0.0;
        self.pending = None;
        self.scroll_to_top = true;
        self.last_celebrated_node = None;
    }

    fn queue_choice(&mut self, idx: usize, click_pos: Pos2) {
        if self.pending.is_some() {
            return;
        }
        let Some(curr_id) = self.save.current_node_id.clone() else {
            return;
        };
        let Some(choice) = self
            .active_book()
            .nodes
            .get(&curr_id)
            .and_then(|n| n.choices.get(idx))
            .cloned()
        else {
            return;
        };

        let trait_col = self.current_palette.trait_color(&choice.trait_name);
        self.fx
            .spawn_choice_burst(click_pos, trait_col, self.current_palette.gold);

        self.pending = Some(PendingTransition {
            target_id: choice.target,
            chosen_label: choice.label,
            chosen_trait: choice.trait_name,
            choice_index: idx,
            timer: 0.24,
        });
    }

    fn commit_pending(&mut self, p: PendingTransition, viewport: Rect) {
        if let Some(prev_id) = self.save.current_node_id.clone() {
            self.save.history.push(HistoryEntry {
                node_id: prev_id,
                chosen_label: Some(p.chosen_label),
                chosen_trait: if p.chosen_trait.is_empty() {
                    None
                } else {
                    Some(p.chosen_trait.clone())
                },
            });
        }
        if !p.chosen_trait.is_empty() {
            *self.save.traits.entry(p.chosen_trait).or_insert(0) += 1;
        }
        self.save.current_node_id = Some(p.target_id.clone());
        self.save.visited_nodes.insert(p.target_id.clone());

        let ending_tone = self
            .active_book()
            .nodes
            .get(&p.target_id)
            .filter(|n| n.is_ending)
            .map(|n| n.ending_tone.clone().unwrap_or_else(|| "sereno".to_string()));

        if let Some(tone) = ending_tone {
            self.save.discovered_endings.insert(p.target_id.clone());
            let tone_col = self.current_palette.ending_tone_color(&tone);
            self.fx
                .spawn_ending_celebration(viewport, tone_col, self.current_palette.gold, &tone);
            self.last_celebrated_node = Some(p.target_id.clone());
        }

        self.save.save();
        self.scene_age = 0.0;
        self.scroll_to_top = true;
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context, viewport: Rect) {
        if ctx.input(|i| i.key_pressed(Key::T)) {
            self.set_theme(self.theme_kind.next());
        }
        if ctx.input(|i| i.key_pressed(Key::L)) {
            self.toggle_language(viewport.center());
        }
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            if self.confirm_reset {
                self.confirm_reset = false;
            } else if self.screen != Screen::Home {
                self.screen = Screen::Home;
                self.scene_age = 0.0;
            }
        }
        if self.screen == Screen::Playing {
            let digit_keys = [
                Key::Num1,
                Key::Num2,
                Key::Num3,
                Key::Num4,
                Key::Num5,
                Key::Num6,
                Key::Num7,
                Key::Num8,
                Key::Num9,
            ];
            for (idx, k) in digit_keys.iter().enumerate() {
                if ctx.input(|i| i.key_pressed(*k)) {
                    self.queue_choice(idx, viewport.center());
                    break;
                }
            }
        }
    }

    fn render_top_bar(&mut self, ctx: &egui::Context) {
        let palette = self.current_palette;
        let lang = self.lang();
        let total_endings = self.active_book().total_endings();
        let discovered = self.save.discovered_endings.len();

        egui::TopBottomPanel::top("top_bar")
            .exact_height(56.0)
            .frame(
                egui::Frame::NONE
                    .fill(palette.bg_paper)
                    .stroke(Stroke::new(1.0_f32, palette.border))
                    .inner_margin(egui::Margin::symmetric(18, 8)),
            )
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    paint_icon_inline(ui, 16.0, IconKind::Starburst, palette.gold);
                    ui.add_space(4.0);
                    let title_resp = ui.add(
                        egui::Label::new(
                            RichText::new(lang.t(
                                "CONSIDERE AS CONSEQUÊNCIAS",
                                "CONSIDER THE CONSEQUENCES",
                            ))
                            .font(font_serif_bold(16.0))
                            .color(palette.gold),
                        )
                        .sense(egui::Sense::click()),
                    );
                    if title_resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                    if title_resp.clicked() {
                        self.screen = Screen::Home;
                        self.scene_age = 0.0;
                    }

                    ui.add_space(10.0);
                    ui.label(
                        RichText::new("1930")
                            .font(font_ui(11.0))
                            .color(palette.ink_muted),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Language toggle (PT-BR / EN 1930)
                        let (lang_clicked, lang_pos) = animated_deco_button(
                            ui,
                            "btn_lang",
                            lang.badge(),
                            Some(IconKind::Book),
                            &palette,
                            ButtonStyle::Secondary,
                        );
                        if lang_clicked {
                            self.toggle_language(lang_pos);
                        }

                        // Theme cycle button
                        let (clicked, pos) = animated_deco_button(
                            ui,
                            "btn_theme",
                            self.theme_kind.label(lang),
                            Some(IconKind::HalfMoon),
                            &palette,
                            ButtonStyle::Secondary,
                        );
                        if clicked {
                            self.set_theme(self.theme_kind.next());
                            self.fx.spawn_choice_burst(pos, palette.gold, palette.accent);
                        }

                        let (plus_clicked, _) = animated_deco_button(
                            ui,
                            "btn_font_plus",
                            "A+",
                            None,
                            &palette,
                            ButtonStyle::Secondary,
                        );
                        if plus_clicked {
                            self.save.font_scale = (self.save.font_scale + 0.08).min(1.32);
                            self.save.save();
                        }
                        let (minus_clicked, _) = animated_deco_button(
                            ui,
                            "btn_font_minus",
                            "A-",
                            None,
                            &palette,
                            ButtonStyle::Secondary,
                        );
                        if minus_clicked {
                            self.save.font_scale = (self.save.font_scale - 0.08).max(0.84);
                            self.save.save();
                        }

                        let gallery_label = format!(
                            "{} ({discovered}/{total_endings})",
                            lang.t("Finais", "Endings")
                        );
                        let (g_clicked, pos) = animated_deco_button(
                            ui,
                            "btn_gallery",
                            &gallery_label,
                            Some(IconKind::Trophy),
                            &palette,
                            if self.screen == Screen::Endings {
                                ButtonStyle::Primary
                            } else {
                                ButtonStyle::Secondary
                            },
                        );
                        if g_clicked {
                            self.screen = if self.screen == Screen::Endings {
                                if self.save.current_node_id.is_some() {
                                    Screen::Playing
                                } else {
                                    Screen::Home
                                }
                            } else {
                                if let Some(cid) = &self.save.current_node_id {
                                    self.endings_filter = Protagonist::from_node_id(cid);
                                }
                                Screen::Endings
                            };
                            self.scene_age = 0.0;
                            self.fx.spawn_choice_burst(pos, palette.gold, palette.emerald);
                        }

                        if self.screen == Screen::Playing {
                            let compass_lbl = if self.show_compass {
                                lang.t("Ocultar Bússola", "Hide Compass")
                            } else {
                                lang.t("Bússola", "Compass")
                            };
                            let (c_clicked, _) = animated_deco_button(
                                ui,
                                "btn_compass",
                                compass_lbl,
                                Some(IconKind::Compass),
                                &palette,
                                ButtonStyle::Secondary,
                            );
                            if c_clicked {
                                self.show_compass = !self.show_compass;
                            }
                        }
                    });
                });
            });
    }

    fn render_home(&mut self, ui: &mut egui::Ui) {
        let palette = self.current_palette;
        let lang = self.lang();
        let enter = ease_out_cubic(self.scene_age / 0.55);
        let foreword_paragraphs = self.active_book().foreword.paragraphs.clone();

        ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            let max_w = ui.available_width().min(960.0);
            let side_pad = ((ui.available_width() - max_w) * 0.5).max(16.0);

            ui.add_space(22.0);
            ui.horizontal(|ui| {
                ui.add_space(side_pad);
                ui.vertical(|ui| {
                    ui.set_width(max_w);

                    let hero_rect_top = ui.cursor().top();
                    egui::Frame::NONE
                        .fill(palette.bg_paper)
                        .corner_radius(10.0)
                        .inner_margin(egui::Margin::symmetric(34, 30))
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new("DORIS WEBSTER & MARY ALDEN HOPKINS · 1930")
                                        .font(font_ui(11.5))
                                        .color(with_alpha(palette.gold, enter)),
                                );
                                ui.add_space(8.0);
                                ui.label(
                                    RichText::new(lang.t(
                                        "Considere as Consequências!",
                                        "Consider the Consequences!",
                                    ))
                                    .font(font_serif_bold(36.0))
                                    .color(with_alpha(palette.ink, enter)),
                                );
                                ui.add_space(6.0);
                                ui.label(
                                    RichText::new(lang.t(
                                        "O primeiro livro-jogo interativo de escolhas da história — agora em Rust",
                                        "The first interactive choose-your-own-adventure book in history — now in Rust",
                                    ))
                                    .font(font_serif_italic(16.5))
                                    .color(with_alpha(palette.ink_soft, enter)),
                                );
                                ui.add_space(16.0);
                                let div_y = ui.cursor().top() + 4.0;
                                draw_art_deco_divider(
                                    ui.painter(),
                                    ui.min_rect().center().x,
                                    div_y,
                                    130.0 * enter,
                                    palette.gold,
                                );
                                ui.add_space(18.0);

                                let has_progress = self.save.has_any_progress();
                                ui.horizontal_wrapped(|ui| {
                                    let est_w = if self.save.current_node_id.is_some() {
                                        615.0
                                    } else if has_progress {
                                        395.0
                                    } else {
                                        380.0
                                    };
                                    let pad = ((ui.available_width() - est_w) * 0.5).max(0.0);
                                    ui.add_space(pad);

                                    if let Some(curr_id) = self.save.current_node_id.clone() {
                                        let p = Protagonist::from_node_id(&curr_id);
                                        let label = format!(
                                            "{} ({})",
                                            lang.t("Continuar", "Continue"),
                                            p.name()
                                        );
                                        let (clicked, pos) = animated_deco_button(
                                            ui,
                                            "btn_continue",
                                            &label,
                                            Some(IconKind::Play),
                                            &palette,
                                            ButtonStyle::Primary,
                                        );
                                        if clicked {
                                            self.screen = Screen::Playing;
                                            self.scene_age = 0.0;
                                            self.scroll_to_top = true;
                                            self.fx.spawn_choice_burst(pos, palette.gold, palette.accent);
                                        }
                                        ui.add_space(8.0);
                                    }

                                    let fw_lbl = if self.show_foreword {
                                        lang.t("Ocultar Prefácio", "Hide Foreword")
                                    } else {
                                        lang.t("Ler Prefácio de 1930", "Read 1930 Foreword")
                                    };
                                    let (fw_clicked, _) = animated_deco_button(
                                        ui,
                                        "btn_foreword",
                                        fw_lbl,
                                        Some(if self.show_foreword {
                                            IconKind::ChevronUp
                                        } else {
                                            IconKind::Book
                                        }),
                                        &palette,
                                        ButtonStyle::Secondary,
                                    );
                                    if fw_clicked {
                                        self.show_foreword = !self.show_foreword;
                                    }

                                    ui.add_space(8.0);
                                    let (reset_clicked, _) = animated_deco_button(
                                        ui,
                                        "btn_reset_all",
                                        lang.t("Resetar Jogo", "Reset Game"),
                                        Some(IconKind::Reset),
                                        &palette,
                                        if self.confirm_reset {
                                            ButtonStyle::Danger
                                        } else {
                                            ButtonStyle::Secondary
                                        },
                                    );
                                    if reset_clicked {
                                        self.confirm_reset = !self.confirm_reset;
                                    }
                                });

                                if self.confirm_reset {
                                    ui.add_space(14.0);
                                    egui::Frame::NONE
                                        .fill(with_alpha(palette.accent, 0.14))
                                        .corner_radius(8.0)
                                        .stroke(Stroke::new(1.2_f32, palette.accent))
                                        .inner_margin(egui::Margin::symmetric(20, 12))
                                        .show(ui, |ui| {
                                            ui.vertical_centered(|ui| {
                                                ui.label(
                                                    RichText::new(lang.t(
                                                        "Deseja zerar todas as escolhas, caminhos visitados e finais descobertos?",
                                                        "Reset all choices, visited paths, and discovered endings?",
                                                    ))
                                                    .font(font_serif_bold(14.5))
                                                    .color(palette.ink),
                                                );
                                                ui.add_space(10.0);
                                                ui.horizontal(|ui| {
                                                    let pad = ((ui.available_width() - 310.0) * 0.5).max(0.0);
                                                    ui.add_space(pad);
                                                    let (ok, pos) = animated_deco_button(
                                                        ui,
                                                        "btn_confirm_reset",
                                                        lang.t("Sim, Zerar Tudo", "Yes, Reset All"),
                                                        Some(IconKind::Check),
                                                        &palette,
                                                        ButtonStyle::Danger,
                                                    );
                                                    if ok {
                                                        self.reset_all_progress(pos);
                                                    }
                                                    ui.add_space(10.0);
                                                    let (cancel, _) = animated_deco_button(
                                                        ui,
                                                        "btn_cancel_reset",
                                                        lang.t("Cancelar", "Cancel"),
                                                        Some(IconKind::Close),
                                                        &palette,
                                                        ButtonStyle::Secondary,
                                                    );
                                                    if cancel {
                                                        self.confirm_reset = false;
                                                    }
                                                });
                                            });
                                        });
                                }

                                if self.show_foreword {
                                    ui.add_space(18.0);
                                    egui::Frame::NONE
                                        .fill(palette.bg_card)
                                        .corner_radius(8.0)
                                        .stroke(Stroke::new(1.0_f32, palette.border))
                                        .inner_margin(egui::Margin::symmetric(22, 16))
                                        .show(ui, |ui| {
                                            for para in &foreword_paragraphs {
                                                ui.label(
                                                    RichText::new(para)
                                                        .font(font_serif(15.5 * self.save.font_scale))
                                                        .color(palette.ink_soft),
                                                );
                                                ui.add_space(8.0);
                                            }
                                        });
                                }
                            });
                        });
                    let hero_rect_bottom = ui.min_rect().bottom();
                    let hero_rect = Rect::from_min_max(
                        Pos2::new(ui.min_rect().left(), hero_rect_top),
                        Pos2::new(ui.min_rect().left() + max_w, hero_rect_bottom),
                    );
                    draw_art_deco_frame(ui.painter(), hero_rect, palette.gold, 0.85 * enter);

                    ui.add_space(26.0);
                    ui.vertical_centered(|ui| {
                        ui.label(
                            RichText::new(lang.t(
                                "ESCOLHA UM DESTINO PARA CONDUZIR",
                                "CHOOSE A DESTINY TO GUIDE",
                            ))
                            .font(font_ui(12.0))
                            .color(palette.gold),
                        );
                    });
                    ui.add_space(14.0);

                    let gap = 16.0;
                    let card_w = ((max_w - gap * 2.0) / 3.0).max(220.0);
                    let mut chosen_start: Option<(Protagonist, Pos2)> = None;

                    ui.horizontal(|ui| {
                        for (i, p) in Protagonist::ALL.iter().enumerate() {
                            let disc = self.save.discovered_endings_for(*p);
                            let total_e = self.active_book().endings_for(*p).len();
                            let total_s = self.active_book().total_nodes_for(*p);
                            let portrait_id = self
                                .art
                                .texture(ui.ctx(), ArtKind::for_protagonist(*p))
                                .id();
                            let (clicked, pos) = protagonist_card(
                                ui,
                                *p,
                                portrait_id,
                                lang,
                                disc,
                                total_e,
                                total_s,
                                &palette,
                                card_w,
                            );
                            if clicked {
                                chosen_start = Some((*p, pos));
                            }
                            if i < 2 {
                                ui.add_space(gap);
                            }
                        }
                    });

                    if let Some((p, pos)) = chosen_start {
                        self.start_protagonist(p, pos);
                    }

                    ui.add_space(28.0);
                    ui.vertical_centered(|ui| {
                        ui.label(
                            RichText::new(lang.t(
                                "Atalhos: teclas 1-9 escolhem opções · T alterna o tema · L alterna o idioma · Esc volta ao menu",
                                "Shortcuts: keys 1-9 pick choices · T switches theme · L switches language · Esc returns to menu",
                            ))
                            .font(font_ui(11.5))
                            .color(palette.ink_muted),
                        );
                    });
                    ui.add_space(24.0);
                });
            });
        });
    }

    fn render_playing(&mut self, ui: &mut egui::Ui) {
        let palette = self.current_palette;
        let lang = self.lang();
        let Some(curr_id) = self.save.current_node_id.clone() else {
            self.screen = Screen::Home;
            return;
        };
        let Some(node) = self.active_book().nodes.get(&curr_id).cloned() else {
            self.screen = Screen::Home;
            return;
        };

        // Pre-resolve history titles in the active language so switching language updates the trail too
        let history_items: Vec<(String, Option<String>)> = self
            .save
            .history
            .iter()
            .map(|entry| {
                let title = self
                    .active_book()
                    .nodes
                    .get(&entry.node_id)
                    .map(|n| n.title.clone())
                    .unwrap_or_else(|| entry.node_id.clone());
                (title, entry.chosen_label.clone())
            })
            .collect();

        let protagonist = Protagonist::from_node_id(&node.id);
        let avail_w = ui.available_width();
        let show_side = self.show_compass && avail_w >= 940.0;
        let side_w = if show_side { 265.0 } else { 0.0 };
        let main_w = (avail_w - side_w - if show_side { 44.0 } else { 32.0 }).min(760.0);
        let total_content_w = main_w + if show_side { side_w + 20.0 } else { 0.0 };
        let left_pad = ((avail_w - total_content_w) * 0.5).max(14.0);

        let mut scroll = ScrollArea::vertical().auto_shrink([false, false]);
        if self.scroll_to_top {
            scroll = scroll.vertical_scroll_offset(0.0);
            self.scroll_to_top = false;
        }

        let mut clicked_choice: Option<(usize, Pos2)> = None;
        let mut restart_click: Option<Pos2> = None;
        let mut go_home_click = false;

        scroll.show(ui, |ui| {
            ui.add_space(20.0);
            ui.horizontal_top(|ui| {
                ui.add_space(left_pad);

                // === MAIN MANUSCRIPT COLUMN ===
                ui.vertical(|ui| {
                    ui.set_width(main_w);

                    let card_top = ui.cursor().top();
                    egui::Frame::NONE
                        .fill(palette.bg_paper)
                        .corner_radius(10.0)
                        .inner_margin(egui::Margin::symmetric(34, 28))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(protagonist.chapter_label(lang))
                                        .font(font_ui(11.5))
                                        .color(palette.gold),
                                );
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        let step_lbl = match lang {
                                            Language::PtBr => format!(
                                                "Cena {} · Passo {}",
                                                node.id,
                                                self.save.history.len() + 1
                                            ),
                                            Language::En => format!(
                                                "Scene {} · Step {}",
                                                node.id,
                                                self.save.history.len() + 1
                                            ),
                                        };
                                        ui.label(
                                            RichText::new(step_lbl)
                                                .font(font_ui(11.0))
                                                .color(palette.ink_muted),
                                        );
                                    },
                                );
                            });

                            ui.add_space(8.0);

                            let title_t = ease_out_cubic(self.scene_age / 0.35);
                            ui.label(
                                RichText::new(&node.title)
                                    .font(font_serif_bold(27.0 * self.save.font_scale))
                                    .color(with_alpha(palette.ink, title_t)),
                            );

                            ui.add_space(10.0);
                            let div_y = ui.cursor().top() + 4.0;
                            draw_art_deco_divider(
                                ui.painter(),
                                ui.min_rect().center().x,
                                div_y,
                                (main_w * 0.28) * title_t,
                                palette.gold,
                            );
                            ui.add_space(16.0);

                            // 1930s pen-and-ink bookplate illustration for this scene
                            let art_kind = ArtKind::for_node(&node);
                            let art_t = ease_out_cubic((self.scene_age - 0.04) / 0.45);
                            let plate_w = (main_w - 76.0).min(540.0);
                            self.art.paint_bookplate(ui, art_kind, plate_w, 250.0, &palette, art_t);
                            ui.add_space(4.0);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new(art_kind.caption(lang))
                                        .font(font_serif_italic(11.5))
                                        .color(with_alpha(palette.ink_muted, art_t)),
                                );
                            });
                            ui.add_space(16.0);

                            for (i, para) in node.paragraphs.iter().enumerate() {
                                let delay = 0.06 + (i as f32) * 0.09;
                                let p_t = ease_out_cubic((self.scene_age - delay) / 0.40);
                                ui.label(
                                    RichText::new(para)
                                        .font(font_serif(18.0 * self.save.font_scale))
                                        .color(with_alpha(palette.ink, p_t)),
                                );
                                ui.add_space(13.0);
                            }

                            if node.is_ending {
                                ui.add_space(8.0);
                                let tone = node.ending_tone.as_deref().unwrap_or("sereno");
                                let tone_col = palette.ending_tone_color(tone);
                                let end_t = ease_out_cubic((self.scene_age - 0.18) / 0.45);

                                egui::Frame::NONE
                                    .fill(with_alpha(tone_col, 0.12 * end_t))
                                    .corner_radius(10.0)
                                    .stroke(Stroke::new(1.5_f32, with_alpha(tone_col, 0.85 * end_t)))
                                    .inner_margin(egui::Margin::symmetric(24, 20))
                                    .show(ui, |ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.horizontal(|ui| {
                                                let pad = ((ui.available_width() - 210.0) * 0.5).max(0.0);
                                                ui.add_space(pad);
                                                paint_icon_inline(
                                                    ui,
                                                    14.0,
                                                    IconKind::Starburst,
                                                    with_alpha(tone_col, end_t),
                                                );
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{} · {}",
                                                        lang.t("DESFECHO", "ENDING"),
                                                        ending_tone_label(tone, lang)
                                                    ))
                                                    .font(font_ui(12.0))
                                                    .color(with_alpha(tone_col, end_t)),
                                                );
                                                paint_icon_inline(
                                                    ui,
                                                    14.0,
                                                    IconKind::Starburst,
                                                    with_alpha(tone_col, end_t),
                                                );
                                            });
                                            if let Some(et) = &node.ending_title {
                                                ui.add_space(6.0);
                                                ui.label(
                                                    RichText::new(et)
                                                        .font(font_serif_bold(22.0 * self.save.font_scale))
                                                        .color(with_alpha(palette.ink, end_t)),
                                                );
                                            }
                                            ui.add_space(16.0);
                                            ui.horizontal(|ui| {
                                                let pad = ((ui.available_width() - 360.0) * 0.5).max(0.0);
                                                ui.add_space(pad);
                                                let (r, pos) = animated_deco_button(
                                                    ui,
                                                    "end_restart",
                                                    lang.t("Iniciar Novamente", "Play Again"),
                                                    Some(IconKind::Reset),
                                                    &palette,
                                                    ButtonStyle::Primary,
                                                );
                                                if r {
                                                    restart_click = Some(pos);
                                                }
                                                ui.add_space(10.0);
                                                let (home_c, _) = animated_deco_button(
                                                    ui,
                                                    "end_home",
                                                    lang.t("Menu Inicial", "Main Menu"),
                                                    Some(IconKind::ArrowLeft),
                                                    &palette,
                                                    ButtonStyle::Secondary,
                                                );
                                                if home_c {
                                                    go_home_click = true;
                                                }
                                            });
                                        });
                                    });
                            } else {
                                ui.add_space(6.0);
                                if let Some(prompt) = &node.prompt {
                                    ui.label(
                                        RichText::new(prompt)
                                            .font(font_serif_italic(17.0 * self.save.font_scale))
                                            .color(palette.gold),
                                    );
                                    ui.add_space(12.0);
                                }

                                let base_delay = 0.14 + (node.paragraphs.len().min(4) as f32) * 0.06;
                                for (idx, choice) in node.choices.iter().enumerate() {
                                    let enter_t = (self.scene_age - base_delay - idx as f32 * 0.08) / 0.35;
                                    let is_flashing = self
                                        .pending
                                        .as_ref()
                                        .is_some_and(|p| p.choice_index == idx);
                                    let res = animated_choice_card(
                                        ui,
                                        &format!("choice_{}_{}", node.id, idx),
                                        idx,
                                        choice,
                                        lang,
                                        &palette,
                                        self.save.font_scale,
                                        enter_t,
                                        self.fx.time,
                                        is_flashing,
                                    );
                                    if res.clicked {
                                        clicked_choice = Some((idx, res.click_pos));
                                    }
                                    ui.add_space(10.0);
                                }
                            }
                        });

                    let card_bottom = ui.min_rect().bottom();
                    let card_rect = Rect::from_min_max(
                        Pos2::new(ui.min_rect().left(), card_top),
                        Pos2::new(ui.min_rect().left() + main_w, card_bottom),
                    );
                    draw_art_deco_frame(ui.painter(), card_rect, palette.gold, 0.75);
                    ui.add_space(26.0);
                });

                // === SIDE PANEL: CHARACTER COMPASS + READ-ONLY HISTORY ===
                if show_side {
                    ui.add_space(20.0);
                    ui.vertical(|ui| {
                        ui.set_width(side_w);

                        egui::Frame::NONE
                            .fill(palette.bg_paper)
                            .corner_radius(10.0)
                            .stroke(Stroke::new(1.0_f32, palette.border))
                            .inner_margin(egui::Margin::symmetric(18, 18))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    paint_icon_inline(ui, 14.0, IconKind::Compass, palette.gold);
                                    ui.label(
                                        RichText::new(lang.t(
                                            "BÚSSOLA DE CARÁTER",
                                            "CHARACTER COMPASS",
                                        ))
                                        .font(font_ui(11.5))
                                        .color(palette.gold),
                                    );
                                });
                                ui.add_space(4.0);
                                ui.label(
                                    RichText::new(lang.t(
                                        "Suas decisões moldam o perfil desta jornada.",
                                        "Your decisions shape the profile of this journey.",
                                    ))
                                    .font(font_serif_italic(13.0))
                                    .color(palette.ink_muted),
                                );
                                ui.add_space(12.0);

                                let max_val = self
                                    .save
                                    .traits
                                    .values()
                                    .copied()
                                    .max()
                                    .unwrap_or(1)
                                    .max(1) as f32;

                                for key in ALL_TRAITS {
                                    let val = *self.save.traits.get(key).unwrap_or(&0);
                                    let t_col = palette.trait_color(key);
                                    let target_frac = (val as f32) / max_val;
                                    let anim_frac = ui.ctx().animate_value_with_time(
                                        ui.make_persistent_id(("trait_bar", key)),
                                        target_frac,
                                        0.35,
                                    );

                                    ui.horizontal(|ui| {
                                        paint_icon_inline(
                                            ui,
                                            12.0,
                                            icon_for_trait(key),
                                            if val > 0 { t_col } else { palette.ink_muted },
                                        );
                                        ui.label(
                                            RichText::new(trait_label(key, lang))
                                                .font(font_ui(12.0))
                                                .color(if val > 0 { palette.ink } else { palette.ink_muted }),
                                        );
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    RichText::new(format!("{val}"))
                                                        .font(font_ui(11.5))
                                                        .color(t_col),
                                                );
                                            },
                                        );
                                    });

                                    let (bar_rect, _) = ui.allocate_exact_size(
                                        Vec2::new(ui.available_width(), 6.0),
                                        egui::Sense::hover(),
                                    );
                                    ui.painter().rect_filled(
                                        bar_rect,
                                        3.0,
                                        with_alpha(palette.border, 0.55),
                                    );
                                    if anim_frac > 0.01 {
                                        let fill_rect = Rect::from_min_max(
                                            bar_rect.min,
                                            Pos2::new(
                                                bar_rect.left() + bar_rect.width() * anim_frac,
                                                bar_rect.max.y,
                                            ),
                                        );
                                        ui.painter().rect_filled(fill_rect, 3.0, t_col);
                                    }
                                    ui.add_space(7.0);
                                }
                            });

                        ui.add_space(14.0);

                        egui::Frame::NONE
                            .fill(palette.bg_paper)
                            .corner_radius(10.0)
                            .stroke(Stroke::new(1.0_f32, palette.border))
                            .inner_margin(egui::Margin::symmetric(18, 16))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    paint_icon_inline(ui, 14.0, IconKind::Trail, palette.gold);
                                    ui.label(
                                        RichText::new(lang.t(
                                            "TRILHA DE ESCOLHAS",
                                            "CHOICE CHRONICLE",
                                        ))
                                        .font(font_ui(11.5))
                                        .color(palette.gold),
                                    );
                                });
                                ui.add_space(8.0);
                                if history_items.is_empty() {
                                    ui.label(
                                        RichText::new(lang.t(
                                            "Nenhuma escolha feita ainda.",
                                            "No choices made yet.",
                                        ))
                                        .font(font_serif_italic(13.0))
                                        .color(palette.ink_muted),
                                    );
                                } else {
                                    for (i, (title, chosen_lbl)) in history_items.iter().enumerate() {
                                        ui.label(
                                            RichText::new(format!("{}. {}", i + 1, title))
                                                .font(font_ui(12.0))
                                                .color(palette.ink_soft),
                                        );
                                        if let Some(lbl) = chosen_lbl {
                                            ui.horizontal(|ui| {
                                                ui.add_space(8.0);
                                                paint_icon_inline(
                                                    ui,
                                                    9.0,
                                                    IconKind::ArrowRight,
                                                    palette.ink_muted,
                                                );
                                                ui.label(
                                                    RichText::new(lbl)
                                                        .font(font_serif_italic(12.0))
                                                        .color(palette.ink_muted),
                                                );
                                            });
                                        }
                                        ui.add_space(5.0);
                                    }
                                }
                            });
                    });
                }
            });
        });

        if let Some((idx, pos)) = clicked_choice {
            self.queue_choice(idx, pos);
        }
        if let Some(pos) = restart_click {
            self.start_protagonist(protagonist, pos);
        }
        if go_home_click {
            self.screen = Screen::Home;
            self.scene_age = 0.0;
        }
    }

    fn render_endings(&mut self, ui: &mut egui::Ui) {
        let palette = self.current_palette;
        let lang = self.lang();

        egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(24, 18))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    paint_icon_inline(ui, 22.0, IconKind::Trophy, palette.gold);
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(lang.t(
                            "Árvore de Caminhos e Desfechos",
                            "Tree of Paths & Endings",
                        ))
                        .font(font_serif_bold(26.0))
                        .color(palette.ink),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (back, _) = animated_deco_button(
                            ui,
                            "gal_back",
                            lang.t("Voltar", "Back"),
                            Some(IconKind::ArrowLeft),
                            &palette,
                            ButtonStyle::Secondary,
                        );
                        if back {
                            self.screen = if self.save.current_node_id.is_some() {
                                Screen::Playing
                            } else {
                                Screen::Home
                            };
                        }
                    });
                });

                ui.add_space(4.0);
                ui.label(
                    RichText::new(lang.t(
                        "A árvore cresce automaticamente conforme você percorre escolhas e conclui novos finais.",
                        "The tree grows automatically as you follow choices and reach new endings.",
                    ))
                    .font(font_serif_italic(14.0))
                    .color(palette.ink_soft),
                );
                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    for p in Protagonist::ALL {
                        let disc = self.save.discovered_endings_for(p);
                        let tot = self.active_book().endings_for(p).len();
                        let lbl = format!("{} ({disc}/{tot})", p.name());
                        let (clicked, _) = animated_deco_button(
                            ui,
                            &format!("tab_{}", p.start_id()),
                            &lbl,
                            None,
                            &palette,
                            if self.endings_filter == p {
                                ButtonStyle::Primary
                            } else {
                                ButtonStyle::Secondary
                            },
                        );
                        if clicked {
                            self.endings_filter = p;
                        }
                        ui.add_space(8.0);
                    }
                });

                ui.add_space(12.0);

                let book_ref = match lang {
                    Language::PtBr => &self.book_pt,
                    Language::En => &self.book_en,
                };
                render_endings_tree(
                    ui,
                    book_ref,
                    &self.save,
                    self.endings_filter,
                    lang,
                    &palette,
                    self.fx.time,
                );
            });
    }
}

impl eframe::App for ConsequencesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.stable_dt).clamp(0.001, 0.05);
        self.scene_age += dt;
        self.fx.update(dt);

        self.current_palette =
            ThemePalette::lerp(self.current_palette, self.target_palette, (dt * 9.0).min(1.0));
        self.current_palette.apply_to_ctx(ctx);

        let viewport = ctx.screen_rect();
        self.handle_keyboard(ctx, viewport);

        if let Some(p) = self.pending.as_mut() {
            p.timer -= dt;
            if p.timer <= 0.0
                && let Some(done) = self.pending.take()
            {
                self.commit_pending(done, viewport);
            }
        }

        self.render_top_bar(ctx);

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(self.current_palette.bg_outer))
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                self.fx
                    .draw_ambient(ui.painter(), rect, self.current_palette.gold);

                match self.screen {
                    Screen::Home => self.render_home(ui),
                    Screen::Playing => self.render_playing(ui),
                    Screen::Endings => self.render_endings(ui),
                }
            });

        let fg_painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("fx_foreground"),
        ));
        self.fx.draw_foreground(&fg_painter);

        // Adaptive repaint: 60 FPS during active FX/transitions/hover, gentle 30 FPS ambient when idle
        let pointer_active = ctx.input(|i| i.pointer.has_pointer() && i.pointer.is_moving());
        if self.fx.has_active_bursts()
            || self.pending.is_some()
            || self.scene_age < 1.3
            || pointer_active
        {
            ctx.request_repaint();
        } else {
            ctx.request_repaint_after(std::time::Duration::from_millis(33));
        }
    }
}
