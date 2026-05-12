use raylib::prelude::*;

use crate::shop::ShopItem;

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
        shop_offers: &[Option<ShopItem>],
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
            let panel_h = 220;
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
            let mut line_y = panel_y + 80;
            for (index, offer) in shop_offers.iter().enumerate() {
                let label = match offer {
                    Some(item) => format!("{} ) {} - {}c", index + 1, item.name, item.cost),
                    None => format!("{} ) SOLD", index + 1),
                };
                d.draw_text(&label, panel_x + 16, line_y, 18, Color::RAYWHITE);
                line_y += 22;
            }
            d.draw_text(
                "Press 1-3 to buy, E to continue",
                panel_x + 16,
                panel_y + panel_h - 28,
                16,
                Color::RAYWHITE,
            );
        }
    }
}
