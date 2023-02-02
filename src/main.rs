use application_manager::gman::Gm;

extern crate sfml;
extern crate num;

mod application_manager;
mod io;
mod traits;
mod mandelbrot;
mod math;
mod gui;
mod gui_traits;
mod colorscheme;

fn main() {
    let mut g_manager: Gm = Gm::new();
    g_manager.run();
}
