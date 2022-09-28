use crate::shapes::{Mesh};
use crate::rendering::Renderer;
use crate::obj::extract_obj;
use nannou::event::Key::{Up, Down, Left, Right, Z, Q, S, D};
use nannou::prelude::*;

use std::iter::zip;
use crate::scene::{Camera};

pub struct Model {
    renderer: Renderer,
    window: WindowId,
    camera: Camera,
    mesh: Mesh,
    c_speed: f64,
}

pub fn model(app: &App) -> Model {
    let renderer = Renderer::new();
    let mesh = extract_obj("res/cube.obj".to_string());
    let camera = Camera::new(0.0, 0.0, 0.0, 0.0, 90.0, 0.0, renderer.settings.f_length);

    println!("mesh : {:?}", mesh.vertices);
    mesh.triangles.iter().for_each(|i| print!(" ⟁ {}", i));
    let window = app
        .new_window()
        .size(renderer.settings.window_size, renderer.settings.window_size)
        .title("")
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    // let vs_desc = wgpu::include_wgsl!("shaders\\vs.wgsl");
    // let vs_mod = device.create_shader_module(&vs_desc);
    let mut model = Model { renderer, window, camera, mesh, c_speed: 1.0 };

    model.mesh.project(&model.camera);
    println!("P_vertices : {:?}", model.mesh.projected_vertices);

    model
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
    match _key {
        Z => { _model.camera.pos.y += _model.c_speed }
        S => { _model.camera.pos.y -= _model.c_speed }
        D => { _model.camera.pos.x += _model.c_speed }
        Q => { _model.camera.pos.x -= _model.c_speed }
        Up => { _model.camera.pivot("x", _model.c_speed) }
        Down => { _model.camera.pivot("x", -_model.c_speed) }
        Right => { _model.camera.pivot("y", _model.c_speed) }
        Left => { _model.camera.pivot("y", -_model.c_speed) }
        _ => {}
    }

    _model.mesh.project(&_model.camera);
    println!("pos {:?}", _model.camera.pos);
    println!("angles {:?}", _model.camera.angles);
}
