#![feature(is_some_with)]

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
    let (drawable_width, drawable_height) = ggez::graphics::drawable_size(context);

    let (new_drawable_width, new_drawable_height) =
        if drawable_width / drawable_height > WINDOW_WIDTH / WINDOW_HEIGHT {
            (
                WINDOW_HEIGHT * (drawable_width / drawable_height),
                WINDOW_HEIGHT,
            )
        } else {
            (
                WINDOW_WIDTH,
                WINDOW_WIDTH * (drawable_height / drawable_width),
            )
        };

    ggez::graphics::set_drawable_size(context, new_drawable_width, new_drawable_height)?;

    let tot_border_width = new_drawable_width - WINDOW_WIDTH;
    let tot_border_height = new_drawable_height - WINDOW_HEIGHT;

    ggez::graphics::set_screen_coordinates(
        context,
        Rect::new(
            -tot_border_width / 2.,
            -tot_border_height / 2.,
            new_drawable_width,
            new_drawable_height,
        ),
    )
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
