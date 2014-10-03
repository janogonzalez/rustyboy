mod cpu;
mod gpu;
mod memory;
mod rom;
mod timer;

static LOGO: &'static str =
r"
                _         _
 _ __ _   _ ___| |_ _   _| |__   ___  _   _
| '__| | | / __| __| | | | '_ \ / _ \| | | |
| |  | |_| \__ \ |_| |_| | |_) | (_) | |_| |
|_|   \__,_|___/\__|\__, |_.__/ \___/ \__, |
                    |___/             |___/
";

fn main() {
    let args = std::os::args();
    let path = Path::new(args[1].clone());
    let rom = rom::Rom::from_path(&path);
    let gpu = gpu::Gpu::new();
    let timer = timer::Timer::new();
    let memory = memory::Memory::new(rom, gpu, timer);
    let mut cpu = cpu::Cpu::new(memory);

    println!("{}", LOGO);
    println!("");
    println!("Rom name: {}", cpu.memory.rom.name());
    println!("Catridge type: {:#04X}", cpu.memory.rom.catrigde_type());
    println!("ROM size: {:#04X}", cpu.memory.rom.rom_size());
    println!("RAM size: {:#04X}", cpu.memory.rom.ram_size());
    println!("");
    println!("Running game:");

    cpu.run();
}
