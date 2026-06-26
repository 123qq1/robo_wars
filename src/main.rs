mod game;
use macroquad::{miniquad::window::set_window_size, prelude::*};

use game::Manager;

#[macroquad::main("Robo Wars")]
async fn main() {

    let mut window_s = (1500.0,1000.0);


    //set_fullscreen(true);

    let mut manager = Manager::new();

    loop {
        clear_background(GRAY);
        window_s = (screen_width(),screen_height());

        manager.step();

        next_frame().await
    }
}
