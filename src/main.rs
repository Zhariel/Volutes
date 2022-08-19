use crate::window::{model};
use nannou::prelude::*;

mod obj;
mod window;
mod math;
mod shapes;
mod settings;
mod rendering;
mod scene;

fn main() {

    nannou::app(model).run();
}

