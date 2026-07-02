use macroquad::{prelude::*, ui::{hash, root_ui}};
use super::MenuAction;

const WINDOW_CORNER : Vec2 = vec2(20.0,20.0); 
const WINDOW_SIZE : Vec2 = vec2(200.0,200.0); 

const BUTTON_X : f32 = 20.0; 

pub struct MainMenuPainter{
    
}

impl MainMenuPainter {
    pub fn new() -> MainMenuPainter{
        MainMenuPainter {  }
    }

    pub fn step(&mut self, action : &mut MenuAction){

        root_ui().window(hash!(), WINDOW_CORNER, WINDOW_SIZE, |ui|{
            ui.label(vec2(BUTTON_X,20.0), "MAIN MENU");

            if ui.button(vec2(BUTTON_X,50.0), "PLAY"){
                *action = MenuAction::LevelMenuSelected;
            }
            
            if ui.button(vec2(BUTTON_X,80.0), "SETTINGS"){
                println!("Todo");
            }
            
            if ui.button(vec2(BUTTON_X,110.0), "QUIT"){
                *action = MenuAction::QuitSelected;
            }
        });
    }
}