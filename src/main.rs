mod consts;
mod ecs;
mod renderer;

use clap::{crate_description, crate_name, crate_version, App, Arg};

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use std::time::Duration;

use consts::*;
use ecs::components::*;
use ecs::systems::*;

pub enum MovementCommand {
    Stop,
    Move(Direction),
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

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(physics::Physics, "Physics", &["Keyboard"])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::SystemData::setup(&mut world);

    let movement_command: Option<MovementCommand> = None;
    world.insert(movement_command);

    let textures = [texture_creator.load_texture("assets/neko.png")?];

    let spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 45, 45);
    let (frame_width, frame_height) = player_top_left_frame.size();

    let sprite = Sprite {
        spritesheet,
        region: Rect::new(
            player_top_left_frame.x(),
            player_top_left_frame.y(),
            frame_width,
            frame_height,
        ),
    };

    world
        .create_entity()
        .with(KeyboardControlled)
        .with(Position(Point::new(0, 0)))
        .with(Velocity { speed: 0, direction: Direction::Right })
        .with(sprite)
        .build();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        let mut movement_command = None;

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                }
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                }
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                }
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                }
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. }
                | Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. }
                | Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. }
                | Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop);
                }
                _ => {}
            }
        }

        *world.write_resource() = movement_command;

        // Update
        i = (i + 1) % 255;
        dispatcher.dispatch(&world);
        world.maintain();

        // Render
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), &textures, world.system_data())?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    Ok(())
}
