use macroquad::color::BLACK;

use crate::game::{ManagerAction, ManagerState, factory::Building, ui::TextPaintOptions};

pub struct EnemyStats{
    income_step: i32,
    income_payout: i32,
    income: i32,
    money : i32,
    ai : EnemyAI,
    text_painter_options: TextPaintOptions
}


impl EnemyStats {
    pub fn new(money: i32,income:i32, income_payout : i32) -> EnemyStats{
        EnemyStats { 
            money,
            income ,
            income_step:0,
            income_payout,
            ai:EnemyAI::new(),
            text_painter_options : TextPaintOptions{ text: "".to_string(), x: 30.0, y: 200.0, font_size: 15.0, color: BLACK }
        }
    }
    pub fn step(&mut self, man: &ManagerState) -> ManagerAction{
        self.income_step += 1;
        if self.income_step > self.income_payout {
            self.income_step = 0;
            self.money += self.income;
        }

        match self.ai.step(&self, man) {
            EnemyAIStates::Build(b) =>{
                return ManagerAction::E_Build(b);
            },
            EnemyAIStates::Wait => {
                return ManagerAction::Wait
            },
        }
    }

    pub fn pay(&mut self,cost: i32) -> Option<i32>{
        if self.money < cost {return None} 
        self.money -= cost;
        Some(self.money)
    }

    pub fn text(&self)->&TextPaintOptions{
        &self.text_painter_options
    }
}

pub enum EnemyAIStates {
    Build(Building),
    Wait,

}

pub struct EnemyAI{

}

impl EnemyAI {
    pub fn new()->EnemyAI{
        EnemyAI{
        }   
    }

    pub fn step(&self, self_state: &EnemyStats, man_state: &ManagerState)->EnemyAIStates{

        if let Some(b) = man_state.shop_state.get_buyable(self_state.money) {
            return EnemyAIStates::Build(b);
        }

        EnemyAIStates::Wait
    }
}
