use eframe::egui::{self, Color32, Painter, Pos2, Rect, Shape, Stroke, Vec2};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconKind {
    Starburst,
    HalfMoon,
    Trophy,
    Compass,
    Play,
    Book,
    ChevronUp,
    Reset,
    ArrowLeft,
    ArrowRight,
    Heart,
    Diamond,
    Scales,
    Crown,
    Shield,
    Check,
    Close,
    Trail,
}

pub fn icon_for_trait(trait_key: &str) -> IconKind {
    match trait_key {
        "coracao" => IconKind::Heart,
        "razao" => IconKind::Diamond,
        "ousadia" => IconKind::Starburst,
        "dever" => IconKind::Scales,
        "ambicao" => IconKind::Crown,
        "integridade" => IconKind::Shield,
        _ => IconKind::Diamond,
    }
}

/// Allocates a small square in a horizontal UI layout and paints the vector icon centered inside it.
pub fn paint_icon_inline(ui: &mut egui::Ui, size: f32, kind: IconKind, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
    draw_icon(ui.painter(), rect.center(), size, kind, color);
}

/// Draws a crisp Art Deco geometric vector icon centered at `c` with bounding diameter `size`.
pub fn draw_icon(painter: &Painter, c: Pos2, size: f32, kind: IconKind, color: Color32) {
    let r = size * 0.44;
    let sw = (size * 0.11).clamp(1.2, 2.2);
    let stroke = Stroke::new(sw, color);

    match kind {
        IconKind::Starburst => {
            // 4-point Art Deco star
            let inner = r * 0.32;
            let pts = vec![
                Pos2::new(c.x, c.y - r),
                Pos2::new(c.x + inner, c.y - inner),
                Pos2::new(c.x + r, c.y),
                Pos2::new(c.x + inner, c.y + inner),
                Pos2::new(c.x, c.y + r),
                Pos2::new(c.x - inner, c.y + inner),
                Pos2::new(c.x - r, c.y),
                Pos2::new(c.x - inner, c.y - inner),
            ];
            painter.add(Shape::convex_polygon(pts, color, Stroke::NONE));
        }

        IconKind::HalfMoon => {
            painter.circle_stroke(c, r, stroke);
            // Filled left semicircle
            let steps = 12;
            let mut pts = Vec::with_capacity(steps + 1);
            for i in 0..=steps {
                let a = PI * 0.5 + (i as f32 / steps as f32) * PI;
                pts.push(Pos2::new(c.x + a.cos() * r * 0.92, c.y + a.sin() * r * 0.92));
            }
            painter.add(Shape::convex_polygon(pts, color, Stroke::NONE));
        }

        IconKind::Trophy => {
            // Art Deco chalice / cup
            let cup = vec![
                Pos2::new(c.x - r * 0.72, c.y - r * 0.80),
                Pos2::new(c.x + r * 0.72, c.y - r * 0.80),
                Pos2::new(c.x + r * 0.45, c.y + r * 0.15),
                Pos2::new(c.x - r * 0.45, c.y + r * 0.15),
            ];
            painter.add(Shape::convex_polygon(cup, color, Stroke::NONE));
            // Stem & base
            painter.line_segment(
                [Pos2::new(c.x, c.y + r * 0.15), Pos2::new(c.x, c.y + r * 0.72)],
                Stroke::new(sw * 1.15, color),
            );
            painter.line_segment(
                [
                    Pos2::new(c.x - r * 0.55, c.y + r * 0.78),
                    Pos2::new(c.x + r * 0.55, c.y + r * 0.78),
                ],
                Stroke::new(sw * 1.2, color),
            );
            // Side handles
            painter.line_segment(
                [
                    Pos2::new(c.x - r * 0.70, c.y - r * 0.62),
                    Pos2::new(c.x - r * 0.95, c.y - r * 0.35),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(c.x - r * 0.95, c.y - r * 0.35),
                    Pos2::new(c.x - r * 0.58, c.y - r * 0.12),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(c.x + r * 0.70, c.y - r * 0.62),
                    Pos2::new(c.x + r * 0.95, c.y - r * 0.35),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(c.x + r * 0.95, c.y - r * 0.35),
                    Pos2::new(c.x + r * 0.58, c.y - r * 0.12),
                ],
                stroke,
            );
        }

        IconKind::Compass => {
            painter.circle_stroke(c, r, stroke);
            let needle = vec![
                Pos2::new(c.x, c.y - r * 0.78),
                Pos2::new(c.x + r * 0.30, c.y),
                Pos2::new(c.x, c.y + r * 0.78),
                Pos2::new(c.x - r * 0.30, c.y),
            ];
            painter.add(Shape::convex_polygon(needle, color, Stroke::NONE));
        }

        IconKind::Play => {
            let pts = vec![
                Pos2::new(c.x - r * 0.55, c.y - r * 0.80),
                Pos2::new(c.x + r * 0.85, c.y),
                Pos2::new(c.x - r * 0.55, c.y + r * 0.80),
            ];
            painter.add(Shape::convex_polygon(pts, color, Stroke::NONE));
        }

        IconKind::Book => {
            // Open book outline
            let left_page = vec![
                Pos2::new(c.x, c.y - r * 0.58),
                Pos2::new(c.x - r * 0.92, c.y - r * 0.76),
                Pos2::new(c.x - r * 0.92, c.y + r * 0.62),
                Pos2::new(c.x, c.y + r * 0.78),
            ];
            let right_page = vec![
                Pos2::new(c.x, c.y - r * 0.58),
                Pos2::new(c.x + r * 0.92, c.y - r * 0.76),
                Pos2::new(c.x + r * 0.92, c.y + r * 0.62),
                Pos2::new(c.x, c.y + r * 0.78),
            ];
            painter.add(Shape::closed_line(left_page, stroke));
            painter.add(Shape::closed_line(right_page, stroke));
            // Center spine
            painter.line_segment(
                [Pos2::new(c.x, c.y - r * 0.58), Pos2::new(c.x, c.y + r * 0.78)],
                stroke,
            );
        }

        IconKind::ChevronUp => {
            painter.line_segment(
                [
                    Pos2::new(c.x - r * 0.75, c.y + r * 0.32),
                    Pos2::new(c.x, c.y - r * 0.45),
                ],
                Stroke::new(sw * 1.2, color),
            );
            painter.line_segment(
                [
                    Pos2::new(c.x, c.y - r * 0.45),
                    Pos2::new(c.x + r * 0.75, c.y + r * 0.32),
                ],
                Stroke::new(sw * 1.2, color),
            );
        }

        IconKind::Reset => {
            // Circular arc (~275 deg) + arrowhead
            let steps = 16;
            let start_a = -PI * 0.30;
            let sweep = PI * 1.55;
            let mut pts = Vec::with_capacity(steps + 1);
            for i in 0..=steps {
                let a = start_a + (i as f32 / steps as f32) * sweep;
                pts.push(Pos2::new(c.x + a.cos() * r * 0.84, c.y + a.sin() * r * 0.84));
            }
            painter.add(Shape::line(pts, stroke));
            // Arrowhead at top-right
            let tip = Pos2::new(
                c.x + start_a.cos() * r * 0.84,
                c.y + start_a.sin() * r * 0.84,
            );
            let head = vec![
                tip + Vec2::new(r * 0.22, r * 0.14),
                tip + Vec2::new(-r * 0.42, -r * 0.08),
                tip + Vec2::new(r * 0.05, -r * 0.46),
            ];
            painter.add(Shape::convex_polygon(head, color, Stroke::NONE));
        }

        IconKind::ArrowLeft => {
            painter.line_segment(
                [Pos2::new(c.x + r * 0.80, c.y), Pos2::new(c.x - r * 0.55, c.y)],
                stroke,
            );
            let head = vec![
                Pos2::new(c.x - r * 0.90, c.y),
                Pos2::new(c.x - r * 0.22, c.y - r * 0.56),
                Pos2::new(c.x - r * 0.22, c.y + r * 0.56),
            ];
            painter.add(Shape::convex_polygon(head, color, Stroke::NONE));
        }

        IconKind::ArrowRight => {
            painter.line_segment(
                [Pos2::new(c.x - r * 0.80, c.y), Pos2::new(c.x + r * 0.55, c.y)],
                stroke,
            );
            let head = vec![
                Pos2::new(c.x + r * 0.90, c.y),
                Pos2::new(c.x + r * 0.22, c.y - r * 0.56),
                Pos2::new(c.x + r * 0.22, c.y + r * 0.56),
            ];
            painter.add(Shape::convex_polygon(head, color, Stroke::NONE));
        }

        IconKind::Heart => {
            // Geometric Art Deco heart (two lobes + bottom diamond tip)
            let lobe_r = r * 0.44;
            painter.circle_filled(Pos2::new(c.x - r * 0.40, c.y - r * 0.22), lobe_r, color);
            painter.circle_filled(Pos2::new(c.x + r * 0.40, c.y - r * 0.22), lobe_r, color);
            let tri = vec![
                Pos2::new(c.x - r * 0.82, c.y - r * 0.08),
                Pos2::new(c.x + r * 0.82, c.y - r * 0.08),
                Pos2::new(c.x, c.y + r * 0.88),
            ];
            painter.add(Shape::convex_polygon(tri, color, Stroke::NONE));
        }

        IconKind::Diamond => {
            let pts = vec![
                Pos2::new(c.x, c.y - r * 0.92),
                Pos2::new(c.x + r * 0.72, c.y),
                Pos2::new(c.x, c.y + r * 0.92),
                Pos2::new(c.x - r * 0.72, c.y),
            ];
            painter.add(Shape::convex_polygon(pts, color, Stroke::NONE));
        }

        IconKind::Scales => {
            // Balance scales for Dever
            painter.line_segment(
                [Pos2::new(c.x, c.y - r * 0.80), Pos2::new(c.x, c.y + r * 0.78)],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(c.x - r * 0.48, c.y + r * 0.78),
                    Pos2::new(c.x + r * 0.48, c.y + r * 0.78),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(c.x - r * 0.85, c.y - r * 0.52),
                    Pos2::new(c.x + r * 0.85, c.y - r * 0.52),
                ],
                stroke,
            );
            // Left pan
            painter.line_segment(
                [
                    Pos2::new(c.x - r * 0.82, c.y - r * 0.52),
                    Pos2::new(c.x - r * 0.98, c.y + r * 0.14),
                ],
                Stroke::new(sw * 0.8, color),
            );
            painter.line_segment(
                [
                    Pos2::new(c.x - r * 0.82, c.y - r * 0.52),
                    Pos2::new(c.x - r * 0.52, c.y + r * 0.14),
                ],
                Stroke::new(sw * 0.8, color),
            );
            painter.line_segment(
                [
                    Pos2::new(c.x - r * 1.00, c.y + r * 0.16),
                    Pos2::new(c.x - r * 0.50, c.y + r * 0.16),
                ],
                stroke,
            );
            // Right pan
            painter.line_segment(
                [
                    Pos2::new(c.x + r * 0.82, c.y - r * 0.52),
                    Pos2::new(c.x + r * 0.52, c.y + r * 0.14),
                ],
                Stroke::new(sw * 0.8, color),
            );
            painter.line_segment(
                [
                    Pos2::new(c.x + r * 0.82, c.y - r * 0.52),
                    Pos2::new(c.x + r * 0.98, c.y + r * 0.14),
                ],
                Stroke::new(sw * 0.8, color),
            );
            painter.line_segment(
                [
                    Pos2::new(c.x + r * 0.50, c.y + r * 0.16),
                    Pos2::new(c.x + r * 1.00, c.y + r * 0.16),
                ],
                stroke,
            );
        }

        IconKind::Crown => {
            // Art Deco 3-peak crown
            let base = Rect::from_min_max(
                Pos2::new(c.x - r * 0.78, c.y + r * 0.25),
                Pos2::new(c.x + r * 0.78, c.y + r * 0.65),
            );
            painter.rect_filled(base, 1.5, color);
            let peaks = [
                (c.x - r * 0.78, c.x - r * 0.22, c.y - r * 0.55),
                (c.x - r * 0.32, c.x + r * 0.32, c.y - r * 0.82),
                (c.x + r * 0.22, c.x + r * 0.78, c.y - r * 0.55),
            ];
            for (x0, x1, top_y) in peaks {
                let tri = vec![
                    Pos2::new((x0 + x1) * 0.5, top_y),
                    Pos2::new(x1, c.y + r * 0.28),
                    Pos2::new(x0, c.y + r * 0.28),
                ];
                painter.add(Shape::convex_polygon(tri, color, Stroke::NONE));
            }
        }

        IconKind::Shield => {
            let pts = vec![
                Pos2::new(c.x, c.y - r * 0.88),
                Pos2::new(c.x + r * 0.78, c.y - r * 0.52),
                Pos2::new(c.x + r * 0.64, c.y + r * 0.25),
                Pos2::new(c.x, c.y + r * 0.90),
                Pos2::new(c.x - r * 0.64, c.y + r * 0.25),
                Pos2::new(c.x - r * 0.78, c.y - r * 0.52),
            ];
            painter.add(Shape::convex_polygon(pts, color, Stroke::NONE));
        }

        IconKind::Check => {
            painter.line_segment(
                [
                    Pos2::new(c.x - r * 0.72, c.y + r * 0.05),
                    Pos2::new(c.x - r * 0.16, c.y + r * 0.62),
                ],
                Stroke::new(sw * 1.25, color),
            );
            painter.line_segment(
                [
                    Pos2::new(c.x - r * 0.16, c.y + r * 0.62),
                    Pos2::new(c.x + r * 0.80, c.y - r * 0.54),
                ],
                Stroke::new(sw * 1.25, color),
            );
        }

        IconKind::Close => {
            let d = r * 0.64;
            painter.line_segment(
                [Pos2::new(c.x - d, c.y - d), Pos2::new(c.x + d, c.y + d)],
                Stroke::new(sw * 1.2, color),
            );
            painter.line_segment(
                [Pos2::new(c.x + d, c.y - d), Pos2::new(c.x - d, c.y + d)],
                Stroke::new(sw * 1.2, color),
            );
        }

        IconKind::Trail => {
            // Vertical path with 3 connected nodes
            painter.line_segment(
                [Pos2::new(c.x, c.y - r * 0.72), Pos2::new(c.x, c.y + r * 0.72)],
                stroke,
            );
            painter.circle_filled(Pos2::new(c.x, c.y - r * 0.72), r * 0.26, color);
            painter.circle_filled(c, r * 0.22, color);
            painter.circle_filled(Pos2::new(c.x, c.y + r * 0.72), r * 0.26, color);
        }
    }
}
