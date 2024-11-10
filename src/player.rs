use ruscii::keyboard::Key;

use crate::map;

pub struct Player {
    pos_x: f32,
    pos_y: f32,
    angle: f32,

    move_speed: f32,
    turn_speed: f32,
}

impl Player {
    // Constructor
    pub fn new(
        pos_x: f32,
        pos_y: f32,
        angle: f32,
        move_speed: f32,
        turn_speed: f32,
    ) -> Self {
        Self {
            pos_x, pos_y, angle, move_speed, turn_speed,
        }
    }

    // Handle Keyboard Input
    pub fn process_keys_down(&mut self, keys_down: Vec<Key>) {
        for key in keys_down {
            match key {
                // Turning
                Key::A => self.angle -= self.turn_speed,
                Key::D => self.angle += self.turn_speed,

                // Move Forward
                Key::W => {
                    let vx = self.angle.sin() * self.move_speed;
                    let vy = self.angle.cos() * self.move_speed;
                    self.try_movement(vx, vy);
                },

                // Move Backward
                Key::S => {
                    let vx = self.angle.sin() * -self.move_speed;
                    let vy = self.angle.cos() * -self.move_speed;
                    self.try_movement(vx, vy);
                },

                _ => (),
            }
        }
    }

    // Check collisions before moving
    fn try_movement(&mut self, vx: f32, vy: f32) {
        // TODO: Implement 'is_movement_valid'
        // - Needs to convert the desired position to integer coordinates
        // - Needs a reference to the map to check if said coordinates are empty
        // -
        // - Following snippet moves the player backwards if they're in a wall after having moved forward.
        // if map[(player_y as i32 * MAP_X + player_x as i32) as usize] == '#' {
        //     player_x -= player_a.sin() * PLAYER_MOVE_SPEED;
        //     player_y  -= player_a.cos() * PLAYER_MOVE_SPEED;
        // }

        // Get desired position in integer/map space
        let new_x = (self.pos_x + vx) as usize;
        let new_y = (self.pos_y + vy) as usize;

        // Apply velocity if the desired cell is empty
        if map::DATA[new_y * map::SIZE_X + new_x] == ' ' {
            self.pos_x += vx;
            self.pos_y += vy;
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    
    pub fn get_x(&self) -> f32 { self.pos_x }
    pub fn get_y(&self) -> f32 { self.pos_y }
    pub fn get_a(&self) -> f32 { self.angle }
    
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

}

// for key_down in app_state.keyboard().get_keys_down() {
//     match key_down {
//         Key::A => player_a -= PLAYER_TURN_SPEED,
//         Key::D => player_a += PLAYER_TURN_SPEED,

//         // Move Forward
//         Key::W => {
//             player_x += player_a.sin() * PLAYER_MOVE_SPEED;
//             player_y  += player_a.cos() * PLAYER_MOVE_SPEED;

//             // TODO: Test movement before applying, rather than trying to undo it.
//             if map[(player_y as i32 * MAP_X + player_x as i32) as usize] == '#' {
//                 player_x -= player_a.sin() * PLAYER_MOVE_SPEED;
//                 player_y  -= player_a.cos() * PLAYER_MOVE_SPEED;
//             }
//         },

//         // Move Backward
//         Key::S => {
//             player_x -= player_a.sin() * PLAYER_MOVE_SPEED;
//             player_y  -= player_a.cos() * PLAYER_MOVE_SPEED;

//             // TODO: Test movement before applying, rather than trying to undo it.
//             if map[(player_y as i32 * MAP_X + player_x as i32) as usize] == '#' {
//                 player_x += player_a.sin() * PLAYER_MOVE_SPEED;
//                 player_y  += player_a.cos() * PLAYER_MOVE_SPEED;
//             }
//         }

//         _ => (),
//     }
// }