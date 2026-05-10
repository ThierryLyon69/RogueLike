use raylib::prelude::*;

use crate::input::InputState;

pub struct Player {
    pos: Vector2,
    speed: f32,
    pub hp: i32,
    upgrade: PlayerUpgrade,
}

#[derive(Clone, Copy)]
enum PlayerUpgrade {
    None,
}

impl Player {
    pub fn new(pos: Vector2) -> Self {
        Self {
            pos,
            speed: 220.0,
            hp: 5,
            upgrade: PlayerUpgrade::None,
        }
    }

    pub fn update(&mut self, input: &InputState, dt: f32, bounds: Rectangle) {
        let mut dir = input.move_dir;
        if dir.length() > 1.0 {
            dir = dir.normalized();
        }

        self.pos += dir * self.speed * dt;

        let margin = 10.0;
        self.pos.x = self.pos.x.clamp(bounds.x + margin, bounds.x + bounds.width - margin);
        self.pos.y = self.pos.y.clamp(bounds.y + margin, bounds.y + bounds.height - margin);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let size = 20.0;
        let half = size * 0.5;
        let top_left = Vector2::new(self.pos.x - half, self.pos.y - half);
        d.draw_rectangle_v(top_left, Vector2::new(size, size), Color::GREEN);
    }

    pub fn upgrade_label(&self) -> &'static str {
        match self.upgrade {
            PlayerUpgrade::None => "None",
        }
    }
}
