use std::convert::TryInto;
use std::thread;

use num::{Complex, complex::ComplexFloat};
use sfml::{graphics::{Drawable, Sprite, Image, Texture, Rect, Color}, SfBox, system::Vector2i};

pub struct Mandelbrot {
    pixels: sfml::graphics::Image,
    tex: SfBox<Texture>,
    max_iter: i32,
}

impl Mandelbrot {
    pub fn new(size_x: i32, size_y: i32) -> Self {
        let mut t: Image = Image::new(size_x.try_into().unwrap(), size_y.try_into().unwrap());
        let mut tex: SfBox<Texture> = match Texture::new() {
            Some(x) =>x,
            None => panic!("error creating texture"),
        };
        match tex.load_from_image(&t, Rect::from_vecs(Vector2i::new(0, 0), Vector2i::new(size_x, size_y))) {
            Ok(_) => (),
            Err(_) => panic!("error loading texture from image"),
        };
        
        let max_workers: i32 = 16;
        let mut workers: Vec<thread::JoinHandle<(i32, i32, f64)>> = Vec::new();
        let mut results: Vec<(i32, i32, f64)> = Vec::new();

        for i in 0..size_x {
            for j in 0..size_y {
                if workers.len() != max_workers.try_into().unwrap() {
                    workers.push(thread::spawn(move || {
                        let max_iter: i32 = 80;
                        let c: Complex<f64> = num::complex::Complex::new(-2.0 + (i as f64 / size_x as f64) * (1.0 - -2.0), -1.0 + (j as f64 / size_y as f64) * (1.0 - -1.0));
                
                        let n: f64 = Mandelbrot::run_mandelbrot(max_iter, c);
                        return (i, j, n)
                    }));
                }
                for thread in &workers {
                    let r: bool = thread.is_finished();
                    if r {
                        results.push(thread.join().unwrap())
                    }
                }
            }
        }
        
        for k in results {
            let max_iter: i32 = 80;
            let i: i32 = k.0;
            let j: i32 = k.1;
            let n: f64 = k.2;
            //let color: Color = Color::rgba(0, 0, 0, (255.0 - n * 255.0 / MAX_ITER as f32) as u8);

            let mut rgb: (u8, u8, u8) = (Color::BLACK.r, Color::BLACK.g, Color::BLACK.b);
            if n < max_iter as f64 && n > 0.0 {
                let l: i32 = (n % 16.0) as i32;
                rgb = match l {
                0 => (66, 30, 15),
                1 => (25, 7, 26),
                2 => (9, 1, 47),
                3 => (4, 4, 73),
                4 => (0, 7, 100),
                5 => (12, 44, 138),
                6 => (24, 82, 177),
                7 => (57, 125, 209),
                8 => (134, 181, 229),
                9 => (211, 236, 248),
                10 => (241, 233, 191),
                11 => (248, 201, 0),
                12 => (255, 170, 0),
                13 => (204, 128, 0),
                14 => (153, 87, 0),
                15 => (106, 52, 3),
                _ => (66, 30, 15),
                }
            }
            let color: Color = Color::rgb(rgb.0, rgb.1, rgb.2);
            unsafe {
                t.set_pixel(i as u32, j as u32, color);
            }
        }

        return Mandelbrot { pixels: t, tex, max_iter: 100};
    }

    fn run_mandelbrot(max_iter: i32, num: Complex<f64>) -> f64 {
        let mut z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
        let mut n: f64 = 0.0;
        while z.abs() <= 2.0 && n < max_iter as f64 {
            z = z*z + num;
            n += 1.0;
        }
        return n + 1.0 - z.abs().ln().ln() / 2.0.ln();
    }

    pub fn prepare_for_render(&mut self) {
        match self.tex.load_from_image(&self.pixels, Rect::from_vecs(Vector2i::new(0, 0), Vector2i::new(self.pixels.size().x as i32, self.pixels.size().y as i32))) {
            Ok(_) => (),
            Err(_) => panic!("error loading texture from image"),
        };
    }
}

impl Drawable for Mandelbrot {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn sfml::graphics::RenderTarget, states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,) {
        let mut spr: Sprite = Sprite::new();
        spr.set_texture(&self.tex, false);
        spr.draw(target, states);
        drop(spr);
    }
}
