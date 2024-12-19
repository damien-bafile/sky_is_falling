use macroquad::prelude::*;

pub enum ObjectType {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
}

pub enum ObjectRol {
    Player,
    Enemy,
}

pub struct Object {
    pub loc: Vec2,
    pub color: Color,
    pub obj_type: ObjectType,
    pub obj_role: ObjectRol,
}

impl Object {
    pub fn draw(&self) {
        match self.obj_type {
            ObjectType::Rectangle { width, height } => {
                draw_rectangle(self.loc.x, self.loc.y, width, height, self.color);
            }
            ObjectType::Circle { radius } => {
                draw_circle(self.loc.x, self.loc.y, radius, self.color);
            }
        }
    }
}
