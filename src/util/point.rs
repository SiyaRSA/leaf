//point.rs
use crate::util::vector::Vec2;
use bytemuck::{Pod, Zeroable};
use wgpu::VertexBufferLayout;

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
    
    pub fn dec() -> VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<f32>() as wgpu::BufferAddress * 6,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4
                },
            ],
        }
    }
}