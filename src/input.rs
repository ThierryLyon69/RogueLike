use raylib::prelude::*;

#[derive(Clone, Copy)]
pub struct InputState {
    pub move_dir: Vector2,
    pub shoot: bool,
    pub aim_pos: Vector2,
    pub shop_confirm: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            move_dir: Vector2::new(0.0, 0.0),
            shoot: false,
            aim_pos: Vector2::new(0.0, 0.0),
            shop_confirm: false,
        }
    }
}

impl InputState {
    pub fn read(rl: &mut RaylibHandle) -> Self {
        let mut dir = Vector2::new(0.0, 0.0);

        if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_Z) {
            dir.y -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            dir.y += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_Q) {
            dir.x -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            dir.x += 1.0;
        }

        Self {
            move_dir: dir,
            shoot: rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT),
            aim_pos: rl.get_mouse_position(),
            shop_confirm: rl.is_key_pressed(KeyboardKey::KEY_E),
        }
    }
}
