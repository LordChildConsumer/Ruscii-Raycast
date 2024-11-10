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

    ////////////////////////////////////////////////////////////////
    
    // Could probably just make these variables public but this is fine for now.

    pub fn get_x(&self) -> f32 { self.pos_x }
    pub fn get_y(&self) -> f32 { self.pos_y }
    pub fn get_a(&self) -> f32 { self.angle }
    
    ////////////////////////////////////////////////////////////////
    

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
        // Get desired position in integer/map space
        let new_x = (self.pos_x + vx) as usize;
        let new_y = (self.pos_y + vy) as usize;

        // Apply velocity if the desired cell is empty
        if map::DATA[new_y * map::SIZE_X + new_x] == ' ' {
            self.pos_x += vx;
            self.pos_y += vy;
        }
    }

}