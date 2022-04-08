use crate::settings::{Settings};
use std::collections::HashMap;
use crate::math::project;

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

#[derive(Debug, Clone, Copy)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Vec2D{
    pub fn from_3d(vec: Vec3D) -> Vec2D{
        let s: Settings = Settings::load();
        project(&vec, s.fov)
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

pub struct ProjectedMesh {
    pub vertices: Vec<Vec2D>
}

impl ProjectedMesh {

    pub fn new(mesh: &Mesh) -> ProjectedMesh {

        ProjectedMesh{
            vertices: mesh.vertices
                .clone()
                .into_iter()
                .map(|v| Vec2D::from_3d(v))
                .collect()
        }
    }
}
