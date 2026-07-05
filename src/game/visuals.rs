use crate::game::combat::LaneManager;

use super::combat::UnitAction;

use super::robot::Unit;
use super::factory::Building;
use super::combat::Faction;
use macroquad::{prelude::*, texture};

#[derive(Debug)]
pub struct V_Unit{
    faction: Faction,
    x: f32,
    y: f32,
    lane: usize,
    stats: Unit,
    dmg_taken: f32,
    cur_action: UnitAction,
}

impl V_Unit {
    pub fn new(faction: Faction,x:f32,y:f32,lane: usize,stats: Unit) -> V_Unit{
        let d_y = rand::gen_range(-2.0, 2.0);
        V_Unit {faction, x, y: y + d_y, lane, stats ,dmg_taken: 0.0, cur_action: UnitAction::Wait}
    }
    pub fn move_unit(&mut self){
        match self.faction {
            Faction::Enemy =>{
                if (self.x - self.range()) < LaneManager::x_by_faction(&Faction::Player) {return}
                self.x -= self.stats.speed();
            }
            Faction::Player => {
                if (self.x + self.range()) > LaneManager::x_by_faction(&Faction::Enemy) {return}
                self.x += self.stats.speed();
            }
        }
    }
    pub fn pos(&self) -> (f32,f32){
        (self.x,self.y)
    }

    pub fn range(&self) -> f32{
        self.stats.range()
    }

    pub fn faction(&self) -> &Faction{
       &self.faction
    }

    pub fn dmg(&mut self) -> Option<f32>{
        self.stats.dmg()
    }

    pub fn update_action(&mut self, action: UnitAction){
        self.cur_action = action;
    }

    pub fn cur_action(&self) -> &UnitAction{
        &self.cur_action
    }

    pub fn is_fighting(&self) -> bool{
        self.cur_action == UnitAction::Fighting
    }

    pub fn is_running(&self) -> bool{
        self.cur_action == UnitAction::Running
    }

    pub fn take_damage(&mut self, dmg : f32) -> bool{
        self.dmg_taken += dmg;

        return self.dmg_taken > self.stats.health()
    }
    
    pub fn draw(&self, color: Color){
        let x = self.x - 8.0;
        let y = self.y - 16.0;
        draw_texture(self.stats.texture(), x, y, color);
    }
}

pub struct V_Building{
    faction:Faction,
    x: f32,
    y: f32,
    lane: usize,
    stats: Building,
    x_offset: f32,
}

impl V_Building{
    pub fn new(faction:Faction,x:f32,y:f32,lane:usize,stats:Building, x_offset: f32) -> V_Building{
        V_Building {faction, x, y, lane, stats , x_offset}
    }
    pub fn produce(&mut self) -> Option<V_Unit>{
        let p = self.stats.produce();

        if p.is_none() {return None}

        Some(V_Unit::new(self.faction.clone(),self.x, self.y, self.lane, p.unwrap()))
    }
    pub fn pos(&self) -> (f32,f32){
        (self.x,self.y)
    }

    pub fn faction(&self) -> &Faction{
        &self.faction
    }

    pub fn draw(&self){
        let x = self.x - 8.0 + self.x_offset;
        let y = self.y - 16.0;
        draw_texture(self.stats.texture(), x, y, WHITE);
    }
}
