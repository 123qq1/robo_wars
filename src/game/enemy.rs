use macroquad::color::BLACK;

use crate::game::ui::TextPaintOptions;

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
    pub fn step(&mut self){
        self.income_step += 1;
        if self.income_step > self.income_payout {
            self.income_step = 0;
            self.money += self.income;
        }
    }

    pub fn text(&self)->&TextPaintOptions{
        &self.text_painter_options
    }
}


pub struct EnemyAI{
}

impl EnemyAI {
    pub fn new()->EnemyAI{
        EnemyAI{
        }   
    }
}
