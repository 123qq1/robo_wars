use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub enum RawAIActions{
    Wait(i32),
    Build{shop_index : usize, lane_index : usize},
    End,
}

pub fn get_json() -> String{
    r#"
        [
            {"Wait": 100},
            {"Build": {"shop_index" :0,"lane_index": 1}},
            {"Wait": 500},
            {"Build": {"shop_index" :0,"lane_index": 0}},
            {"Build": {"shop_index" :0,"lane_index": 2}},
            "End"        
        ]
        "#.to_string()
}