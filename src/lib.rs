mod gltk;
mod init;

pub use self::gltk::Gltk;
pub use self::init::GltkBuilder;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub type GResult<T> = anyhow::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type GError = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn main_loop(ctx: Gltk) -> GResult<()> {
    env_logger::init();

    let el = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello.")
        .build(&el)
        .unwrap();

    let my_window = window.id();

    el.run(move |event, _, control_flow| {
        if ctx.quitting {
            *control_flow = ControlFlow::Exit;
        }

        match &event {
            Event::WindowEvent { event, window_id } => {
                if *window_id != my_window {
                    return;
                }

                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            }
            _ => {}
        }
    });
}
