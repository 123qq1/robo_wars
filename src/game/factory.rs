use super::robot::{Unit,RawUnit};
use serde::Deserialize;

#[derive(Debug)]
pub struct Building{
    name: String,
    progress: f32,
    finish: f32,
    speed: f32,
    product: Unit,
}

#[derive(Debug,Deserialize)]
pub struct RawBuilding{
    name: String,
    progress: f32,
    finish: f32,
    speed: f32,
    product: RawUnit,
}

impl RawBuilding {
    pub async fn into_b(self) -> Building {
        Building { 
            name: self.name, 
            progress: self.progress, 
            finish: self.finish, 
            speed: self.speed, 
            product: self.product.into_u().await, 
        }
    }
}

impl Building{
    pub fn new(name: String, speed:f32,product:Unit,finish:f32,) -> Building{
        Building { 
            name,
            speed, 
            product,
            progress: 0.0,
            finish, 
        }
    }

    pub fn produce(&mut self) -> Option<Unit>{

        self.progress += self.speed;

        if self.progress < self.finish {
            return None
        }
        
        self.progress -= self.finish;
        Some(self.product.clone())
    }

    pub fn name(&self) -> String{
        self.name.clone()
    }
}

impl Clone for Building{
    fn clone(&self) -> Self {
        Self { 
            name: self.name.clone(),
            speed: self.speed, 
            product: self.product.clone(), 
            progress: 0.0, 
            finish: self.finish }
    }
}