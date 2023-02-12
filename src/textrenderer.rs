use sfml::{graphics::{Text, Font, Transformable, Color, RenderWindow, RenderTarget}, SfBox};

// textrenderer that uses sfml to render text
pub struct Textrenderer {
    font: SfBox<Font>,
    font_size: u32,

}

impl Textrenderer {
    pub fn new(font_size: u32, font_name: &str) -> Self {
        let font = Font::from_file(font_name).unwrap();
        return Textrenderer { font, font_size };
    }

    pub fn render(&mut self, text: String, x: f32, y: f32, window: &mut RenderWindow) {
        let mut text = Text::new(&text, &self.font, self.font_size);
        text.set_position((x, y));
        text.set_fill_color(Color::BLACK);
        text.set_outline_color(Color::WHITE);
        window.draw(&text);
    }
}