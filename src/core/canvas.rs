//canvas.rs
use crate::util::vector::{Vec3, Vec4};
use wgpu::util::DeviceExt;
use crate::shapes::VecG;
use crate::util::Path;
use wgpu::Device;

#[allow(unused)]
pub const CANVAS_SHADER: &str = include_str!("../shader/canvas.wgsl");

#[repr(C)]
pub struct Canvas {
    inner: VecG,
    shapes: Vec<VecG>
}

impl Canvas {
    pub fn c1(shapes: Vec<VecG>) -> Self {
        let siz: f32 = 1024.0;
        let inner = VecG::new(
            Path::new(&[
                0.0,0.0,
                siz,0.0,
                siz,siz,
                0.0,siz,
            ]), 
            Vec3::new(siz * 0.5, siz * 0.5, 0.0), 
            Vec4::new(1.0, 1.0, 1.0, 1.0),
        );
        return Canvas { inner , shapes };
    }

    pub fn c_clear(mut self, clr: Vec4) -> Self {
        self.inner.clr = clr;
        self
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

    /*fn get_vecg_path(&self) -> Vec<Path> {
        self.shapes
            .iter()
            .map(|shape| shape.path.clone())
            .collect()
    }

    fn flatten_paths(path: &[Path]) -> Vec<Point> {
        path.iter()
            .flat_map(|p| p.points.iter().copied())
            .collect()
    }

    fn path_to_bytes(points: &[Point]) -> Vec<u8> {
        let vec = points.to_vec();
        bytemuck::cast_vec(vec)
    }*/
}