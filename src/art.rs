use crate::fx::draw_art_deco_frame;
use crate::story::{Language, Protagonist, StoryNode};
use crate::theme::{with_alpha, ThemePalette};
use eframe::egui::{self, Color32, ColorImage, Pos2, Rect, TextureHandle, TextureOptions, Vec2};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtKind {
    Helen,
    Jed,
    Saunders,
    SceneCrossroads,
    SceneJourney,
    SceneEnding,
}

impl ArtKind {
    pub fn for_protagonist(p: Protagonist) -> Self {
        match p {
            Protagonist::Helen => Self::Helen,
            Protagonist::Jed => Self::Jed,
            Protagonist::Saunders => Self::Saunders,
        }
    }

    pub fn for_node(node: &StoryNode) -> Self {
        let p = Protagonist::from_node_id(&node.id);
        if node.id.ends_with("-0") {
            return Self::for_protagonist(p);
        }

        let hay = format!(
            "{} {}",
            node.title.to_lowercase(),
            node.paragraphs
                .first()
                .map(|s| s.to_lowercase())
                .unwrap_or_default()
        );

        if node.is_ending {
            if hay.contains("viagem")
                || hay.contains("navio")
                || hay.contains("europa")
                || hay.contains("trem")
                || hay.contains("paris")
                || hay.contains("mar")
            {
                return Self::SceneJourney;
            }
            return Self::SceneEnding;
        }

        if hay.contains("viagem")
            || hay.contains("navio")
            || hay.contains("trem")
            || hay.contains("europa")
            || hay.contains("verão")
            || hay.contains("praia")
            || hay.contains("barco")
            || hay.contains("partir")
        {
            return Self::SceneJourney;
        }

        if hay.contains("baile")
            || hay.contains("festa")
            || hay.contains("cidade")
            || hay.contains("teatro")
            || hay.contains("dinheiro")
            || hay.contains("negócio")
            || hay.contains("casamento")
            || hay.contains("fortuna")
            || hay.contains("sociedade")
        {
            return Self::SceneCrossroads;
        }

        // Alternate smoothly based on scene number so adjacent scenes feel varied
        let num = node
            .id
            .split('-')
            .nth(1)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        match num % 3 {
            0 => Self::for_protagonist(p),
            1 => Self::SceneCrossroads,
            _ => Self::SceneJourney,
        }
    }

    fn bytes(self) -> &'static [u8] {
        match self {
            Self::Helen => include_bytes!("../assets/art/helen.jpg"),
            Self::Jed => include_bytes!("../assets/art/jed.jpg"),
            Self::Saunders => include_bytes!("../assets/art/saunders.jpg"),
            Self::SceneCrossroads => include_bytes!("../assets/art/scene_crossroads.jpg"),
            Self::SceneJourney => include_bytes!("../assets/art/scene_journey.jpg"),
            Self::SceneEnding => include_bytes!("../assets/art/scene_ending.jpg"),
        }
    }

    fn tex_name(self) -> &'static str {
        match self {
            Self::Helen => "art_helen",
            Self::Jed => "art_jed",
            Self::Saunders => "art_saunders",
            Self::SceneCrossroads => "art_crossroads",
            Self::SceneJourney => "art_journey",
            Self::SceneEnding => "art_ending",
        }
    }

    pub fn caption(self, lang: Language) -> &'static str {
        match (self, lang) {
            (Self::Helen, Language::PtBr) => "Ilustração a bico de pena · Helen Rogers",
            (Self::Jed, Language::PtBr) => "Ilustração a bico de pena · Jed Harringdale",
            (Self::Saunders, Language::PtBr) => "Ilustração a bico de pena · Saunders",
            (Self::SceneCrossroads, Language::PtBr) => {
                "Ilustração a bico de pena · Encruzilhada da Sociedade (1930)"
            }
            (Self::SceneJourney, Language::PtBr) => {
                "Ilustração a bico de pena · Novos Horizontes (1930)"
            }
            (Self::SceneEnding, Language::PtBr) => {
                "Ilustração a bico de pena · O Peso das Consequências"
            }
            (Self::Helen, Language::En) => "Pen-and-ink illustration · Helen Rogers",
            (Self::Jed, Language::En) => "Pen-and-ink illustration · Jed Harringdale",
            (Self::Saunders, Language::En) => "Pen-and-ink illustration · Saunders",
            (Self::SceneCrossroads, Language::En) => {
                "Pen-and-ink illustration · Society Crossroads (1930)"
            }
            (Self::SceneJourney, Language::En) => {
                "Pen-and-ink illustration · New Horizons (1930)"
            }
            (Self::SceneEnding, Language::En) => {
                "Pen-and-ink illustration · The Weight of Consequences"
            }
        }
    }
}

