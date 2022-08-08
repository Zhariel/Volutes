use nannou::prelude::*;
use crate::settings::{Settings};
use crate::shapes::{Triangle, Mesh};

pub struct Renderer{
    pub rays: Rays,
    pub settings: Settings,
}

impl Renderer {

    pub fn new() -> Renderer {
        Renderer {
            rays: Rays,
            settings: Settings::load(),
        }
    }

    pub fn render(&self, draw: &Draw, mesh: &Mesh){
        match self.settings.rendering.as_str() {
            "casting" => self.rays.casting(draw, mesh),
            "marching" => self.rays.marching(draw, mesh),
            "tracing" => self.rays.tracing(draw, mesh),
            "vertices" => self.rays.vertices(draw, mesh, self.settings.vertex_size),
            _ => self.rays.casting(draw, mesh),
        }
    }
}

pub struct Rays;
impl Rays{

    fn casting(&self, draw: &Draw, mesh: &Mesh) {
        println!("Casting");
    }

    fn marching(&self, draw: &Draw, mesh: &Mesh) {println!("Marching");}

    fn tracing(&self, draw: &Draw, mesh: &Mesh) {println!("tracing");}

    fn vertices(&self, draw: &Draw, mesh: &Mesh, size: f32) {

        draw.rect()
            .color(BLACK)
            .w(size)
            .h(size)
            .x_y(0.0, 0.0);
    }
}


