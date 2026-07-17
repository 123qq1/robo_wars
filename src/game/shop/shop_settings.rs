use crate::game::{factory::RawBuilding, shop::ShopOption};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct RawShopOption{
    product: RawBuilding,
    price: i32,
    available: bool,
}

impl RawShopOption {
    pub async fn into_s_o(self) -> ShopOption{
        ShopOption {
            product : self.product.into_b().await,
            price : self.price,
            available: self.available,
        }
    }
}

pub async fn get_shop_options() -> Vec<ShopOption>{
    let r_s : Vec<RawShopOption> = serde_json::from_str(&get_json()).unwrap();
    let mut v = Vec::new();
    
    for r in r_s {
        v.push(r.into_s_o().await);
    }

    v
}


fn get_json() -> String{
    r#"
        [
            {"price": 55, "available": true, "product": {"texture" : "dude_building.png", "name": "Bow","finish": 8, "progress": 0, "speed": 2, "product": {"texture" : "dude.png", "name":"One One", "health": 200.0, "speed": 35, "dmg": 60.0, "range": 60.0, "finish": 1, "progress": 0, "rate": 1.0}}   },
            {"price": 45, "available": true, "product": {"texture" : "dude_building.png", "name": "Sword","finish": 10, "progress": 0, "speed": 4, "product": {"texture" : "dude.png", "name":"One Two", "health": 600.0, "speed": 30, "dmg": 100.0, "range": 10.0, "finish": 1, "progress": 0, "rate": 1.0}}   },
            {"price": 85, "available": true, "product": {"texture" : "dude_building.png", "name": "Brute","finish": 20, "progress": 0, "speed": 1, "product": {"texture" : "dude.png", "name":"One Three", "health": 1500.0, "speed": 10, "dmg": 600.0, "range": 5.0, "finish": 1, "progress": 0, "rate": 0.2}}   },
            {"price": 75, "available": true, "product": {"texture" : "dude_building.png", "name": "Sniper","finish": 20, "progress": 0, "speed": 4, "product": {"texture" : "dude.png", "name":"One Four", "health": 50.0, "speed": 15, "dmg": 800.0, "range": 100.0, "finish": 1, "progress": 0, "rate": 0.1}}   }
        ]
        "#.to_string()
}