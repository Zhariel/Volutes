use crate::settings::{Settings};
use crate::scene::{Camera, Plane};
use crate::math::project3d;
use vek::{Vec3, Vec2};

pub struct Triangle {
    pub a: Vec3<f64>,
    pub b: Vec3<f64>,
    pub c: Vec3<f64>,
}

pub struct Mesh {
    pub vertices: Vec<Vec3<f64>>,
    pub projected_vertices: Vec<Vec2<f64>>,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: vec![],
            projected_vertices: vec![],
            triangles: vec![],
        }
    }

    pub fn project(&mut self, cam: &Camera, plane: &Plane) {
        self.projected_vertices = self.vertices
            .clone()
            .into_iter()
            .map(|v| project3d(v, cam.pos, cam.angles, plane.pos))
            .collect();
    }
}