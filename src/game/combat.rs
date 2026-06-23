use std::collections::HashMap;

use macroquad::color::{BLUE, GREEN, RED, YELLOW};

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
    runners: HashMap<Faction,Vec<usize>>,
    fighters: HashMap<Faction,Vec<usize>>,
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
        let mut fighters = HashMap::new();
        fighters.insert(Faction::Player, Vec::new());
        fighters.insert(Faction::Enemy, Vec::new());
        
        let mut runners = HashMap::new();
        runners.insert(Faction::Player, Vec::new());
        runners.insert(Faction::Enemy, Vec::new());

        Lane { 
            y,
            buildings: Vec::new(), 
            units: Vec::new(),
            fighters,
            runners,
            forerunners: HashMap::new(),
        }
    }

    fn step(&mut self){
        self.step_runners();

        let mut us = Vec::new();
        self.buildings.iter_mut().for_each(|b|{
            if let Some(u) = b.produce(){
                us.push(u);
            }
        });
        self.add_units(us);

        self.update_forerunners();
        self.update_fighters();

        self.buildings.iter().for_each(|b|{b.draw();});
        self.units.iter().for_each(|(i,u)|{self.debug_draw_units_different_color(i,u);});
    }

    fn step_runners(&mut self){
        for (_,v) in &self.runners {
            for i in v {
                self.units[*i].1.move_unit();
            }
        }
    }

    fn is_forerunner(&self, i_1 : &usize, u : &V_Unit) -> bool{
        if let Some(i_2) = self.forerunners.get(u.faction()){
            return i_1 == i_2;
        }

        false
    }

    fn debug_draw_units_different_color(&self ,i : &usize,u : &V_Unit){
        if self.is_forerunner(i, u){
            u.draw(GREEN);
        }
        else if self.fighters.get(u.faction()).unwrap().contains(i){
            u.draw(RED);
        }
        else if self.runners.get(u.faction()).unwrap().contains(i){
            u.draw(YELLOW);
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
        let f = u.faction();
        self.runners.get_mut(f).unwrap().push(i);
        self.units.push((i,u));
    }

    fn update_forerunners(&mut self){
        self.forerunners.clear();

        if let Some(p) = self.find_forerunner(Faction::Player){
            self.forerunners.insert(Faction::Player, p); 
        }

        if let Some(e) = self.find_forerunner(Faction::Enemy){
            self.forerunners.insert(Faction::Enemy, e);
        }
    }

    fn find_forerunner(&self, faction: Faction) -> Option<usize>{
        let candidates : Vec<&(usize, V_Unit)> = self.units.iter().filter(|(i,u)|{*u.faction() == faction}).collect();

        if candidates.len() == 0 {return None;}

        let mut  winner = (0,0.0); 
        candidates.iter().for_each(|(i,u)|{
            let s = Lane::forerunner_score(&faction, u);
            if winner.1 < s {
                winner = (*i,s);
            }
        });
        Some(winner.0)
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

        self.fighters.get_mut(&us).unwrap().clear();
        self.runners.get_mut(&us).unwrap().clear();

        let u_p = self.units.iter().filter(|(_,u)|{*u.faction() == us});

        let i = *i.unwrap();

        let f_p = &self.units[i];

        u_p.for_each(|(i,u)|{
            let x_1 = u.pos().0;
            let x_2 = f_p.1.pos().0;
            let dif = x_2 - x_1;
            let r = u.range();
            
            //println!("{}",dif);
            if dif.abs() < r{
                self.fighters.get_mut(&us).unwrap().push(*i);
            }
            else{
                self.runners.get_mut(&us).unwrap().push(*i);
            }

        });

    }

    fn add_units(&mut self,mut us: Vec<V_Unit>){
        let i_1 = self.units.len();
        let i_2 = i_1 + us.len();
        let is : Vec<usize> = (i_1..i_2).collect();
        let mut nu: Vec<(usize, V_Unit)> = Vec::new();

        for i in &is {
            nu.push((*i,us.pop().expect("other empty")));
        }
        
        self.units.append(&mut nu);
        is.iter().for_each(|i|{
            let f = self.units[*i].1.faction();
            self.runners.get_mut(f).unwrap().push(*i);
        });
    }
}
