
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

pub struct Manager{
    shop: Shop,
    player: PlayerStats,
    enemy: EnemyStats,
    lane_manager: LaneManager,
    text_painter: TextPainter,
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
        let s_b = self.shop.buy(&mut self.player, 0);
        match s_b{
            Status::Success(b) => {self.lane_manager.add_building(combat::Faction::Player, lane, b);}
            Status::Faliure(s) => {println!("{}",s);}
        }
    }

    pub fn step(&mut self){
        self.lane_manager.step();
        self.enemy.step();
        self.player.step();
        self.shop.step();

        self.text_painter.paint_text(&self.enemy.text());
        self.text_painter.paint_text(&self.player.text());
        self.text_painter.paint_text(&self.shop.text());
    }
}


