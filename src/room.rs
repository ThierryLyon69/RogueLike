use raylib::prelude::*;

pub struct Room {
    pub bounds: Rectangle,
}

impl Room {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_lines(
            self.bounds.x as i32,
            self.bounds.y as i32,
            self.bounds.width as i32,
            self.bounds.height as i32,
            Color::DARKGRAY,
        );
    }
}
