use raylib::prelude::*;

pub struct SpriteAnimation {
    pub texture: Texture2D,
    pub frame_count: usize,
    pub frame_width: i32,
    pub frame_height: i32,
    pub fps: f32,
    pub timer: f32,
    pub current_frame: usize,
}

impl SpriteAnimation {
    pub fn new(texture: Texture2D, frame_count: usize, frame_width: i32, frame_height: i32, fps: f32) -> Self {
        Self {
            texture,
            frame_count,
            frame_width,
            frame_height,
            fps,
            timer: 0.0,
            current_frame: 0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        let frame_time = 1.0 / self.fps;
        if self.timer >= frame_time {
            self.current_frame = (self.current_frame + 1) % self.frame_count;
            self.timer -= frame_time;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, pos: Vector2, scale: f32, color: Color) {
        let src = Rectangle::new(
            ((self.current_frame as i32) * self.frame_width) as f32,
            0.0,
            self.frame_width as f32,
            self.frame_height as f32,
        );
        let dest = Rectangle::new(
            pos.x - (self.frame_width as f32 * scale) * 0.5,
            pos.y - (self.frame_height as f32 * scale) * 0.5,
            self.frame_width as f32 * scale,
            self.frame_height as f32 * scale,
        );
        d.draw_texture_pro(&self.texture, src, dest, Vector2::zero(), 0.0, color);
    }
}
