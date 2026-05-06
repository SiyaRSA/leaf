//path.rs
use crate::util::operations::joint::*;
use crate::util::Point;
/// A collection of points defining a geometric path.
#[derive(Debug, Clone)]
pub struct Path {
    pub points: Vec<Point>,
}

impl Default for Path {
    fn default() -> Self {
        Self { 
            points: Vec::new(),
        }
    }
}

impl Path {
    /// Creates a new [[`Path`]] from a flat slice of coordinates.
    /// 
    /// This method treats every two consecutive `f32` values as a single [[`Point`]].
    /// # Arguments
    /// * `coords` - A slice of `f32` values in the format `[x1, y1, x2, y2, ...]`.
    ///
    /// # Behavior
    /// If the slice contains an odd number of elements, the last element is ignored 
    /// due to the use of `chunks_exact(2)`.
    ///
    pub fn new(coords: &[f32]) -> Self {
        let points = coords
        .chunks_exact(2)
        .map(|chunk| Point {
            x: chunk[0],
            y: chunk[1],
        })
        .collect();

        Self { points }
    }

    pub fn set_joints(&self, style: JoinStyle, amount: f32) -> Self {
        let n = self.points.len();

        if n < 3 {
            return self.clone();
        }

        let mut out = Vec::new();

        out.push(self.points[0]);

        for i in 1..n -1 {
            let a = self.points[i - 1];
            let b = self.points[i];
            let c = self.points[i + 1];

            let joined = joint(a, b, c, amount, style.clone());

            out.extend(joined);
        }

        out.push(self.points[n - 1]);

        Self { points: out }
    }

    /// Returns the number of points in the [`Path`].
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Clears points in the [`Path`].
    pub fn clear(&mut self) {
        self.points.clear();
    }
}


/*/// Creates a [Path] from a list of coordinate literals.
///
/// This is a convenience macro for quickly initializing a path without
/// manually defining a slice or calling Path::new.
///
#[macro_export]
macro_rules! path {
    ($($coords:expr),*) => {
        $crate::Path::new(&[$($coords),*])
    };
    ($($coords:expr,)*) => {
        $crate::path![$($coords),*]
    };
}*/

use std::ops::Index;
impl Index<usize> for Path {
    type Output = Point;
    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}