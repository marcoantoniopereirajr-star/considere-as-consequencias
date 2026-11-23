use crate::theme::with_alpha;
use eframe::egui::{self, Color32, Painter, Pos2, Rect, Stroke, Vec2};

pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

#[derive(Clone, Copy)]
pub enum ParticleShape {
    Diamond,
    Circle,
    Spark,
}

#[derive(Clone)]
pub struct Particle {
    pub pos: Pos2,
    pub vel: Vec2,
    pub gravity: f32,
    pub drag: f32,
    pub size: f32,
    pub age: f32,
    pub max_age: f32,
    pub color: Color32,
    pub shape: ParticleShape,
}

#[derive(Clone)]
pub struct Ripple {
    pub origin: Pos2,
    pub radius: f32,
    pub max_radius: f32,
    pub age: f32,
    pub max_age: f32,
    pub color: Color32,
}

struct AmbientMote {
    x_norm: f32,
    y_norm: f32,
    speed_y: f32,
    drift_phase: f32,
    size: f32,
    alpha: f32,
}

pub struct FxEngine {
    particles: Vec<Particle>,
    ripples: Vec<Ripple>,
    motes: Vec<AmbientMote>,
    rng_state: u64,
    pub time: f32,
}

impl FxEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            particles: Vec::with_capacity(256),
            ripples: Vec::with_capacity(16),
            motes: Vec::with_capacity(34),
            rng_state: 0x1930_CAFE_BABE_1337,
            time: 0.0,
        };
        for _ in 0..34 {
            let x_norm = engine.next_f32();
            let y_norm = engine.next_f32();
            let speed_y = 0.008 + engine.next_f32() * 0.018;
            let drift_phase = engine.next_f32() * std::f32::consts::TAU;
            let size = 1.4 + engine.next_f32() * 2.4;
            let alpha = 0.10 + engine.next_f32() * 0.22;
            engine.motes.push(AmbientMote {
                x_norm,
                y_norm,
                speed_y,
                drift_phase,
                size,
                alpha,
            });
        }
        engine
    }

    fn next_u32(&mut self) -> u32 {
        // Fast xorshift64* PRNG (no external crate needed)
        let mut x = self.rng_state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.rng_state = x;
        ((x.wrapping_mul(0x2545_F491_4F6C_DD1D)) >> 32) as u32
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u32() as f32) / (u32::MAX as f32)
    }

    pub fn spawn_choice_burst(&mut self, origin: Pos2, primary: Color32, gold: Color32) {
        self.ripples.push(Ripple {
            origin,
            radius: 10.0,
            max_radius: 170.0,
            age: 0.0,
            max_age: 0.55,
            color: primary,
        });
        self.ripples.push(Ripple {
            origin,
            radius: 4.0,
            max_radius: 115.0,
            age: 0.0,
            max_age: 0.42,
            color: gold,
        });

        for i in 0..28 {
            let angle = (i as f32 / 28.0) * std::f32::consts::TAU + (self.next_f32() - 0.5) * 0.35;
            let speed = 75.0 + self.next_f32() * 195.0;
            let color = if i % 3 == 0 { gold } else { primary };
            let shape = match i % 3 {
                0 => ParticleShape::Diamond,
                1 => ParticleShape::Spark,
                _ => ParticleShape::Circle,
            };
            let size = 2.5 + self.next_f32() * 3.8;
            let max_age = 0.55 + self.next_f32() * 0.45;
            self.particles.push(Particle {
                pos: origin,
                vel: Vec2::new(angle.cos() * speed, angle.sin() * speed - 25.0),
                gravity: 160.0,
                drag: 0.94,
                size,
                age: 0.0,
                max_age,
                color,
                shape,
            });
        }
    }

    pub fn spawn_ending_celebration(&mut self, rect: Rect, color: Color32, gold: Color32, tone: &str) {
        let count = match tone {
            "triunfo" => 65,
            "tragedia" => 40,
            _ => 48,
        };
        let center = rect.center();
        self.ripples.push(Ripple {
            origin: center,
            radius: 20.0,
            max_radius: rect.width().min(rect.height()) * 0.45,
            age: 0.0,
            max_age: 0.95,
            color,
        });

        for i in 0..count {
            let x = rect.left() + self.next_f32() * rect.width();
            let y = if tone == "tragedia" {
                rect.top() + self.next_f32() * rect.height() * 0.3
            } else {
                center.y + (self.next_f32() - 0.5) * 140.0
            };
            let vx = (self.next_f32() - 0.5) * 220.0;
            let vy = if tone == "tragedia" {
                30.0 + self.next_f32() * 80.0
            } else {
                -110.0 - self.next_f32() * 210.0
            };
            let c = if i % 2 == 0 { color } else { gold };
            let shape = if i % 2 == 0 {
                ParticleShape::Diamond
            } else {
                ParticleShape::Spark
            };
            let size = 3.0 + self.next_f32() * 4.2;
            let max_age = 1.2 + self.next_f32() * 1.1;
            self.particles.push(Particle {
                pos: Pos2::new(x, y),
                vel: Vec2::new(vx, vy),
                gravity: if tone == "sereno" { 35.0 } else { 150.0 },
                drag: 0.97,
                size,
                age: 0.0,
                max_age,
                color: c,
                shape,
            });
        }
    }

    pub fn has_active_bursts(&self) -> bool {
        !self.particles.is_empty() || !self.ripples.is_empty()
    }

    pub fn update(&mut self, dt: f32) {
        let dt = dt.min(0.05);
        self.time += dt;

        for m in &mut self.motes {
            m.y_norm -= m.speed_y * dt;
            if m.y_norm < -0.04 {
                m.y_norm = 1.04;
            }
        }

        for p in &mut self.particles {
            p.age += dt;
            p.vel.y += p.gravity * dt;
            let damp = p.drag.powf(dt * 60.0);
            p.vel *= damp;
            p.pos += p.vel * dt;
        }
        self.particles.retain(|p| p.age < p.max_age);

        for r in &mut self.ripples {
            r.age += dt;
            let t = ease_out_cubic(r.age / r.max_age);
            r.radius = r.max_radius * t;
        }
        self.ripples.retain(|r| r.age < r.max_age);
    }

    pub fn draw_ambient(&self, painter: &Painter, rect: Rect, gold: Color32) {
        for m in &self.motes {
            let drift = (self.time * 0.7 + m.drift_phase).sin() * 18.0;
            let x = rect.left() + m.x_norm * rect.width() + drift;
            let y = rect.top() + m.y_norm * rect.height();
            let twinkle = 0.65 + 0.35 * (self.time * 1.6 + m.drift_phase).sin();
            let c = with_alpha(gold, m.alpha * twinkle);
            draw_diamond(painter, Pos2::new(x, y), m.size, c);
        }
    }

    pub fn draw_foreground(&self, painter: &Painter) {
        for r in &self.ripples {
            let progress = (r.age / r.max_age).clamp(0.0, 1.0);
            let alpha = (1.0 - progress).powi(2) * 0.55;
            let stroke_w = 2.4 * (1.0 - progress) + 0.6;
            painter.circle_stroke(r.origin, r.radius, Stroke::new(stroke_w, with_alpha(r.color, alpha)));
        }

        for p in &self.particles {
            let progress = (p.age / p.max_age).clamp(0.0, 1.0);
            let alpha = (1.0 - progress).powf(1.4);
            let c = with_alpha(p.color, alpha);
            let sz = p.size * (1.0 - progress * 0.35);
            match p.shape {
                ParticleShape::Diamond => draw_diamond(painter, p.pos, sz, c),
                ParticleShape::Circle => {
                    painter.circle_filled(p.pos, sz * 0.65, c);
                }
                ParticleShape::Spark => {
                    let tail = p.pos - p.vel.normalized() * (sz * 2.2);
                    painter.line_segment([p.pos, tail], Stroke::new(1.6_f32, c));
                    painter.circle_filled(p.pos, sz * 0.45, c);
                }
            }
        }
    }
}

