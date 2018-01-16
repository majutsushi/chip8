pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { keys: [false; 16] }
    }

    pub fn key_press(&mut self, key: u8) {
        if key > 0xF {
            println!("Warning: pressed unsupported key {}", key);
        } else {
            self.keys[key as usize] = true;
        }
    }

    pub fn key_release(&mut self, key: u8) {
        if key > 0xF {
            println!("Warning: released unsupported key {}", key);
        } else {
            self.keys[key as usize] = false;
        }
    }

    pub fn key_pressed(&self, key: u8) -> bool {
        if key > 0xF {
            println!("Warning: unsupported key {}", key);
            false
        } else {
            self.keys[key as usize]
        }
    }
}
