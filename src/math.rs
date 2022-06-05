use nalgebra::{Matrix4, Vector4};

pub struct Projector {
    pub a: f64,
    pub f: f64,
    pub q: f64,
    pub projection_matrix: Matrix4<f64>,
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
            projection_matrix: Matrix4::new(
                a*f, 0.0, 0.0, 0.0,
                0.0, f, 0.0, 0.0,
                0.0, 0.0, q, 1.0,
                0.0, 0.0, -zn*q, 0.0
            )
        }
    }

    pub fn project(self, vec: Vector4<f64>) -> Matrix4<f64> {
        vec * self.projection_matrix
    }
}