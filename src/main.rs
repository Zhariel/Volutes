use crate::rendering::{Renderer, Ray, AnyRay};
use crate::shapes::{Vec3D, Triangle, Mesh};
use crate::settings::{Settings};
use crate::obj::ObjParser;
use crate::render::RenderWindow;
use serde::Serialize;
use std::collections::{HashMap};

mod obj;
mod render;
mod math;
mod shapes;
mod settings;
mod rendering;


fn main() {

    let mut a = Renderer::new();
    a.render();

    // let parser = ObjParser{filename: "res\\cube.obj".to_string()};
    //
    // let b = parser.extract_obj();

    let window = RenderWindow::new();

    window.run();

}