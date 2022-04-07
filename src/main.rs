use crate::rendering::{Renderer};
use crate::shapes::{Vec3D, Triangle, Mesh};
use crate::settings::{Settings};
use crate::obj::ObjParser;
use crate::window::RenderWindow;
use serde::Serialize;
use std::collections::{HashMap};

mod obj;
mod window;
mod math;
mod shapes;
mod settings;
mod rendering;


fn main() {
    let window = RenderWindow::new();

    window.run();
}