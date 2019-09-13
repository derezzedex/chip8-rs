#[macro_use]
extern crate glium;

use std::fs::File;
use std::path::Path;
use std::io::Read;

use glium::glutin::{Event, KeyboardInput, ElementState, VirtualKeyCode};
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
fn keyboard_to_keypad(keycode: VirtualKeyCode) -> i8{
    match keycode{
        VirtualKeyCode::Key1 => 0x1,
        VirtualKeyCode::Key2 => 0x2,
        VirtualKeyCode::Key3 => 0x3,
        VirtualKeyCode::Key4 => 0xC,
        VirtualKeyCode::Q => 0x4,
        VirtualKeyCode::W => 0x5,
        VirtualKeyCode::E => 0x6,
        VirtualKeyCode::R => 0xD,
        VirtualKeyCode::A => 0x7,
        VirtualKeyCode::S => 0x8,
        VirtualKeyCode::D => 0x9,
        VirtualKeyCode::F => 0xE,
        VirtualKeyCode::Z => 0xA,
        VirtualKeyCode::X => 0x0,
        VirtualKeyCode::C => 0xB,
        VirtualKeyCode::V => 0xF,
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
        roms_path.join("tests/test_opcode.ch8")
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
                        KeyboardInput { virtual_keycode, state, ..} =>{
                            if let Some(keycode) = virtual_keycode{
                                let keydown = keyboard_to_keypad(keycode);
                                if keydown != -1 {
                                    if state == ElementState::Pressed{
                                        chip8.set_key(keydown as u8, 1);
                                    }else{
                                        chip8.set_key(keydown as u8, 0);
                                    }
                                }
                            }
                        }
                    },
                    _ => (),
                },
                _ => (),
            }
        }

        chip8.emulate_cycle();

        if chip8.draw_flag{
            // println!("Drawing");
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
