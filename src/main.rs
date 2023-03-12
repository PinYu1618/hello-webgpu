fn main() -> gltk::GError {
    use gltk::Gltk;

    let context = Gltk::default();

    gltk::main_loop(context)
}
