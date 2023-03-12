fn main() -> gltk::GError {
    use gltk::GltkBuilder;

    let context = GltkBuilder::default();

    gltk::main_loop(context)
}
