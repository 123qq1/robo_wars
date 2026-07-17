mod wall_painter;

use wall_painter::WallPainter;

use crate::game::{combat::Faction, visuals::VUnit};

pub struct WallManager{
    painter: WallPainter,

    player_wall_health: f32,
    player_cur_wall_damage: f32,

    enemy_wall_health: f32,
    enemy_cur_wall_damage: f32,

    wall_states: WallStates
}

#[derive(Clone)]
pub enum WallState{
    Alive{max:f32,dmg:f32},
    Dead,
}

#[derive(Clone)]
pub struct WallStates{
    player: WallState,
    enemy: WallState,
}

impl WallStates{
    pub fn player(&self) -> WallState{
        self.player.clone()
    }
    pub fn enemy(&self) -> WallState{
        self.enemy.clone()
    }
}

impl WallManager{
    pub async fn new(player_wall_health: f32, enemy_wall_health  : f32) -> WallManager{
        WallManager{
            painter : WallPainter::new().await,
            player_wall_health,
            player_cur_wall_damage: 0.0,
            enemy_wall_health,
            enemy_cur_wall_damage: 0.0,
            wall_states: WallStates { player: WallState::Alive{max:player_wall_health,dmg:0.0}, enemy: WallState::Alive{max:enemy_wall_health,dmg:0.0} }
        }
    }

    pub fn step(&mut self, sieging_units: Vec<VUnit>)-> WallStates{
        self.painter.paint(&self.wall_states.clone());
        
        for u in sieging_units {
            self.damage_wall(u);
        }

        self.wall_states.clone()
    }

    pub fn damage_wall(&mut self,unit: VUnit){
        let faction = unit.faction();

        let dmg = unit.cur_health();

        match faction {
            Faction::Enemy => {
                self.player_cur_wall_damage += dmg;
                self.wall_states.player = WallState::Alive { max: self.player_wall_health, dmg:self.player_cur_wall_damage };
                if self.player_cur_wall_damage > self.player_wall_health {self.wall_states.player = WallState::Dead;}
            }
            Faction::Player => {
                self.enemy_cur_wall_damage += dmg;
                self.wall_states.enemy = WallState::Alive { max: self.enemy_wall_health, dmg:self.enemy_cur_wall_damage };
                if self.enemy_cur_wall_damage > self.enemy_wall_health {self.wall_states.enemy = WallState::Dead;}
            }
        }

    }
}
