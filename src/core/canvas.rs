//canvas.rs
use crate::util::vector::Vec4;
use wgpu::util::DeviceExt;
use crate::shapes::VecG;
use wgpu::Device;

#[repr(C)]
pub struct Canvas {
    shapes: Vec<VecG>
}

impl Canvas {
    pub fn c1(shapes: Vec<VecG>) -> Self {
        return Canvas { shapes };
    }

    pub fn build_c1(&self, device: &Device) -> Vec<wgpu::Buffer> {
        let mut buffers = Vec::new();

        for shape in self.shapes.clone() {
            let clr: Vec4 = shape.clr;

            let mut data: Vec<f32> = Vec::new();

            for p in shape.path.points {
                data.push(p.x);
                data.push(p.y);

                data.push(clr.x);
                data.push(clr.y);
                data.push(clr.z);
                data.push(clr.w);
            }

            let contents = bytemuck::cast_slice(&data);

            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("LEAF Canvas build_c1"),
                contents: &contents,
                usage: wgpu::BufferUsages::VERTEX
            });

            buffers.push(buffer);
        }

        buffers
    }

    pub fn vertex_counts(&self) -> Vec<u32> {
        self.shapes
            .iter()
            .map(|s| s.path.points.len() as u32)
            .collect()
    }
}