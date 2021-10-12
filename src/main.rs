use machine::Machine;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
    let mut display = display::Display::new(&sdl_context);

    let mut last_op_time = Instant::now();
    let mut last_display_time = Instant::now();

    let mut events = sdl_context.event_pump().unwrap();

    'event_loop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'event_loop,
                _ => {}
            }
        }

        if Instant::now() - last_op_time > Duration::from_millis(2) {
            let op = machine.decode_op();
            machine.execute_op(op);
            last_op_time = Instant::now();
        }

        if Instant::now() - last_display_time > Duration::from_millis(10) {
            display.draw(machine.read_display());
            last_display_time = Instant::now();
        }
    }
}
