use glium::glutin;
use glium::Surface;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3]
}

impl Vertex{
    pub fn new(position: [f32; 3]) -> Self{
        Self{
            position
        }
    }
}

implement_vertex!(Vertex, position);

const PIXEL_SIZE: u32 = 20;

pub struct Renderer {
    display: glium::Display,
    events_loop: glutin::EventsLoop,
    frame: Option<glium::Frame>,
    program: glium::Program,
}

impl Renderer {
    pub fn new() -> Self {
        let events_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new()
            .with_title("CHIP-8 Emulator")
            .with_dimensions((64 * PIXEL_SIZE, 32 * PIXEL_SIZE).into());
        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_multisampling(4)
            .with_vsync(true);
        let display =
            glium::Display::new(wb, cb, &events_loop).expect("Couldn't create glium display!");
        let frame = None;

        let vertex = "
            #version 140
            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                vColor = color;
            }
        ";
        let fragment = "
            #version 140

            out vec4 f_color;
            void main() {
                f_color = vec4(1.0, 1.0, 1.0, 1.0);
            }
        ";

        let program = glium::Program::from_source(&display, &vertex, &fragment, None)
            .expect("Couldn't create shader program!");

        Self {
            display,
            events_loop,
            frame,
            program,
        }
    }

    pub fn clear_screen(&mut self) {
        self.frame
            .as_mut()
            .expect("No frame to clear color!")
            .clear_color(0.0, 0.0, 0.0, 1.0);
    }

    pub fn draw_sprite(&mut self) {

    }

    pub fn new_frame(&mut self) {
        let target = self.display.draw();
        self.frame = Some(target);
    }

    #[allow(unused_must_use)]
    pub fn finish_frame(&mut self) {
        if let Some(frame) = self.frame.take() {
            frame.finish(); // Possibly returns an error, but it should be safe.
        }
    }
}
