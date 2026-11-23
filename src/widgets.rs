use crate::fx::{draw_art_deco_frame, draw_diamond, ease_out_cubic};
use crate::icons::{draw_icon, icon_for_trait, IconKind};
use crate::story::{trait_label, Choice, Language, Protagonist};
use crate::theme::{lerp_color, with_alpha, ThemePalette};
use eframe::egui::{self, Color32, FontFamily, FontId, Pos2, Rect, Sense, Stroke, Vec2};

pub fn font_serif(size: f32) -> FontId {
    FontId::new(size, FontFamily::Proportional)
}

pub fn font_serif_bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("serif_bold".into()))
}

pub fn font_serif_italic(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("serif_italic".into()))
}

pub fn font_ui(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("ui_sans".into()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    Secondary,
    Primary,
    Danger,
}

pub struct ChoiceCardResult {
    pub clicked: bool,
    pub click_pos: Pos2,
}

/// Interactive Art Deco choice card with hover lift, border glow, shimmer sweep, and vector trait icon.
#[allow(clippy::too_many_arguments)]
pub fn animated_choice_card(
    ui: &mut egui::Ui,
    id_salt: &str,
    index: usize,
    choice: &Choice,
    lang: Language,
    palette: &ThemePalette,
    font_scale: f32,
    enter_t: f32,
    time: f32,
    is_selected_flash: bool,
) -> ChoiceCardResult {
    let enter = ease_out_cubic(enter_t);
    let avail_w = ui.available_width();
    let trait_color = palette.trait_color(&choice.trait_name);

    let text_max_w = (avail_w - 92.0).max(160.0);
    let label_galley = ui.painter().layout(
        choice.label.clone(),
        font_serif_bold(17.5 * font_scale),
        palette.ink,
        text_max_w,
    );
    let detail_galley = if !choice.detail.is_empty() {
        Some(ui.painter().layout(
            choice.detail.clone(),
            font_serif_italic(14.0 * font_scale),
            palette.ink_muted,
            text_max_w,
        ))
    } else {
        None
    };

    let detail_h = detail_galley
        .as_ref()
        .map(|g| g.size().y + 5.0)
        .unwrap_or(0.0);
    let card_h = (label_galley.size().y + detail_h + 44.0).max(74.0);

    let (base_rect, response) =
        ui.allocate_exact_size(Vec2::new(avail_w, card_h), Sense::click());

    let anim_id = ui.make_persistent_id(id_salt);
    let hover_t = ui.ctx().animate_bool_with_time(
        anim_id,
        response.hovered() || response.has_focus() || is_selected_flash,
        0.18,
    );
    let press_t = ui.ctx().animate_bool_with_time(
        anim_id.with("press"),
        response.is_pointer_button_down_on(),
        0.08,
    );

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    let enter_offset_y = (1.0 - enter) * 18.0;
    let lift_y = -2.5 * hover_t + 1.5 * press_t;
    let rect = base_rect.translate(Vec2::new(0.0, enter_offset_y + lift_y));

    let painter = ui.painter();
    let alpha = enter;

    if hover_t > 0.01 {
        let glow_rect = rect.expand(3.0 * hover_t);
        painter.rect_filled(
            glow_rect,
            10.0,
            with_alpha(trait_color, 0.14 * hover_t * alpha),
        );
    }

    let fill = lerp_color(palette.bg_card, palette.bg_card_hover, hover_t);
    let fill = if is_selected_flash {
        lerp_color(fill, trait_color, 0.22)
    } else {
        fill
    };
    painter.rect_filled(rect, 8.0, with_alpha(fill, alpha));

    let bar_w = 4.0 + 3.5 * hover_t;
    let bar_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top() + 6.0),
        Pos2::new(rect.left() + bar_w, rect.bottom() - 6.0),
    );
    painter.rect_filled(
        bar_rect,
        3.0,
        with_alpha(
            lerp_color(palette.gold_dim, trait_color, 0.45 + 0.55 * hover_t),
            alpha,
        ),
    );

    // Animated gold shimmer sweep along top edge when hovered
    if hover_t > 0.05 {
        let sweep = ((time * 1.15 + index as f32 * 0.37) % 1.4) - 0.2;
        let x0 = rect.left() + (sweep - 0.18).clamp(0.0, 1.0) * rect.width();
        let x1 = rect.left() + (sweep + 0.18).clamp(0.0, 1.0) * rect.width();
        if x1 > x0 {
            painter.line_segment(
                [Pos2::new(x0, rect.top() + 1.0), Pos2::new(x1, rect.top() + 1.0)],
                Stroke::new(2.0_f32, with_alpha(palette.gold, 0.75 * hover_t * alpha)),
            );
        }
    }

    let border_col = lerp_color(palette.border, trait_color, hover_t * 0.85);
    painter.rect_stroke(
        rect,
        8.0,
        Stroke::new(1.0_f32 + 0.8_f32 * hover_t, with_alpha(border_col, alpha)),
        egui::StrokeKind::Inside,
    );

    // Shortcut number diamond badge on the left
    let badge_center = Pos2::new(rect.left() + 28.0 + 2.0 * hover_t, rect.center().y);
    let badge_bg = lerp_color(palette.bg_paper, trait_color, hover_t * 0.85);
    draw_diamond(painter, badge_center, 13.0 + 1.2 * hover_t, with_alpha(badge_bg, alpha));
    let num_color = if hover_t > 0.45 {
        Color32::from_rgb(20, 16, 14)
    } else {
        palette.gold
    };
    painter.text(
        badge_center,
        egui::Align2::CENTER_CENTER,
        format!("{}", index + 1),
        font_ui(12.5),
        with_alpha(num_color, alpha),
    );

    let text_x = rect.left() + 52.0 + 3.0 * hover_t;
    let mut cursor_y = rect.top() + 13.0;

    // Vector trait icon + label
    if !choice.trait_name.is_empty() {
        let tag_col = with_alpha(
            lerp_color(palette.ink_muted, trait_color, 0.55 + 0.45 * hover_t),
            alpha,
        );
        draw_icon(
            painter,
            Pos2::new(text_x + 5.0, cursor_y + 6.0),
            11.0,
            icon_for_trait(&choice.trait_name),
            tag_col,
        );
        painter.text(
            Pos2::new(text_x + 15.0, cursor_y),
            egui::Align2::LEFT_TOP,
            trait_label(&choice.trait_name, lang).to_uppercase(),
            font_ui(10.5),
            tag_col,
        );
        cursor_y += 16.0;
    }

    let label_color = lerp_color(palette.ink, palette.gold, hover_t * 0.22);
    let label_galley = ui.painter().layout(
        choice.label.clone(),
        font_serif_bold(17.5 * font_scale),
        with_alpha(label_color, alpha),
        text_max_w,
    );
    painter.galley(Pos2::new(text_x, cursor_y), label_galley.clone(), Color32::TRANSPARENT);
    cursor_y += label_galley.size().y + 4.0;

    if !choice.detail.is_empty() {
        let detail_galley = ui.painter().layout(
            choice.detail.clone(),
            font_serif_italic(14.0 * font_scale),
            with_alpha(palette.ink_soft, 0.85 * alpha),
            text_max_w,
        );
        painter.galley(Pos2::new(text_x, cursor_y), detail_galley, Color32::TRANSPARENT);
    }

    // Animated vector right arrow gliding right on hover
    let arrow_x = rect.right() - 22.0 + 5.0 * hover_t;
    let arrow_col = lerp_color(palette.ink_muted, palette.gold, hover_t);
    draw_icon(
        painter,
        Pos2::new(arrow_x, rect.center().y),
        14.0,
        IconKind::ArrowRight,
        with_alpha(arrow_col, alpha),
    );

    let click_pos = response
        .interact_pointer_pos()
        .unwrap_or_else(|| rect.center());

    ChoiceCardResult {
        clicked: response.clicked(),
        click_pos,
    }
}

