use super::{GameAction,GameState};
mod main_menu;
mod level_menu;

pub struct MenuManager{
    main_menu : main_menu::MainMenuPainter,
    level_menu : level_menu::LevelMenuPainter,
}

impl MenuManager{
    pub fn new() -> MenuManager{
        MenuManager {
            main_menu: main_menu::MainMenuPainter::new(),
            level_menu: level_menu::LevelMenuPainter::new(),
          }
    }

    pub fn step_main(&mut self) -> GameAction{
        let mut action = MenuAction::Wait;
        self.main_menu.step(&mut action);

        match action {
            MenuAction::LevelMenuSelected => return GameAction::ChangeState(GameState::LevelMenu),

            MenuAction::QuitSelected => return GameAction::ChangeState(GameState::Quit),
            _ => (),
        }

        GameAction::Wait
    }

    pub fn step_level(&mut self) -> GameAction{
        let mut action = MenuAction::Wait;
        self.level_menu.step(&mut action);

        match action {
            MenuAction::LevelSelected(i) => return GameAction::ChangeState(GameState::Level(i)),

            MenuAction::MainMenuSelected => return GameAction::ChangeState(GameState::MainMenu),
            _ => (),
        }

        GameAction::Wait
    }
}

pub enum MenuAction{
    Wait,
    LevelMenuSelected,
    LevelSelected(usize),
    MainMenuSelected,
    QuitSelected,
}


