use crate::game::shop::ShopAction;

use super::ShopState;
use macroquad::{prelude::*, ui::{hash, root_ui}};

pub struct Painter{

}

impl Painter {
    pub fn show(&self, state : ShopState, action : &mut ShopAction){
        root_ui().window(
            hash!(), 
            vec2(100.0, 400.0), 
            vec2(500.0, 70.0), 
            |ui|{
                ui.label(vec2(10.0, 10.0), "SHOP");

                for (i,o) in state.options.iter().enumerate() {
                    let y = 30.0;
                    let x = 10.0 + (i * 100) as f32;
                    let label = format!("{} : {}$",o.product.name(),o.price);
                    if o.available{
                        if ui.button(vec2(x, y), label){
                            *action = ShopAction::Buy(i);
                        }
                    }
                    else{
                        ui.label(vec2(x,y), &label);
                    }
                }

            });
    }
}