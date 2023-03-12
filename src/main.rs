use crayon::Crayon;

fn main() -> crayon::CError {
    let context = Crayon::default();

    crayon::main_loop(context)
}
