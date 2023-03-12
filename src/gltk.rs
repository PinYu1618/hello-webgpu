use crate::GltkBuilder;

#[derive(Clone, Debug)]
pub struct Gltk {
    pub quitting: bool,
}

impl Gltk {
    pub fn quit(&mut self) {
        self.quitting = true;
    }
}

impl From<GltkBuilder> for Gltk {
    fn from(_value: GltkBuilder) -> Self {
        Self { quitting: false }
    }
}