pub fn draw_diamond(painter: &Painter, center: Pos2, radius: f32, color: Color32) {
    let pts = vec![
        Pos2::new(center.x, center.y - radius),
        Pos2::new(center.x + radius * 0.72, center.y),
        Pos2::new(center.x, center.y + radius),
        Pos2::new(center.x - radius * 0.72, center.y),
    ];
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

/// Draws crisp 1930s Art Deco double-line geometric corner brackets around a card or page.
pub fn draw_art_deco_frame(painter: &Painter, rect: Rect, gold: Color32, alpha: f32) {
    let outer = with_alpha(gold, 0.34 * alpha);
    let inner = with_alpha(gold, 0.16 * alpha);
    let corner = with_alpha(gold, 0.72 * alpha);

    painter.rect_stroke(rect, 4.0, Stroke::new(1.0_f32, outer), egui::StrokeKind::Inside);
    painter.rect_stroke(rect.shrink(5.0), 2.0, Stroke::new(0.8_f32, inner), egui::StrokeKind::Inside);

    let len = 18.0_f32.min(rect.width() * 0.15).min(rect.height() * 0.15);
    let st = Stroke::new(1.8_f32, corner);

    // Four Art Deco stepped corners
    let corners = [
        (rect.left_top(), 1.0, 1.0),
        (rect.right_top(), -1.0, 1.0),
        (rect.left_bottom(), 1.0, -1.0),
        (rect.right_bottom(), -1.0, -1.0),
    ];
    for (p, dx, dy) in corners {
        painter.line_segment([p, Pos2::new(p.x + dx * len, p.y)], st);
        painter.line_segment([p, Pos2::new(p.x, p.y + dy * len)], st);
        let q = Pos2::new(p.x + dx * 5.0, p.y + dy * 5.0);
        painter.line_segment([q, Pos2::new(q.x + dx * (len * 0.55), q.y)], Stroke::new(1.1_f32, corner));
        painter.line_segment([q, Pos2::new(q.x, q.y + dy * (len * 0.55))], Stroke::new(1.1_f32, corner));
    }
}

pub fn draw_art_deco_divider(painter: &Painter, center_x: f32, y: f32, half_width: f32, color: Color32) {
    let line_c = with_alpha(color, 0.42);
    painter.line_segment(
        [Pos2::new(center_x - half_width, y), Pos2::new(center_x - 16.0, y)],
        Stroke::new(1.0_f32, line_c),
    );
    painter.line_segment(
        [Pos2::new(center_x + 16.0, y), Pos2::new(center_x + half_width, y)],
        Stroke::new(1.0_f32, line_c),
    );
    draw_diamond(painter, Pos2::new(center_x, y), 5.5, color);
    draw_diamond(painter, Pos2::new(center_x - 11.0, y), 2.5, with_alpha(color, 0.65));
    draw_diamond(painter, Pos2::new(center_x + 11.0, y), 2.5, with_alpha(color, 0.65));
}
