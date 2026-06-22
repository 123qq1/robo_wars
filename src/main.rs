mod game;
use macroquad::{miniquad::window::set_window_size, prelude::*};

use game::Manager;

#[macroquad::main("Robo Wars")]
async fn main() {

    let mut window_s = (1500.0,1000.0);


    //set_fullscreen(true);

    let mut manager = Manager::new();
    manager.test_buy(0);
    manager.test_buy(1);
    manager.test_buy(2);
    manager.test_buy_2(2);

    loop {
        clear_background(GRAY);
        window_s = (screen_width(),screen_height());

        manager.step();

        next_frame().await
    }
}
