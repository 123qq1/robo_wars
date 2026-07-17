mod shop_settings;

use crate::game::ManagerAction;

use super::Status::{self, Faliure, Success};

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

struct ShopOption{
    product: Building,
    price: i32,
    available: bool,
}

impl Shop {
    pub async fn new() -> Shop{
        let vs : Vec<ShopOption> = shop_settings::get_shop_options().await;

        Shop { options: vs, painter: shop_painter::Painter {}}
    }

    pub fn player_buy(&self, p_stats : &mut PlayerStats, option_index:usize) -> Status<Building>{
        let b = &self.options[option_index];

        if !b.available {return Faliure("Item not available".to_string());}

        if let Some(_) = p_stats.pay(b.price){
            return Success(b.product.clone());
        }

        Faliure("Not Enough Money".to_string())
    }

    pub fn step(&mut self) -> ManagerAction{
        let mut action = ShopAction::Wait;
        self.painter.show(ShopState::new(self), &mut action);

        match action {
            ShopAction::Buy(i) => {
                return ManagerAction::PBuild(i)
                
            }
            ShopAction::Wait => {}
        }
        ManagerAction::Wait
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
}