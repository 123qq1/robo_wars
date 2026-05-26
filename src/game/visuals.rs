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
}

impl V_Unit {
    pub fn new(faction: Faction,x:f32,y:f32,lane: usize,stats: Unit) -> V_Unit{
        V_Unit {faction, x, y, lane, stats }
    }
    pub fn move_unit(&mut self){
        self.x += self.stats.speed();
    }
    pub fn pos(&self) -> (f32,f32){
        (self.x,self.y)
    }
    
    pub fn draw(&self){
        draw_circle(self.x, self.y, 5.0, BLUE);
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
