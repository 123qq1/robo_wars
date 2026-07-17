use std::collections::HashMap;

use macroquad::color::WHITE;

use crate::game::ManagerAction;

use super::factory::Building;
use super::visuals::{VBuilding,VUnit};
use super::wall::WallManager;

const X_SIZE : f32 = 32.0;

#[derive(Debug,Clone,Eq, Hash, PartialEq)]
pub enum Faction {
    Player,
    Enemy,
}

#[derive(Debug,Clone,Eq, Hash, PartialEq)]
pub enum UnitAction{
    Wait,
    Running,
    Fighting,
    Sieging,
}

pub struct LaneManager{
    lanes: Vec<Lane>,
    wall_manager: WallManager, 
}

struct Lane{
    y : f32,
    buildings: Vec<VBuilding>,
    units: Vec<(usize,VUnit)>,
    forerunners: HashMap<Faction,usize>
}

impl LaneManager{
    pub async fn new(lane_count : usize) -> LaneManager{
        let mut v = Vec::new();

        for y in 0..lane_count {
            v.push(Lane::new(y as f32 * 50.0 + 50.0));
        }

        LaneManager { 
            lanes: v,
            wall_manager: WallManager::new(5000.0,5000.0).await,
        }
    }

    pub fn step(&mut self) -> ManagerAction{
        self.lanes.iter_mut().for_each(|l|{l.step()});
        let s_u = self.lanes.iter_mut().fold(Vec::new(), |mut v,l|{
            v.append(&mut l.do_sieges());
            v
        });
        let walls = self.wall_manager.step(s_u);

        let p_s = walls.player();
        let e_s = walls.enemy();

        match e_s {
            super::wall::WallState::Alive {..} => (),
            super::wall::WallState::Dead => return ManagerAction::Win,
        }
        match p_s {
            super::wall::WallState::Alive {..} => (),
            super::wall::WallState::Dead => return ManagerAction::Lose,
        }

        ManagerAction::Wait
    }

    pub fn add_building(&mut self,faction:Faction, lane: usize, b: Building){
        let lane_man = &mut self.lanes[lane];

        let building_count = lane_man.count_buildings(&faction);

        let y = lane_man.y;
        let x = LaneManager::x_by_faction(&faction);
        let x_offset = LaneManager::calc_x_offset(building_count as f32, &faction);

        let v_b = VBuilding::new(faction,x, y, lane, b,x_offset);
        lane_man.add_building(v_b);
    }

    fn calc_x_offset(building_count: f32, faction: &Faction) -> f32{
        if *faction == Faction::Player { -X_SIZE * (building_count as f32 + 1.0)}
        else {X_SIZE * (building_count as f32 + 1.0)}
    }

    pub fn x_by_faction(faction: &Faction)-> f32{
        match faction {
            Faction::Player => {return 100.0;}
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
            forerunners: HashMap::new(),
        }
    }

    fn count_buildings(&self, faction: &Faction) -> usize{
        self.buildings.iter().filter(|b|{*b.faction() == *faction}).count()
    }

    fn step(&mut self){        
        self.update_forerunners();
        self.update_fighters();
        
        self.fight();

        self.step_runners();

        let mut us = Vec::new();
        self.buildings.iter_mut().for_each(|b|{
            if let Some(u) = b.produce(){
                us.push(u);
            }
        });
        self.add_units(us);


        self.buildings.iter().for_each(|b|{b.draw();});
        self.units.iter().for_each(|(_,u)|{u.draw(WHITE);});
    }

    pub fn do_sieges(&mut self) -> Vec<VUnit>{
        let mut v_i = Vec::new();
        let mut v_u = Vec::new();

        for (i,v_u) in self.units.iter().rev() {
            if v_u.is_sieging() {
                v_i.push(*i);
            }
        }

        for i in v_i {
            v_u.push(self.units.remove(i).1);
        }

        self.update_indexes();

        v_u
    }

    fn step_runners(&mut self){
        self.units.iter_mut().filter(|(_,u)|{u.is_running()}).for_each(|(_,u)|{
            u.move_unit();
        });
    }

    fn fight(&mut self){
        let o_p_d = self.faction_calc_damage(Faction::Player, Faction::Enemy);
        let o_e_d = self.faction_calc_damage(Faction::Enemy, Faction::Player);

        self.apply_damage(o_p_d);
        self.apply_damage(o_e_d);
    }

    fn apply_damage(&mut self, dmg: Option<(usize,f32)>){
        if dmg == None {return;}

        let i_d = dmg.unwrap();

        let o_t= self.units.get_mut(i_d.0);

        match o_t {
            None => return,
            _ => (),
        }

        let target = &mut o_t.unwrap().1;

        if target.take_damage(i_d.1) {
            let f = target.faction();
            self.forerunners.remove(f);
            
            self.units.remove(i_d.0);
            self.update_indexes();
        }
    }

    fn update_indexes(&mut self){
        for (n_u,u) in self.units.iter_mut().enumerate() {
            u.0 = n_u;
        }
    }

    fn faction_calc_damage(&mut self, us: Faction, them : Faction, ) -> Option<(usize,f32)>{
        let target = self.forerunners.get(&them);
        if target == None {return None;}
        
        let t_i = target.unwrap();

        let mut dmg = 0.0;
        let f_s = self.units.iter_mut().filter(|(_,u)|{u.is_fighting() && *u.faction() == us});

        for (_,f) in f_s{
            if let Some(d) = f.dmg() {
                dmg += d;
            }
        }

        Some((*t_i,dmg))
    }

    fn add_building(&mut self, b: VBuilding){
        self.buildings.push(b);
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
        let candidates : Vec<&(usize, VUnit)> = self.units.iter().filter(|(_,u)|{*u.faction() == faction}).collect();

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

    fn forerunner_score(faction : &Faction, u: &VUnit) -> f32{
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
        if i == None {
            let u_p = self.units.iter_mut().filter(|(_,u)|{*u.faction() == us});
            u_p.for_each(|(_,u)|{
                u.update_action(UnitAction::Running);
            });
            return
        }

        /*
        for (_,_u) in &mut self.units {
            _u.update_action(UnitAction::Wait);
        }
        */

        let i = *i.unwrap();

        let x_2 = self.units[i].1.pos().0;
        let u_p = self.units.iter_mut().filter(|(_,u)|{*u.faction() == us});


        u_p.for_each(|(_,u)|{
            let x_1 = u.pos().0;

            let dif = x_2 - x_1;
            let r = u.range();
            
            if dif.abs() < r{
                u.update_action(UnitAction::Fighting);
            }
            
            else{
                u.update_action(UnitAction::Running);
            }
            

        });

    }

    fn add_units(&mut self,mut us: Vec<VUnit>){
        let i_1 = self.units.len();
        let i_2 = i_1 + us.len();
        let is : Vec<usize> = (i_1..i_2).collect();
        let mut nu: Vec<(usize, VUnit)> = Vec::new();

        for u in &mut us {
            u.update_action(UnitAction::Running);
        }

        for i in &is {
            nu.push((*i,us.pop().expect("other empty")));
        }
        
        self.units.append(&mut nu);
    }
}
