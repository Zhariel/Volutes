use glium::{glutin, Surface};

pub struct RenderWindow{
    pub name: String,
    pub width: i32,
    pub height: i32,
}

impl RenderWindow{

    // glium boilerplate
    pub fn run(&self){

        let mut event_loop = glium::glutin::event_loop::EventLoop::new();

        let wb = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(self.width, self.height))
            .with_title(&self.name);

        let cb = glium::glutin::ContextBuilder::new();

        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        event_loop.run(move |ev, _, control_flow| {

            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            target.finish().unwrap();

            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },
                _ => (),
            }
        });
    }
}