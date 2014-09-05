mod memory;
mod rom;

fn main() {
    let args = std::os::args();
    let path = Path::new(args[1].clone());
    let rom = rom::Rom::from_path(&path);
    let memory = memory::Memory::new(rom);
    println!("Catridge type: {:#04X}", memory.rom.catrigde_type());
    println!("ROM size: {:#04X}", memory.rom.rom_size());
    println!("RAM size: {:#04X}", memory.rom.ram_size());
    println!("Reading catridge header area from memory:");

    for addr in range(0x0100, 0x014F) {
        print!("{:#04X} ", memory.read_byte(addr));
    }
}
