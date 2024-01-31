pub mod error;
pub mod gl_bootstrap;
pub mod renderer;
pub mod samples;
pub mod shaders;

use std::num::NonZeroU32;

use crate::gl_bootstrap::bootstrap_gl_window;
use glutin::{
    context::{NotCurrentGlContext, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, Surface, SwapInterval, WindowSurface},
};
use glutin_winit::GlWindow;
use renderer::Renderer;
use samples::Sample;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoopBuilder,
    window::WindowBuilder,
};

const SAMPLE: Sample = Sample::SimpleSquare;

fn main() {
    let event_loop = EventLoopBuilder::<()>::default().build().unwrap();

    let win_bld = WindowBuilder::new()
        .with_transparent(true)
        .with_title("Gl Window");

    let (window, gl_config, mut not_current_context) =
        bootstrap_gl_window(&event_loop, win_bld).expect("Bootstrap gl window failed.");

    let mut state = None;

    event_loop
        .run(|event, target| {
            match event {
                Event::Resumed => {
                    let attrs = window.build_surface_attributes(Default::default());
                    let gl_display = gl_config.display();
                    let gl_surface = unsafe {
                        gl_display
                            .create_window_surface(&gl_config, &attrs)
                            .unwrap()
                    };

                    let gl_context = not_current_context
                        .take()
                        .unwrap()
                        .make_current(&gl_surface)
                        .expect("Make current context failed.");

                    if let Err(res) = gl_surface.set_swap_interval(
                        &gl_context,
                        SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
                    ) {
                        eprintln!("Error setting vsync: {res:?}");
                    }

                    state = Some(GlState {
                        context: gl_context,
                        surface: gl_surface,
                        renderer: Renderer::new(&gl_display, SAMPLE),
                    })
                }

                Event::AboutToWait => {
                    if let Some(GlState { renderer, .. }) = state.as_ref() {
                        if renderer.snapshot() {
                            window.request_redraw();
                        }
                    }
                }

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => target.exit(),

                    WindowEvent::Resized(size) => {
                        if let Some(GlState { renderer, .. }) = state.as_ref() {
                            renderer.resize(size.width as i32, size.height as i32)
                        }
                    }

                    WindowEvent::RedrawRequested => {
                        if let Some(GlState {
                            context,
                            surface,
                            renderer,
                        }) = state.as_ref()
                        {
                            renderer.draw();
                            surface.swap_buffers(context).unwrap();
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .unwrap();
}

struct GlState {
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
    renderer: Renderer,
}
