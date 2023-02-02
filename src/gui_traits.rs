use crate::gui::Gui;

pub trait GuiComponent {
    fn render<'a: 'shader, 'texture, 'shader, 'shader_texture>(&'a self, target: &mut dyn sfml::graphics::RenderTarget, states: &sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>, gui: &Gui);
    fn coordinate_inside(&self, x: f32, y: f32) -> bool;
    
}