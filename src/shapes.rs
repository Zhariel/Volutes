use crate::settings::{Settings};
use crate::scene::{Camera};
use crate::math::{project_perspective, rotate};
use vek::{Vec3, Vec2, Mat3};
use std::fmt;
use std::fmt::{Formatter};

pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl fmt::Display for Triangle{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {},", self.a, self.b, self.c)
    }
}

pub struct Mesh {
    pub vertices: Vec<Vec3<f64>>,
    pub projected_vertices: Vec<Vec2<f64>>,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new() -> Mesh {
        let settings = Settings::load();
        Mesh {
            vertices: vec![],
            projected_vertices: vec![],
            triangles: vec![],
        }
    }

    pub fn project(&mut self, cam: &Camera) {
        self.projected_vertices = self.vertices
            .clone()
            .into_iter()
            .map(|v| project_perspective(v, cam.pos, cam.angles, cam.lens))
            .collect();
    }

    pub fn pivot(&mut self, axis: &str, th: f64) {
        self.vertices = self.vertices
            .clone()
            .into_iter()
            .map(|v| rotate(&v, axis, th))
            .collect();
    }
}