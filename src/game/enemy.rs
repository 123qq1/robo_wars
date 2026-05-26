use super::ui::Showable;

pub struct EnemyStats{
    income_step: i32,
    income_payout: i32,
    income: i32,
    money : i32,
    ai : EnemyAI,
}


impl EnemyStats {
    pub fn new(money: i32,income:i32, income_payout : i32) -> EnemyStats{
        EnemyStats { money,income ,income_step:0,income_payout,ai:EnemyAI::new()}
    }
    pub fn step(&mut self){
        self.income_step += 1;
        if self.income_step > self.income_payout {
            self.income_step = 0;
            self.money += self.income;
        }
    }
}

impl Showable for EnemyStats{
    fn show(&self) -> String{
        format!("Enemy :{}$",self.money)
    }
}

pub struct EnemyAI{
}

impl EnemyAI {
    pub fn new()->EnemyAI{
        EnemyAI{
        }   
    }
}
