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

fn main() {
    let colors: Colorscheme = Colorscheme::new("colorscheme.yaml", 50);
    //let mut g_manager: Gm = Gm::new();
    //g_manager.run();
}
