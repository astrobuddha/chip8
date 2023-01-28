use rand;
use std::fs::File;

use display::Display;
use keypad::Keypad;

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

    }
}

