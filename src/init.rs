use crate::Gltk;

pub struct GltkBuilder;

impl GltkBuilder {
    pub fn default() -> Gltk {
        Gltk { quitting: false }
    }
}
