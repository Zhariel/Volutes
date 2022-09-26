use vek::{Vec3};
use crate::settings::Settings;

pub struct Camera {
    pub fov: f32,
    pub pos: Vec3<f64>,
    pub angles: Vec3<f64>,
    pub lens: Vec3<f64>,
}

impl Camera {
    pub fn new(x: f64, y: f64, z: f64, x_th: f64, y_th: f64, z_th: f64, f_length: f64) -> Camera {
        let settings = Settings::load();

        Camera {
            fov: settings.fov,
            pos: Vec3 { x, y, z },
            angles: Vec3 { x: x_th, y: y_th, z: z_th },
            lens: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        }
    }

    pub fn pivot(&mut self, axis: &str, amount: f64) {
        let incr_angle = |a: f64| (a + amount).rem_euclid(360.0);

        match axis {
            "x" => self.angles.x = incr_angle(self.angles.x),
            "y" => self.angles.y = incr_angle(self.angles.y),
            "z" => self.angles.z = incr_angle(self.angles.z),
            _ => {}
        }
    }
}
