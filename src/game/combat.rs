use std::collections::HashMap;

use macroquad::color::{BLUE, GREEN};

use super::robot::Unit;
use super::factory::Building;
use super::visuals::{V_Building,V_Unit};

#[derive(Debug,Clone,Eq, Hash, PartialEq)]
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
    units: Vec<(usize,V_Unit)>,
    runners: Vec<usize>,
    fighters: Vec<usize>,
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
        let x = LaneManager::x_by_faction(&faction);
        let v_b = V_Building::new(faction,x, y, lane, b);
        self.lanes[lane].add_building(v_b);
    }

    fn add_unit(&mut self,faction:Faction, lane: usize, u: Unit){
        let y = self.lanes[lane].y;
        let v_u = V_Unit::new(faction,50.0, y, lane, u);
        self.lanes[lane].add_unit(v_u);
    }

    fn x_by_faction(faction: &Faction)-> f32{
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
            units: Vec::new(),
            fighters: Vec::new(),
            runners: Vec::new(),
            forerunners: HashMap::new(),
        }
    }

    fn step(&mut self){
        self.units.iter_mut().for_each(|(_,u)|{u.move_unit();});

        let mut us = Vec::new();
        self.buildings.iter_mut().for_each(|b|{
            if let Some(u) = b.produce(){
                us.push(u);
            }
        });
        self.add_units(us);

        self.update_forerunners();

        self.buildings.iter().for_each(|b|{b.draw();});
        self.units.iter().for_each(|(i,u)|{self.debug_draw_forerunner_other_color(i,u);});
    }

    fn is_forerunner(&self, i_1 : &usize, u : &V_Unit) -> bool{
        if let Some(i_2) = self.forerunners.get(u.faction()){
            return i_1 == i_2;
        }

        false
    }

    fn debug_draw_forerunner_other_color(&self ,i : &usize,u : &V_Unit){
        if self.is_forerunner(i, u){
            u.draw(GREEN);
        }
        else{
            u.draw(BLUE);
        }
    }

    fn add_building(&mut self, b: V_Building){
        self.buildings.push(b);
    }

    fn add_unit(&mut self, u: V_Unit){
        let i = self.units.len();
        self.units.push((i,u));
        self.runners.push(i);
    }

    fn update_forerunners(&mut self){
        self.forerunners.insert(Faction::Player, self.find_forerunner(Faction::Player)); 
        self.forerunners.insert(Faction::Enemy, self.find_forerunner(Faction::Enemy));
    }

    fn find_forerunner(&self, faction: Faction) -> usize{
        let runners  =  &self.runners;
        let candidates = self.units.iter().filter(|(i,u)|{(*u.faction() == faction) && runners.contains(i)});
        let mut  winner = (1,0.0); 
        candidates.for_each(|(i,u)|{
            let s = Lane::forerunner_score(&faction, u);
            if winner.1 < s {
                winner = (*i,s);
            }
        });
        winner.0
    }

    fn forerunner_score(faction : &Faction, u: &V_Unit) -> f32{
        match faction {
            Faction::Player => {u.pos().0 - LaneManager::x_by_faction(&faction) },
            Faction::Enemy => {LaneManager::x_by_faction(&faction) - u.pos().0},
        }
    }

    fn update_fighters(&mut self){
        self.update_faction_fighter(Faction::Player,Faction::Enemy);
        self.update_faction_fighter(Faction::Enemy,Faction::Player);
    }

    fn update_faction_fighter(&mut self, us : Faction, them : Faction){
        
        let i = self.forerunners.get(&them);
        if i == None {return}

        let u_p = self.units.iter().filter(|(_,u)|{*u.faction() == us});

        let i = *i.unwrap();

        let f_p = &self.units[i];

        let f_u = u_p.filter(|(_,u)|{
            let x_1 = u.pos().0;
            let x_2 = f_p.1.pos().0;
            let dif = x_1 - x_2;
            let r = u.range();

            (dif < r) && (-r > dif)

        });


    }

    fn add_units(&mut self,mut us: Vec<V_Unit>){
        let i_1 = self.units.len();
        let i_2 = i_1 + us.len();
        let mut is : Vec<usize> = (i_1..i_2).collect();
        let mut nu: Vec<(usize, V_Unit)> = Vec::new();

        for i in &is {
            nu.push((*i,us.pop().expect("other empty")));
        }

        self.units.append(&mut nu);
        self.runners.append(&mut is);
    }
}
