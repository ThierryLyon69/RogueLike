use raylib::prelude::*;

use crate::{
    bullet::Bullet, enemy::Enemy, input::InputState, player::Player, room::Room, ui::Ui,
};

pub struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    room: Room,
    ui: Ui,
    input: InputState,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(Vector2::new(480.0, 270.0)),
            enemies: Vec::new(),
            bullets: Vec::new(),
            room: Room::new(0.0, 0.0, 960.0, 540.0),
            ui: Ui::new(),
            input: InputState::default(),
            game_over: false,
        }
    }

    pub fn handle_input(&mut self, rl: &mut RaylibHandle) {
        self.input = InputState::read(rl);
    }

    pub fn update(&mut self, dt: f32) {
        if self.game_over {
            return;
        }

        self.player.update(&self.input, dt, self.room.bounds);

        if self.player.hp <= 0 {
            self.game_over = true;
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, fps: i32) {
        self.room.draw(d);
        self.player.draw(d);

        for enemy in &self.enemies {
            enemy.draw(d);
        }

        for bullet in &self.bullets {
            bullet.draw(d);
        }

        self.ui.draw(
            d,
            fps,
            self.player.hp,
            self.enemies.len() as i32,
            self.player.upgrade_label(),
        );

        if self.game_over {
            d.draw_text("GAME OVER", 360, 250, 24, Color::RED);
        }
    }
}
