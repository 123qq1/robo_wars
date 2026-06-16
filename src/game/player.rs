use macroquad::color::BLACK;

use crate::game::ui::TextPaintOptions;


pub struct PlayerStats{
    income_step: i32,
    income_payout: i32,
    income: i32,
    money: i32,
    text_paint_options: TextPaintOptions
}

impl PlayerStats {
    pub fn new(money: i32,income:i32, income_payout: i32)->PlayerStats{
        PlayerStats { 
            income_payout,
            income_step: 0,
            income,
            money,
            text_paint_options: TextPaintOptions { text: "".to_string(), x: 30.0, y: 250.0, font_size: 15.0, color: BLACK }
        }
    }

    pub fn money(&self)-> i32{
        self.money
    }

    pub fn pay(&mut self,cost: i32) -> Option<i32>{
        if self.money < cost {return None} 
        self.money -= cost;
        Some(self.money)
    }

    pub fn gain(&mut self, gain: i32){
        self.money += gain;
    }

    pub fn step(&mut self){
        self.income_step += 1;

        if self.income_step > self.income_payout {
            self.income_step = 0;
            self.money += self.income;
        }
        self.text_paint_options.text = format!("Player: {}$ : ^{}$",self.money,self.income);
        //self.text_paint_options.text = &format!("Player: {}$ : ^{}$",self.money,self.income), 30.0, 250.0);
    }

    pub fn text(&self)->&TextPaintOptions{
        &self.text_paint_options
    }
}