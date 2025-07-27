mod app;
mod render;
mod vertex;

use std::error::Error;
use winit::event_loop::EventLoop;
use app::App;

fn main() -> Result<(), impl Error> {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(&event_loop);

    event_loop.run_app(&mut app)
}
