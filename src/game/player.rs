use macroquad::{color::BLACK, time::get_frame_time};

use crate::game::ui::TextPaintOptions;
mod player_settings;


pub struct PlayerStats{
    income_step: f32,
    income_payout: f32,
    income: i32,
    money: i32,
    text_paint_options: TextPaintOptions
}


impl PlayerStats {
    pub fn new(level : usize)->PlayerStats{

        let p_s = player_settings::get_player_settings(level);

        PlayerStats { 
            income_payout : p_s.income_payout(),
            income_step: 0.0,
            income : p_s.income(),
            money : p_s.money(),
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
        self.income_step += get_frame_time();

        if self.income_step > self.income_payout {
            self.income_step = 0.0;
            self.money += self.income;
        }
        self.text_paint_options.text = format!("Player: {}$ : ^{}$",self.money,self.income);
        //self.text_paint_options.text = &format!("Player: {}$ : ^{}$",self.money,self.income), 30.0, 250.0);
    }

    pub fn text(&self)->&TextPaintOptions{
        &self.text_paint_options
    }
}