use rand;

use rand::Rng;

use crate::display::Display;
use crate::kbd::Keyboard;
use crate::memory::Memory;

pub struct Cpu {
    v: [u8; 16],
    i: u16,
    pc: u16,

    stack: [u16; 16],
    sp: usize,

    needs_redraw: bool,

    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 16],
            i: 0,
            pc: 0x200,

            stack: [0; 16],
            sp: 0,

            needs_redraw: true,

            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn cycle(&mut self, memory: &mut Memory, display: &mut Display, kbd: &Keyboard) {
        self.needs_redraw = false;

        let opcode = memory.read_word(self.pc);

        self.pc += 2;

        // Extract common variables
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        // Break up into nibbles
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        match (op_1, op_2, op_3, op_4) {
            // CLS
            (0, 0, 0xE, 0) => {
                display.clear();
                self.needs_redraw = true;
            }
            // RET
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp] + 2;
            }
            // SYS addr
            (0, _, _, _) => (),
            // JP addr
            (1, _, _, _) => self.pc = nnn,
            // CALL addr
            (2, _, _, _) => {
                self.stack[self.sp] = self.pc - 2;
                self.sp += 1;
                self.pc = nnn;
            }
            // SE Vx, byte
            (3, _, _, _) => {
                if vx == kk {
                    self.pc += 2;
                }
            }
            // SNE Vx, byte
            (4, _, _, _) => {
                if vx != kk {
                    self.pc += 2;
                }
            }
            // SE Vx, Vy
            (5, _, _, _) => {
                if vx == vy {
                    self.pc += 2;
                }
            }
            // LD Vx, byte
            (6, _, _, _) => self.v[x] = kk,
            // ADD Vx, byte
            (7, _, _, _) => self.v[x] = vx.wrapping_add(kk),
            // LD Vx, Vy
            (8, _, _, 0) => self.v[x] = vy,
            // OR Vx, Vy
            (8, _, _, 1) => self.v[x] = vx | vy,
            // AND Vx, Vy
            (8, _, _, 2) => self.v[x] = vx & vy,
            // XOR Vx, Vy
            (8, _, _, 3) => self.v[x] = vx ^ vy,
            // ADD Vx, Vy
            (8, _, _, 4) => {
                let (res, overflow) = vx.overflowing_add(vy);
                self.v[x] = res;
                self.v[0xF] = if overflow { 1 } else { 0 };
            }
            // SUB Vx, Vy
            (8, _, _, 5) => {
                let (res, overflow) = vx.overflowing_sub(vy);
                self.v[x] = res;
                self.v[0xF] = if overflow { 1 } else { 0 };
            }
            // SHR Vx {, Vy}
            (8, _, _, 6) => {
                self.v[0xF] = vy & 0b00000001;
                self.v[y] >>= 1;
                self.v[x] = self.v[y];
            }
            // SUBN Vx, Vy
            (8, _, _, 7) => {
                let (res, overflow) = vy.overflowing_sub(vx);
                self.v[x] = res;
                self.v[0xF] = if overflow { 1 } else { 0 };
            }
            // SHL Vx {, Vy}
            (8, _, _, 0xE) => {
                self.v[0xF] = (vy & 0b10000000) >> 7;
                self.v[y] <<= 1;
                self.v[x] = self.v[y];
            }
            // SNE Vx, Vy
            (9, _, _, 0) => {
                if vx != vy {
                    self.pc += 2;
                }
            }
            // LD I, addr
            (0xA, _, _, _) => self.i = nnn,
            // JP V0, addr
            (0xB, _, _, _) => self.pc = nnn + self.v[0] as u16,
            // RND Vx, byte
            (0xC, _, _, _) => {
                let r = rand::thread_rng().gen_range(0, 255);
                self.v[x] = r & kk;
            }
            // DRW Vx, Vy, nibble
            (0xD, _, _, _) => {
                self.v[0xF] = 0;

                let sprite_start = self.i;
                let sprite_end = self.i + n as u16;
                let sprite = Vec::from(memory.read_range(sprite_start, sprite_end));

                if display.draw_sprite(vx, vy, sprite) {
                    self.v[0xF] = 1;
                }
                self.needs_redraw = true;
            }
            // SKP Vx
            (0xE, _, 9, 0xE) => {
                if kbd.key_pressed(vx) {
                    self.pc += 2;
                }
            }
            // SKNP Vx
            (0xE, _, 0xA, 1) => {
                if !kbd.key_pressed(vx) {
                    self.pc += 2;
                }
            }
            // LD Vx, DT
            (0xF, _, 0, 7) => self.v[x] = self.delay_timer,
            // LD Vx, K
            (0xF, _, 0, 0xA) => {
                if !kbd.key_pressed(vx) {
                    self.pc -= 2;
                }
            }
            // LD DT, Vx
            (0xF, _, 1, 5) => self.delay_timer = vx,
            // LD ST, Vx
            (0xF, _, 1, 8) => self.sound_timer = vx,
            // ADD I, Vx
            (0xF, _, 1, 0xE) => self.i += vx as u16,
            // LD F, Vx
            (0xF, _, 2, 9) => self.i = memory.get_char_addr(vx as u16),
            // LD B, Vx
            (0xF, _, 3, 3) => {
                memory.write(self.i, vx / 100);
                memory.write(self.i + 1, vx % 100 / 10);
                memory.write(self.i + 2, vx % 10);
            }
            // LD [I], Vx
            (0xF, _, 5, 5) => {
                for i in 0..x + 1 {
                    memory.write(self.i + i as u16, self.v[i]);
                }
            }
            // LD Vx, [I]
            (0xF, _, 6, 5) => {
                for i in 0..x + 1 {
                    self.v[i] = memory.read(self.i + i as u16);
                }
            }
            _ => panic!("Unknown opcode: {:>04X}", opcode),
        }
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    pub fn sound_on(&self) -> bool {
        self.sound_timer > 0
    }

    pub fn print_regs(&self) {
        println!(
            "{}",
            self.v
                .iter()
                .enumerate()
                .map(|(i, v)| format!("V{:X}: {:>02X}", i, v))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
