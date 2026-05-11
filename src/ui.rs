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
        coins: i32,
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
        d.draw_text(&format!("Coins: {}", coins), 12, 122, 18, Color::GOLD);
        if shop_open {
            let panel_w = 360;
            let panel_h = 160;
            let panel_x = 300;
            let panel_y = 170;
            d.draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::DARKGRAY);
            d.draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, Color::GOLD);
            d.draw_text("SHOP", panel_x + 16, panel_y + 12, 24, Color::GOLD);
            d.draw_text(
                &format!("Coins: {}", coins),
                panel_x + 16,
                panel_y + 46,
                20,
                Color::RAYWHITE,
            );
            d.draw_text(
                "Press E to continue",
                panel_x + 16,
                panel_y + 80,
                18,
                Color::RAYWHITE,
            );
        }
    }
}
