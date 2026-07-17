use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct PlayerSettings{
    income_payout: f32,
    income: i32,
    money: i32,
}

impl PlayerSettings{
    pub fn income_payout(&self) -> f32 {
        self.income_payout
    }
    pub fn income(&self) -> i32 {
        self.income
    }
    pub fn money(&self) -> i32 {
        self.money
    }
}

pub fn get_player_settings(_level : usize) -> PlayerSettings{
    let str =     r#"
        {
            "income_payout": 5,
            "income": 30,
            "money": 100
        }
        "#;

    let p_s : PlayerSettings = serde_json::from_str(str).unwrap();

    p_s
}
