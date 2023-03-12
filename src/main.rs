fn main() -> gltk::GError {
    use gltk::GltkBuilder;

    let context = GltkBuilder::new().with_title("Hello.").build()?;

    gltk::main_loop(context)
}
