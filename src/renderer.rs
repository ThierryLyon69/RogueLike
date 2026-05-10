use raylib::prelude::*;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn draw_player(&self, d: &mut RaylibDrawHandle, pos: Vector2) {
        let size = 20.0;
        let half = size * 0.5;
        let top_left = Vector2::new(pos.x - half, pos.y - half);
        d.draw_rectangle_v(top_left, Vector2::new(size, size), Color::GREEN);
    }
}
