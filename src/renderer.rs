use raylib::prelude::*;
use std::env;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HeroAnimKind {
    Idle,
    Walk,
    Attack,
}

pub struct Renderer {
    pub hero_idle_frames: Vec<Texture2D>,
    pub hero_walk_frames: Vec<Texture2D>,
    pub hero_attack_frames: Vec<Texture2D>,
    pub bullet_texture: Option<Texture2D>,
    pub coin_texture: Option<Texture2D>,
}

impl Renderer {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let hero_idle_frames = load_textures_env(
            rl,
            thread,
            "HERO_IDLE_FRAMES",
            "assets/sprites/frames/frame_0_0.png;assets/sprites/frames/frame_0_1.png",
        );
        let hero_walk_frames = load_textures_env(
            rl,
            thread,
            "HERO_WALK_FRAMES",
            "assets/sprites/frames/frame_1_0.png;assets/sprites/frames/frame_1_1.png",
        );
        let hero_attack_frames = load_textures_env(
            rl,
            thread,
            "HERO_ATTACK_FRAMES",
            "assets/sprites/frames/frame_2_0.png;assets/sprites/frames/frame_2_1.png",
        );
        let bullet_texture = load_texture_env(
            rl,
            thread,
            "BULLET_SPRITE",
            "assets/sprites/frames/frame_3_0.png",
        );
        let coin_texture = load_texture_env(
            rl,
            thread,
            "COIN_SPRITE",
            "assets/sprites/frames/frame_1_2.png",
        );
        Self {
            hero_idle_frames,
            hero_walk_frames,
            hero_attack_frames,
            bullet_texture,
            coin_texture,
        }
    }

    pub fn draw_player(
        &self,
        d: &mut RaylibDrawHandle,
        pos: Vector2,
        anim: HeroAnimKind,
        frame_index: usize,
    ) {
        let frames = self.hero_frames(anim);
        if !frames.is_empty() {
            let tex = &frames[frame_index % frames.len()];
            let target_size = 64.0;
            let top_left = Vector2::new(pos.x - target_size * 0.5, pos.y - target_size * 0.5);
            let scale_x = target_size / tex.width() as f32;
            let scale_y = target_size / tex.height() as f32;
            d.draw_texture_ex(tex, top_left, 0.0, scale_x.min(scale_y), Color::WHITE);
        } else {
            let size = 20.0;
            let half = size * 0.5;
            let top_left = Vector2::new(pos.x - half, pos.y - half);
            d.draw_rectangle_v(top_left, Vector2::new(size, size), Color::GREEN);
        }
    }

    pub fn draw_bullet(&self, d: &mut RaylibDrawHandle, pos: Vector2) {
        if let Some(tex) = &self.bullet_texture {
            let target_size = 32.0;
            let top_left = Vector2::new(pos.x - target_size * 0.5, pos.y - target_size * 0.5);
            let scale_x = target_size / tex.width() as f32;
            let scale_y = target_size / tex.height() as f32;
            d.draw_texture_ex(tex, top_left, 0.0, scale_x.min(scale_y), Color::WHITE);
        } else {
            let size = 6.0;
            let half = size * 0.5;
            let top_left = Vector2::new(pos.x - half, pos.y - half);
            d.draw_rectangle_v(top_left, Vector2::new(size, size), Color::YELLOW);
        }
    }
}

impl Renderer {
    fn hero_frames(&self, anim: HeroAnimKind) -> &Vec<Texture2D> {
        match anim {
            HeroAnimKind::Idle => &self.hero_idle_frames,
            HeroAnimKind::Walk => &self.hero_walk_frames,
            HeroAnimKind::Attack => &self.hero_attack_frames,
        }
    }
}

fn load_textures_env(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    key: &str,
    default_paths: &str,
) -> Vec<Texture2D> {
    let raw = env::var(key).unwrap_or_else(|_| default_paths.to_string());
    let paths = parse_paths(&raw);
    let mut textures = Vec::new();
    for path in paths {
        if let Ok(tex) = rl.load_texture(thread, &path) {
            textures.push(tex);
        }
    }
    textures
}

fn load_texture_env(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    key: &str,
    default_path: &str,
) -> Option<Texture2D> {
    let path = env::var(key)
        .ok()
        .map(|value| value.trim().trim_matches('"').to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default_path.to_string());
    rl.load_texture(thread, &path).ok()
}

fn parse_paths(raw: &str) -> Vec<String> {
    raw.split(';')
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .collect()
}
