use crate::rendering::{Renderer, Ray, AnyRay};
use crate::shapes::{Vec3D, Triangle, Mesh};
use crate::settings::{Settings};
use crate::obj::ObjParser;
use serde::Serialize;
use std::collections::{HashMap};

#[macro_use]
extern crate glium;

mod obj;
mod draw;
mod math;
mod shapes;
mod settings;
mod rendering;


fn main() {

    let mut a = Renderer::new();
    a.render();

    let parser = ObjParser{filename: "res\\cube.obj".to_string()};

    parser.extract_obj();
}