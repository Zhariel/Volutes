use vek::{Mat4, Vec4};
use std::ops::Mul;

pub struct Projector {
    pub a: f64,
    pub f: f64,
    pub q: f64,
    pub projection_matrix: Mat4<f64>,
}

impl Projector {
    pub fn new(w: f64, h: f64, zf: f64, zn: f64, fov: f64) -> Projector {
        let a = w/h;
        let f = 1.0 / (fov/2.0).tan();
        let q = zf/(zf-zn);

        Projector {
            a,
            f,
            q,
            projection_matrix: Mat4::new(
                    a*f, 0.0, 0.0, 0.0,
                    0.0, f, 0.0, 0.0,
                    0.0, 0.0, q, 1.0,
                    0.0, 0.0, zn*q, 0.0
            )
        }
    }

    pub fn project(self, vec: Vec4<f64>) -> Vec4<f64> {

        Vec4::new(
            vec.x * self.projection_matrix[(0, 0)],
            vec.y * self.projection_matrix[(1, 1)],
            vec.z * self.projection_matrix[(2, 2)] - self.projection_matrix[(2, 3)],
            self.projection_matrix[(3, 2)],
        )
    }
}