mod crayon;
mod gpu;

pub use self::crayon::Crayon;
pub use self::gpu::Gpu;

pub type CResult<T> = anyhow::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type CError = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn main_loop(mut ctx: Crayon) -> CResult<()> {
    env_logger::init();

    let el = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello.")
        .build(&el)
        .unwrap();

    let my_window = window.id();

    el.run(move |event, _, control_flow| {
        if ctx.should_quit() {
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
                    WindowEvent::CloseRequested => ctx.quit(),
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
