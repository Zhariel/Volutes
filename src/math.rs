use vek::{Vec2, Vec3, Vec4, Mat3};
use crate::settings::Settings;
use std::collections::HashMap;
use std::ops::Mul;

pub fn rotate(v: &Vec3<f64>, axis: &str, th: f64) -> Vec3<f64> {
    match axis {
        "x" => rotate_x(v, th),
        "y" => rotate_y(v, th),
        "z" => rotate_z(v, th),
        _ => Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

fn rotate_x(v: &Vec3<f64>, th: f64) -> Vec3<f64> {
    Mat3::new(
        1.0, 0.0, 0.0,
        0.0, th.cos(), -th.sin(),
        0.0, th.sin(), th.cos(),
    ).mul(*v)
}

fn rotate_y(v: &Vec3<f64>, th: f64) -> Vec3<f64> {
    Mat3::new(
        th.cos(), 0.0, th.sin(),
        0.0, 1.0, 0.0,
        -th.sin(), 0.0, th.cos(),
    ).mul(*v)
}

fn rotate_z(v: &Vec3<f64>, th: f64) -> Vec3<f64> {
    Mat3::new(
        th.cos(), -th.sin(), 0.0,
        th.sin(), th.cos(), 0.0,
        0.0, 0.0, 1.0,
    ).mul(*v)
}

pub fn project3d(a: Vec3<f64>, c: Vec3<f64>, c_th: Vec3<f64>, e: Vec3<f64>) -> Vec2<f64> {
    Vec2 { x: 0.0, y: 0.0 }
}
// pub fn project3d(a: Vec3<f64>, c: Vec3<f64>, t: Vec3<f64>, e: Vec3<f64>) -> Vec2<f64> {
//
//
//     Vec2{x: 0, y: 0}
// }

// pub fn project3d(a: Vec3<f64>, c: Vec3<f64>, t: Vec3<f64>, e: Vec3<f64>) -> Vec2<f64> {
//
// }

// pub fn project3d(a: Vec3<f64>, c: Vec3<f64>, t: Vec3<f64>, e: Vec3<f64>) -> Vec2<f64> {
//     let x = a.x - c.x;
//     let y = a.y - c.y;
//     let z = a.z - c.z;
//
//     let d = Vec3::new(
//         t.y.cos() * (t.z.sin() * y + t.z.cos() * x) - t.y.sin() * z,
//         t.x.sin() * (t.y.cos() * z + t.y.sin() * (t.z.sin() * y + t.z.cos() * x)) + t.x.cos() * (t.z.cos() * y - t.z.sin() * x),
//         t.x.cos() * (t.y.cos() * z + t.y.sin() * (t.z.sin() * y + t.z.cos() * x)) - t.x.sin() * (t.z.cos() * y - t.z.sin() * x),
//     );
//     println!("d : {:?}", d);
//     Vec2 {
//         x: (e.z / d.z) * d.x + e.x,
//         y: (e.z / d.z) * d.y + e.y,
//     }
// }
