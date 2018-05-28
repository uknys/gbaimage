extern crate image;

use self::color::RGB15;
pub use self::color::RGB24;
use self::iterator::ImageIterator;
use self::tile::{Shape, Size, Tile};
use image::{load_from_memory, DynamicImage, GenericImage, ImageRgba8, Rgb};

mod color;
mod iterator;
pub mod tile;

pub struct GBAImagePal(pub Vec<u16>, pub Vec<u8>);

impl GBAImagePal {
    pub fn new(
        data: &[u8],
        transparency: Option<RGB24>,
        tile: Option<Tile>,
    ) -> Result<GBAImagePal, String> {
        let mut src = match load_from_memory(data) {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        };

        let mut pal: Vec<u16> = Vec::with_capacity(256);
        let mut img: Vec<u8> = Vec::with_capacity((src.width() * src.height()) as usize);

        if let Some(t) = transparency {
            let RGB15(color) = t.into();
            pal.push(color);
        };

        match tile {
            Some(Tile(Shape::Square, Size::X1)) => GBAImagePal::gen_tiled(&mut pal, &mut img, &src),
            Some(t) => GBAImagePal::gen_metatiled(&mut pal, &mut img, &mut src, t),
            None => GBAImagePal::gen_linear(&mut pal, &mut img, &src),
        }.map(|_| GBAImagePal(pal, img))
    }

    fn insert_to_palette(
        palette: &mut Vec<u16>,
        image: &mut Vec<u8>,
        color: &Rgb<u8>,
    ) -> Result<(), String> {
        let color = *color;
        let RGB15(rgb15_color) = color.into();

        match palette.into_iter().position(|&mut x| x == rgb15_color) {
            Some(index) => {
                image.push(index as u8);
            }
            None => {
                if palette.len() > 0xFF {
                    return Err("Palette size is over 256 colors".to_string());
                } else {
                    palette.push(rgb15_color);
                    image.push((palette.len() - 1) as u8);
                }
            }
        }

        Ok(())
    }

    fn gen_linear(
        palette: &mut Vec<u16>,
        image: &mut Vec<u8>,
        source: &DynamicImage,
    ) -> Result<(), String> {
        source
            .to_rgb()
            .pixels()
            .into_iter()
            .map(|x| GBAImagePal::insert_to_palette(palette, image, x))
            .collect::<Result<(), String>>()
    }

    fn gen_tiled(
        palette: &mut Vec<u16>,
        image: &mut Vec<u8>,
        source: &DynamicImage,
    ) -> Result<(), String> {
        ImageIterator::new(source.width(), source.height())
            .map(|(x, y)| {
                GBAImagePal::insert_to_palette(palette, image, source.to_rgb().get_pixel(x, y))
            })
            .collect::<Result<(), String>>()
    }

    fn gen_metatiled(
        palette: &mut Vec<u16>,
        image: &mut Vec<u8>,
        source: &mut DynamicImage,
        tile: tile::Tile,
    ) -> Result<(), String> {
        let (w, h) = (source.width(), source.height());
        let (mw, mh) = tile.into();

        (0..(h / mh))
            .flat_map(|y| (0..(w / mw)).map(move |x| (x, y)))
            .map(|(x, y)| {
                GBAImagePal::gen_tiled(
                    palette,
                    image,
                    &ImageRgba8(source.sub_image(x * mw, y * mh, mw, mh).to_image()),
                )
            })
            .collect::<Result<(), String>>()
    }
}
