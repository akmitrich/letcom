fn main() {
    let backend = cursive::backends::curses::n::Backend::init().unwrap();
    let siv = cursive::Cursive::default();
    let mut runner = cursive::CursiveRunner::new(siv, backend);
    loop {
        runner.step();
        if !runner.is_running() {
            break;
        }
    }
}
