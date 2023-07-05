extern crate sdl2;
extern crate cimgui_sys;

// use sdl2 dependencies
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};

// use sdl2::sys::*;

// use cimgui dependencies
use cimgui_sys::*;

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

    let mut ig_context: *mut ImGuiContext = std::ptr::null_mut(); // main imgui context

    unsafe { // ffi interface is unsafe as we need work directly with the c api
        // initialize imgui
        let shared_font_atlas: *mut ImFontAtlas = std::ptr::null_mut();
        ig_context = igCreateContext(shared_font_atlas);
        let mut io: *mut ImGuiIO = igGetIO();

        // deref unsafe inherently
        (*io).ConfigFlags |= ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard;

        // to set the imgui style, pointer to ImGuiStyle is required
        // igStyleColorsLight(*mut ImGuiStyle)

        let ig_style: *mut ImGuiStyle = igGetStyle();
        igStyleColorsDark(ig_style);

        // initialize backend
        let _ret = ImGui_ImplSDL2_InitForSDLRenderer(canvas.window().raw() as *mut cimgui_sys::SDL_Window, canvas.raw() as *mut cimgui_sys::SDL_Renderer);
        debug_assert!(true == _ret); // must be initialized

        let _ret = ImGui_ImplSDLRenderer_Init(canvas.raw() as *mut cimgui_sys::SDL_Renderer);
        debug_assert!(true == _ret); // must be initialized
    }

    let mut angle = 0.0;

    'mainloop: loop {

        // imgui event process ->
        // instead of calling the iterator to poll events
        // we'll call the SDL_PollEvent to get the raw event from SDL
        unsafe {
            let mut ll_event = std::mem::MaybeUninit::uninit();
            while sdl2::sys::SDL_PollEvent(ll_event.as_mut_ptr()) == 1 {
                // process imgui
                let _ret = ImGui_ImplSDL2_ProcessEvent(ll_event.as_mut_ptr() as *mut SDL_Event);

                let event = Event::from_ll(ll_event.assume_init());
                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    }
                    | Event::Quit { .. } => break 'mainloop,
                    _ => {}
                }

            } // poll events
        }

        unsafe {
            ImGui_ImplSDLRenderer_NewFrame();
            ImGui_ImplSDL2_NewFrame();

            igNewFrame();
        }

        // display demo window
        // ShowDemoWindow(true);
        let mut demo_window = true;
        unsafe { igShowDemoWindow(&mut demo_window); }

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

        // ImGui::Render();
        // ImGui_ImplSDLRenderer2_RenderDrawData(ImGui::GetDrawData());

        unsafe {
        igRender();
        ImGui_ImplSDLRenderer_RenderDrawData(igGetDrawData());
        }

        canvas.present();
    }

    // clean imgui
    unsafe {
        ImGui_ImplSDLRenderer_Shutdown();
        ImGui_ImplSDL2_Shutdown();
        igDestroyContext(ig_context);
    }

    Ok(())
}