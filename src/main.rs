use crate::window::{model};
use vek::{Vec3, Vec4, Mat4};
use nannou::prelude::*;
use std::ops::Mul;
use crate::obj::extract_obj;
use crate::math::project_point;

mod obj;
mod window;
mod math;
mod shapes;
mod settings;
mod rendering;
mod scene;

fn main() {
    // let r = 2.83;
    // let c = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    // let th = Vec3 { x: 0.0, y: 90.0, z: 0.0 };
    // let g = project_point(c, th, r);
    // print!("{}", g);

    // use std::time::{Duration, Instant};
    // let start = Instant::now();
    //


    // use std::f64::consts::PI;
    // let x = (2.0*(PI/90.0)).sin();
    // println!("{}", x);
    // let y = (2.0*(PI/90.0)).cos();
    // println!("{}", y);

    // for i in 1..100000 {
    //     let mut mesh = extract_obj("res/cone.obj".to_string());
    //     // mesh.vertices.iter().for_each(|i| println!("{}", i));
    //     // println!();
    //     mesh.pivot("x", 90.0);
    //     mesh.pivot("y", 90.0);
    //     // mesh.vertices.iter().for_each(|i| println!("{}", i));
    // }
    //
    // let duration = start.elapsed();
    // println!("Time elapsed: {:?}", duration);
    //
    // let mut mesh = extract_obj("res/cone.obj".to_string());
    // println!();
    // mesh.rotate("y");
    // mesh.vertices.iter().for_each(|i| println!("{}", i));
    //
    //
    // let mut mesh = extract_obj("res/cone.obj".to_string());
    // println!();
    // mesh.rotate("z");
    // mesh.vertices.iter().for_each(|i| println!("{}", i));


    // println!();
    // mesh.rotate("y");
    // mesh.vertices.iter().for_each(|i| println!("{}", i));
    // println!();
    // mesh.rotate("z");
    // mesh.vertices.iter().for_each(|i| println!("{}", i));


    // let a = Mat4::new(
    //     1,2,3,4,
    //     5,6,7,8,
    //     9,8,7,6,
    //     5,4,3,2
    // );
    // let i = Mat4::identity();
    // let b = Vec4{ x: 3, y: 3, z: 3, w: 4};
    //
    // let c = a.mul(b);
    // print!("{}\n", c);
    //
    // let ii = a.mul(i);
    // print!("{}", ii);


    nannou::app(model).run();
}

