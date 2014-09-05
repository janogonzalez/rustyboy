mod cpu;
mod memory;
mod rom;

fn main() {
    let args = std::os::args();
    let path = Path::new(args[1].clone());
    let rom = rom::Rom::from_path(&path);
    let memory = memory::Memory::new(rom);
    let mut cpu = cpu::Cpu::new(memory);

    println!("Catridge type: {:#04X}", cpu.memory.rom.catrigde_type());
    println!("ROM size: {:#04X}", cpu.memory.rom.rom_size());
    println!("RAM size: {:#04X}", cpu.memory.rom.ram_size());

    println!("");
    println!("Reading catridge header area from memory:");

    for addr in range(0x0100, 0x014F) {
        print!("{:#04X} ", cpu.memory.read_byte(addr));
    }

    println!("");
    println!("");
    println!("Running game:");

    cpu.run();
}
