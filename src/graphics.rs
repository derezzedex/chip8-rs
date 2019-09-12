use glium::glutin;
use glium::Surface;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

// const FRAMEBUFFER_WIDTH:  u32 = 64;
// const FRAMEBUFFER_HEIGHT: u32 = 32;
// const PIXEL_SIZE: u32 = 20;

pub type Color = (u8, u8, u8);

const FRAMEBUFFER_VERTICES: [Vertex; 4] = [
    Vertex {
        position:   [-1.0,  1.0 ],
        tex_coords: [ 0.0,  0.0 ],
    },
    Vertex {
        position:   [-1.0, -1.0 ],
        tex_coords: [ 0.0,  1.0 ],
    },
    Vertex {
        position:   [ 1.0, -1.0 ],
        tex_coords: [ 1.0,  1.0 ],
    },
    Vertex {
        position:   [ 1.0,  1.0 ],
        tex_coords: [ 1.0,  0.0 ],
    },
];
const FRAMEBUFFER_INDICES: [u32; 4] = [1, 2, 0, 3];

pub type Texture = glium::texture::Texture2d;
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
            .with_dimensions((1024, 768).into());
        let cb = glutin::ContextBuilder::new()
            .with_multisampling(4)
            .with_vsync(true);
        let display =
            glium::Display::new(wb, cb, &events_loop).expect("Couldn't create glium display!");
        let frame = None;

        let vertex = "
            #version 140

            attribute vec2 position;
            attribute vec2 tex_coords;
            varying vec2 v_tex_coords;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                v_tex_coords = tex_coords;
            }
        ";
        let fragment = "
            #version 140

            uniform sampler2D tex;

            varying vec2 v_tex_coords;
            void main() {
                gl_FragColor = texture2D(tex, v_tex_coords);
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

    pub fn draw_screen(&mut self, content: Vec<Vec<Color>>) {
        let texture = Texture::new(&self.display, content).expect("Couldn't create empty texture!");

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &FRAMEBUFFER_VERTICES)
            .expect("Coudln't create vertex buffer!");
        let index_buffer = glium::index::IndexBuffer::new(
            &self.display,
            glium::index::PrimitiveType::TriangleStrip,
            &FRAMEBUFFER_INDICES,
        )
        .expect("Coudln't create index buffer!");

        let uniforms = uniform! {
            tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };

        self.frame
            .as_mut()
            .expect("No frame to draw!")
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .expect("Couldn't draw to screen!");
    }

    pub fn poll_events(&mut self) -> Vec<glutin::Event> {
        let mut events = Vec::new();
        self.events_loop.poll_events(|event| events.push(event));
        events
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
