use macroquad::prelude::*;

/// Milkshake laser - AoE (Area of Effect)
#[derive(Debug)]
pub struct LaserBeam {
    // Shooting
    pub shoot: bool,
    // Distance that the laser travels before evaporating
    pub distance: f32,
    // Width of the laser beam
    pub width: f32,
    // Speed of the laser beam
    pub speed: f32,
}

/// Milkshake position on the 2D coordinate plane
#[derive(Debug)]
pub struct Position {
    // Position along the x axis
    pub x: f32,
    // Position along the y axis
    pub y: f32,
}

/// Speed for Milkshake
/// Includes both speed along the x-axis and along the y-axis
#[derive(Debug)]
pub struct Speed {
    // Speed along x axis
    pub x: f32,
    // Speed along y axis
    pub y: f32,
}

/// Jump state for Milkshake (up or down)
#[derive(Debug)]
pub struct JumpState {
    // Speed along x axis
    pub up: bool,
    // Speed along y axis
    pub down: bool,
}

/// Milkshake
#[derive(Debug)]
pub struct Milkshake {
    // Position
    pub pos: Position,
    // Speed
    pub speed: Speed,
    // Jump state
    pub jump_state: JumpState,
    // Jump state
    pub laser: LaserBeam,
}

impl Default for JumpState {
    fn default() -> Self {
        JumpState {
            up: false,
            down: false,
        }
    }
}

impl Milkshake {
    /// Make new Milkshake
    pub fn new(
        x_pos: f32,
        y_pos: f32,
        x_speed: f32,
        y_speed: f32,
        laser_shoot: bool,
        laser_distance: f32,
        laser_width: f32,
        laser_speed: f32,
    ) -> Self {
        Milkshake {
            pos: Position { x: x_pos, y: y_pos },
            speed: Speed {
                x: x_speed,
                y: y_speed,
            },
            jump_state: JumpState::default(),
            laser: LaserBeam {
                shoot: laser_shoot,
                distance: laser_distance,
                width: laser_width,
                speed: laser_speed,
            },
        }
    }

    /// Jump implementation for Milkshake
    pub fn jump(&mut self, initial_y: f32, jump_height: f32, gravity: f32) {
        if self.jump_state.up {
            if self.pos.y > jump_height {
                self.jump_state.up = false;
                self.jump_state.down = true;
            } else {
                self.pos.y += self.speed.y * gravity;
            }
        } else if self.jump_state.down {
            if self.pos.y < initial_y {
                self.jump_state.up = false;
                self.jump_state.down = false;
            } else {
                self.pos.y -= self.speed.y * gravity;
            }
        }
    }

    /// Shoot implementation for Milkshake
    pub fn shoot(
        &mut self,
        initial_distance: f32,
        laser_beam: Texture2D,
        laser_speed: f32,
    ) {
        if self.laser.shoot && self.laser.distance < screen_width() {
            draw_texture(
                laser_beam,
                self.pos.x + self.laser.distance,
                screen_height() - self.pos.y + 65.,
                WHITE,
            );
            self.laser.distance += laser_speed;
        } else {
            self.laser.shoot = false;
            self.laser.distance = initial_distance;
        }
    }
}

// Show game help
pub fn show_help(bytes: &[u8], offset: f32) {
    // Create the font object
    let font = load_ttf_font_from_bytes(bytes).unwrap();

    // Define the parameters
    let params = TextParams {
        font,
        font_size: 24u16,
        font_scale: 1.,
        font_scale_aspect: 1.,
        color: BLUE,
    };

    // Put the rectangle for better visibility
    draw_rectangle(0., 0., 15. * offset, 5.5 * offset, LIGHTGRAY);

    // Put the help
    draw_text_ex("KEYBINDINGS", 25., offset, params);
    draw_text_ex("<L> / <R-Arrow> - Move Right", 25., 2. * offset, params);
    draw_text_ex("<H> / <L-Arrow> - Move Left", 25., 3. * offset, params);
    draw_text_ex("<K> / <U-Arrow> - Jump", 25., 4. * offset, params);
    draw_text_ex("<Space> - Shoot Laser Beam", 25., 5. * offset, params);
}
