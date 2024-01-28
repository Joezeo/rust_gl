use glutin::{
    api::egl::{device::Device, display::Display},
    config::{Config, ConfigSurfaceTypes, ConfigTemplateBuilder, GlConfig},
    context::{ContextApi, ContextAttributesBuilder, NotCurrentContext, Version},
    display::{GetGlDisplay, GlDisplay},
};
use raw_window_handle::HasRawWindowHandle;
use glutin_winit::DisplayBuilder;
use winit::{event_loop::EventLoopWindowTarget, window::{Window, WindowBuilder}};
use std::error::Error;

pub fn bootstrap_gl_window(
    target: &EventLoopWindowTarget<()>,
    win_builder: WindowBuilder,
) -> Result<(Window, Config, Option<NotCurrentContext>), Box<dyn Error>> {
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(cfg!(cgl_backend));

    let display_builder = DisplayBuilder::new().with_window_builder(Some(win_builder));
    let (window, gl_config) = display_builder.build(target, template, |configs| {
        // Find the config with the maximum number of samples, so our triangle will
        // be smooth.
        configs
            .reduce(|accum, config| {
                let transparency_check = config.supports_transparency().unwrap_or(false)
                    & !accum.supports_transparency().unwrap_or(false);

                if transparency_check || config.num_samples() > accum.num_samples() {
                    config
                } else {
                    accum
                }
            })
            .unwrap()
    })?;

    let raw_window_handle = window
        .as_ref()
        .map(|window| window.raw_window_handle());

    // XXX The display could be obtained from any object created by it, so we can
    // query it from the config.
    let gl_display = gl_config.display();

    // The context creation part. It can be created before surface and that's how
    // it's expected in multithreaded + multiwindow operation mode, since you
    // can send NotCurrentContext, but not Surface.
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    let not_current_gl_context = Some(unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(&gl_config, &fallback_context_attributes)
                    .unwrap_or_else(|_| {
                        gl_display
                            .create_context(&gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    })
            })
    });

    Ok((window.unwrap(), gl_config, not_current_gl_context))
}

/// Use Egl to create an off-screen gl context.
pub fn bootstrap_off_screen_gl() -> Result<(Config, NotCurrentContext), Box<dyn Error>>
{
    let devices = Device::query_devices()
        .expect("Failed to query devices")
        .collect::<Vec<_>>();

    for (index, device) in devices.iter().enumerate() {
        println!(
            "Device {}: Name: {} Vendor: {}",
            index,
            device.name().unwrap_or("UNKNOWN"),
            device.vendor().unwrap_or("UNKNOWN")
        );
    }

    let device = devices.first().expect("No available devices");

    // Create a display using the device.
    let display = unsafe { Display::with_device(device, None)? };

    let template = ConfigTemplateBuilder::default()
        .with_alpha_size(8)
        // Offscreen rendering has no support window surface support.
        .with_surface_type(ConfigSurfaceTypes::empty())
        .build();

    let config = unsafe { display.find_configs(template) }
        .unwrap()
        .reduce(|config, acc| {
            if config.num_samples() > acc.num_samples() {
                config
            } else {
                acc
            }
        })
        .expect("No available configs");

    println!("Picked a config with {} samples", config.num_samples());

    // Context creation.
    //
    // In particular, since we are doing offscreen rendering we have no raw window
    // handle to provide.
    let context_attributes = ContextAttributesBuilder::new().build(None);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(None);

    let not_current_context = unsafe {
        display
            .create_context(&config, &context_attributes)
            .unwrap_or_else(|_| {
                display
                    .create_context(&config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
    };

    Ok((Config::Egl(config), NotCurrentContext::Egl(not_current_context)))
}
