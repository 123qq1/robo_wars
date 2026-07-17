use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub enum RawAIActions{
    Wait(f32),
    Build{shop_index : usize, lane_index : usize},
    End,
}

pub fn get_json(level : usize) -> String{
    match level {
        0 => { return get_lvl_0() }
        1 => { return get_lvl_1() }
        _ => (),
    }

    "Fail".to_string()
}

fn get_lvl_0() -> String{
    r#"
        [
            {"Wait": 2},
            {"Build": {"shop_index" :0,"lane_index": 1}},
            {"Wait": 10},
            {"Build": {"shop_index" :0,"lane_index": 0}},
            {"Build": {"shop_index" :0,"lane_index": 2}},
            "End"        
        ]
        "#.to_string()
}

fn get_lvl_1() -> String{
    r#"
        [
            {"Wait": 2},
            {"Build": {"shop_index" :0,"lane_index": 1}},
            {"Build": {"shop_index" :1,"lane_index": 1}},
            {"Wait": 10},
            {"Build": {"shop_index" :0,"lane_index": 0}},
            {"Build": {"shop_index" :1,"lane_index": 0}},
            {"Wait": 10},
            {"Build": {"shop_index" :0,"lane_index": 2}},
            {"Build": {"shop_index" :1,"lane_index": 2}},
            "End"        
        ]
        "#.to_string()
}