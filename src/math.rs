use vek::{Vec2, Vec3};
use crate::settings::Settings;

// pub fn project3d(a: Vec3<f64>, c: Vec3<f64>, t: Vec3<f64>, e: Vec3<f64>) -> Vec2<f64> {
//     let f = 50.0;
//
//     let x_p = (a.x - c.x) * (f/a.z) + c.x;
//     let y_p = (a.y - c.y) * (f/a.z) + c.y;
//
//     Vec2{x: x_p, y: y_p}
// }

pub fn project3d(a: Vec3<f64>, c: Vec3<f64>, t: Vec3<f64>, e: Vec3<f64>) -> Vec2<f64> {
    
}

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
