use macroquad::{prelude::*, ui::{hash, root_ui}};
use super::MenuAction;

const WINDOW_CORNER : Vec2 = vec2(20.0,20.0); 
const WINDOW_SIZE : Vec2 = vec2(200.0,200.0); 

const BUTTON_X : f32 = 20.0; 

pub struct LevelMenuPainter{
    
}

impl LevelMenuPainter {
    pub fn new() -> LevelMenuPainter{
        LevelMenuPainter {  }
    }

    pub fn step(&mut self, action : &mut MenuAction){

        root_ui().window(hash!(), WINDOW_CORNER, WINDOW_SIZE, |ui|{
            ui.label(vec2(BUTTON_X,20.0), "LEVELS");

            if ui.button(vec2(100.0,20.0), "BACK"){
                *action = MenuAction::MainMenuSelected;
            }

            for i in 0..1 {
                let y = 50.0 + i as f32 * 30.0;
                if ui.button(vec2(BUTTON_X,y), format!("Level {}",i + 1)){
                    *action = MenuAction::LevelSelected(i);
                }
            }
            
        });
    }
}