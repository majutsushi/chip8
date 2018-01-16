pub const DISPLAY_WIDTH: u8 = 64;
pub const DISPLAY_HEIGHT: u8 = 32;

pub struct Display {
    display: [[bool; DISPLAY_HEIGHT as usize]; DISPLAY_WIDTH as usize],
}

impl Display {
    pub fn new() -> Display {
        Display {
            display: [[false; DISPLAY_HEIGHT as usize]; DISPLAY_WIDTH as usize],
        }
    }

    pub fn clear(&mut self) {
        self.display = [[false; DISPLAY_HEIGHT as usize]; DISPLAY_WIDTH as usize];
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: Vec<u8>) -> bool {
        let mut collision = false;

        for (row, rowdata) in sprite.iter().enumerate() {
            for col in 0..8 {
                if rowdata & (0x80 >> col) != 0 {
                    // Coordinates wrap around
                    let p_x = (x as usize + col) % DISPLAY_WIDTH as usize;
                    let p_y = (y as usize + row) % DISPLAY_HEIGHT as usize;

                    if self.display[p_x][p_y] {
                        collision = true;
                    }

                    self.display[p_x][p_y] ^= true;
                }
            }
        }

        collision
    }
}

impl<'a> IntoIterator for &'a Display {
    type Item = bool;
    type IntoIter = DisplayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DisplayIterator {
            display: self,
            x: 0,
            y: 0,
        }
    }
}

pub struct DisplayIterator<'a> {
    display: &'a Display,
    x: u8,
    y: u8,
}

impl<'a> Iterator for DisplayIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.y >= DISPLAY_HEIGHT {
            None
        } else {
            Some(self.display.display[self.x as usize][self.y as usize])
        };

        self.x += 1;
        if self.x >= DISPLAY_WIDTH {
            self.y += 1;
            self.x %= DISPLAY_WIDTH;
        }

        result
    }
}
