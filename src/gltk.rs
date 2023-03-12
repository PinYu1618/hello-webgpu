#[derive(Clone, Debug)]
pub struct Gltk {
    pub quitting: bool,
}

impl Gltk {
    pub fn quit(&mut self) {
        self.quitting = true;
    }
}
