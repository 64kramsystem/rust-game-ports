#![allow(clippy::type_complexity, clippy::comparison_chain)]

mod audio_entity;
mod ball;
mod bat;
mod controls;
mod game;
mod global_state;
mod graphic_entity;
mod impact;
mod state;

use std::env;
use std::path::PathBuf;

use ggez::{event, graphics::Rect, Context, GameError, GameResult};

use global_state::GlobalState;

const RESOURCES_DIR_NAME: &str = "resources";
const RESOURCE_SUBDIRS: [&str; 3] = ["images", "music", "sounds"];

const GAME_ID: &str = "Boing!";
const AUTHOR: &str = "Saverio Miroddi";

const WINDOW_TITLE: &str = GAME_ID;
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 480.;

const HALF_WIDTH: f32 = WINDOW_WIDTH / 2.;
const HALF_HEIGHT: f32 = WINDOW_HEIGHT / 2.;

fn get_resource_dirs() -> Vec<PathBuf> {
    let resources_root_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push(RESOURCES_DIR_NAME);
        path
    } else {
        PathBuf::from(RESOURCES_DIR_NAME)
    };

    RESOURCE_SUBDIRS
        .iter()
        .map(|subdir| resources_root_dir.join(subdir))
        .collect()
}

fn configure_window_and_viewport(context: &mut Context) -> Result<(), GameError> {
    // Attempted workaround (see below)
    let (screen_width, screen_height) = (800., 600.);

    ggez::graphics::set_drawable_size(context, screen_width, screen_height)?;

    // Broken; returns 800x600
    // let (screen_width, screen_height) = ggez::graphics::drawable_size(context);

    let screen_ratio = screen_width / screen_height;
    let viewport_ratio = WINDOW_WIDTH / WINDOW_HEIGHT; // game area ratio

    let (viewport_width, viewport_height) = if screen_ratio > viewport_ratio {
        // Screen is wider; keep the height, but stretch the width.
        (screen_height * viewport_ratio, screen_height)
    } else {
        // screen is taller; keep the width, but stretch the height.
        // we divide instead of multiplying because the ratio itself is calculated on the inverse
        // relationship between W/H.
        (screen_width, screen_width / viewport_ratio)
    };

    let tot_border_width = screen_width - viewport_width;
    let tot_border_height = screen_height - viewport_height;

    /*
    Extremely confusing.

    # Drawable size set to 1920x1080

    SS:(1920.0, 1080.0)
    VR:Rect { x: -60.0, y: -0.0, w: 1800.0, h: 1080.0 }

    - the rendering of the viewport is around 1/4th of what it should be
    - there is display corruption (viewport displayed multiple times, in different sizes and
      locations)

    Other viewport coordinates (e.g. Rect::new(0.,0.,viewport_width + tot_border_width,viewport_height + tot_border_height))
    still cause corruption.

    # Drawable size set to 800x600

    With this Rect, there is no corruption, however, there are vertical bars to the side, instead
    of the top:

        VR:Rect { x: 0.0, y: 0.0, w: 800.0, h: 480.0 }
    */

    let viewport_rect = Rect::new(
        -tot_border_width / 2.,
        -tot_border_height / 2.,
        viewport_width,
        viewport_height,
    );

    println!("SS:{:?}", (screen_width, screen_height));
    println!("VR:{:?}", viewport_rect);

    ggez::graphics::set_screen_coordinates(context, viewport_rect)
}

fn main() -> GameResult {
    let resource_dirs = get_resource_dirs();

    let mut context_builder = ggez::ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(ggez::conf::WindowSetup::default().title(WINDOW_TITLE))
        .window_mode(
            ggez::conf::WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::True),
        );

    for dir in resource_dirs {
        context_builder = context_builder.add_resource_path(dir);
    }

    let (mut context, event_loop) = context_builder.build()?;

    configure_window_and_viewport(&mut context)?;

    let mut state = GlobalState::new(&mut context);

    state.play_music(&mut context)?;

    event::run(context, event_loop, state)
}
