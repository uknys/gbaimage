use std::str::FromStr;

pub enum Size {
    X1 = 1,
    X2 = 2,
    X4 = 4,
    X8 = 8,
}

pub enum Shape {
    Square,
    Wide,
    Tall,
}

pub struct Tile(pub Shape, pub Size);

impl Into<(u32, u32)> for Tile {
    fn into(self) -> (u32, u32) {
        match self {
            Tile(Shape::Square, Size::X1) => (8, 8),
            Tile(Shape::Square, Size::X2) => (16, 16),
            Tile(Shape::Square, Size::X4) => (32, 32),
            Tile(Shape::Square, Size::X8) => (64, 64),
            Tile(Shape::Wide, Size::X1) => (16, 8),
            Tile(Shape::Wide, Size::X2) => (32, 8),
            Tile(Shape::Wide, Size::X4) => (32, 16),
            Tile(Shape::Wide, Size::X8) => (64, 32),
            Tile(Shape::Tall, Size::X1) => (8, 16),
            Tile(Shape::Tall, Size::X2) => (8, 32),
            Tile(Shape::Tall, Size::X4) => (16, 32),
            Tile(Shape::Tall, Size::X8) => (32, 64),
        }
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tile_str = s.trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect::<Vec<&str>>();

        let width = match tile_str[0].parse::<u32>() {
            Ok(w) => w,
            Err(e) => return Err(e.to_string()),
        };

        let height = match tile_str[1].parse::<u32>() {
            Ok(h) => h,
            Err(e) => return Err(e.to_string()),
        };

        match (width, height) {
            (8, 8) => Ok(Tile(Shape::Square, Size::X1)),
            (16, 16) => Ok(Tile(Shape::Square, Size::X2)),
            (32, 32) => Ok(Tile(Shape::Square, Size::X4)),
            (64, 64) => Ok(Tile(Shape::Square, Size::X8)),
            (16, 8) => Ok(Tile(Shape::Wide, Size::X1)),
            (32, 8) => Ok(Tile(Shape::Wide, Size::X2)),
            (32, 16) => Ok(Tile(Shape::Wide, Size::X4)),
            (64, 32) => Ok(Tile(Shape::Wide, Size::X8)),
            (8, 16) => Ok(Tile(Shape::Tall, Size::X1)),
            (8, 32) => Ok(Tile(Shape::Tall, Size::X2)),
            (16, 32) => Ok(Tile(Shape::Tall, Size::X4)),
            (32, 64) => Ok(Tile(Shape::Tall, Size::X8)),
            (w, h) => Err(format!("Invalid Tile size: ({},{})", w, h)),
        }
    }
}
