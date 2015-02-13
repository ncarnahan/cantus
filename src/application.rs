use glium::{Display, Surface, VertexBuffer, IndexBuffer, Program};

pub fn run() {
    let mut app = Application::new();
    while app.is_running {
        app.update();
    }
}



#[vertex_format]
#[derive(Copy)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

pub struct Application {
    is_running: bool,

    display: Display,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer,
    program: Program,
}

impl Application {
    fn new() -> Application {
        use glium::DisplayBuild;

        //Create the window and display
        let display = ::glutin::WindowBuilder::new()
            .build_glium()
            .unwrap();

        let vertex_buffer = VertexBuffer::new(&display, vec![
            Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
        ]);

        let index_buffer = IndexBuffer::new(&display,
            ::glium::index::TrianglesList(vec![0u16, 1, 2]));

        let program = Program::from_source(&display,
            "
                #version 110
                uniform mat4 matrix;
                attribute vec2 position;
                attribute vec3 color;
                varying vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",
            "
                #version 110
                varying vec3 vColor;
                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
            None).unwrap();

        Application {
            is_running: true,

            display: display,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            program: program,
        }
    }

    fn update(&mut self) {
        use std::old_io::timer;
        use std::time::Duration;

        // building the uniforms
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // drawing a frame
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(
            &self.vertex_buffer, &self.index_buffer, &self.program,
            &uniforms, &::std::default::Default::default()).unwrap();
        target.finish();

        // sleeping for some time in order not to use up too much CPU
        timer::sleep(Duration::milliseconds(17));

        // polling and handling the events received by the window
        for event in self.display.poll_events() {
            match event {
                ::glutin::Event::Closed => self.is_running = false,
                _ => ()
            }
        }
    }
}
