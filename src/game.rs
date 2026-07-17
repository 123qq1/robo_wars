
mod robot;
mod factory;
mod visuals;
mod player;
mod shop;
mod combat;
mod enemy;
mod ui;
mod wall;

pub enum Status<T>{
    Success(T),
    Faliure(String),
}

use macroquad::{prelude::*,color::{GREEN, RED}, math::Vec2, shapes::draw_rectangle, texture::DrawTextureParams, ui::{hash, root_ui}};
use shop::Shop;
use player::PlayerStats;
use combat::LaneManager;
use enemy::EnemyStats;
use crate::GameAction;

use {ui::TextPainter, ui::LanePainter};
use shop::ShopState;

use factory::Building;

pub struct GameManager{
    level: Option<usize>,
    shop: Shop,
    player: PlayerStats,
    enemy: EnemyStats,
    lane_manager: LaneManager,
    text_painter: TextPainter,
    player_action: ManagerAction,
    enemy_action: ManagerAction,
    selected_lane: usize,
    lane_painter: LanePainter,
    game_action: ManagerAction,
}

pub struct GameManagerState{
    shop_state: ShopState,
}

impl GameManagerState {
    pub fn new(man: &GameManager)->GameManagerState{
        GameManagerState { 
            shop_state: ShopState::new(&man.shop), 
        }
    }
}

pub enum ManagerAction {
    Wait,
    EBuild{building: Building, lane_index : usize},
    PBuild(usize),
    Win,
    Lose,
}

impl GameManager {
    pub async fn new(level:usize) -> GameManager{
        GameManager { 
            level: None,
            shop: Shop::new().await, 
            player: PlayerStats::new(level), 
            enemy : EnemyStats::new(level).await,
            lane_manager: LaneManager::new(4).await, 
            text_painter: TextPainter::new(),
            player_action: ManagerAction::Wait,
            enemy_action: ManagerAction::Wait,
            game_action: ManagerAction::Wait,
            selected_lane: 0,
            lane_painter: LanePainter {  },
        }
    }

    pub fn step(&mut self) -> GameAction{

        let m_s = GameManagerState::new(self);

        self.game_action = self.lane_manager.step();
        self.enemy_action = self.enemy.step(&m_s);
        self.player.step();
        self.player_action = self.shop.step();
        self.update_selected_lane();

        let g_a = self.act();

        //self.lane_painter.draw_lane_bounds();
        self.text_painter.paint_text(&self.player.text());

        g_a
    }

    pub fn update_selected_lane(&mut self){
        let o_l = self.lane_painter.paint_lane_selector(self.selected_lane);

        match o_l {
            Some(i) => self.selected_lane = i,
            None => (),
        }
    }

    pub fn act(&mut self) -> GameAction{
        match &self.enemy_action {
            ManagerAction::EBuild{building:b,lane_index:l} => {
                self.lane_manager.add_building(combat::Faction::Enemy, *l, b.clone());
                
            }
            _ => (),
        }

        match &self.player_action {
            ManagerAction::PBuild(i) => {
                let s = self.shop.player_buy(&mut self.player, *i);
                    
                match s {
                    Status::Faliure(_) => {},
                    Status::Success(b) =>{
                        self.lane_manager.add_building(combat::Faction::Player, self.selected_lane, b);
                    }
                }
                
            }
            _ => (),
        }

        match &self.game_action {
            ManagerAction::Win => return self.draw_end_window("Victory"),
            ManagerAction::Lose => return self.draw_end_window("Defeat"),
            _ => (),
        }

        GameAction::Wait
    }

    fn draw_end_window(&self,text: &str)-> GameAction{
        let mut action = GameAction::Wait;

        root_ui().window(
            hash!(), 
            vec2(300.0, 100.0), 
            vec2(70.0, 70.0), 
            |ui|{
                ui.label(vec2(10.0, 10.0), text);
                    if ui.button(vec2(10.0, 30.0), "Menu"){
                        action = GameAction::ChangeState(crate::GameState::LevelMenu);
                    }
                });

        action
    }

    pub fn default_texture_params(x : f32, y : f32) -> DrawTextureParams{
        DrawTextureParams { 
            dest_size: Some(Vec2{x,y}), 
            ..Default::default()
        }
    }

    pub fn draw_life_bar(x:f32,y:f32, w:f32,max_life : f32,cur_damage : f32, height:f32,hide:bool){
        if cur_damage == 0.0 && hide {return;}
        
        let n_max = (w*max_life)/max_life;
        let n_dmg = (w*cur_damage)/max_life;

        let x = x - n_max/2.0;
        let dx = n_max - n_dmg;
        
        draw_rectangle(x, y, n_max, height, GREEN);
        draw_rectangle(x+dx, y, n_dmg, height, RED);
    }
}


