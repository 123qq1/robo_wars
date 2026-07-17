use macroquad::time::get_frame_time;

use super::{GameManagerState, ManagerAction, factory::Building};

mod ai_settings;

mod enemy_buildings;

pub struct EnemyStats{
    ai : EnemyAI,
    buildings : Vec<Building>,
}


impl EnemyStats {
    pub async fn new(level : usize) -> EnemyStats{
        EnemyStats { 
            ai:EnemyAI::new(level),
            buildings: enemy_buildings::get_buildings().await,
        }
    }
    pub fn step(&mut self, _man: &GameManagerState) -> ManagerAction{

        match self.ai.step() {
            EnemyAIAction::Build{building_index:i,lane_index:l} =>{
                let b = self.buildings.get(i).unwrap();

                return ManagerAction::EBuild{building: b.clone(),lane_index: l};
            },
            EnemyAIAction::Wait => {
                return ManagerAction::Wait
            },
        }
    }
}

pub enum EnemyAIAction {
    Build{building_index : usize, lane_index : usize},
    Wait,
}

enum EnemyAIState{
    Wait,
    Act,
    End,
}

pub struct EnemyAI{
    state: EnemyAIState,
    steps: Vec<ai_settings::RawAIActions>,
    cur_step: usize,
    progress: f32,
    finish : f32,
}

impl EnemyAI {
    pub fn new(level : usize)->EnemyAI{

        let steps = serde_json::from_str(&ai_settings::get_json(level)).unwrap();
        println!("{:?}",&steps);
        EnemyAI{
            state: EnemyAIState::Act,
            cur_step: 0,
            progress: 0.0,
            finish: 0.0,
            steps
        }   
    }

    pub fn step(&mut self)->EnemyAIAction{

        match &self.state {
            EnemyAIState::Act => {
                let a = self.act();
                return a;
            }
            EnemyAIState::Wait => {
                self.state_step();
                return EnemyAIAction::Wait;
            }
            EnemyAIState::End => return EnemyAIAction::Wait,
        }
    }

    pub fn act(&mut self)->EnemyAIAction{
        let c_s = self.cur_step;
        let s = self.steps.get(c_s).unwrap();
        self.cur_step += 1;

        match s {
            ai_settings::RawAIActions::Build { shop_index: s, lane_index : l } => {
                return EnemyAIAction::Build { building_index: *s, lane_index: *l }
            }
            ai_settings::RawAIActions::Wait(i) => {
                self.progress = 0.0;
                self.finish = *i;
                self.state = EnemyAIState::Wait;

                return EnemyAIAction::Wait
            }
            ai_settings::RawAIActions::End => {
                self.state = EnemyAIState::End;
                return EnemyAIAction::Wait
            },
        }
    }

    pub fn state_step(&mut self){
        self.progress += get_frame_time();

        if self.progress > self.finish {
            self.state = EnemyAIState::Act;
        }
    }
}


