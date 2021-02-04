//! A demonstration using winit to provide events and glium for drawing the Ui.
#![allow(dead_code)]
#[macro_use] 
extern crate conrod_core;
extern crate conrod_glium; 
extern crate conrod_winit;
extern crate find_folder;
extern crate glium;
extern crate image;

#[allow(unused_imports)]
use std::{thread, time};

mod support;
mod gui;
mod sid_player;
mod oscillator;
mod track;
mod instrument;

use gui::{WIN_W, WIN_H};
use conrod_glium::Renderer;
use glium::Surface;
use sid_player::SidPlayer;


fn main() {

    /*let thread_id = thread_native_id();
    assert!(set_thread_priority(thread_id,
                                ThreadPriority::Max,
                                ThreadSchedulePolicy::Normal(NormalThreadSchedulePolicy::Normal)).is_ok());
    */
    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("sidbang64")
        .with_dimensions((WIN_W, WIN_H).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = support::GliumDisplayWinitWrapper(display);

    // Construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([WIN_W as f64, WIN_H as f64]).theme(gui::theme()).build();

    // The `widget::Id` of each widget instantiated in `gui::gui`.
    let mut ids = gui::Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let image_map : conrod_core::image::Map<glium::texture::Texture2d> = conrod_core::image::Map::new();
    let mut app = gui::DemoApp::new();
    

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    //
    // Internally, the `Renderer` maintains:
    // - a `backend::glium::GlyphCache` for caching text onto a `glium::texture::Texture2d`.
    // - a `glium::Program` to use as the shader program when drawing to the `glium::Surface`.
    // - a `Vec` for collecting `backend::glium::Vertex`s generated when translating the
    // `conrod_core::render::Primitive`s.
    // - a `Vec` of commands that describe how to draw the vertices.
    let mut renderer = Renderer::new(&display.0).unwrap();

    let mut player = SidPlayer::new();
    
    player.playback();

    // Start the loop:
    //
    // - Poll the window for available events.
    // - Update the widgets via the `gui::gui` fn.
    // - Render the current state of the `Ui`.
    // - Repeat.
    let mut event_loop = support::EventLoop::new();

    'main: loop {
        player.check_update();


        // Handle all events.
        for event in event_loop.next(&mut events_loop) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = support::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,

                    glium::glutin::WindowEvent::KeyboardInput { input, .. } => {
                        if let glium::glutin::ElementState::Pressed = input.state {
                            app.input_keycode = input.virtual_keycode;
                            app.input_delay = 1;

                            match input.virtual_keycode {
                                Some(glium::glutin::VirtualKeyCode::LShift) => {
                                    app.shift_key = true;
                                },

                                _ => {}
                            }
                                                   }
                        else if let glium::glutin::ElementState::Released = input.state {
                            match input.virtual_keycode {
                                Some(glium::glutin::VirtualKeyCode::LShift) => {
                                    app.shift_key = false;
                                },

                                _ => {}
                            }
                        }
                    },
                    _ => {},
                },
                _ => (),
            }
        }

        // Instantiate a GUI demonstrating every widget type provided by conrod.
        gui::gui(&mut ui.set_widgets(), &mut ids, &mut app, &mut player);

        // Draw the `Ui`.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut target = display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display.0, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }

        //let ten_millis = time::Duration::from_millis(1);
        //let _now = time::Instant::now();
       // thread::sleep(ten_millis);        
    }
}
