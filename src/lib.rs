#![forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;

pub mod cpu;
pub mod display;
pub mod kbd;
pub mod memory;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::cpu::Cpu;
use crate::display::{Display, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::kbd::Keyboard;
use crate::memory::Memory;

const PIXEL_SIZE: u32 = 15;

lazy_static! {
    static ref KEYMAP: HashMap<Keycode, u8> = {
        let mut map = HashMap::new();
        map.insert(Keycode::X, 0x0);
        map.insert(Keycode::Num1, 0x1);
        map.insert(Keycode::Num2, 0x2);
        map.insert(Keycode::Num3, 0x3);
        map.insert(Keycode::Q, 0x4);
        map.insert(Keycode::W, 0x5);
        map.insert(Keycode::E, 0x6);
        map.insert(Keycode::A, 0x7);
        map.insert(Keycode::S, 0x8);
        map.insert(Keycode::D, 0x9);
        map.insert(Keycode::Z, 0xA);
        map.insert(Keycode::C, 0xB);
        map.insert(Keycode::Num4, 0xC);
        map.insert(Keycode::R, 0xD);
        map.insert(Keycode::F, 0xE);
        map.insert(Keycode::V, 0xF);
        map
    };
}

pub struct Chip8 {
    memory: Memory,
    display: Display,
    kbd: Keyboard,
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: Memory::new(),
            display: Display::new(),
            kbd: Keyboard::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load(&mut self, path: &str) {
        let mut rom = Vec::new();
        let mut file = File::open(path).unwrap();

        file.read_to_end(&mut rom).unwrap();

        // Copy rom into memory
        self.memory.load_rom(&rom);
    }

    pub fn run(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "CHIP-8",
                PIXEL_SIZE * DISPLAY_WIDTH as u32,
                PIXEL_SIZE * DISPLAY_HEIGHT as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .unwrap();

        println!("Using SDL_Renderer \"{}\"", canvas.info().name);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let texture_creator = canvas.texture_creator();
        let mut pixel_texture = texture_creator
            .create_texture_target(None, PIXEL_SIZE, PIXEL_SIZE)
            .unwrap();
        canvas
            .with_texture_canvas(&mut pixel_texture, |texture_canvas| {
                texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                texture_canvas.clear();
            })
            .unwrap();

        struct SquareWave {
            phase_inc: f32,
            phase: f32,
            volume: f32,
        }

        impl AudioCallback for SquareWave {
            type Channel = f32;

            fn callback(&mut self, out: &mut [f32]) {
                // Generate a square wave
                for x in out.iter_mut() {
                    *x = if self.phase <= 0.5 {
                        self.volume
                    } else {
                        -self.volume
                    };
                    self.phase = (self.phase + self.phase_inc) % 1.0;
                }
            }
        }

        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        let audio_device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                // initialize the audio callback
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(key),
                        repeat: false,
                        ..
                    } => {
                        if let Some(&chip8key) = KEYMAP.get(&key) {
                            self.kbd.key_press(chip8key);
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => {
                        if let Some(&chip8key) = KEYMAP.get(&key) {
                            self.kbd.key_release(chip8key);
                        }
                    }
                    _ => {}
                }
            }

            self.cpu
                .cycle(&mut self.memory, &mut self.display, &self.kbd);
            self.cpu.update_timers();

            if self.cpu.sound_on() {
                audio_device.resume();
            } else {
                audio_device.pause();
            }

            if self.cpu.needs_redraw() {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();

                for (i, pixel) in self.display.into_iter().enumerate() {
                    let i = i as u32;
                    if pixel {
                        canvas
                            .copy(
                                &pixel_texture,
                                None,
                                Rect::new(
                                    ((i % DISPLAY_WIDTH as u32) * PIXEL_SIZE) as i32,
                                    ((i / DISPLAY_WIDTH as u32) * PIXEL_SIZE) as i32,
                                    PIXEL_SIZE,
                                    PIXEL_SIZE,
                                ),
                            )
                            .unwrap();
                    }
                }
                canvas.present();
            }

            // std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
}
