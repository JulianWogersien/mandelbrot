use std::rc::Rc;

use sfml::{graphics::{RectangleShape, Rect, Transformable, Shape, Drawable, Color, Text, Font}, system::Vector2f, SfBox};

use crate::{gui_traits::GuiComponent, colorscheme::{Colorscheme, ColorSchemeNames}};



pub struct Gui {
    components: Vec<Box<Button>>,
    colorscheme: Colorscheme,
    font: Rc<SfBox<Font>>,
    font_size: u32,
    last_mouse_state: (bool, bool, bool),
}

impl Gui{
    pub fn new(font: &str, font_size: u32) -> Self {
        let font = Rc::new(sfml::graphics::Font::from_file(font).unwrap());
        return Gui { components: Vec::with_capacity(10), colorscheme: Colorscheme::new("default", 200), font, font_size, last_mouse_state: (false, false, false) };
    }

    pub fn update(&mut self, mouse_x: i32, mouse_y: i32, mouse_state: (bool, bool, bool)) {

        if !mouse_state.0 && self.last_mouse_state.0 {
            println!("{} {}", mouse_x, mouse_y);
            self.components.iter().for_each(|comp| {
                if comp.coordinate_inside(mouse_x as f32,  mouse_y as f32) {
                    (comp.callback)();
                }
            })
        }
        self.last_mouse_state = mouse_state;
    }

    pub fn add_button(&mut self, x: f32, y: f32, width: f32, height: f32, text: String, callback: impl Fn() + 'static) {
        Button::create(self, x, y, width, height, text, callback);
    }
}

impl Drawable for Gui {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn sfml::graphics::RenderTarget, states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>) {
        self.components.iter().for_each(|comp| comp.render(target, states, self));
    }

    pub fn add_button(&mut self, x: f32, y: f32, width: f32, height: f32, text: String, callback: impl Fn() + 'static) {
        Button::create(self, x, y, width, height, text, callback);
    }
}

struct Button {
    pos_x: f32,
    pos_y: f32,
    dim_width: f32,
    dim_height: f32,
    text: String,
    callback: Box<dyn Fn()>,
}

impl Button {
    pub fn create(gui: &mut Gui, x: f32, y: f32, width: f32, height: f32, text: String, callback: impl Fn()  + 'static) {
        let btn = Box::new(Button{pos_x: x, pos_y: y, dim_height: height, dim_width: width, text, callback: Box::new(callback)});
        gui.components.push(btn);
    }
}

impl GuiComponent for Button {
    fn render<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn sfml::graphics::RenderTarget, states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>, gui: &Gui) {
        let mut b: RectangleShape = RectangleShape::new();
        b.set_position(Vector2f::new(self.pos_x, self.pos_y));
        b.set_size(Vector2f::new(self.dim_width, self.dim_height));
        b.set_fill_color(gui.colorscheme.sfml_color(ColorSchemeNames::Fill));
        b.set_outline_color(gui.colorscheme.sfml_color(ColorSchemeNames::Outline));
        b.set_outline_thickness(1.0);
        target.draw(&b);
        let mut text = Text::new(&self.text, &gui.font, gui.font_size);
        text.set_origin(Vector2f::new(text.local_bounds().width / 2.0, text.local_bounds().height / 2.0));
        text.set_position(Vector2f::new(self.pos_x + self.dim_width / 2.0, self.pos_y + self.dim_height / 7.0));
        text.set_fill_color(gui.colorscheme.sfml_color(ColorSchemeNames::Text));
        target.draw(&text);
    }

    fn coordinate_inside(&self, x: f32, y: f32) -> bool {
        if x > self.pos_x && x > self.pos_x + self.dim_width && y > self.pos_y && y < self.pos_y + self.dim_height {
            true;
        }
        false
    }
}
