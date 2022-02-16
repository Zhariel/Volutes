use crate::settings::{Settings};

pub trait Ray {
    fn execute(&self);
}

pub struct Casting;
impl Ray for Casting {
    fn execute(&self) {
        println!("Casting");
    }
}

pub struct Marching;
impl Ray for Marching {
    fn execute(&self) {println!("Marching");}
}

pub struct Tracing;
impl Ray for Tracing {
    fn execute(&self) {println!("Tracing");}
}

pub enum AnyRay {
    Casting(Casting),
    Marching(Marching),
    Tracing(Tracing),
}

impl Ray for AnyRay {
    fn execute(&self) {
        match self {
            Self::Casting(casting) => casting.execute(),
            Self::Marching(marching) => marching.execute(),
            Self::Tracing(tracing) => tracing.execute(),
        }
    }
}

pub struct RayBuilder{}
impl RayBuilder{
    pub fn make_ray(&self, s: String) -> AnyRay{
        match s.as_str() {
            "casting" => AnyRay::Casting(Casting),
            "marching" => AnyRay::Marching(Marching),
            "tracing" => AnyRay::Tracing(Tracing),
            _ => AnyRay::Casting(Casting),
        }
    }
}

pub struct Renderer{
    pub fov: usize,
    pub ray: AnyRay,
}

impl Renderer {
    pub fn render(&self) {
        self.ray.execute();
    }

    pub fn from_json() -> Renderer {
        let settings: Settings = Settings::load();

        Renderer {fov: settings.fov, ray: RayBuilder{}.make_ray(settings.rendering)}
    }
}