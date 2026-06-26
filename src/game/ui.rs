use macroquad::{prelude::*, ui::root_ui};

use crate::game::combat::LaneManager;



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

pub struct LanePainter{

}

impl LanePainter{
    pub fn draw_lane_bounds(&self){
        let p_x = LaneManager::x_by_faction(&super::combat::Faction::Enemy);
        let e_x = LaneManager::x_by_faction(&super::combat::Faction::Player);

        let y_1 = 40.0;
        let y_2 = 160.0;
        
        draw_line(p_x, y_1, p_x, y_2, 1.0, WHITE);
        draw_line(e_x, y_1, e_x, y_2, 1.0, WHITE);
    }
    pub fn paint_lane_selector(&self, lane: usize)->Option<usize>{
        let x = 20.0;
        let mut l_1 = "[]";
        let mut l_2 = "[]";
        let mut l_3 = "[]";

        match lane {
            0 => l_1 = "->",
            1 => l_2 = "->",
            2 => l_3 = "->",
            _ => (),
        }

        if root_ui().button(vec2(x,35.0), l_1) {
            return Some(0);
        }
        if root_ui().button(vec2(x,85.0), l_2) {
            return Some(1);
        }
        if root_ui().button(vec2(x,135.0), l_3) {
            return Some(2);
        }

        None
    }
}
