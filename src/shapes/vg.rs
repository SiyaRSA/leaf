//vg.rs
use crate::util::vector::*;
use crate::util::Path;

///Vector Graphic
#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct VecG {
    pub path: Path,
    pub pos: Vec3,
    pub clr: Vec4,
}

impl VecG {
    /// Creates a new [[`VecG`]] from a flat slice of coordinates.
    /// 
    /// This method treats every two consecutive `f32` values as a single [[`Point`]].
    /// # Arguments
    /// * `path` - A [[`Path`]]` used to represent a polygon.
    /// * `clr` - A [[`Vec4`]]` used to for color `fill` in the shape.
    /// * `pos` - A [[`Vec3`]] used to place the shape on the window`.
    ///
    pub fn new(path: Path, pos: Vec3, clr: Vec4) -> Self {
        Self { path, pos, clr }
    }

    pub fn set_color(mut self, clr: Vec4) {
        self.clr = clr;
    }

    pub fn set_pos(mut self, pos: Vec3) {
        self.pos = pos;
    }

    pub fn dec() -> wgpu::VertexBufferLayout<'static> {
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