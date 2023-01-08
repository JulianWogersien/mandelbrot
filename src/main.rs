use application_manager::gman::Gm;

extern crate sfml;

mod application_manager;
mod io;
mod traits;
mod mandelbrot;
mod math;
fn main() {
    let mut g_manager: Gm = Gm::new();
    g_manager.run();
}
