use std::fs;

use sfml::graphics::Color;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};


pub struct RgbColor{
    r: u8,
    g: u8,
    b: u8,
}

pub struct Colorscheme {
    outline: RgbColor,
    fill: RgbColor,
    text: RgbColor,
    hover: RgbColor,
    selected: RgbColor,
    opaqueness: u8,
}

impl RgbColor {
    pub fn new(r: u8, b: u8, g: u8) -> Self {
        return RgbColor { r, g, b }
    }
}

impl Colorscheme {
    pub fn new(scheme: &str, opaqueness: u8) -> Self {
        let outline: RgbColor = RgbColor::new(33, 31, 32);
        let fill: RgbColor = RgbColor::new(79, 75, 77);
        let text: RgbColor = RgbColor::new(255, 255, 255);
        let hover: RgbColor = RgbColor::new(28, 13, 20);
        let selected: RgbColor = RgbColor::new(12, 5, 9);
        return Colorscheme { outline, fill, text, hover, selected, opaqueness };
    }
}