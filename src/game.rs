
mod robot;
mod factory;
mod visuals;
mod player;
mod shop;
mod combat;
mod enemy;
mod ui;

pub enum Status<T>{
    Success(T),
    Faliure(String),
}

use shop::Shop;
use player::PlayerStats;
use combat::LaneManager;
use enemy::EnemyStats;
use ui::TextPainter;
use shop::ShopState;

use crate::game::factory::Building;

pub struct Manager{
    shop: Shop,
    player: PlayerStats,
    enemy: EnemyStats,
    lane_manager: LaneManager,
    text_painter: TextPainter,
}

pub struct ManagerState{
    shop_state: ShopState,
}

impl ManagerState {
    pub fn new(man: &Manager)->ManagerState{
        ManagerState { 
            shop_state: ShopState::new(&man.shop), 
        }
    }
}

pub enum ManagerAction {
    Wait,
    E_Build(Building),
}

impl Manager {
    pub fn new() -> Manager{
        Manager { 
            shop: Shop::new(), 
            player: PlayerStats::new(400,30,50), 
            enemy : EnemyStats::new(300,30,50),
            lane_manager: LaneManager::new(4), 
            text_painter: TextPainter::new(),
        }
    }

    pub fn test_buy(&mut self, lane: usize){
        let s_b = self.shop.player_buy(&mut self.player, 0);
        match s_b{
            Status::Success(b) => {self.lane_manager.add_building(combat::Faction::Player, lane, b);}
            Status::Faliure(s) => {println!("{}",s);}
        }
    }

    pub fn test_buy_2(&mut self, lane: usize){
        let s_b = self.shop.player_buy(&mut self.player, 1);
        match s_b{
            Status::Success(b) => {self.lane_manager.add_building(combat::Faction::Player, lane, b);}
            Status::Faliure(s) => {println!("{}",s);}
        }
    }

    pub fn step(&mut self){
        let m_s = ManagerState::new(self);

        let mut cur_action;

        self.lane_manager.step();
        
        cur_action = self.enemy.step(&m_s);
        self.act(cur_action);

        self.player.step();
        self.shop.step();

        self.text_painter.paint_text(&self.enemy.text());
        self.text_painter.paint_text(&self.player.text());
        self.text_painter.paint_text(&self.shop.text());
    }

    pub fn act(&mut self, action: ManagerAction){
        match action {
            ManagerAction::E_Build(b) => {
                if let Some(i) = self.shop.get_index(b){
                    let s = self.shop.enemy_buy(&mut self.enemy, i);

                    match s {
                        Status::Faliure(_) => {},
                        Status::Success(b) =>{
                            self.lane_manager.add_building(combat::Faction::Enemy, 1, b);
                        }
                    }
                }
            }
            ManagerAction::Wait => {}
        }
    }
}


