use keypad::scan_key;
use machine::Machine;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod display;
mod keypad;
mod machine;
mod op_code;

fn data_from_file(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut data = Vec::<u8>::new();
    f.read_to_end(&mut data).expect("File not found...");
    data
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();
    let mut machine = Machine::new();
    machine.load_rom(&data_from_file(filename));
    let sdl_context = sdl2::init().unwrap();
    let mut display = display::Display::new(&sdl_context);
    let mut events = sdl_context.event_pump().unwrap();

    'event_loop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'event_loop,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(keyindex) = scan_key(keycode) {
                        machine.keydown(keyindex);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(keyindex) = scan_key(keycode) {
                        machine.keyup(keyindex);
                    }
                }
                _ => {}
            }
        }

        for _ in 0..10 {
            let op = machine.decode_op();
            machine.execute_op(&op);
        }

        display.draw(machine.read_display());
        machine.tick();
    }
}
