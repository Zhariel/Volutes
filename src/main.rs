use crate::rendering::{Renderer, Ray, AnyRay, RayBuilder};
use crate::shapes::{Cube, Triangle, Vertex, Face,};
use crate::settings::{Settings};
use serde::Serialize;
use std::collections::{HashMap};

mod math;
mod shapes;
mod settings;
mod rendering;


fn main() {
    // let c = Cube{
    //     front : Face { t1: Triangle{a:Vertex{x: 0.0, y: 0.0, z: 0.0 }, b:Vertex{x: 1.0, y: 0.0, z: 0.0}, c:Vertex{x: 0.0, y: 1.0, z: 0.0}}, t2: Triangle{a:Vertex{x: 0.0, y: 1.0, z: 0.0}, b:Vertex{x: 1.0, y: 1.0, z: 0.0}, c:Vertex{x: 1.0, y: 0.0, z: 0.0}}},
    //     right : Face { t1: Triangle{a:Vertex{x: 1.0, y: 1.0, z: 0.0 }, b:Vertex{x: 1.0, y: 0.0, z: 0.0}, c:Vertex{x: 1.0, y: 0.0, z: 1.0}}, t2: Triangle{a:Vertex{x: 1.0, y: 1.0, z: 0.0}, b:Vertex{x: 1.0, y: 1.0, z: 1.0}, c:Vertex{x: 1.0, y: 0.0, z: 1.0}}},
    //     back : Face { t1: Triangle{a:Vertex{x: 1.0, y: 1.0, z: 1.0 }, b:Vertex{x: 1.0, y: 0.0, z: 1.0}, c:Vertex{x: 0.0, y: 0.0, z: 1.0}}, t2: Triangle{a:Vertex{x: 1.0, y: 1.0, z: 1.0}, b:Vertex{x: 0.0, y: 1.0, z: 0.0}, c:Vertex{x: 0.0, y: 0.0, z: 1.0}}},
    //     left : Face { t1: Triangle{a:Vertex{x: 0.0, y: 1.0, z: 1.0 }, b:Vertex{x: 0.0, y: 0.0, z: 1.0}, c:Vertex{x: 0.0, y: 0.0, z: 0.0}}, t2: Triangle{a:Vertex{x: 0.0, y: 1.0, z: 1.0}, b:Vertex{x: 0.0, y: 1.0, z: 0.0}, c:Vertex{x: 0.0, y: 0.0, z: 0.0}}},
    //     bottom : Face { t1: Triangle{a:Vertex{x: 0.0, y: 0.0, z: 0.0 }, b:Vertex{x: 0.0, y: 0.0, z: 1.0}, c:Vertex{x: 1.0, y: 0.0, z: 0.0}}, t2: Triangle{a:Vertex{x: 1.0, y: 0.0, z: 0.0}, b:Vertex{x: 1.0, y: 0.0, z: 0.0}, c:Vertex{x: 1.0, y: 0.0, z: 1.0}}},
    //     top : Face { t1: Triangle{a:Vertex{x: 0.0, y: 1.0, z: 0.0 }, b:Vertex{x: 0.0, y: 1.0, z: 1.0}, c:Vertex{x: 1.0, y: 1.0, z: 0.0}}, t2: Triangle{a:Vertex{x: 1.0, y: 1.0, z: 1.0}, b:Vertex{x: 1.0, y: 1.0, z: 0.0}, c:Vertex{x: 0.0, y: 1.0, z: 1.0}}},
    // };
    // let b = 1;
    // let a = Renderer {
    //     fov: 1,
    //     ray: Casting,
    // };

    let mut a = Renderer::from_json();
    a.render();
}