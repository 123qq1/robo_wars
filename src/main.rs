mod game;
mod menu;
use macroquad::prelude::*;

use game::GameManager;
use menu::MenuManager;

#[macroquad::main("Robo Wars")]
async fn main() {

    let mut window_s = (1500.0,1000.0);

    set_pc_assets_folder("assets");


    //set_fullscreen(true);

    let mut game_manager = GameManager::new(0).await;
    let mut menu_manager = MenuManager::new();
    
    let mut cur_game_state= GameState::MainMenu;

    build_textures_atlas();

    loop {
        clear_background(GRAY);
        window_s = (screen_width(),screen_height());

        let mut action = GameAction::Wait;

        match cur_game_state {
            GameState::MainMenu => {
                action = menu_manager.step_main();
            }
            GameState::LevelMenu => {
                action = menu_manager.step_level();
            }
            GameState::Level(_) => {
                action = game_manager.step();
            }
            GameState::Quit => {
                break;
            }
        }

        match &action {
            GameAction::ChangeState(g) =>
            match g {
                GameState::Level(l) => {
                    game_manager = GameManager::new(*l).await;
                },
                _ => (),
            }
            _ => (),
        }

        act(action,&mut cur_game_state);

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
