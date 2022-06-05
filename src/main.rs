use crate::window::RenderWindow;

mod obj;
mod window;
mod math;
mod shapes;
mod settings;
mod rendering;
mod camera;


fn main() {
    let window = RenderWindow::new();

    window.run();
}