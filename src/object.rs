use macroquad::prelude::*;

pub struct Object {
    pub loc: Vec2,
    pub color: Color,
}

impl Object {
    pub fn draw(&self) {
        draw_circle(self.loc.x as f32, self.loc.y as f32, 15.0, self.color);
    }
}
