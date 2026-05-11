use raylib::prelude::*;

pub struct Ui;

impl Ui {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(
        &self,
        d: &mut RaylibDrawHandle,
        fps: i32,
        hp: i32,
        enemy_count: i32,
        upgrade_label: &str,
        wave: i32,
        shop_open: bool,
    ) {
        d.draw_text(&format!("HP: {}", hp), 12, 10, 20, Color::RAYWHITE);
        d.draw_text(&format!("FPS: {}", fps), 12, 34, 18, Color::RAYWHITE);
        d.draw_text(
            &format!("Enemies: {}", enemy_count),
            12,
            56,
            18,
            Color::RAYWHITE,
        );
        d.draw_text(
            &format!("Upgrade: {}", upgrade_label),
            12,
            78,
            18,
            Color::RAYWHITE,
        );
        d.draw_text(&format!("Wave: {}", wave), 12, 100, 18, Color::RAYWHITE);
        if shop_open {
            d.draw_text("Shop: OPEN (E)", 12, 122, 18, Color::GOLD);
        }
    }
}
