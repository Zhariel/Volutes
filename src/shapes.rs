use crate::settings::{Settings};
use std::collections::HashMap;

#[derive(Debug)]
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
    pub verts: [Vec3D; 3]
}

pub struct Mesh {
    pub triangles: Vec<Triangle>
}