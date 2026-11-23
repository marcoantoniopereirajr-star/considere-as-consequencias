#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod art;
mod fx;
mod icons;
mod story;
mod theme;
mod tree;
mod widgets;

use app::ConsequencesApp;
use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "noto_serif_regular".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSerif-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        "noto_serif_bold".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSerif-Bold.ttf"
        ))),
    );
    fonts.font_data.insert(
        "noto_serif_italic".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSerif-Italic.ttf"
        ))),
    );
    fonts.font_data.insert(
        "noto_sans_medium".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSans-Medium.ttf"
        ))),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "noto_serif_regular".to_owned());

    fonts.families.insert(
        FontFamily::Name("serif_bold".into()),
        vec![
            "noto_serif_bold".to_owned(),
            "noto_serif_regular".to_owned(),
        ],
    );
    fonts.families.insert(
        FontFamily::Name("serif_italic".into()),
        vec![
            "noto_serif_italic".to_owned(),
            "noto_serif_regular".to_owned(),
        ],
    );
    fonts.families.insert(
        FontFamily::Name("ui_sans".into()),
        vec![
            "noto_sans_medium".to_owned(),
            "noto_serif_regular".to_owned(),
        ],
    );

    ctx.set_fonts(fonts);
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Considere as Consequências! (1930)")
            .with_inner_size([1200.0, 820.0])
            .with_min_inner_size([720.0, 560.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Considere as Consequências! (1930)",
        native_options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(ConsequencesApp::new(cc)))
        }),
    )
}
