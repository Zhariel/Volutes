use crate::shapes::{Mesh};
use crate::math::Projector;
use crate::rendering::Renderer;
use crate::obj::ObjParser;

use nannou::prelude::*;
use std::iter::zip;

pub struct RenderWindow{
    pub name: String,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}


impl RenderWindow{

    pub fn new() -> RenderWindow{
        RenderWindow{
            name: "Waves".to_string(),
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    pub fn run(&self){

        struct Model {
            renderer: Renderer,
            window: WindowId,
            mesh: Mesh,
            projector: Projector,

        }

        fn model(app: &App) -> Model {

            let renderer = Renderer::new();
            let parser = ObjParser{filename: "res\\cube.obj".to_string()};
            let mesh = parser.extract_obj();
            let projector = Projector::new(0.0, 0.0, 0.0, 0.0, 0.0);

            let window = app
                .new_window()
                .size(renderer.settings.window_size, renderer.settings.window_size)
                .title("Waves")
                .key_pressed(key_pressed)
                .view(view)
                .build()
                .unwrap();

            // let vs_desc = wgpu::include_wgsl!("shaders\\vs.wgsl");
            // let vs_mod = device.create_shader_module(&vs_desc);
            Model {
                renderer,
                window,
                mesh,
                projector,
            }
        }

        fn event(app: &App, _model: &mut Model, event: WindowEvent) {
            println!("{:?}", event);
        }

        fn view(app: &App, _model: &Model, frame: Frame) {
            // let win = app.window_rect();

            let draw = app.draw();
            draw.background().color(WHITE);


            _model.renderer.render(&draw, &_model.mesh);
            

            draw.to_frame(app, &frame).unwrap();

        }

        fn key_pressed(app: &App, _model: &mut Model, _key: Key) {
            println!("{:?}", _key);
        }

        nannou::app(model).run();

    }
}
