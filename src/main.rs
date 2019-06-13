use skylark::emu;
use std::{thread, time, fs};

fn main() {
    run_emulator("roms/scrolling_logo.ch8");
}

fn run_emulator(file_name: &str){
    let mut emulator = emu::Emulator::new();

    // let mut file = File::open(file_name).expect("Failed to read file");
    let rom_bytes = match fs::read(file_name) {
        Err(e) => panic!("couldn't open {}: {}", file_name, e),
        Ok(file) => file
    };

    emulator.load_rom(rom_bytes);

    loop {
        thread::sleep(time::Duration::from_millis(10));

        clear_screen();
        print!("{}", emulator.display_out());
        emulator.tick();
    }
}

fn run_universe(){
    let mut universe = emu::Universe::new();

    loop {
        thread::sleep(time::Duration::from_millis(10));

        clear_screen();
        print!("{}", universe.to_string());
        universe.tick();
    }
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
