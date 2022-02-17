use crate::settings::{Settings};
use std::collections::HashMap;

pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Triangle {
    pub a: Vec3D,
    pub b: Vec3D,
    pub c: Vec3D,
}

pub struct Mesh {
    pub triangles: Vec<Triangle>
}