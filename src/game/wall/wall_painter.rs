use macroquad::{color::WHITE, texture::{Texture2D, draw_texture, load_texture}};

use crate::game::combat::{Faction,LaneManager};

pub struct WallPainter{
    texture: Texture2D,
}

impl WallPainter {
    pub async fn new() ->WallPainter{
        
        let texture = load_texture("wall.png").await.unwrap();
        
        WallPainter{
            texture
        }
    }

    pub fn paint(&mut self){

        let y = 0.0;
        let x_1 = LaneManager::x_by_faction(&Faction::Player) - 16.0;
        let x_2 = LaneManager::x_by_faction(&Faction::Enemy) - 16.0;

        draw_texture(&self.texture, x_1, y, WHITE);
        draw_texture(&self.texture, x_2, y, WHITE);
    }
}
