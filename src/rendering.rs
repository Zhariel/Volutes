use nannou::prelude::*;
use crate::settings::{Settings};
use crate::shapes::{Triangle, Mesh};

pub struct Renderer {
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

    pub fn render(&self, draw: &Draw, mesh: &Mesh) {
        match self.settings.rendering.as_str() {
            "casting" => self.rays.casting(draw, mesh),
            "marching" => self.rays.marching(draw, mesh),
            "tracing" => self.rays.tracing(draw, mesh),
            "vertex" => self.rays.vertex(draw, mesh, self.settings.vertex_size),
            _ => self.rays.casting(draw, mesh),
        }
    }
}

pub struct Rays;

impl Rays {
    fn vertex(&self, draw: &Draw, mesh: &Mesh, size: f32) {
        for vert in &mesh.projected_vertices {
            // println!("drawing {:?}", vert);
            draw.rect()
                .color(BLACK)
                .w(size)
                .h(size)
                .x_y(vert.x as f32, vert.y as f32);
        }
    }

    fn wireframe(&self, draw: &Draw, mesh: &Mesh, size: f32) {}

    fn casting(&self, draw: &Draw, mesh: &Mesh) {
        println!("Casting");
    }

    fn marching(&self, draw: &Draw, mesh: &Mesh) { println!("Marching"); }

    fn tracing(&self, draw: &Draw, mesh: &Mesh) { println!("tracing"); }
}


