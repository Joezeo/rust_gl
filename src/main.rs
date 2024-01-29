use glutin::{
    context::{NotCurrentGlContext, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, Surface, WindowSurface},
};
use glutin_winit::GlWindow;
use renderer::Renderer;
use samples::Sample;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoopBuilder,
    window::WindowBuilder,
};

use crate::gl_bootstrap::bootstrap_gl_window;

pub mod gl_bootstrap;
pub mod renderer;
pub mod samples;
pub mod shaders;

struct GlState {
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
    renderer: Renderer,
}

fn main() {
    let event_loop = EventLoopBuilder::<()>::default().build().unwrap();

    let win_bld = WindowBuilder::new()
        .with_transparent(true)
        .with_title("Gl Window");

    let (window, gl_config, mut not_current_context) =
        bootstrap_gl_window(&event_loop, win_bld).expect("Bootstrap gl window failed.");

    let mut state = None;

    event_loop
        .run(|event, target| match event {
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

                state = Some(GlState {
                    context: gl_context,
                    surface: gl_surface,
                    renderer: Renderer::new(&gl_display, Sample::SimpleTriangle),
                })
            }
            Event::AboutToWait => {
                if let Some(ref state) = state {
                    state.renderer.draw();
                    window.request_redraw();

                    state.surface.swap_buffers(&state.context).unwrap();
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => target.exit(),
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}
