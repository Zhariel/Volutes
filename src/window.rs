use nannou::prelude::*;
use std::iter::zip;
use nannou::image::RgbImage;
use crate::obj::ObjParser;
use crate::rendering::Renderer;

pub struct RenderWindow{
    pub name: String,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
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
        }

        fn model(app: &App) -> Model {

            let renderer = Renderer::new();
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
                window
            }
        }

        fn event(app: &App, _model: &mut Model, event: WindowEvent) {
            println!("{:?}", event);
        }

        fn view(app: &App, _model: &Model, frame: Frame) {
            // let win = app.window_rect();

            let parser = ObjParser{filename: "res\\cube.obj".to_string()};
            let mesh = parser.extract_obj();

            let draw = app.draw();
            draw.background().color(WHITE);


            _model.renderer.render(&draw, &mesh);
            

            draw.to_frame(app, &frame).unwrap();


            // let tris = zip((1..win.w() as usize), (1..win.h() as usize))
            //     .for_each(|i| {
            //         print!("{} {}\n", i.0, i.1)
            //     });

        }

        fn key_pressed(app: &App, _model: &mut Model, _key: Key) {
            println!("{:?}", _key);
        }

        nannou::app(model).run();

    }
}
