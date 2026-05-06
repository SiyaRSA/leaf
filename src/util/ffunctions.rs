//ffunctions.rs
//! # Free Functions (Geometry Utilities)
//!
//! This module provides small helper functions for basic 2D geometry
//! operations on `Point` and `Vec2` types.
//!
//! ## Features
//! - Vector creation between two points
//! - Normalized direction (tangent)
//! - Perpendicular vector (normal)
//! - Linear interpolation between points
//! - Angle calculation between three points


    use crate::util::vector::*;
    use crate::util::Point;

    /// Creates a vector from `b` to `a`.
    pub fn vec(a: Point, b: Point) -> Vec2 {
        Vec2 {
            x: a.x - b.x,
            y: a.y - b.y,
        }
    }

    /// Returns the normalized direction vector from `b` to `a`.
    /// If the points are identical, returns `(0, 0)`.
    pub fn tangent(a: Point, b: Point) -> Vec2 {
        let v = vec(a, b);
        let len = (v.x * v.x + v.y * v.y).sqrt();

        if len == 0.0 {
            Vec2 { x: 0.0, y: 0.0 }
        } else {
            Vec2 {
                x: v.x / len,
                y: v.y / len,
            }
        }
    }

    /// Returns a perpendicular (normal) vector to the tangent from `b` to `a`.
    pub fn normal(a: Point, b: Point) -> Vec2 {
        let t = tangent(a, b);
        Vec2 {
            x: -t.y,
            y: t.x,
        }
    }

    /// Linearly interpolates between points `a` and `b` by `t` (0.0–1.0).
    pub fn lerp(a: Point, b: Point, t: f32) -> Point {
        Point {
            x: a.x + (b.x - a.x) * t,
            y: a.y + (b.y - a.y) * t,
        }
    }

    /// Returns the angle (in degrees) at point `b` formed by `a - b - c`.
    pub fn angle(a: Point, b: Point, c: Point) -> f32 {
        let ab = (a.x - b.x, a.y - b.y);
        let cb = (c.x - b.x, c.y - b.y);

        let dot = ab.0 * cb.0 + ab.1 * cb.1;

        let ab_mag = (ab.0 * ab.0 + ab.1 * ab.1).sqrt();
        let cb_mag = (cb.0 * cb.0 + cb.1 * cb.1).sqrt();

        let cos_theta = (dot / (ab_mag * cb_mag)).clamp(-1.0, 1.0);
        cos_theta.acos().to_degrees()
    }

    pub fn dir(a: Point, b: Point) -> Vec2 {
        tangent(a, b)
    }

    /// Returns a vector perpendicular to `v`.
    pub fn perp(v: Vec2) -> Vec2 {
        Vec2 { x: -v.y, y: v.x }
    }

    /// Returns the angle in radians of the vector `v` relative to the positive x-axis.
    /// Uses the standard `f32::atan2(y, x)` implementation.
    pub fn atan2(v: Vec2) -> f32 {
        v.y.atan2(v.x)
    }

    /// Rotates the vector `v` by the given `angle` (in radians) counter-clockwise.
    pub fn rotate(v: Vec2, angle: f32) -> Vec2 {
        let (sin, cos) = angle.sin_cos();
        Vec2 {
            x: v.x * cos - v.y * sin,
            y: v.x * sin + v.y * cos,
        }
    }

    pub fn normalize(v: Vec2) -> Vec2 {
        let len = (v.x * v.x + v.y * v.y).sqrt();
        if len == 0.0 {
            return Vec2 { x: 0.0, y: 0.0 };
        }
        Vec2 { x: v.x / len, y: v.y / len }
    }
