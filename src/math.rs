use vek::{Vec2, Vec3, Vec4, Mat3};
use nannou::math::ConvertAngle;
use crate::settings::Settings;
use std::collections::HashMap;
use std::ops::Mul;

pub fn rotate(v: &Vec3<f64>, axis: &str, th: f64) -> Vec3<f64> {
    match axis {
        "x" => rotate_x(v, th),
        "y" => rotate_y(v, th),
        "z" => rotate_z(v, th),
        _ => panic!("Rotation along a non-existing axis.")
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

pub fn project_point(c: Vec3<f64>, th: Vec3<f64>, r: f64) -> Vec3<f64> {
    // x′=rcosθcosα
    // y′=rsinθ
    // z′=rcosθsinα
    Vec3 {
        x: c.x + r * th.x.deg_to_rad().cos() * th.y.deg_to_rad().cos(),
        y: c.y + r * th.x.deg_to_rad().sin(),
        z: c.z + r * th.x.deg_to_rad().cos() * th.y.deg_to_rad().sin(),
    }
}

pub fn project_perspective(a: Vec3<f64>, c: Vec3<f64>, c_th: Vec3<f64>, e: Vec3<f64>) -> Vec2<f64> {
    let d: Vec3<f64> = rotate_z(&rotate_y(&rotate_x(&(a - c), c_th.x), c_th.y), c_th.z);
    // let d = a - c;

    Vec2 {
        x: (e.z / d.z) * d.x + e.x,
        y: (e.z / d.z) * d.y + e.y,
    }
}

