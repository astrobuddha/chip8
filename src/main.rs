use cpu::Cpu;

mod cpu;

fn main() {
    let mut cpu = Cpu::new();

    cpu.load_game("/home/jozef/");
}
