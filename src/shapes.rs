use crate::settings::{Settings};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3D{
    pub fn from_vec(vec: Vec<f32>) -> Vec3D{
        Vec3D{x: vec[0], y: vec[1], z: vec[2]}
    }
}

pub struct Triangle {
    // pub verts: [Vec3D; 3]
    pub a: Vec3D,
    pub b: Vec3D,
    pub c: Vec3D,
}

pub struct Mesh {
    pub vertices: Vec<Vec3D>,
    pub triangles: Vec<Triangle>
}

impl Mesh {

    pub fn new() -> Mesh {
        Mesh{
            vertices: vec![],
            triangles: vec![]
        }
    }
}