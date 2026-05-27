mod shop_settings;

use super::Status::{self, Faliure, Success};
use serde::Deserialize;

use super::player::PlayerStats;
use super::ui::Showable;

use super::factory::Building;
pub struct Shop{
    options: Vec<ShopOption>,
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

        Shop { options: vs }
    }

    pub fn buy(&self, p_stats : &mut PlayerStats, option_index:usize) -> Status<Building>{
        let b = &self.options[option_index];

        if !b.available {return Faliure("Item not available".to_string());}

        if let Some(m) = p_stats.pay(b.price){
            return Success(b.product.clone());
        }

        Faliure("Not Enough Money".to_string())
    }
}

impl Showable for Shop {
    fn show_text(&self) -> String {
        let mut s = "".to_string();
        self.options.iter().for_each(|o|{
            s += &o.show_text();
            s += "\n"
        });
        s
    }
}

impl Showable for ShopOption{
    fn show_text(&self) -> String {
        if !self.available {return "".to_string()}

        format!("{} : {}$",self.product.show_text(),self.price)
    }
}