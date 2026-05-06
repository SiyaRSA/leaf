//vector.rs
//! # Vector Types for LEAF
//!
//! This module defines simple vector types commonly used in 2D rendering,
//! transformations, and math utilities. While only `Vec2` is strictly required
//! for 2D work, `Vec3` and `Vec4` are included for compatibility with graphics
//! pipelines (e.g. homogeneous coordinates, shaders, and color representations).

/// A 2D vector.
///
/// # Fields
/// - `x` – Horizontal component
/// - `y` – Vertical component
///
/// # Common Uses
/// - Object positions in world or screen space
/// - Movement directions and velocities
/// - Texture coordinates (UVs)
/// - Dimensions (width, height)
/// 
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// A 3D vector.
///
/// # Fields
/// - `x` – Horizontal component
/// - `y` – Vertical component
/// - `z` – Depth or auxiliary value
///
/// # Common Uses (2D Context)
/// - Layering (z-order / depth sorting)
/// - RGB color representation (`r`, `g`, `b`)
/// - Intermediate math for transforms
///
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// A 4D vector.
///
/// # Fields
/// - `x` – **R** -> First component
/// - `y` – **G** -> Second component
/// - `z` – **B** -> Third component
/// - `w` – **A** -> Fourth component
///
/// # Common Uses (2D Context)
/// - RGBA color (`r`, `g`, `b`, `a`)
/// - Homogeneous coordinates for transformations
/// - Shader inputs and GPU data structures
///
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}