//rect.rs
use crate::shapes::vg::VecG;
use crate::util::vector::*;
use crate::util::Path;


pub struct Rect {
    inner: VecG
}

impl Rect {
    pub fn new(width: f32, height: f32, x: f32, y: f32) -> Self {
        let path = Path::new(&[
            0.0,0.0, 
            width,0.0,
            width, height,
            0.0,height
        ]);

        let clr = Vec4 { 
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0
        };

        let pos = Vec3 { x, y, z: 0.0 };

        let inner= VecG { path, clr, pos };

        return Rect { inner };
    }

    pub fn inner(&self) -> &VecG {
        &self.inner
    }
}