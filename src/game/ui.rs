use macroquad::prelude::*;

pub trait Showable {
    fn show_text(&self) -> String;
}

pub struct Painter{

}

impl Painter{
    pub fn paint_text(&self, el: &impl Showable, x : f32, y:f32){
        let s = el.show_text();
        draw_text(&s, x, y, 20.0, BLACK);
    }
}
