use super::ui::Showable;

pub struct PlayerStats{
    income_step: i32,
    income_payout: i32,
    income: i32,
    money: i32
}

impl PlayerStats {
    pub fn new(money: i32,income:i32, income_payout: i32)->PlayerStats{
        PlayerStats { 
            income_payout,
            income_step: 0,
            income,
            money
        }
    }

    pub fn money(&self)-> i32{
        self.money
    }

    pub fn pay(&mut self,cost: i32) -> Option<i32>{
        if self.money < cost {return None} 
        self.money -= cost;
        Some(self.money)
    }

    pub fn gain(&mut self, gain: i32){
        self.money += gain;
    }

    pub fn step(&mut self){
        self.income_step += 1;

        if self.income_step > self.income_payout {
            self.income_step = 0;
            self.money += self.income;
        }
    }
}

impl Showable for PlayerStats{
    fn show(&self) -> String {
        format!("Player: {}$ : ^{}$",self.money,self.income)
    }
}