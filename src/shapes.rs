use crate::settings::{Settings};
use std::collections::HashMap;

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

pub struct Face {
    pub t1: Triangle,
    pub t2: Triangle,
}

pub struct Cube {
    pub front: Face,
    pub right: Face,
    pub back: Face,
    pub left: Face,
    pub bottom: Face,
    pub top: Face,
}
