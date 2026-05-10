mod bullet;
mod collision;
mod enemy;
mod game;
mod input;
mod player;
mod renderer;
mod room;
mod ui;
mod upgrade;

use game::Game;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(960, 540)
        .title("RogueLike Prototype")
        .vsync()
        .build();

    let mut game = Game::new();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let fps = rl.get_fps() as i32;

        game.handle_input(&mut rl);
        game.update(dt);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        game.render(&mut d, fps);
    }
}
