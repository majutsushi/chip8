use std::env;

use chip8::Chip8;

fn main() {
    let rom = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {} <rom>", env::args().next().unwrap());
            std::process::exit(1);
        }
    };

    let mut chip8 = Chip8::new();
    chip8.load(&rom);
    chip8.run();
}
