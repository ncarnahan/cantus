use glium::Surface;

pub fn run() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = ::glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[vertex_format]
        #[derive(Copy)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }

        ::glium::VertexBuffer::new(&display, 
            vec![
                Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
                Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
            ]
        )
    };

    // building the index buffer
    let index_buffer = ::glium::IndexBuffer::new(&display,
        ::glium::index::TrianglesList(vec![0u16, 1, 2]));

    // compiling shaders and linking them together
    let program = ::glium::Program::from_source(&display,
        // vertex shader
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

        // fragment shader
        "
            #version 110
            varying vec3 vColor;
            void main() {
                gl_FragColor = vec4(vColor, 1.0);
            }
        ",

        // geometry shader
        None)
        .unwrap();
    
    // the main loop
    // each cycle will draw once
    'main: loop {
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
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &::std::default::Default::default()).unwrap();
        target.finish();

        // sleeping for some time in order not to use up too much CPU
        timer::sleep(Duration::milliseconds(17));

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                ::glutin::Event::Closed => break 'main,
                _ => ()
            }
        }
    }
}
