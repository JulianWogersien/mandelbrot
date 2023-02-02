use std::rc::Rc;

use sfml::{graphics::{RectangleShape, Rect, Transformable}, system::Vector2f};

use crate::gui_traits::GuiComponent;



pub struct Gui {
    components: Vec<Box<Button>>,
}

impl Gui{
    pub fn new() -> Self {
        return Gui { components: Vec::with_capacity(10) }
    }

    pub fn update(&self, mouse_x: i32, mouse_y: i32, mouse_state: (bool, bool, bool)) {
        self.components.iter().for_each(|comp| {
            if mouse_state.0 && comp.coordinate_inside(mouse_x as f32,  mouse_y as f32) {
                (comp.callback)();
            }
        })
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
        
    }

    fn coordinate_inside(&self, x: f32, y: f32) -> bool {
        if x > self.pos_x && x > self.pos_x + self.dim_width && y > self.pos_y && y < self.pos_y + self.dim_height {
            true;
        }
        false
    }
}