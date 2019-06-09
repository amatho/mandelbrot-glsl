#![cfg_attr(
    all(not(debug_assertions), not(feature = "console")),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate gfx;

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::Api;
use glutin::WindowEvent;
use glutin::{Event, GlContext, GlRequest, VirtualKeyCode};

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const DIMENSIONS: (u32, u32) = (800, 800);
const MAX_ITERATIONS: u32 = 1000;
const ZOOM_FACTOR: f64 = 1.05;
const CENTER_POINT: (f64, f64) = (-0.5, 0.0);
const PIXEL_DELTA: f64 = 0.003_141_5;

const VERTICES: [Vertex; 6] = [
    Vertex {
        pos: [-1.0, -1.0, 0.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        pos: [-1.0, -1.0, 0.0, 1.0],
    },
    Vertex {
        pos: [1.0, -1.0, 0.0, 1.0],
    },
];

type ColorFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
    }

    // Unfortunately we have to declare all fields as f64 in order to not get
    // errors related to padding
    constant Locals {
        width: f64 = "i_ScreenWidth",
        height: f64 = "i_ScreenHeight",
        max_iterations: f64 = "i_MaxIterations",
        pixel_delta: f64 = "i_PixelDelta",
        center_re: f64 = "i_CenterRe",
        center_im: f64 = "i_CenterIm",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Mandelbrot Visualizer")
        .with_dimensions(DIMENSIONS.into())
        .with_resizable(false);
    let context_builder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)))
        .with_vsync(true);
    let (window, mut device, mut factory, color_view, mut _depth_view) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &events_loop);

    let pso = factory
        .create_pipeline_simple(
            include_bytes!("shader/mandelbrot.vert"),
            include_bytes!("shader/mandelbrot.frag"),
            pipe::new(),
        )
        .unwrap();

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut locals = Locals {
        width: DIMENSIONS.0.into(),
        height: DIMENSIONS.1.into(),
        max_iterations: MAX_ITERATIONS.into(),
        pixel_delta: PIXEL_DELTA,
        center_re: CENTER_POINT.0,
        center_im: CENTER_POINT.1,
    };

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&VERTICES, ());
    let locals_buffer = factory.create_constant_buffer(1);
    let data = pipe::Data {
        vbuf: vertex_buffer,
        locals: locals_buffer,
        out: color_view.clone(),
    };

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => running = false,
                    WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                virtual_keycode: Some(key_code),
                                ..
                            },
                        ..
                    } => handle_input(key_code, &mut locals),
                    _ => {}
                }
            }
        });

        encoder.clear(&color_view, BLACK);
        encoder.update_buffer(&data.locals, &[locals], 0).unwrap();
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}

fn handle_input(key_code: VirtualKeyCode, locals: &mut Locals) {
    let shortest_dim = if DIMENSIONS.0 < DIMENSIONS.1 {
        DIMENSIONS.0
    } else {
        DIMENSIONS.1
    };

    let step = locals.pixel_delta * f64::from(shortest_dim / 100);
    let mut transform_re = 0.0;
    let mut transform_im = 0.0;

    if let VirtualKeyCode::A = key_code {
        transform_re -= step;
    }
    if let VirtualKeyCode::D = key_code {
        transform_re += step;
    }
    if let VirtualKeyCode::W = key_code {
        transform_im += step;
    }
    if let VirtualKeyCode::S = key_code {
        transform_im -= step;
    }

    if let VirtualKeyCode::Up = key_code {
        locals.pixel_delta /= ZOOM_FACTOR;
    }
    if let VirtualKeyCode::Down = key_code {
        locals.pixel_delta *= ZOOM_FACTOR;
    }

    locals.center_re += transform_re;
    locals.center_im += transform_im;
}
