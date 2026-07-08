mod wall_painter;

use wall_painter::WallPainter;

pub struct WallManager{
    painter: WallPainter,
}

impl WallManager{
    pub async fn new() -> WallManager{
        WallManager{
            painter : WallPainter::new().await
        }
    }

    pub fn step(&mut self){
        self.painter.paint();
    }
}
