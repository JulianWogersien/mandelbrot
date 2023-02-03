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
        outline: RgbColor = RgbColor::new(r, b, g)
    }
}