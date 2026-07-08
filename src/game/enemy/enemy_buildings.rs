use crate::game::factory::{RawBuilding,Building};

pub async fn get_buildings() -> Vec<Building>{
    let r_v : Vec<RawBuilding> = serde_json::from_str(&get_json()).unwrap();
    let mut v = Vec::new();
    for r in r_v {
        v.push(r.into_b().await);
    }

    v
}



fn get_json() -> String{
r#"
        [ 
            {"texture" : "goblin_building.png", "name": "Goblin Hill", "finish": 100, "progress": 0, "speed": 0.4, "product": {"texture" : "goblin.png", "name":"Goblin",   "health": 400.0,  "speed": 0.2,  "dmg": 100.0, "range": 10.0,  "finish": 60, "progress": 0, "rate": 1.0}},
            {"texture" : "orc_building.png", "name": "Orc Hut",   "finish": 80,  "progress": 0, "speed": 0.2, "product": {"texture" : "orc_archer.png", "name":"Orc Archer",   "health": 600.0,  "speed": 0.25, "dmg": 60.0,  "range": 60.0,  "finish": 60, "progress": 0, "rate": 0.7}},
            {"texture" : "goblin_building.png", "name": "Brute", "finish": 200, "progress": 0, "speed": 0.1, "product": {"texture" : "goblin.png", "name":"One Three", "health": 1500.0, "speed": 0.1,  "dmg": 600.0, "range": 5.0,   "finish": 60, "progress": 0, "rate": 0.2}},
            {"texture" : "goblin_building.png", "name": "Sniper","finish": 200, "progress": 0, "speed": 0.4, "product": {"texture" : "goblin.png", "name":"One Four",  "health": 50.0,   "speed": 0.15, "dmg": 800.0, "range": 100.0, "finish": 60, "progress": 0, "rate": 0.1}}
              
        ]
        "#.to_string()
}
