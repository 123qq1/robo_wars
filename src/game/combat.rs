use std::collections::HashMap;

use super::robot::Unit;
use super::factory::Building;
use super::visuals::{V_Building,V_Unit};

#[derive(Debug,Clone)]
pub enum Faction {
    Player,
    Enemy,
}

pub struct LaneManager{
    lanes: Vec<Lane>,
}

struct Lane{
    y : f32,
    buildings: Vec<V_Building>,
    runners: Vec<V_Unit>,
    fighters: Vec<V_Unit>,
    forerunners: HashMap<Faction,usize>
}

impl LaneManager{
    pub fn new(lane_count : usize) -> LaneManager{
        let mut v = Vec::new();

        for y in 0..lane_count {
            v.push(Lane::new(y as f32 * 50.0 + 50.0));
        }

        LaneManager { 
            lanes: v
        }
    }

    pub fn step(&mut self){
        self.lanes.iter_mut().for_each(|l|{l.step()});
    }

    pub fn add_building(&mut self,faction:Faction, lane: usize, b: Building){
        let y = self.lanes[lane].y;
        let x = self.x_by_faction(&faction);
        let v_b = V_Building::new(faction,x, y, lane, b);
        self.lanes[lane].add_building(v_b);
    }

    fn add_unit(&mut self,faction:Faction, lane: usize, u: Unit){
        let y = self.lanes[lane].y;
        let v_u = V_Unit::new(faction,50.0, y, lane, u);
        self.lanes[lane].add_unit(v_u);
    }

    fn x_by_faction(&self, faction: &Faction)-> f32{
        match faction {
            Faction::Player => {return 50.0; }
            Faction::Enemy  => {return 650.0;}
        }
    }
}

impl Lane{
    fn new(y: f32) -> Lane{
        Lane { 
            y,
            buildings: Vec::new(), 
            runners: Vec::new(),
            fighters: Vec::new(),
            forerunners: HashMap::new(),
        }
    }

    fn step(&mut self){
        self.runners.iter_mut().for_each(|u|{u.move_unit();});

        let mut us = Vec::new();
        self.buildings.iter_mut().for_each(|b|{
            if let Some(u) = b.produce(){
                us.push(u);
            }
        });
        self.add_units(&mut us);

        self.buildings.iter().for_each(|b|{b.draw();});
        self.runners.iter().for_each(|u|{u.draw();});
    }

    fn add_building(&mut self, b: V_Building){
        self.buildings.push(b);
    }

    fn add_unit(&mut self, u: V_Unit){
        self.runners.push(u);
    }

    fn add_units(&mut self, us:&mut Vec<V_Unit>){
        self.runners.append(us);
    }
}
