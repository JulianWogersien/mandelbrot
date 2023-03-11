use std::convert::TryInto;
use std::thread;

use num::{Complex, complex::ComplexFloat};
use sfml::{graphics::{Drawable, Sprite, Image, Texture, Rect, Color}, SfBox, system::Vector2i};

use crate::math::math;

pub struct Mandelbrot {
    pixels: sfml::graphics::Image,
    tex: SfBox<Texture>,
    results: Vec<Vec<(i32, i32, (f64, Complex<f64>))>>,
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
        
        let max_worker: i32 = 8;
        let mut workers: Vec<thread::JoinHandle<Vec<(i32, i32, (f64, Complex<f64>))>>> = Vec::new();
        let mut results: Vec<Vec<(i32, i32, (f64, Complex<f64>))>> = Vec::new();

        for i in 0..max_worker {
            workers.push(thread::spawn(move || {
                let max_x: i32 = (i+1) * (size_x / max_worker);
                let mut results: Vec<(i32, i32, (f64, Complex<f64>))> = Vec::new();
                for x in (size_x / max_worker) * i..max_x {
                    for y in 0..size_y {
                        let max_iter: i32 = 80;
                        let c: Complex<f64> = num::complex::Complex::new(-2.0 + (x as f64 / size_x as f64) * (1.0 - -2.0), -1.0 + (y as f64 / size_y as f64) * (1.0 - -1.0));
        
                        let n: (f64, Complex<f64>) = Mandelbrot::run_mandelbrot(max_iter, c);
                        results.push((x, y, n));
                    }
                }
                return results
            })) 
        }
        
        workers.into_iter().for_each(|worker| results.push(worker.join().unwrap()));

        for i in &results {
            for j in i {
                let max_iter: i32 = 80;
                let x: i32 = j.0;
                let y: i32 = j.1;
                let n: (f64, Complex<f64>) = j.2;
                
                let mut rgb: (i32, i32, i32) = (0, 0, 0);
                if n.0 as i32 != max_iter {
                    rgb = Self::map_color(n.0, n.1.re(), n.1.im(), 90.0, 70.0, 10.0);
                }
                //let color: Color = Color::rgba(0, 0, 0, (255.0 - n.0 * 255.0 / max_iter as f64) as u8);
                /*let mut rgb: (u8, u8, u8) = (Color::BLACK.r, Color::BLACK.g, Color::BLACK.b);
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
                }*/
                let color: Color = Color::rgb(rgb.0 as u8, rgb.1 as u8, rgb.2 as u8);
                unsafe {
                    t.set_pixel(x as u32, y as u32, color);
                }
            }
        }

        return Mandelbrot { pixels: t, tex, results};
    }

    pub fn set_color(&mut self, value: f32, saturation: f32, modifier: f64) {
        for i in 0..self.results.len() {
            for j in 0..self.results[i].len() {
                let (x, y, (n, z)) = self.results[i][j];
                let (r, g, b) = Mandelbrot::map_color(n, z.re, z.im, value, saturation, modifier);
                unsafe {
                self.pixels.set_pixel(x.try_into().unwrap(), y.try_into().unwrap(), Color::rgb(r.try_into().unwrap(), g.try_into().unwrap(), b.try_into().unwrap()));
                }
            }
        }
    }

    fn map_color(di: f64, r: f64, c: f64, saturation: f32, value: f32, modifier: f64) -> (i32, i32, i32) {
        let mut hue: f64;

        let _zn: f64 = (r + c).sqrt();
        hue = di;
        hue = 0.95 + modifier * hue;
        while hue > 360.0
            {hue -= 360.0;}
        while hue < 0.0
            {hue += 360.0;}
        return math::hsv_to_rgb(hue as f32, saturation, value) 
    }

    fn run_mandelbrot(max_iter: i32, num: Complex<f64>) -> (f64, Complex<f64>) {
        let mut z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
        let mut n: f64 = 0.0;
        while z.abs() <= 2.0 && n < max_iter as f64 {
            z = z*z + num;
            n += 1.0;
        }
        return (n + 1.0 - z.abs().ln().ln() / 2.0.ln(), z);
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
