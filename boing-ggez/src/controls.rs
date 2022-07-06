use ggez::{
    event::{Axis, Button, KeyCode},
    input::{
        gamepad::{self, Gamepad},
        keyboard,
    },
    Context,
};

use crate::{ball::Ball, bat::Bat, HALF_HEIGHT, HALF_WIDTH};

const PLAYER_SPEED: f32 = 6.;
const MAX_AI_SPEED: f32 = 6.;

pub enum PadNum {
    Zero,
    One,
}

pub const ANALOG_STICK_TOLERANCE: f32 = 0.1;

// The pad functions are for convenience.
//
pub fn pad_input(context: &Context, pad_number: PadNum, test: fn(&Gamepad) -> bool) -> bool {
    let mut pad_iter = gamepad::gamepads(context);

    let pad = match pad_number {
        PadNum::Zero => pad_iter.next(),
        PadNum::One => pad_iter.skip(1).next(),
    };

    pad.is_some_and(|(_id, pad)| test(pad))
}

pub fn is_pad_up_pressed(context: &Context, pad_number: PadNum) -> bool {
    pad_input(context, pad_number, |pad| {
        pad.value(Axis::LeftStickY) > ANALOG_STICK_TOLERANCE
    })
}

pub fn is_pad_down_pressed(context: &Context, pad_number: PadNum) -> bool {
    pad_input(context, pad_number, |pad| {
        pad.value(Axis::LeftStickY) < -ANALOG_STICK_TOLERANCE
    })
}

pub fn is_fire_button_pressed(context: &Context, pad_number: PadNum) -> bool {
    // Oddly, on two pads tested, X was mapped to a different button, so we catch both.
    //
    pad_input(context, pad_number, |pad| {
        pad.is_pressed(Button::West)
            || pad.is_pressed(Button::North)
            || pad.is_pressed(Button::South)
    })
}

pub fn is_quit_button_pressed(context: &Context, pad_number: PadNum) -> bool {
    pad_input(context, pad_number, |pad| pad.is_pressed(Button::East))
}

// Functional approach to controls; in a more type-oriented design, these are represented by a trait,
// but we currently keep close to the original design.

pub fn p1_controls(context: &Context, _ball: &Ball, _ai_offset: f32, _bat: &Bat) -> f32 {
    // Note that some devices that are not actually analog (eg. a given arcade stick) may be reported
    // as analog.
    let pad_0_up_pressed = pad_input(context, PadNum::Zero, |pad| {
        pad.is_pressed(Button::DPadUp) || pad.value(Axis::LeftStickY) > ANALOG_STICK_TOLERANCE
    });
    let pad_0_down_pressed = pad_input(context, PadNum::Zero, |pad| {
        pad.is_pressed(Button::DPadDown) || pad.value(Axis::LeftStickY) < -ANALOG_STICK_TOLERANCE
    });

    let keys_pressed = keyboard::pressed_keys(context);

    let move_down = pad_0_down_pressed
        || keys_pressed.contains(&KeyCode::Z)
        || keys_pressed.contains(&KeyCode::Down);
    let move_up = pad_0_up_pressed
        || keys_pressed.contains(&KeyCode::A)
        || keys_pressed.contains(&KeyCode::Up);

    if move_down {
        PLAYER_SPEED
    } else if move_up {
        -PLAYER_SPEED
    } else {
        0.
    }
}

pub fn p2_controls(context: &Context, _ball: &Ball, _ai_offset: f32, _bat: &Bat) -> f32 {
    let pad_1_up_pressed = pad_input(context, PadNum::One, |pad| {
        pad.is_pressed(Button::DPadUp) || pad.value(Axis::LeftStickY) > ANALOG_STICK_TOLERANCE
    });
    let pad_1_down_pressed = pad_input(context, PadNum::One, |pad| {
        pad.is_pressed(Button::DPadDown) || pad.value(Axis::LeftStickY) < -ANALOG_STICK_TOLERANCE
    });

    let keys_pressed = keyboard::pressed_keys(context);

    let move_down = pad_1_down_pressed || keys_pressed.contains(&KeyCode::M);
    let move_up = pad_1_up_pressed || keys_pressed.contains(&KeyCode::K);

    if move_down {
        PLAYER_SPEED
    } else if move_up {
        -PLAYER_SPEED
    } else {
        0.
    }
}

/// Returns a number indicating how the computer player will move - e.g. 4 means it will move 4 pixels
/// down the screen.
pub fn ai(_context: &Context, ball: &Ball, ai_offset: f32, bat: &Bat) -> f32 {
    // To decide where we want to go, we first check to see how far we are from the ball.
    let x_distance: f32 = (ball.x - bat.x).abs();

    // If the ball is far away, we move towards the centre of the screen (HALF_HEIGHT), on the basis
    // that we don't yet know whether the ball will be in the top or bottom half of the screen when
    // it reaches our position on the X axis. By waiting at a central position, we're as ready as
    // it's possible to be for all eventualities.
    let target_y_1 = HALF_HEIGHT;

    // If the ball is close, we want to move towards its position on the Y axis. We also apply a small
    // offset which is randomly generated each time the ball bounces. This is to make the computer player
    // slightly less robotic - a human player wouldn't be able to hit the ball right in the centre of
    // the bat each time.
    let target_y_2 = ball.y + ai_offset;

    // The final step is to work out the actual Y position we want to move towards. We use what's called
    // a weighted average - taking the average of the two target Y positions we've previously calculated,
    // but shifting the balance towards one or the other depending on how far away the ball is. If the
    // ball is more than 400 pixels (half the screen width) away on the X axis, our target will be half
    // the screen height (target_y_1). If the ball is at the same position as us on the X axis, our
    // target will be target_y_2. If it's 200 pixels away, we'll aim for halfway between target_y_1
    // and target_y_2. This reflects the idea that as the ball gets closer, we have a better idea of
    // where it's going to end up.
    let weight1 = 1.0_f32.min(x_distance / HALF_WIDTH);
    let weight2 = 1. - weight1;

    let target_y = (weight1 * target_y_1) + (weight2 * target_y_2);

    // Subtract target_y from our current Y position, then make sure we can't move any further than
    // MAX_AI_SPEED each frame
    (target_y - bat.y).clamp(-MAX_AI_SPEED, MAX_AI_SPEED)
}
