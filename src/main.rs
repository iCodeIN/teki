mod app;
mod components;
mod consts;
mod pad;
mod sdl_renderer;
mod system_player;

use clap::{crate_description, crate_name, crate_version, App, Arg};

use app::EcsApp;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::Sdl;
use sdl_renderer::SdlRenderer;
use std::time::Duration;

use consts::*;

#[derive(Clone, Copy, PartialEq)]

pub enum VKey {
    Space,
    Escape,
    Left,
    Right,
    Up,
    Down,
}

fn main() -> Result<(), String> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("full").help("Use fullscreen").short("f").long("fullscreen"))
        .arg(
            Arg::with_name("scale")
                .help("Specify window scale (default: 3)")
                .short("s")
                .long("scale")
                .takes_value(true),
        )
        .get_matches();
    let fullscreen = matches.is_present("full");
    let scale = if let Some(scale) = matches.value_of("scale") {
        String::from(scale).parse().unwrap()
    } else {
        3
    };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG)?;

    let mut window_builder =
        video_subsystem.window(APP_NAME, WIDTH as u32 * scale, HEIGHT as u32 * scale);

    if fullscreen {
        window_builder.fullscreen();
    } else {
        window_builder.position_centered().resizable();
    }

    let window = window_builder.opengl().build().map_err(|e| e.to_string())?;

    if fullscreen {
        sdl_context.mouse().show_cursor(false);
    }

    let canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    let mut renderer = SdlRenderer::new(canvas, (WIDTH as u32, HEIGHT as u32));

    let mut app = EcsApp::new();

    let skip_count = 0;
    'running: loop {
        if !pump_events(&sdl_context, &mut app)? {
            break 'running;
        }
        let step = 1 + skip_count;

        for _ in 0..step {
            if !app.update() {
                break 'running;
            }
        }
        app.draw(&mut renderer);
        renderer.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
    Ok(())
}

fn pump_events(sdl_context: &Sdl, app: &mut EcsApp) -> Result<bool, String> {
    let mut event_pump = sdl_context.event_pump()?;
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                return Ok(false);
            }
            Event::KeyDown { keycode: Some(key), .. } => {
                app.on_key(key, true);
            }
            Event::KeyUp { keycode: Some(key), .. } => {
                app.on_key(key, false);
            }
            _ => {}
        }
    }
    Ok(true)
}