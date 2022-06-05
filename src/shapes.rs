use crate::settings::{Settings};
use crate::math::Projector;
use nalgebra::{Vector2, Vector3, Vector4};

pub struct Triangle {
    pub a: Vector3<f64>,
    pub b: Vector3<f64>,
    pub c: Vector3<f64>,
}


pub struct Mesh {
    pub vertices: Vec<Vector3<f64>>,
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
                .map(|v| Vector4::new(v.x, v.y, v.z, 1.0) * proj.projection_matrix)
                .collect();
        Mesh {
            vertices,
            triangles: vec![],
        }
    }
}