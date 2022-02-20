use crate::rendering::{Renderer, Ray, AnyRay};
use crate::shapes::{Vec3D, Triangle, Mesh};
use crate::settings::{Settings};
use crate::obj::ObjParser;
use crate::draw::RenderWindow;
use serde::Serialize;
use std::collections::{HashMap};
use glium::glutin::CreationError::Window;

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

    let window = RenderWindow{ name: "Waves".to_string(), width: 1000, height: 1000};
    window.run();

}