use raylib::prelude::*;

pub struct Bullet {
    pos: Vector2,
    vel: Vector2,
    damage: i32,
    alive: bool,
    remaining_range: f32,
}

impl Bullet {
    pub fn new(pos: Vector2, vel: Vector2, damage: i32, max_range: f32) -> Self {
        Self {
            pos,
            vel,
            damage,
            alive: true,
            remaining_range: max_range.max(0.0),
        }
    }

    pub fn update(&mut self, dt: f32, bounds: Rectangle) {
        let step = self.vel * dt;
        self.pos += step;
        self.remaining_range -= step.length();
        if self.remaining_range <= 0.0 || !bounds.check_collision_point_rec(self.pos) {
            self.alive = false;
        }
    }

    pub fn draw(&self, renderer: &crate::renderer::Renderer, d: &mut RaylibDrawHandle) {
        renderer.draw_bullet(d, self.pos);
    }

    pub fn hitbox(&self) -> Rectangle {
        let size = 6.0;
        let half = size * 0.5;
        Rectangle::new(self.pos.x - half, self.pos.y - half, size, size)
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }
}
