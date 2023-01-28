use cpu::Cpu;

mod cpu;
mod keypad;
mod display;

fn main() {
    let mut cpu = Cpu::new();

    cpu.load_game("/home/jozef/");
}
