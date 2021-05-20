#![allow(dead_code)]
mod consts;
use consts::*;

mod lib;
use lib::*;

use macroquad::prelude::*;

#[macroquad::main("Milkshake")]
async fn main() {
    // Make a Milkshake
    let mut milkshake = Milkshake::new(
        INITIAL_X_POS,
        INITIAL_Y_POS,
        X_SPEED,
        Y_SPEED,
        LASER_SHOOT,
        LASER_DISTANCE,
        LASER_WIDTH,
        LASER_SPEED,
    );

    // Milkshake picture
    let picture = Texture2D::from_file_with_format(
        include_bytes!("../assets/milkshake.png"),
        Some(ImageFormat::Png),
    );

    // Background image
    let background = Texture2D::from_file_with_format(
        include_bytes!("../assets/background.png"),
        Some(ImageFormat::Png),
    );

    // Laser image
    let laser_beam = Texture2D::from_file_with_format(
        include_bytes!("../assets/laser_beam.png"),
        Some(ImageFormat::Png),
    );

    // Define the font
    let font_bytes = include_bytes!("../assets/AvenirNextLTPro-Regular.otf");

    // Game loop
    loop {
        // Set the screen color
        clear_background(WHITE);

        // Draw the background
        draw_texture_ex(
            background,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw the Milkshake
        draw_texture(
            picture,
            milkshake.pos.x,
            screen_height() - milkshake.pos.y,
            WHITE,
        );

        // Possible actions
        milkshake.jump(INITIAL_Y_POS, JUMP_HEIGHT, GRAVITY);
        milkshake.shoot(LASER_DISTANCE, laser_beam, LASER_SPEED);

        // Keypresses
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::L) {
            milkshake.pos.x += milkshake.speed.x;
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::H) {
            milkshake.pos.x -= milkshake.speed.x;
        } else if is_key_pressed(KeyCode::Up) || is_key_down(KeyCode::K) {
            milkshake.jump_state.up = true;
        } else if is_key_down(KeyCode::Space) {
            milkshake.laser.shoot = true;
        } else if is_key_down(KeyCode::M) {
            show_help(font_bytes, HELP_OFFSET);
        } else if is_key_down(KeyCode::Q) {
            break;
        }

        // Get to the next frame
        next_frame().await;
    }
}
