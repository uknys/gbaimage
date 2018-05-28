use image::Rgb;
use std::str::FromStr;

pub struct RGB15(pub u16);

impl From<Rgb<u8>> for RGB15 {
    fn from(color: Rgb<u8>) -> Self {
        RGB15(
            (u16::from(color[0]) >> 3) | (u16::from(color[1]) >> 3 << 5)
                | (u16::from(color[2]) >> 3 << 10),
        )
    }
}

pub struct RGB24(pub u8, pub u8, pub u8);

impl From<RGB24> for RGB15 {
    fn from(color: RGB24) -> Self {
        let RGB24(r, g, b) = color;
        RGB15((u16::from(r) >> 3) | (u16::from(g) >> 3 << 5) | (u16::from(b) >> 3 << 10))
    }
}

impl FromStr for RGB24 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('#') {
            return Err(format!("Expected # as first char, got: {}", &s[0..1]));
        }

        let (r, g, b) = match s.len() {
            4 => {
                let r = match u8::from_str_radix(&s[1..2], 16) {
                    Ok(v) => v * 17,
                    Err(e) => return Err(e.to_string()),
                };

                let g = match u8::from_str_radix(&s[2..3], 16) {
                    Ok(v) => v * 17,
                    Err(e) => return Err(e.to_string()),
                };

                let b = match u8::from_str_radix(&s[3..4], 16) {
                    Ok(v) => v * 17,
                    Err(e) => return Err(e.to_string()),
                };

                (r, g, b)
            }
            7 => {
                let r = match u8::from_str_radix(&s[1..3], 16) {
                    Ok(v) => v,
                    Err(e) => return Err(e.to_string()),
                };

                let g = match u8::from_str_radix(&s[3..5], 16) {
                    Ok(v) => v,
                    Err(e) => return Err(e.to_string()),
                };

                let b = match u8::from_str_radix(&s[5..7], 16) {
                    Ok(v) => v,
                    Err(e) => return Err(e.to_string()),
                };

                (r, g, b)
            }
            x => return Err(format!("Hex Color must be 4 or 7 characters, got: {}", x)),
        };

        Ok(RGB24(r, g, b))
    }
}
