use crate::rendering::{Renderer, Ray, AnyRay};
use crate::shapes::{Vec3D, Triangle, Mesh};
use crate::settings::{Settings};
use serde::Serialize;
use std::collections::{HashMap};

mod obj;
mod draw;
mod math;
mod shapes;
mod settings;
mod rendering;


fn main() {

    let mut a = Renderer::from_json();
    a.render();
}