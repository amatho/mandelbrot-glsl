#[macro_use]
extern crate glium;

use glium::glutin;
use glium::glutin::{Event, KeyboardInput, VirtualKeyCode};
use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

const DIMENSIONS: (u32, u32) = (800, 800);
const MAX_ITERATIONS: u32 = 1000;
const ZOOM_FACTOR: f64 = 1.05;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Mandelbrot Visualizer")
        .with_dimensions(DIMENSIONS.into())
        .with_resizable(false);
    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)));
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let program = glium::Program::from_source(
        &display,
        include_str!("mandelbrot.glslv"),
        include_str!("mandelbrot.glslf"),
        None,
    ).unwrap();

    let mut center_point = (-0.5, 0.0);
    let mut pixel_delta = 0.003_141_5;

    // Render 2 triangles covering the whole screen
    let vertices = [
        // Top-left corner
        Vertex {
            position: [-1.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
        // Bottom-right corner
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
        },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();

    let mut running = true;
    while running {
        let mut target = display.draw();
        target
            .draw(
                &vertex_buffer,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &program,
                &uniform!{
                    screen_size: DIMENSIONS,
                    center_point: center_point,
                    pixel_delta: pixel_delta,
                    max_iterations: MAX_ITERATIONS,
                },
                &Default::default(),
            ).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: glutin::WindowEvent::CloseRequested,
                ..
            } => running = false,
            Event::DeviceEvent {
                event:
                    glutin::DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    }),
                ..
            } => running = false,
            Event::DeviceEvent {
                event:
                    glutin::DeviceEvent::Key(KeyboardInput {
                        virtual_keycode: Some(key_code),
                        ..
                    }),
                ..
            } => {
                let shortest_dim = if DIMENSIONS.0 < DIMENSIONS.1 {
                    DIMENSIONS.0
                } else {
                    DIMENSIONS.1
                };

                let step = pixel_delta * f64::from(shortest_dim / 100);

                if let VirtualKeyCode::A = key_code {
                    transform(&mut center_point, -step, 0.0);
                }
                if let VirtualKeyCode::D = key_code {
                    transform(&mut center_point, step, 0.0);
                }
                if let VirtualKeyCode::W = key_code {
                    transform(&mut center_point, 0.0, step);
                }
                if let VirtualKeyCode::S = key_code {
                    transform(&mut center_point, 0.0, -step);
                }

                if let VirtualKeyCode::Up = key_code {
                    pixel_delta /= ZOOM_FACTOR;
                }
                if let VirtualKeyCode::Down = key_code {
                    pixel_delta *= ZOOM_FACTOR;
                }
            }
            _ => (),
        });
    }
}

fn transform(ul: &mut (f64, f64), x: f64, y: f64) {
    *ul = (ul.0 + x, ul.1 + y);
}
