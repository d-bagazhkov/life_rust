#![warn(dead_code)]
pub enum ColorsRGBA {
    RED,
    // GREEN,
    BLUE,
    BLACK,
    WHITE,
    YELLOW,
}

impl ColorsRGBA {
    pub fn get(&self) -> [f32; 4] {
        match self {
            ColorsRGBA::RED => [1.0, 0.0, 0.0, 1.0],
            // ColorsRGBA::GREEN => [0.0, 1.0, 0.0, 1.0],
            ColorsRGBA::BLUE => [0.0, 0.0, 1.0, 1.0],
            ColorsRGBA::BLACK => [0.0, 0.0, 0.0, 1.0],
            ColorsRGBA::WHITE => [1.0, 1.0, 1.0, 1.0],
            ColorsRGBA::YELLOW => [1.0, 1.0, 0.0, 1.0],
        }
    }
}