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
    pub fn new(x: f32, y: f32,) -> Self {
        Self { x, y }
    }

    pub fn vec2(&self) -> Vec2 {
       Vec2 { x: self.x, y: self.y }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout { 
            array_stride: std::mem::size_of::<Point>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}