#[derive(Default)]
pub struct ArtGallery {
    textures: HashMap<ArtKind, TextureHandle>,
}

impl ArtGallery {
    pub fn texture(&mut self, ctx: &egui::Context, kind: ArtKind) -> &TextureHandle {
        self.textures.entry(kind).or_insert_with(|| {
            let decoded = image::load_from_memory(kind.bytes())
                .expect("Falha ao decodificar ilustração embutida")
                .to_rgba8();
            let size = [decoded.width() as usize, decoded.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, decoded.as_raw());
            ctx.load_texture(kind.tex_name(), color_image, TextureOptions::LINEAR)
        })
    }

    /// Paints an Art Deco framed pen-and-ink bookplate illustration with fade-in and theme tinting.
    pub fn paint_bookplate(
        &mut self,
        ui: &mut egui::Ui,
        kind: ArtKind,
        max_width: f32,
        max_height: f32,
        palette: &ThemePalette,
        alpha: f32,
    ) {
        let tex = self.texture(ui.ctx(), kind).clone();
        let tex_sz = tex.size_vec2();
        let aspect = if tex_sz.y > 0.0 { tex_sz.x / tex_sz.y } else { 16.0 / 9.0 };

        let mut w = max_width;
        let mut h = w / aspect;
        if h > max_height {
            h = max_height;
            w = h * aspect;
        }

        let avail = ui.available_width();
        let pad_x = ((avail - w) * 0.5).max(0.0);

        ui.horizontal(|ui| {
            ui.add_space(pad_x);
            let (rect, _) = ui.allocate_exact_size(Vec2::new(w, h), egui::Sense::hover());
            let painter = ui.painter();

            // Warm vintage paper tint so the ink etching harmonizes in both dark & light themes
            let tint = if palette.is_dark {
                with_alpha(Color32::from_rgb(228, 214, 186), alpha)
            } else {
                with_alpha(Color32::WHITE, alpha)
            };

            let uv = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0));
            painter.image(tex.id(), rect, uv, tint);

            // Subtle dark vignette edge in Noir theme so it melts smoothly into the dark card
            if palette.is_dark {
                painter.rect_stroke(
                    rect,
                    4.0,
                    egui::Stroke::new(3.0_f32, with_alpha(palette.bg_paper, 0.65 * alpha)),
                    egui::StrokeKind::Inside,
                );
            }

            draw_art_deco_frame(painter, rect, palette.gold, 0.85 * alpha);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::BookData;

    const ALL_ART: [ArtKind; 6] = [
        ArtKind::Helen,
        ArtKind::Jed,
        ArtKind::Saunders,
        ArtKind::SceneCrossroads,
        ArtKind::SceneJourney,
        ArtKind::SceneEnding,
    ];

    #[test]
    fn all_embedded_jpegs_decode_with_nonzero_dimensions() {
        for kind in ALL_ART {
            let img = image::load_from_memory(kind.bytes())
                .unwrap_or_else(|e| panic!("Failed to decode {:?}: {e}", kind));
            assert!(
                img.width() > 0 && img.height() > 0,
                "Expected non-zero dimensions for {:?}, got {}x{}",
                kind,
                img.width(),
                img.height()
            );
            assert!(!kind.tex_name().is_empty());
            assert!(!kind.caption(Language::PtBr).is_empty());
            assert!(!kind.caption(Language::En).is_empty());
        }
    }

    #[test]
    fn art_kind_for_node_covers_all_story_nodes() {
        let book = BookData::load();
        assert_eq!(book.nodes.len(), 86);

        assert_eq!(ArtKind::for_node(&book.nodes["H-0"]), ArtKind::Helen);
        assert_eq!(ArtKind::for_node(&book.nodes["J-0"]), ArtKind::Jed);
        assert_eq!(ArtKind::for_node(&book.nodes["S-0"]), ArtKind::Saunders);

        for node in book.nodes.values() {
            let kind = ArtKind::for_node(node);
            assert!(ALL_ART.contains(&kind));
        }
    }
}
