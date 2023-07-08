// extern crate sdl2;
// extern crate cimgui_sys;

// use sdl2 dependencies
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};

// use sdl2::sys::*;

// use cimgui dependencies
use cimgui_sys::{IgContext, IgNavInputFlags, IgStyle, IgDrawData};
use cimgui_sys::IgSDL2Renderer;
use cimgui_sys::show_demo_window;

fn main() -> Result<(), String> {
    // initialize sdl
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?; // VideoSubsystem
    let window = video_subsystem
        .window("cimgui demo", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?; //CreateWindow
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?; // CreateRendere
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, 400, 300)
        .map_err(|e| e.to_string())?;

    let mut ig_context = IgContext::new();
    ig_context.set_style(IgStyle::Dark);
    ig_context.set_flags(IgNavInputFlags::NavEnableKeyboard);

    let ig_renderer = IgSDL2Renderer::new(&mut canvas);
    let mut event_pump = sdl_context.event_pump()?;
    let mut angle = 0.0;

    'mainloop: loop {
        // imgui event process ->
        // instead of calling the iterator to poll events
        // we'll call the SDL_PollEvent to get the raw event from SDL
        // MaybeUninit is inherently unsafe so this requires some more thought how
        // to handle properly the events. some sort of

        for event in event_pump.poll_iter() {
            ig_renderer.process_events(Some(&event)); // process imgui events

            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        ig_renderer.new_frame(); // backend
        ig_context.new_frame();

        // display demo window
        // ShowDemoWindow(true);
        let mut demo_window = true;
        show_demo_window(&mut demo_window);

        angle = (angle + 0.5) % 360.;
        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.clear();
                texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
                texture_canvas
                    .fill_rect(Rect::new(0, 0, 400, 300))
                    .expect("could not fill rect");
            })
            .map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        let dst = Some(Rect::new(0, 0, 400, 300));
        canvas.clear();
        canvas.copy_ex(
            &texture,
            None,
            dst,
            angle,
            Some(Point::new(400, 300)),
            false,
            false,
        )?;

        ig_context.render();
        ig_renderer.render_draw_data(IgDrawData::get());

        canvas.present();
    }

    Ok(())
}
