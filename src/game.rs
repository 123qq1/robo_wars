
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
use {ui::TextPainter, ui::LanePainter};
use shop::ShopState;

use factory::Building;

pub struct GameManager{
    shop: Shop,
    player: PlayerStats,
    enemy: EnemyStats,
    lane_manager: LaneManager,
    text_painter: TextPainter,
    player_action: ManagerAction,
    enemy_action: ManagerAction,
    selected_lane: usize,
    lane_painter: LanePainter
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
    E_Build{shop_index: usize, lane_index : usize},
    P_Build(usize),
}

impl GameManager {
    pub fn new() -> GameManager{
        GameManager { 
            shop: Shop::new(), 
            player: PlayerStats::new(400,30,200), 
            enemy : EnemyStats::new(),
            lane_manager: LaneManager::new(4), 
            text_painter: TextPainter::new(),
            player_action: ManagerAction::Wait,
            enemy_action: ManagerAction::Wait,
            selected_lane: 0,
            lane_painter: LanePainter {  },
        }
    }

    pub fn step(&mut self){
        let m_s = GameManagerState::new(self);

        self.lane_manager.step();
        self.enemy_action = self.enemy.step(&m_s);
        self.player.step();
        self.player_action = self.shop.step();
        self.update_selected_lane();

        self.act();

        self.lane_painter.draw_lane_bounds();
        self.text_painter.paint_text(&self.player.text());
    }

    pub fn update_selected_lane(&mut self){
        let o_l = self.lane_painter.paint_lane_selector(self.selected_lane);

        match o_l {
            Some(i) => self.selected_lane = i,
            None => (),
        }
    }

    pub fn act(&mut self){
        match &self.enemy_action {
            ManagerAction::E_Build{shop_index:i,lane_index:l} => {
                let s = self.shop.enemy_buy(&mut self.enemy, *i);

                match s {
                    Status::Faliure(_) => {},
                    Status::Success(b) =>{
                        self.lane_manager.add_building(combat::Faction::Enemy, *l, b);
                    }
                }
                
            }
            _ => (),
        }

        match &self.player_action {
            ManagerAction::P_Build(i) => {
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
    }
}


