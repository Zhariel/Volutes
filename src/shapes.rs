use crate::settings::{Settings};
use crate::math::Projector;
use vek::{Vec3, Vec4};

pub struct Triangle {
    pub a: Vec3<f64>,
    pub b: Vec3<f64>,
    pub c: Vec3<f64>,
}


pub struct Mesh {
    pub vertices: Vec<Vec3<f64>>,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: vec![],
            triangles: vec![],
        }
    }

    pub fn project(self, proj: &Projector) -> Mesh {
            let vertices = self.vertices
                .clone()
                .into_iter()
                .map(|v| proj.project(v))
                .collect();
        Mesh {
            vertices,
            triangles: vec![],
        }
    }
}