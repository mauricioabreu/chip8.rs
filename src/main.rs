use display::Display;
use machine::Machine;
use std::env;
use std::fs::File;
use std::io::Read;

mod display;
mod machine;
mod op_code;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();
    let mut f = File::open(filename).unwrap();
    let mut data = Vec::<u8>::new();
    f.read_to_end(&mut data).expect("File not found...");
    let mut machine = Machine::new();
    machine.load_rom(data);
    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl_context);

    loop {
        let op = machine.decode_op();
        machine.execute_op(op);
        display.draw(&machine.read_display());
    }
}
