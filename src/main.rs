use application_manager::gman::Gm;
use colorscheme::Colorscheme;

extern crate sfml;
extern crate num;
extern crate yaml_rust;

mod application_manager;
mod io;
mod traits;
mod mandelbrot;
mod math;
mod gui;
mod gui_traits;
mod colorscheme;
mod textrenderer;

fn main() {
    let mut g_manager: Gm = Gm::new();
    g_manager.run();
}
