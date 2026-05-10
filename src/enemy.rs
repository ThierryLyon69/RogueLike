use raylib::prelude::*;

pub enum EnemyKind {
    Chaser,
    Shooter,
}

pub struct Enemy {
    pos: Vector2,
    hp: i32,
    kind: EnemyKind,
}

impl Enemy {
    pub fn new(kind: EnemyKind, pos: Vector2) -> Self {
        Self { pos, hp: 3, kind }
    }

    pub fn update(&mut self, _player_pos: Vector2, _dt: f32) {}

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let size = 18.0;
        let half = size * 0.5;
        let top_left = Vector2::new(self.pos.x - half, self.pos.y - half);
        let color = match self.kind {
            EnemyKind::Chaser => Color::ORANGE,
            EnemyKind::Shooter => Color::PURPLE,
        };

        d.draw_rectangle_v(top_left, Vector2::new(size, size), color);
    }

    pub fn hitbox(&self) -> Rectangle {
        let size = 18.0;
        let half = size * 0.5;
        Rectangle::new(self.pos.x - half, self.pos.y - half, size, size)
    }

    pub fn damage(&mut self, dmg: i32) {
        self.hp -= dmg;
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}
