use macroquad::prelude::*;



pub struct TextPainter{
    
}

pub struct TextPaintOptions{
    pub text:String,
    pub x:f32,
    pub y:f32,
    pub font_size:f32,
    pub color:Color,
}

impl TextPainter{
    pub fn new()-> TextPainter{
        TextPainter {}
    }
    pub fn paint_text(&self, t: &TextPaintOptions){
        draw_text(&t.text, t.x, t.y, t.font_size, t.color);
    }
}
