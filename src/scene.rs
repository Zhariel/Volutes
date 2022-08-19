use vek::{Vec3};
use crate::settings::Settings;

pub struct Camera {
    pub fov: f32,
    pub pos: Vec3<f64>,
    pub angles: Vec3<f64>,
}

impl Camera {
    pub fn new(x: f64, y: f64, z: f64, x_t: f64, y_t: f64, z_t: f64) -> Camera {
        let settings = Settings::load();

        Camera {
            fov: settings.fov,
            pos: Vec3 {x, y, z},
            angles: Vec3 {x: x_t, y: y_t, z: z_t},
        }
    }

    pub fn increment_angle(&mut self, axis: &str, amount: f64){
        let calculate = |a: f64| (a + amount).rem_euclid(360.0);

        if axis.eq("x"){
            self.angles.x = calculate(self.angles.x);
        }
        else if axis.eq("y") {
            self.angles.y = calculate(self.angles.y);
        }
        else if axis.eq("z") {
            self.angles.z = calculate(self.angles.z);
        }
    }
}

pub struct Plane {
    pub pos: Vec3<f64>,
}

impl Plane {
    pub fn new(x: f64, y: f64, z: f64) -> Plane {
        Plane {
            pos: Vec3 {x, y, z},
        }
    }
}
