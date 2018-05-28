pub struct ImageIterator {
    width: u32,
    height: u32,
    row: u32,
    column: u32,
    tile_row: u32,
    tile_column: u32,
}

impl ImageIterator {
    pub fn new(w: u32, h: u32) -> ImageIterator {
        ImageIterator {
            width: w,
            height: h,
            row: 0,
            tile_row: 0,
            column: 0,
            tile_column: 0,
        }
    }
}

impl Iterator for ImageIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.height == self.row {
            return None;
        }

        let cur = (self.column, self.row);

        self.column += 1;
        self.tile_column += 1;

        if self.tile_column >= 8 {
            self.row += 1;
            self.tile_row += 1;
            self.column -= 8;
            self.tile_column = 0;

            if self.tile_row >= 8 {
                self.row -= 8;
                self.tile_row = 0;
                self.column += 8;
            }

            if self.column >= self.width {
                self.tile_column = 0;
                self.tile_row = 0;
                self.column = 0;
                self.row += 8;
            }
        }

        Some(cur)
    }
}
