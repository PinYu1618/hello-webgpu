use crate::Gpu;

#[derive(Debug)]
pub struct Crayon {
    should_quit: bool,
    _gpu: Gpu,
}

impl Crayon {
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

impl Default for Crayon {
    fn default() -> Self {
        Self {
            _gpu: Gpu::new().expect("Webgpu error :("),
            should_quit: false,
        }
    }
}
