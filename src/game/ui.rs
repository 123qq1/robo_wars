use macroquad::prelude::*;

pub trait Showable {
    fn show(&self) -> String;
}

pub struct Painter{

}

impl Painter{
    pub fn paint(&self, el: &impl Showable, x : f32, y:f32){
        let s = el.show();
        draw_text(&s, x, y, 20.0, BLACK);
    }
}
