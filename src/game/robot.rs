use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Unit{
    name: String,
    health: f32,
    speed: f32,
    dmg: f32,
    range: f32,
    progress: f32,
    finish: f32,
    rate: f32,
}

impl Unit{
    pub fn new(name: String,health:f32,speed:f32,dmg:f32,range:f32,finish:f32, rate: f32) -> Unit{
        Unit{
            name,
            health,
            speed,
            dmg,
            range,
            finish,
            progress: 0.0,
            rate,
        }
    }
    pub fn speed(&self)->f32{
        self.speed
    }

    pub fn range(&self) -> f32{
        self.range
    }

    pub fn dmg(&mut self) -> Option<f32>{

        self.progress += self.rate;

        if self.progress < self.finish{return None}
        
        self.progress -= self.finish;
        Some(self.dmg)
    }

    pub fn health(&self) -> f32{
        self.health
    }


}

impl Clone for Unit{
    fn clone(&self) -> Self {
        Self {name: self.name.clone(), health: self.health, speed: self.speed, dmg: self.dmg, range: self.range, finish: self.finish, rate: self.rate , progress: 0.0}
    }
}
