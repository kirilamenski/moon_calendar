use piston_window::{*, types::{Rectangle, Color}};
use std::path::Path;

pub struct Object {
    texture: G2dTexture,
    rect: [f64; 4],
    color: [f64; 4],
}

impl Object {
    pub fn new(texture: G2dTexture, rect: [f64; 4], color: [f64; 4]) -> Object {
        Object {
            texture,
            rect,
            color,
        }
    }

    pub fn get_rect(&self) -> [f64; 4] {
        self.rect
    }

    pub fn draw(&self, ctx: &Context, g2d: &mut G2d) {
        let image = Image::new().rect(self.rect);
        image.draw(
            &self.texture,
            &DrawState::default(),
            ctx.transform,
            g2d,
        );
    }
}