//point.rs
use crate::util::vector::Vec2;
use bytemuck::{Pod, Zeroable};

///Point is used to represent a vetex along a 2D [`Path`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32,) -> (f32, f32) {
        return ( x, y );
    }

    pub fn vec2(x: f32, y: f32,) -> Vec2 {
       return Vec2 { x, y };
    }
}