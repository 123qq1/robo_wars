use macroquad::{color::WHITE, texture::{Texture2D, draw_texture, load_texture}};

use crate::game::{combat::{Faction,LaneManager}, wall::WallStates};

pub struct WallPainter{
    alive_texture: Texture2D,
    dead_texture: Texture2D,
}

impl WallPainter {
    pub async fn new() ->WallPainter{
        
        let alive_texture = load_texture("Wall.png").await.unwrap();
        let dead_texture = load_texture("Wall_Dead.png").await.unwrap();
        
        WallPainter{
            alive_texture,
            dead_texture
        }
    }

    pub fn paint(&mut self, wall_states : &WallStates){

        let y = 0.0;
        let x_1 = LaneManager::x_by_faction(&Faction::Player) - 16.0;
        let x_2 = LaneManager::x_by_faction(&Faction::Enemy) - 16.0;

        match wall_states.player {
            super::WallState::Alive(_) => draw_texture(&self.alive_texture, x_1, y, WHITE),
            super::WallState::Dead => draw_texture(&self.dead_texture, x_1, y, WHITE),
        }
        
        match wall_states.enemy {
            super::WallState::Alive(_) => draw_texture(&self.alive_texture, x_2, y, WHITE),
            super::WallState::Dead => draw_texture(&self.dead_texture, x_2, y, WHITE),
        }
        
    }
}
