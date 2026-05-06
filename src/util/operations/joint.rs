//joint.rs
use crate::util::ffunctions::*;
use crate::util::vector::Vec2;
use crate::util::Point;

#[derive(Debug, Clone)]
pub enum JoinStyle {
    Miter { limit: f32 },
    Bevel,
    Round { segments: usize},
    None, //sharp
}

impl Default for JoinStyle {
    fn default() -> Self {
        JoinStyle::Miter { limit: 4.0 }
    }
}

pub fn joint(
    a: Point,
    b: Point,
    c: Point,
    amount: f32,
    style: JoinStyle,
) -> Vec<Point> {
    let dir1 = tangent(b, a);
    let dir2 = tangent(c, b);

    let n1 = perp(dir1);
    let n2 = perp(dir2);

    match style {
        JoinStyle::None => vec![b],
        JoinStyle::Bevel => vec![
            Point { x: b.x + n1.x * amount, y: b.y + n1.y * amount },
            Point { x: b.x + n2.x * amount, y: b.y + n2.y * amount },
        ],
        JoinStyle::Miter { limit } => {
            let miter = normalize(Vec2 {
                x: n1.x + n2.x,
                y: n1.y + n2.y,
            });

            let denom = miter.x * n2.x + miter.y * n2.y;

            if denom.abs() < 0.0001 {
                return vec![b]; // fallback
            }

            let length = amount / denom;

            if length > limit {
                return vec![
                    Point { x: b.x + n1.x * amount, y: b.y + n1.y * amount },
                    Point { x: b.x + n2.x * amount, y: b.y + n2.y * amount },
                ];
            }

            vec![Point {
                x: b.x + miter.x * length,
                y: b.y + miter.y * length,
            }]
        }
        JoinStyle::Round { segments } => {
            let angle1 = atan2(n1);
            let angle2 = atan2(n2);

            let mut out = Vec::new();

            for i in 0..= segments {
                let t = i as f32 /  segments as f32;
                let angle = angle1 + (angle2 - angle1) * t;

                let dir = rotate(n1, angle);
                out.push(Point {
                    x: b.x + dir.x * amount,
                    y: b.y + dir.y * amount,
                });
            }

            out
        }
    }
}