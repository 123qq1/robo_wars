use crate::game::{ManagerAction, ManagerState};

mod ai_settings;

pub struct EnemyStats{
    money : i32,
    ai : EnemyAI,
}


impl EnemyStats {
    pub fn new() -> EnemyStats{
        EnemyStats { 
            money : 999999999,
            ai:EnemyAI::new(),
        }
    }
    pub fn step(&mut self, man: &ManagerState) -> ManagerAction{
        self.money = 999999999;

        match self.ai.step() {
            EnemyAIAction::Build{shop_index:i,lane_index:l} =>{
                return ManagerAction::E_Build{shop_index: i,lane_index: l};
            },
            EnemyAIAction::Wait => {
                return ManagerAction::Wait
            },
        }
    }

    pub fn pay(&mut self,cost: i32) -> Option<i32>{
        if self.money < cost {return None} 
        self.money -= cost;
        Some(self.money)
    }
}

pub enum EnemyAIAction {
    Build{shop_index : usize, lane_index : usize},
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
    progress: i32,
    finish : i32,
}

impl EnemyAI {
    pub fn new()->EnemyAI{

        let steps = serde_json::from_str(&ai_settings::get_json()).unwrap();
        println!("{:?}",&steps);
        EnemyAI{
            state: EnemyAIState::Act,
            cur_step: 0,
            progress: 0,
            finish: 0,
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
                return EnemyAIAction::Build { shop_index: *s, lane_index: *l }
            }
            ai_settings::RawAIActions::Wait(i) => {
                self.progress = 0;
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
        self.progress += 1;

        if self.progress > self.finish {
            self.state = EnemyAIState::Act;
        }
    }
}


