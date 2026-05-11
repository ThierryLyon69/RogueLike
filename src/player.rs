use raylib::prelude::*;

use crate::{bullet::Bullet, input::InputState, renderer::HeroAnimKind};

const PLAYER_MAX_HP: i32 = 5;

pub struct Player {
    pos: Vector2,
    speed: f32,
    pub hp: i32,
    upgrade: PlayerUpgrade,
    fire_timer: f32,
    fire_interval: f32,
    bullet_speed: f32,
    bullet_damage: i32,
    anim_timer: f32,
    anim_frame: usize,
    anim_frame_count: usize,
    anim_kind: HeroAnimKind,
    attack_anim_timer: f32,
    idle_count: usize,
    walk_count: usize,
    attack_count: usize,
    anim_frame_time: f32,
}

#[derive(Clone, Copy)]
enum PlayerUpgrade {
    None,
}

impl Player {
    pub fn new(pos: Vector2) -> Self {
        let anim_frame_time = env_f32("HERO_FRAME_TIME", 0.5).max(0.01);
        let idle_count = env_frames_len("HERO_IDLE_FRAMES", 1).max(1);
        let walk_count = env_frames_len("HERO_WALK_FRAMES", 1).max(1);
        let attack_count = env_frames_len("HERO_ATTACK_FRAMES", 1).max(1);
        let anim_frame_count = idle_count.max(1);
        Self {
            pos,
            speed: 220.0,
            hp: PLAYER_MAX_HP,
            upgrade: PlayerUpgrade::None,
            fire_timer: 0.0,
            fire_interval: 0.18,
            bullet_speed: 520.0,
            bullet_damage: 1,
            anim_timer: 0.0,
            anim_frame: 0,
            anim_frame_count,
            anim_kind: HeroAnimKind::Idle,
            attack_anim_timer: 0.0,
            idle_count,
            walk_count,
            attack_count,
            anim_frame_time,
        }
    }

    pub fn pos(&self) -> Vector2 {
        self.pos
    }

    pub fn damage(&mut self, dmg: i32) {
        self.hp -= dmg;
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(PLAYER_MAX_HP);
    }

    pub fn update(&mut self, input: &InputState, dt: f32, bounds: Rectangle) {
        let mut dir = input.move_dir;
        if dir.length() > 1.0 {
            dir = dir.normalized();
        }

        if input.shoot {
            let duration = (self.attack_count.max(1) as f32) * self.anim_frame_time;
            self.attack_anim_timer = duration.max(self.anim_frame_time);
        }

        if self.attack_anim_timer > 0.0 {
            self.attack_anim_timer = (self.attack_anim_timer - dt).max(0.0);
        }

        let next_kind = if self.attack_anim_timer > 0.0 {
            HeroAnimKind::Attack
        } else if dir.length() > 0.01 {
            HeroAnimKind::Walk
        } else {
            HeroAnimKind::Idle
        };
        if next_kind != self.anim_kind {
            self.anim_kind = next_kind;
            self.anim_frame = 0;
            self.anim_timer = 0.0;
            self.anim_frame_count = self.frame_count_for_kind();
        }

        self.pos += dir * self.speed * dt;

        self.anim_timer += dt;
        while self.anim_timer >= self.anim_frame_time {
            self.anim_timer -= self.anim_frame_time;
            self.anim_frame = (self.anim_frame + 1) % self.anim_frame_count;
        }

        if self.fire_timer > 0.0 {
            self.fire_timer -= dt;
        }

        let margin = 10.0;
        self.pos.x = self.pos.x.clamp(bounds.x + margin, bounds.x + bounds.width - margin);
        self.pos.y = self.pos.y.clamp(bounds.y + margin, bounds.y + bounds.height - margin);
    }

    pub fn try_shoot(&mut self, input: &InputState) -> Option<Bullet> {
        if !input.shoot || self.fire_timer > 0.0 {
            return None;
        }

        let dir = input.aim_pos - self.pos;
        if dir.length() < 0.01 {
            return None;
        }

        let attack_range = 220.0;
        let vel = dir.normalized() * self.bullet_speed;
        let spawn_offset = Vector2::new(6.0, 6.0);
        self.fire_timer = self.fire_interval;
        Some(Bullet::new(
            self.pos + spawn_offset,
            vel,
            self.bullet_damage,
            attack_range,
        ))
    }

    pub fn draw(&self, renderer: &crate::renderer::Renderer, d: &mut RaylibDrawHandle) {
        renderer.draw_player(d, self.pos, self.anim_kind, self.anim_frame);
    }

    pub fn upgrade_label(&self) -> &'static str {
        match self.upgrade {
            PlayerUpgrade::None => "None",
        }
    }
}

impl Player {
    fn frame_count_for_kind(&self) -> usize {
        match self.anim_kind {
            HeroAnimKind::Idle => self.idle_count,
            HeroAnimKind::Walk => self.walk_count,
            HeroAnimKind::Attack => self.attack_count,
        }
    }
}

fn env_f32(key: &str, default_value: f32) -> f32 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.trim().parse::<f32>().ok())
        .unwrap_or(default_value)
}

fn env_frames_len(key: &str, default_value: usize) -> usize {
    let raw = std::env::var(key).ok().unwrap_or_default();
    let count = raw
        .split(';')
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .count();
    if count == 0 { default_value } else { count }
}
