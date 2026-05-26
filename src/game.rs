
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
use ui::Painter;

pub struct Manager{
    shop: Shop,
    player: PlayerStats,
    enemy: EnemyStats,
    laneManager: LaneManager,
    painter: Painter,
}

impl Manager {
    pub fn new() -> Manager{
        Manager { 
            shop: Shop::new(), 
            player: PlayerStats::new(400,30,50), 
            enemy : EnemyStats::new(300,30,50),
            laneManager: LaneManager::new(4), 
            painter: Painter {},
        }
    }

    pub fn test_buy(&mut self, lane: usize){
        let s_b = self.shop.buy(&mut self.player, 0);
        match s_b{
            Status::Success(b) => {self.laneManager.add_building(combat::Faction::Player, lane, b);}
            Status::Faliure(s) => {println!("{}",s);}
        }
    }

    pub fn step(&mut self){
        self.laneManager.step();
        self.enemy.step();
        self.player.step();

        self.painter.paint(&self.enemy, 30.0, 200.0);
        self.painter.paint(&self.player, 30.0, 250.0);
    }
}


