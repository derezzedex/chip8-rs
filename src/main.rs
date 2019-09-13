#[macro_use]
extern crate glium;

use std::fs::File;
use std::path::Path;
use std::io::Read;

use glium::glutin::{Event, KeyboardInput, ElementState};
use glium::glutin::WindowEvent;

mod graphics;
mod cpu;

///```
///Keypad                   Keyboard
///+-+-+-+-+                +-+-+-+-+
///|1|2|3|C|                |1|2|3|4|
///+-+-+-+-+                +-+-+-+-+
///|4|5|6|D|                |Q|W|E|R|
///+-+-+-+-+       =>       +-+-+-+-+
///|7|8|9|E|                |A|S|D|F|
///+-+-+-+-+                +-+-+-+-+
///|A|0|B|F|                |Z|X|C|V|
///+-+-+-+-+                +-+-+-+-+
///```
fn keyboard_to_keypad(scancode: u32) -> i8{
    match scancode{
        0x16 => 0x1,
        0x1E => 0x2,
        0x26 => 0x3,
        0x25 => 0xC,
        0x15 => 0x4,
        0x1D => 0x5,
        0x24 => 0x6,
        0x2D => 0xD,
        0x1C => 0x7,
        0x1B => 0x8,
        0x23 => 0x9,
        0x2B => 0xE,
        0x1A => 0xA,
        0x22 => 0x0,
        0x21 => 0xB,
        0x2A => 0xF,
        _ => -1,
    }
}

fn main() {
    let roms_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("roms");
    let args: Vec<String> = std::env::args().collect();

    let mut chip8 =  cpu::Chip8::new();

    let rom = if args.len() > 1{
        roms_path.join(&args[1])
    }else{
        roms_path.join("programs/ibm.ch8")
    };

    println!("Loading file {:?}", rom);
    let mut file = File::open(rom)
        .expect("Couldn't open file!");
    //
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Couldn't read to buffer!");

    chip8.initialize(0x200);
    chip8.load_program(buffer);

    let mut renderer = graphics::Renderer::new();
    let mut running = true;
    while running{
        for event in renderer.poll_events(){
            match event{
                Event::WindowEvent{ event, ..} => match event{
                    WindowEvent::CloseRequested => running = false,
                    WindowEvent::Resized(..) => chip8.draw_flag = true,
                    WindowEvent::KeyboardInput{ input, .. } => match input{
                        KeyboardInput { scancode, state, ..} => {

                            let keydown = keyboard_to_keypad(scancode);
                            if keydown != -1 {
                                if state == ElementState::Pressed{
                                    // if chip8 was waiting for a keydown, re-enable emulating
                                    if chip8.waiting_for_keydown(){
                                        chip8.set_key(0x10, 0);
                                    }
                                    chip8.set_key(keydown as u8, 1);
                                }else{
                                    chip8.set_key(keydown as u8, 0);
                                }
                        }

                        }
                    },
                    _ => (),
                },
                _ => (),
            }
        }

        if !chip8.waiting_for_keydown(){
            chip8.emulate_cycle();

            if chip8.draw_flag{
                renderer.new_frame();
                renderer.clear_screen();

                let mut display: Vec<Vec<graphics::Color>> = vec![vec![(0, 0, 0); 64]; 32];
                for (n, pixel) in chip8.get_display().iter().enumerate(){
                    if *pixel != 0{
                        let i = n / 64;
                        let j = n % 64;
                        display[i][j] = (255, 255, 255);
                    }
                }
                renderer.draw_screen(display);
                renderer.finish_frame();

                chip8.draw_flag = false;
            }
        }
    }
}
