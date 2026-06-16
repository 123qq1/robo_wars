mod shop_settings;

use crate::game::ui::TextPaintOptions;

use super::Status::{self, Faliure, Success};
use macroquad::color::BLACK;
use serde::Deserialize;

use super::player::PlayerStats;
use super::factory::Building;

pub struct Shop{
    options: Vec<ShopOption>,
    text_paint_options: TextPaintOptions,
}

#[derive(Debug,Deserialize)]
struct ShopOption{
    product: Building,
    price: i32,
    available: bool,
}

impl Shop {
    pub fn new() -> Shop{
        let json = shop_settings::get_json();
        let vs : Vec<ShopOption> = serde_json::from_str(&json).unwrap();

        Shop { options: vs , text_paint_options:TextPaintOptions { text: "test".to_string(), x: 30.0, y: 300.0, font_size: 15.0, color: BLACK }}
    }

    pub fn buy(&self, p_stats : &mut PlayerStats, option_index:usize) -> Status<Building>{
        let b = &self.options[option_index];

        if !b.available {return Faliure("Item not available".to_string());}

        if let Some(m) = p_stats.pay(b.price){
            return Success(b.product.clone());
        }

        Faliure("Not Enough Money".to_string())
    }

    pub fn step(&mut self){
        let mut s = "".to_string();
        self.options.iter().for_each(|o|{
            s += &o.show_text();
            s += "\n"
        });
        self.text_paint_options.text = s;
    }

    pub fn text(&self)->&TextPaintOptions{
        &self.text_paint_options
    }
}

impl ShopOption{
    fn show_text(&self) -> String {
        if !self.available {return "".to_string()}

        format!("{} : {}$",self.product.name(),self.price)
    }
}