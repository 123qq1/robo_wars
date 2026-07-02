mod game;
mod menu;
use macroquad::{miniquad::window::set_window_size, prelude::*};

use game::GameManager;
use menu::MenuManager;

#[macroquad::main("Robo Wars")]
async fn main() {

    let mut window_s = (1500.0,1000.0);


    //set_fullscreen(true);

    let mut game_manager = GameManager::new();
    let mut menu_manager = MenuManager::new();
    
    let mut cur_game_state= GameState::MainMenu;

    loop {
        clear_background(GRAY);
        window_s = (screen_width(),screen_height());

        match cur_game_state {
            GameState::MainMenu => {
                act(menu_manager.step_main(), &mut cur_game_state);
            }
            GameState::LevelMenu => {
                act(menu_manager.step_level(), &mut cur_game_state);
            }
            GameState::Level(l) => {
                game_manager.step();
            }
            GameState::Quit => {
                break;
            }
        }

        next_frame().await
    }
}

fn act(action : GameAction, game_state : &mut GameState){
     match action {
         GameAction::ChangeState(g) => {
            *game_state = g;
         }
         GameAction::Wait => (),
     }
}

pub enum GameState{
    MainMenu,
    LevelMenu,
    Level(usize),
    Quit,
}

pub enum GameAction {
    Wait,
    ChangeState(GameState),
}
