mod gltk;
mod init;
mod platform;

pub use self::gltk::Gltk;
pub use self::init::GltkBuilder;
pub use self::platform::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub type GResult<T> = anyhow::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type GError = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn main_loop(mut gltk: Gltk) -> GResult<()> {
    env_logger::init();

    let el = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello.")
        .build(&el)
        .unwrap();

    let my_window = window.id();

    el.run(move |event, _, control_flow| {
        if gltk.quitting {
            control_flow.set_exit();
        } else {
            *control_flow = ControlFlow::Poll;
        }

        match &event {
            Event::WindowEvent { event, window_id } => {
                if *window_id != my_window {
                    return;
                }

                match event {
                    WindowEvent::Resized(_) => {}
                    WindowEvent::KeyboardInput { .. } => {}
                    WindowEvent::CloseRequested => gltk.quitting = true,
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) => {
                if *window_id != my_window {
                    return;
                }
            }
            Event::RedrawEventsCleared => {}
            _ => {}
        }
    });
}
