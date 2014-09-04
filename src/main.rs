mod rom;

fn main() {
    let args = std::os::args();
    let path = Path::new(args[1].clone());
    let rom = rom::Rom::from_path(&path);
    println!("Catrigde type: {:#04x}", rom.catrigde_type());
    println!("ROM size: {:#04x}", rom.rom_size());
    println!("RAM size: {:#04x}", rom.ram_size());
}
