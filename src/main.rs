#[macro_use]
extern crate glium;

use std::fs::File;
use std::io::Read;

mod graphics;
mod cpu;

fn main() {
    let mut chip8 =  cpu::Chip8::new();

    let mut file = File::open("rom.ch8")
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
                glium::glutin::Event::WindowEvent{ event, ..} => match event{
                    glium::glutin::WindowEvent::CloseRequested => running = false,
                    _ => (),
                },
                _ => (),
            }
        }

        chip8.emulate_cycle();

        if chip8.draw_flag{
            renderer.new_frame();
            renderer.clear_screen();

            let mut display: Vec<Vec<graphics::Color>> = vec![vec![(0, 0, 0); 64]; 32];
            for (n, pixel) in chip8.get_display().iter().enumerate(){
                print!("{}", pixel);
                if n%64 == 63{
                    println!();
                }

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
