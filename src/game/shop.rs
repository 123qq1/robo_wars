mod shop_settings;

use crate::game::ManagerAction;
use crate::game::enemy::EnemyStats;

use super::Status::{self, Faliure, Success};
use serde::Deserialize;

use super::player::PlayerStats;
use super::factory::Building;

mod shop_painter;

pub struct Shop{
    options: Vec<ShopOption>,
    painter: shop_painter::Painter
}

pub enum ShopAction{
    Wait,
    Buy(usize),
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

        Shop { options: vs, painter: shop_painter::Painter {}}
    }

    pub fn player_buy(&self, p_stats : &mut PlayerStats, option_index:usize) -> Status<Building>{
        let b = &self.options[option_index];

        if !b.available {return Faliure("Item not available".to_string());}

        if let Some(m) = p_stats.pay(b.price){
            return Success(b.product.clone());
        }

        Faliure("Not Enough Money".to_string())
    }

    pub fn enemy_buy(&self, e_stats : &mut EnemyStats, option_index:usize)->Status<Building>{
        let b = &self.options[option_index];

        if !b.available {return Faliure("Item not available".to_string());}

        if let Some(m) = e_stats.pay(b.price){
            return Success(b.product.clone());
        }
        
        Faliure("Not Enough Money".to_string())
    }

    pub fn step(&mut self) -> ManagerAction{
        let mut action = ShopAction::Wait;
        self.painter.show(ShopState::new(self), &mut action);

        match action {
            ShopAction::Buy(i) => {
                return ManagerAction::P_Build(i)
                
            }
            ShopAction::Wait => {}
        }
        ManagerAction::Wait
    }

    pub fn get_index(&self, building : Building)-> Option<usize>{
        let o_o = self.options.iter().enumerate().find(|(i,o)|{
            o.product.name() == building.name()
        });

        match o_o {
            Some(o) => {return Some(o.0)}
            None => {return None;}
        }
    }
}

pub struct ShopState{
    options: Vec<ShopOptionState>,
}

pub struct ShopOptionState{
    product: Building,
    price: i32,
    available: bool,
}

impl ShopOptionState {
    fn new(option: &ShopOption) -> ShopOptionState{
        ShopOptionState { 
            product: option.product.clone(),
            price: option.price,
            available: option.available, 
        }
    }
}

impl ShopState {
    pub fn new(shop: &Shop)->ShopState{
        let mut v_o = Vec::new();

        shop.options.iter().for_each(|o|{
            v_o.push(ShopOptionState::new(o));
        });

        ShopState { 
            options: v_o,
        }
    }

    pub fn get_buyable(&self, budget: i32)->Option<Building>{
        let  o_s = self.options.iter().find(|o|{
            o.available && (o.price < budget)
        });

        match o_s {
            None => {return None;}
            Some(o) => {return Some(o.product.clone())}
        }
    }
}