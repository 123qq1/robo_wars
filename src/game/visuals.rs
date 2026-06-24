use super::combat::UnitAction;

use super::robot::Unit;
use super::factory::Building;
use super::combat::Faction;
use macroquad::prelude::*;

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
        V_Unit {faction, x, y, lane, stats ,dmg_taken: 0.0, cur_action: UnitAction::Wait}
    }
    pub fn move_unit(&mut self){
        match self.faction {
            Faction::Enemy =>{
                self.x -= self.stats.speed();
            }
            Faction::Player => {
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

    pub fn dmg(&self) -> f32{
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
        draw_circle(self.x, self.y, 5.0, color);
    }
}

pub struct V_Building{
    faction:Faction,
    x: f32,
    y: f32,
    lane: usize,
    stats: Building,
}

impl V_Building{
    pub fn new(faction:Faction,x:f32,y:f32,lane:usize,stats:Building) -> V_Building{
        V_Building {faction, x, y, lane, stats }
    }
    pub fn produce(&mut self) -> Option<V_Unit>{
        let p = self.stats.produce();

        if p.is_none() {return None}

        Some(V_Unit::new(self.faction.clone(),self.x, self.y, self.lane, p.unwrap()))
    }
    pub fn pos(&self) -> (f32,f32){
        (self.x,self.y)
    }

    pub fn draw(&self){
        let v1 = Vec2 { x: self.x+5.0, y: self.y };
        let v2 = Vec2 { x: self.x, y: self.y-10.0 };
        let v3 = Vec2 { x: self.x-5.0, y: self.y };
        draw_triangle(v1, v2, v3, GREEN);
    }
}
