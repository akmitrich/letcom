pub mod app;
pub mod ui;

fn main() {
    let app = app::App::new();
    app.go();
}
