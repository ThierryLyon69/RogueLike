use raylib::prelude::*;

pub struct Bullet {
    pos: Vector2,
    vel: Vector2,
    damage: i32,
    alive: bool,
}

impl Bullet {
    pub fn new(pos: Vector2, vel: Vector2, damage: i32) -> Self {
        Self {
            pos,
            vel,
            damage,
            alive: true,
        }
    }

    pub fn update(&mut self, dt: f32, bounds: Rectangle) {
        self.pos += self.vel * dt;
        if !bounds.check_collision_point_rec(self.pos) {
            self.alive = false;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let size = 6.0;
        let half = size * 0.5;
        let top_left = Vector2::new(self.pos.x - half, self.pos.y - half);
        d.draw_rectangle_v(top_left, Vector2::new(size, size), Color::YELLOW);
    }

    pub fn hitbox(&self) -> Rectangle {
        let size = 6.0;
        let half = size * 0.5;
        Rectangle::new(self.pos.x - half, self.pos.y - half, size, size)
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }
}
