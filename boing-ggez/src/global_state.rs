use ggez::audio::{self, SoundSource};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{self, Image};
use ggez::input::keyboard::is_key_pressed;
use ggez::{timer, Context, GameResult};

use crate::ball::Ball;
use crate::bat::Bat;
use crate::controls::{
    is_fire_button_pressed, is_pad_down_pressed, is_pad_up_pressed, is_quit_button_pressed,
    p1_controls, p2_controls, PadNum,
};
use crate::game::Game;
use crate::state::State;

/// Global state, not to be confused with the game state (which is a part of it).
pub struct GlobalState {
    state: State,
    game: Game,
    num_players: usize,
    space_down: bool,
    fire_down: bool,

    menu_images: Vec<Image>,
    game_over_image: Image,

    music: audio::Source,

    down_sound: audio::Source,
    up_sound: audio::Source,
}

impl GlobalState {
    pub fn new(context: &mut Context) -> Self {
        let menu_images = (0..2)
            .map(|i| {
                let menu_image_filename = format!("/menu{}.png", i);
                Image::new(context, menu_image_filename).unwrap()
            })
            .collect();

        let game_over_image = Image::new(context, "/over.png").unwrap();

        // For simplicity, we always assume that it's possible to play the music.
        let music = audio::Source::new(context, "/theme.ogg").unwrap();

        // In the [docs](https://pygame-zero.readthedocs.io/en/stable/builtins.html), it's not explicit
        // what happens if there is an error, so we just implement the intuitive logic.
        let down_sound = audio::Source::new(context, "/down.ogg").unwrap();
        let up_sound = audio::Source::new(context, "/up.ogg").unwrap();

        Self {
            state: State::Menu,
            game: Game::new(context, (None, None)),
            num_players: 1,
            space_down: false,
            fire_down: false,
            menu_images,
            game_over_image,
            music,
            down_sound,
            up_sound,
        }
    }

    pub fn play_music(&mut self, context: &mut Context) -> GameResult {
        self.music.set_volume(0.3);
        self.music.play(context)
    }
}

impl EventHandler for GlobalState {
    /// Pygame Zero and ggez call the update and draw functions each frame
    fn update(&mut self, context: &mut Context) -> GameResult {
        // The project uses the tap concept, but in this game, it's not really needed.

        // Work out whether the space key has just been pressed - i.e. in the previous frame it wasn't
        // down, and in this frame it is.
        let space_pressed = is_key_pressed(context, KeyCode::Space) && !self.space_down;
        self.space_down = is_key_pressed(context, KeyCode::Space);

        // We mimick the source project structure for the pad.
        let fire_pressed = is_fire_button_pressed(context, PadNum::Zero) && !self.fire_down;
        self.fire_down = is_fire_button_pressed(context, PadNum::Zero);

        if is_quit_button_pressed(context, PadNum::Zero) {
            ggez::event::quit(context);
        }

        match self.state {
            State::Menu => {
                if space_pressed || fire_pressed {
                    // Switch to play state, and create a new Game object, passing it the controls function for
                    // player 1, and if we're in 2 player mode, the controls function for player 2 (otherwise the
                    // 'None' value indicating this player should be computer-controlled)
                    self.state = State::Play;

                    // Address confusing error "expected fn pointer, found fn item"; seems related to git.io/JGz2L.
                    let p1_controls: fn(&Context, &Ball, f32, &Bat) -> f32 = p1_controls;
                    let p2_controls: fn(&Context, &Ball, f32, &Bat) -> f32 = p2_controls;

                    let mut controls = (Some(p1_controls), None);
                    if self.num_players == 2 {
                        controls.1 = Some(p2_controls);
                    }

                    self.game = Game::new(context, controls);
                } else {
                    let input_up = is_key_pressed(context, KeyCode::Up)
                        || is_pad_up_pressed(context, PadNum::Zero);
                    let input_down = is_key_pressed(context, KeyCode::Down)
                        || is_pad_down_pressed(context, PadNum::Zero);

                    if self.num_players == 2 && input_up {
                        self.up_sound.play(context)?;
                        self.num_players = 1;
                    } else if self.num_players == 1 && input_down {
                        self.down_sound.play(context)?;
                        self.num_players = 2;
                    }

                    // Update the 'attract mode' game in the background (two AIs playing each other)
                    self.game.update(context, self.state)?
                }
            }
            State::Play => {
                // Has anyone won?
                if self.game.bats[0].score.max(self.game.bats[1].score) > 9 {
                    self.state = State::GameOver;
                } else {
                    self.game.update(context, self.state)?
                }
            }
            State::GameOver => {
                if space_pressed {
                    // Reset to menu state
                    self.state = State::Menu;
                    self.num_players = 1;

                    // Create a new Game object, without any players
                    self.game = Game::new(context, (None, None));
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        self.game.draw(context)?;

        match self.state {
            State::Menu => {
                graphics::draw(
                    context,
                    &self.menu_images[self.num_players - 1],
                    graphics::DrawParam::new(),
                )?;
            }
            State::GameOver => {
                graphics::draw(context, &self.game_over_image, graphics::DrawParam::new())?;
            }
            State::Play => {}
        }

        graphics::present(context)?;

        timer::yield_now();

        Ok(())
    }
}
