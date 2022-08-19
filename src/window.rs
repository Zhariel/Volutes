use crate::shapes::{Mesh};
use crate::rendering::Renderer;
use crate::obj::extract_obj;
use nannou::event::Key::{Up, Down, Left, Right, Z, Q, S, D};
use nannou::prelude::*;

use std::iter::zip;
use crate::scene::{Camera, Plane};

pub struct Model {
    renderer: Renderer,
    window: WindowId,
    camera: Camera,
    plane: Plane,
    mesh: Mesh,
    c_speed: f64,
}

pub fn model(app: &App) -> Model {
    let renderer = Renderer::new();
    let mesh = extract_obj("res/cone.obj".to_string());
    let camera = Camera::new(0.1, 0.1, 0.1, 30.0, 30.0, 30.0);
    let plane = Plane::new(0.1, 0.1, 10.1);

    println!("mesh : {:?}", mesh.vertices);
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
    let mut model = Model { renderer, window, camera, plane, mesh, c_speed: 0.1 };

    model.mesh.project(&model.camera, &model.plane);
    println!("{:?}", model.mesh.projected_vertices);

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
        Up => { _model.camera.pos.y += _model.c_speed }
        Down => { _model.camera.pos.y -= _model.c_speed }
        Right => { _model.camera.pos.x += _model.c_speed }
        Left => { _model.camera.pos.x -= _model.c_speed }
        Z => { _model.camera.increment_angle("y", _model.c_speed) }
        S => { _model.camera.increment_angle("y", -_model.c_speed) }
        D => { _model.camera.increment_angle("x", _model.c_speed) }
        Q => { _model.camera.increment_angle("x", -_model.c_speed) }
        _ => {}
    }

    _model.mesh.project(&_model.camera, &_model.plane);
    println!("pos {:?}", _model.camera.pos);
    println!("angles {:?}", _model.camera.angles);
}
