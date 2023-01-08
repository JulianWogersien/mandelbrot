use std::convert::TryInto;

use sfml::{graphics::{Drawable, Sprite, Image, Texture, Rect, Color}, SfBox, system::Vector2i};

use crate::math::math;

pub struct Mandelbrot {
    pixels: sfml::graphics::Image,
    tex: SfBox<Texture>,
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
        for i in 0..size_x {
            for j in 0..size_y {
                let x0: f32 = math::scale(size_x as f32, 0.0, -2.0, 0.47, i as f32);
                let y0: f32 = math::scale(size_y as f32, 0.0, -1.12, 1.12, i as f32);
                let mut x: f32 = 0.0;
                let mut y: f32 = 0.0;
                let mut iteration: i32 = 0;
                let max_iteration: i32 = 1000;
                while x*x + y*y <= 2.*2. && iteration < max_iteration {
                    let xtemp: f32 = x*x - y*y + x0;
                    y = 2.*x*y + y0;
                    x = xtemp;
                    iteration += 1;
                }
                let mut rgb: (u8, u8, u8) = (Color::BLACK.r, Color::BLACK.g, Color::BLACK.b);
                if iteration < max_iteration && iteration > 0 {
                    let l: i32 = iteration % 16;
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
                unsafe {
                    t.set_pixel(i as u32, j as u32, Color::rgb(rgb.0.try_into().unwrap(), rgb.1.try_into().unwrap(), rgb.2.try_into().unwrap()));
                }
            }
        }

        return Mandelbrot { pixels: t, tex };
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