/// Sleek Art Deco button with optional vector icon, hover glow, and micro-lift.
pub fn animated_deco_button(
    ui: &mut egui::Ui,
    id_salt: &str,
    label: &str,
    icon: Option<IconKind>,
    palette: &ThemePalette,
    style: ButtonStyle,
) -> (bool, Pos2) {
    let galley = ui.painter().layout_no_wrap(
        label.to_string(),
        font_ui(13.0),
        palette.ink,
    );
    let icon_space = if icon.is_some() { 19.0 } else { 0.0 };
    let desired = Vec2::new(galley.size().x + icon_space + 26.0, 34.0);
    let (base_rect, response) = ui.allocate_exact_size(desired, Sense::click());

    let anim_id = ui.make_persistent_id(id_salt);
    let hover_t = ui.ctx().animate_bool_with_time(anim_id, response.hovered(), 0.15);

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    let rect = base_rect.translate(Vec2::new(0.0, -1.2 * hover_t));
    let painter = ui.painter();

    let (bg, border, text_col, icon_col) = match style {
        ButtonStyle::Primary => (
            lerp_color(palette.accent, palette.gold, hover_t * 0.4),
            lerp_color(palette.gold_dim, palette.gold, hover_t),
            Color32::from_rgb(250, 245, 236),
            Color32::from_rgb(252, 240, 212),
        ),
        ButtonStyle::Danger => (
            lerp_color(Color32::from_rgb(148, 36, 44), Color32::from_rgb(184, 46, 54), hover_t),
            lerp_color(Color32::from_rgb(206, 92, 98), Color32::from_rgb(236, 134, 134), hover_t),
            Color32::from_rgb(252, 244, 244),
            Color32::from_rgb(252, 230, 230),
        ),
        ButtonStyle::Secondary => (
            lerp_color(palette.bg_card, palette.bg_card_hover, hover_t),
            lerp_color(palette.border, palette.gold, hover_t),
            lerp_color(palette.ink_soft, palette.ink, hover_t),
            lerp_color(palette.gold_dim, palette.gold, 0.45 + 0.55 * hover_t),
        ),
    };

    painter.rect_filled(rect, 6.0, bg);
    painter.rect_stroke(
        rect,
        6.0,
        Stroke::new(1.0_f32 + 0.5_f32 * hover_t, border),
        egui::StrokeKind::Inside,
    );

    let total_inner_w = galley.size().x + icon_space;
    let start_x = rect.center().x - total_inner_w * 0.5;
    if let Some(kind) = icon {
        draw_icon(
            painter,
            Pos2::new(start_x + 6.5, rect.center().y),
            13.0 + 1.0 * hover_t,
            kind,
            icon_col,
        );
    }

    painter.text(
        Pos2::new(start_x + icon_space, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        font_ui(13.0),
        text_col,
    );

    let pos = response
        .interact_pointer_pos()
        .unwrap_or_else(|| rect.center());
    (response.clicked(), pos)
}

/// Interactive campaign card on the Home screen for Helen, Jed, and Saunders, with pen-and-ink portrait.
#[allow(clippy::too_many_arguments)]
pub fn protagonist_card(
    ui: &mut egui::Ui,
    protagonist: Protagonist,
    portrait_tex: egui::TextureId,
    lang: Language,
    discovered: usize,
    total_endings: usize,
    total_scenes: usize,
    palette: &ThemePalette,
    width: f32,
) -> (bool, Pos2) {
    let height = 310.0;
    let (base_rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());

    let anim_id = ui.make_persistent_id(("protagonist_card", protagonist.start_id()));
    let hover_t = ui.ctx().animate_bool_with_time(anim_id, response.hovered(), 0.2);

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    let rect = base_rect.translate(Vec2::new(0.0, -4.0 * hover_t));
    let painter = ui.painter();

    if hover_t > 0.01 {
        painter.rect_filled(
            rect.expand(4.0 * hover_t),
            10.0,
            with_alpha(palette.gold, 0.14 * hover_t),
        );
    }

    let fill = lerp_color(palette.bg_card, palette.bg_card_hover, hover_t);
    painter.rect_filled(rect, 8.0, fill);

    // Pen-and-ink portrait banner at top of card
    let img_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 10.0, rect.top() + 10.0),
        Pos2::new(rect.right() - 10.0, rect.top() + 136.0),
    );
    let uv = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0));
    let img_tint = if palette.is_dark {
        lerp_color(
            Color32::from_rgb(218, 204, 178),
            Color32::from_rgb(242, 232, 212),
            hover_t,
        )
    } else {
        Color32::WHITE
    };
    painter.image(portrait_tex, img_rect, uv, img_tint);
    painter.rect_stroke(
        img_rect,
        4.0,
        Stroke::new(1.0_f32, with_alpha(palette.gold, 0.45 + 0.4 * hover_t)),
        egui::StrokeKind::Inside,
    );

    draw_art_deco_frame(painter, rect, palette.gold, 0.55 + 0.45 * hover_t);

    // Roman numeral emblem overlapping bottom-center of portrait
    let emblem_center = Pos2::new(rect.center().x, img_rect.bottom());
    draw_diamond(
        painter,
        emblem_center,
        15.0 + 2.0 * hover_t,
        lerp_color(palette.bg_paper, palette.gold, hover_t * 0.85),
    );
    painter.text(
        emblem_center,
        egui::Align2::CENTER_CENTER,
        protagonist.roman(),
        font_serif_bold(13.5),
        if hover_t > 0.5 {
            Color32::from_rgb(20, 16, 14)
        } else {
            palette.gold
        },
    );

    painter.text(
        Pos2::new(rect.center().x, img_rect.bottom() + 20.0),
        egui::Align2::CENTER_TOP,
        protagonist.chapter_label(lang),
        font_ui(10.0),
        palette.gold,
    );

    painter.text(
        Pos2::new(rect.center().x, img_rect.bottom() + 36.0),
        egui::Align2::CENTER_TOP,
        protagonist.name(),
        font_serif_bold(20.0),
        palette.ink,
    );

    let tag_galley = painter.layout(
        protagonist.tagline(lang).to_string(),
        font_serif_italic(13.0),
        palette.ink_soft,
        (width - 34.0).max(120.0),
    );
    let tag_x = rect.center().x - tag_galley.size().x * 0.5;
    painter.galley(
        Pos2::new(tag_x, img_rect.bottom() + 66.0),
        tag_galley,
        Color32::TRANSPARENT,
    );

    let bar_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 20.0, rect.bottom() - 42.0),
        Pos2::new(rect.right() - 20.0, rect.bottom() - 36.0),
    );
    painter.rect_filled(bar_rect, 3.0, with_alpha(palette.border, 0.7));
    let frac = if total_endings > 0 {
        (discovered as f32) / (total_endings as f32)
    } else {
        0.0
    };
    if frac > 0.0 {
        let fill_w = (bar_rect.width() * frac).max(6.0);
        let filled = Rect::from_min_max(
            bar_rect.min,
            Pos2::new(bar_rect.left() + fill_w, bar_rect.max.y),
        );
        painter.rect_filled(filled, 3.0, palette.gold);
    }

    let stats_text = match lang {
        Language::PtBr => format!("{discovered}/{total_endings} finais · {total_scenes} cenas"),
        Language::En => format!("{discovered}/{total_endings} endings · {total_scenes} scenes"),
    };
    painter.text(
        Pos2::new(rect.left() + 20.0, rect.bottom() - 22.0),
        egui::Align2::LEFT_CENTER,
        stats_text,
        font_ui(11.0),
        palette.ink_muted,
    );

    let cta_col = lerp_color(palette.ink_soft, palette.gold, hover_t);
    painter.text(
        Pos2::new(rect.right() - 34.0 + 3.0 * hover_t, rect.bottom() - 22.0),
        egui::Align2::RIGHT_CENTER,
        lang.t("Iniciar", "Start"),
        font_ui(11.5),
        cta_col,
    );
    draw_icon(
        painter,
        Pos2::new(rect.right() - 24.0 + 3.0 * hover_t, rect.bottom() - 22.0),
        11.0,
        IconKind::ArrowRight,
        cta_col,
    );

    let pos = response
        .interact_pointer_pos()
        .unwrap_or_else(|| rect.center());
    (response.clicked(), pos)
}
