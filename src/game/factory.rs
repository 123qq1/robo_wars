use super::robot::Unit;
use super::ui::Showable;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Building{
    name: String,
    progress: f32,
    finish: f32,
    speed: f32,
    product: Unit,
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
        
        self.progress = 0.0;
        Some(self.product.clone())
    }
}

impl Showable for Building{
    fn show_text(&self) -> String {
        self.name.clone()
    }
}

impl Clone for Building{
    fn clone(&self) -> Self {
        Self { 
            name: self.name.clone(),
            speed: self.speed, 
            product: self.product.clone(), 
            progress: self.progress, 
            finish: self.finish }
    }
}