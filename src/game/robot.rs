use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Unit{
    name: String,
    health: f32,
    speed: f32,
    dmg: f32,
    range: f32,
}

impl Unit{
    pub fn new(name: String,health:f32,speed:f32,dmg:f32,range:f32) -> Unit{
        Unit{
            name,
            health,
            speed,
            dmg,
            range,
        }
    }
    pub fn speed(&self)->f32{
        self.speed
    }
}

impl Clone for Unit{
    fn clone(&self) -> Self {
        Self {name: self.name.clone(), health: self.health, speed: self.speed, dmg: self.dmg, range: self.range }
    }
}
