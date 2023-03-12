use crate::{GResult, Gltk};

pub struct GltkBuilder {
    title: Option<String>,
}

impl GltkBuilder {
    pub fn new() -> Self {
        Self { title: None }
    }

    pub fn with_title<S: ToString>(mut self, title: S) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn build(self) -> GResult<Gltk> {
        let gltk = Gltk::from(self);
        Ok(gltk)
    }
}

impl Default for GltkBuilder {
    fn default() -> Self {
        Self::new()
    }
}
