use rand;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time;

use crate::display::Display;
use crate::keypad::Keypad;

pub struct Cpu {
    program: usize, // program counter starts at 512 bytes
    opcode: u16, // current opcode
    stack: [u16; 16], // stack storage
    stack_pointer: usize, // stack pointer

    v: [u8; 16], // cpu registers (V0 through Ee)
    i: usize, // index register

    memory: [u8; 4096], // system memory
    keypad: Keypad, // intercept keyboard calls
    display: Display // visualize on screen
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            program: 0x200,
            opcode: 0,
            stack: [0; 16],
            stack_pointer: 0,

            v: [0; 16],
            i: 0x200,

            memory: [0; 4096],
            keypad: Keypad::new(),
            display: Display::new(),

        }
    }

    pub fn load_game(&mut self, game: &str) {
        // attempt to load supplied ROM
        let mut reader = File::open(game).expect("Unable to locage ROM");

        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer).expect("Unable to read ROM data");

        // load ROM into memory (AFTER system reserved memory)
        for i in 0..buffer.len() {
            self.memory[i + self.program] = buffer[i];
        };


    }

    pub fn emulate_cycle(&mut self) {
        self.fetch_opcode();
        self.execute_opcode();

        // okay, PCs are much faster these days
        // threw this cheeky delay in to slow things down
        thread::sleep(time::Duration::from_micros(500));
    }

    fn fetch_opcode(&mut self) {
        self.opcode = (self.memory[self.program as usize] as u16) << 8 | (self.memory[(self.program + 1)] as u16);
    }

    fn execute_opcode(&mut self) {
        match self.opcode & 0xf000 {
        0x0000 => self.op_0xxx(),
        0x1000 => self.op_1xxx(),
        0x2000 => self.op_2xxx(),
        _ => { not_implemented(self.opcode as usize, self.program) }
        }
    }

    // clear the display or return from sub
    fn op_0xxx(&mut self) {
        match self.opcode & 0x000f {
            0x000 => { self.display.clear() }
        }
    }

    fn op_1xxx(&mut self) {
        self.program = self.op_nnn() as usize;
    }


}

static FONTS: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

