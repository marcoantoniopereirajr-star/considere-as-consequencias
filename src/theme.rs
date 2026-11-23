use eframe::egui::{self, Color32};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeKind {
    NoirArtDeco,
    ClassicParchment,
    Sepia1930,
}

impl ThemeKind {
    pub const ALL: [ThemeKind; 3] = [
        ThemeKind::NoirArtDeco,
        ThemeKind::ClassicParchment,
        ThemeKind::Sepia1930,
    ];

    pub fn from_index(idx: usize) -> Self {
        Self::ALL[idx % Self::ALL.len()]
    }

    pub fn to_index(self) -> usize {
        match self {
            Self::NoirArtDeco => 0,
            Self::ClassicParchment => 1,
            Self::Sepia1930 => 2,
        }
    }

    pub fn next(self) -> Self {
        Self::from_index(self.to_index() + 1)
    }

    pub fn label(self, lang: crate::story::Language) -> &'static str {
        match (self, lang) {
            (Self::NoirArtDeco, crate::story::Language::PtBr) => "Noir Art Déco",
            (Self::ClassicParchment, crate::story::Language::PtBr) => "Pergaminho",
            (Self::Sepia1930, crate::story::Language::PtBr) => "Sépia 1930",
            (Self::NoirArtDeco, crate::story::Language::En) => "Art Deco Noir",
            (Self::ClassicParchment, crate::story::Language::En) => "Parchment",
            (Self::Sepia1930, crate::story::Language::En) => "1930 Sepia",
        }
    }

    pub fn palette(self) -> ThemePalette {
        match self {
            Self::NoirArtDeco => ThemePalette {
                bg_outer: Color32::from_rgb(14, 13, 16),
                bg_paper: Color32::from_rgb(24, 22, 26),
                bg_card: Color32::from_rgb(33, 30, 36),
                bg_card_hover: Color32::from_rgb(44, 39, 46),
                ink: Color32::from_rgb(240, 232, 218),
                ink_soft: Color32::from_rgb(198, 186, 166),
                ink_muted: Color32::from_rgb(140, 128, 112),
                gold: Color32::from_rgb(216, 174, 94),
                gold_dim: Color32::from_rgb(122, 96, 52),
                accent: Color32::from_rgb(198, 86, 98),
                accent_soft: Color32::from_rgba_premultiplied(198, 86, 98, 36),
                emerald: Color32::from_rgb(92, 176, 144),
                border: Color32::from_rgb(62, 54, 46),
                is_dark: true,
            },
            Self::ClassicParchment => ThemePalette {
                bg_outer: Color32::from_rgb(234, 226, 209),
                bg_paper: Color32::from_rgb(250, 245, 234),
                bg_card: Color32::from_rgb(243, 236, 220),
                bg_card_hover: Color32::from_rgb(236, 226, 204),
                ink: Color32::from_rgb(36, 30, 24),
                ink_soft: Color32::from_rgb(77, 66, 55),
                ink_muted: Color32::from_rgb(124, 108, 92),
                gold: Color32::from_rgb(154, 114, 50),
                gold_dim: Color32::from_rgb(196, 174, 136),
                accent: Color32::from_rgb(139, 38, 53),
                accent_soft: Color32::from_rgba_premultiplied(139, 38, 53, 26),
                emerald: Color32::from_rgb(42, 104, 80),
                border: Color32::from_rgb(212, 198, 174),
                is_dark: false,
            },
            Self::Sepia1930 => ThemePalette {
                bg_outer: Color32::from_rgb(218, 204, 176),
                bg_paper: Color32::from_rgb(238, 226, 202),
                bg_card: Color32::from_rgb(228, 212, 182),
                bg_card_hover: Color32::from_rgb(218, 198, 164),
                ink: Color32::from_rgb(44, 33, 22),
                ink_soft: Color32::from_rgb(86, 67, 48),
                ink_muted: Color32::from_rgb(128, 104, 80),
                gold: Color32::from_rgb(142, 96, 34),
                gold_dim: Color32::from_rgb(184, 156, 114),
                accent: Color32::from_rgb(126, 38, 38),
                accent_soft: Color32::from_rgba_premultiplied(126, 38, 38, 28),
                emerald: Color32::from_rgb(42, 86, 66),
                border: Color32::from_rgb(194, 174, 140),
                is_dark: false,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub bg_outer: Color32,
    pub bg_paper: Color32,
    pub bg_card: Color32,
    pub bg_card_hover: Color32,
    pub ink: Color32,
    pub ink_soft: Color32,
    pub ink_muted: Color32,
    pub gold: Color32,
    pub gold_dim: Color32,
    pub accent: Color32,
    pub accent_soft: Color32,
    pub emerald: Color32,
    pub border: Color32,
    pub is_dark: bool,
}

impl ThemePalette {
    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            bg_outer: lerp_color(a.bg_outer, b.bg_outer, t),
            bg_paper: lerp_color(a.bg_paper, b.bg_paper, t),
            bg_card: lerp_color(a.bg_card, b.bg_card, t),
            bg_card_hover: lerp_color(a.bg_card_hover, b.bg_card_hover, t),
            ink: lerp_color(a.ink, b.ink, t),
            ink_soft: lerp_color(a.ink_soft, b.ink_soft, t),
            ink_muted: lerp_color(a.ink_muted, b.ink_muted, t),
            gold: lerp_color(a.gold, b.gold, t),
            gold_dim: lerp_color(a.gold_dim, b.gold_dim, t),
            accent: lerp_color(a.accent, b.accent, t),
            accent_soft: lerp_color(a.accent_soft, b.accent_soft, t),
            emerald: lerp_color(a.emerald, b.emerald, t),
            border: lerp_color(a.border, b.border, t),
            is_dark: if t < 0.5 { a.is_dark } else { b.is_dark },
        }
    }

    pub fn trait_color(&self, trait_key: &str) -> Color32 {
        match (trait_key, self.is_dark) {
            ("coracao", true) => Color32::from_rgb(224, 108, 124),
            ("coracao", false) => Color32::from_rgb(168, 44, 62),
            ("razao", true) => Color32::from_rgb(112, 168, 214),
            ("razao", false) => Color32::from_rgb(44, 96, 148),
            ("ousadia", true) => Color32::from_rgb(232, 174, 82),
            ("ousadia", false) => Color32::from_rgb(174, 108, 28),
            ("dever", true) => Color32::from_rgb(112, 186, 152),
            ("dever", false) => Color32::from_rgb(42, 112, 82),
            ("ambicao", true) => Color32::from_rgb(186, 134, 214),
            ("ambicao", false) => Color32::from_rgb(114, 58, 146),
            ("integridade", true) => Color32::from_rgb(216, 192, 124),
            ("integridade", false) => Color32::from_rgb(138, 106, 42),
            _ => self.gold,
        }
    }

    pub fn ending_tone_color(&self, tone: &str) -> Color32 {
        match (tone, self.is_dark) {
            ("triunfo", true) => Color32::from_rgb(234, 190, 96),
            ("triunfo", false) => Color32::from_rgb(168, 118, 28),
            ("sereno", true) => Color32::from_rgb(108, 188, 156),
            ("sereno", false) => Color32::from_rgb(38, 114, 86),
            ("agridoce", true) => Color32::from_rgb(212, 148, 98),
            ("agridoce", false) => Color32::from_rgb(162, 92, 42),
            ("ironico", true) => Color32::from_rgb(178, 142, 214),
            ("ironico", false) => Color32::from_rgb(106, 64, 148),
            ("tragedia", true) => Color32::from_rgb(218, 92, 96),
            ("tragedia", false) => Color32::from_rgb(158, 36, 42),
            _ => self.gold,
        }
    }

    pub fn apply_to_ctx(&self, ctx: &egui::Context) {
        let mut visuals = if self.is_dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        visuals.panel_fill = self.bg_outer;
        visuals.window_fill = self.bg_paper;
        visuals.extreme_bg_color = self.bg_outer;
        visuals.faint_bg_color = self.bg_card;
        visuals.override_text_color = Some(self.ink);
        visuals.widgets.noninteractive.bg_fill = self.bg_paper;
        visuals.widgets.noninteractive.fg_stroke.color = self.ink;
        visuals.widgets.inactive.bg_fill = self.bg_card;
        visuals.widgets.hovered.bg_fill = self.bg_card_hover;
        visuals.selection.bg_fill = self.accent_soft;
        visuals.selection.stroke.color = self.gold;
        ctx.set_visuals(visuals);
    }
}

pub fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let f = |x: u8, y: u8| -> u8 { ((x as f32) + ((y as f32) - (x as f32)) * t).round().clamp(0.0, 255.0) as u8 };
    Color32::from_rgba_unmultiplied(
        f(a.r(), b.r()),
        f(a.g(), b.g()),
        f(a.b(), b.b()),
        f(a.a(), b.a()),
    )
}

pub fn with_alpha(c: Color32, alpha_0_to_1: f32) -> Color32 {
    let a = ((c.a() as f32) * alpha_0_to_1.clamp(0.0, 1.0)).round() as u8;
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
}
