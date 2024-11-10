use ruscii::{drawing::Pencil, spatial::Vec2};

use crate::{map, player};

const SHADE_FULL: char  = '█';
const SHADE_DARK: char  = '▓';
const SHADE_MID: char   = '▒';
const SHADE_LIGHT: char = '░';
const SHADE_NONE: char  = ' ';


pub fn raycast(
    pencil: &mut Pencil,
    player: &mut player::Player,
    win_size: &Vec2,
    max_depth: f32,
    fov: f32,
) {
    for x in 0..win_size.x {
        // Project ray
        let ray_angle = (player.get_a() - (fov / 2.0)) + (x as f32 / win_size.x as f32) * fov;
    
        // Find distance to wall
        let step_size = 0.01;               // Increment size for ray. Decrease to increase resolution. Should probably make const
        let mut dist_to_wall = 0.0;
    
        let mut hit_wall = false;
    
        // TODO: Boundaries
        // INFO: Timestamp https://youtu.be/xW8skO7MFYw?t=1806
        // Do I even bother adding these or go straight to texture mapping...?
        // let mut hit_boundary = false;         // Edge of wall
    
        // Convert ray_angle to vector
        let eye_x = ray_angle.sin();
        let eye_y = ray_angle.cos();
    
        // Cast from player along ray angle testing for intersection with walls
        while (!hit_wall) && (dist_to_wall < max_depth) {
            dist_to_wall += step_size;
            
            let test_x = (player.get_x() + eye_x * dist_to_wall) as usize;
            let test_y = (player.get_y() + eye_y * dist_to_wall) as usize;
    
            // Is ray in bounds?
            if test_x > map::SIZE_X || test_y > map::SIZE_Y {
                hit_wall = true;
                dist_to_wall = max_depth;
            } else {
                // Ray is in bounds so check if it hits a wall
                if map::DATA[(test_y * map::SIZE_X + test_x) as usize] == '#' {
                    hit_wall = true;
                }
            }
        }
    
    
        let ceiling = ((win_size.y as f32 / 2.0) - (win_size.y as f32 / dist_to_wall)) as i32;
        let floor = win_size.y - ceiling;
    
        // Shading
        let shade = if dist_to_wall <= max_depth / 4.0    { SHADE_FULL  }
        else if dist_to_wall < max_depth / 3.0                  { SHADE_DARK  }
        else if dist_to_wall < max_depth / 2.0                  { SHADE_MID   }
        else if dist_to_wall < max_depth                        { SHADE_LIGHT }
        else                                                         { SHADE_NONE  };
    
    
        for y in 0..win_size.y {
            let pos = Vec2::xy(x, y);
    
            if y <= ceiling {
                // Ceiling
                pencil.draw_char(' ', pos);
            } else if y > ceiling && y <= floor {
                // Wall
                pencil.draw_char(shade, pos);
            } else {
                // Floor shading
                let b = 1.0 - ((y as f32 - win_size.y as f32 / 2.0) / (win_size.y as f32 / 2.0));
                let floor_shade = if b < 0.25 { '#' }
                else if b < 0.5                     { 'x' }
                else if b < 0.75                    { '=' }
                else if b < 0.9                     { '-' }
                else                                { ' ' };
    
                // Floor
                pencil.draw_char(floor_shade, pos);
            }
        }
    }
}