use std::f32::consts::PI;

use ruscii::app::{App, State};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;
use ruscii::drawing::Pencil;
use ruscii::keyboard::{KeyEvent, Key};

fn main() {
    let mut app = App::default();

    ////////////////////////////////////////////////////////////////
    ////  -----------------  Initialization  -----------------  ////
    ////////////////////////////////////////////////////////////////

    const MAX_DEPTH: f32 = 16.0;
    const FOV: f32 = PI / 4.0;

    const MAP_X: i32 = 8;
    const MAP_Y: i32 = 8;
    let mut map  = [
        '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', 
        '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', 
        
    ];

    let player_x: f32 = 8.0;
    let player_y: f32 = 8.0;
    let player_a: f32 = 0.0;


    
    app.run(|app_state: &mut State, window: &mut Window| {

        let win_size = window.size();

        ////////////////////////////////////////////////////////////////
        ////  -----------------  Keyboard Input  -----------------  ////
        ////////////////////////////////////////////////////////////////
        
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                _ => (),
            }
        }




        ////////////////////////////////////////////////////////////////
        ////  ---------------------  Update  ---------------------  ////
        ////////////////////////////////////////////////////////////////




        ////////////////////////////////////////////////////////////////
        ////  ----------------------  Draw  ----------------------  ////
        ////////////////////////////////////////////////////////////////
        
        let mut pencil = Pencil::new(window.canvas_mut());


        for x in 0..win_size.x {
            // Get the projected ray angle
            let ray_angle = (player_a - (FOV / 2.0)) + (x as f32 / win_size.x as f32) * FOV;

            // Find distance to wall
            let step_size = 0.1;         // Increment size for ray. Decrease to increase resolution
            let mut dist_to_wall = 0.0;

            let mut hit_wall = false;       // Ray hits wall
            let boundary = false;       // Ray hits boundary between walls

            // Convert ray_angle to vector
            let eye_x = ray_angle.sin();
            let eye_y = ray_angle.cos();

            // Cast from player along ray angle testing for intersection with walls
            while (!hit_wall) && (dist_to_wall < MAX_DEPTH) {
                dist_to_wall += step_size;
                
                let test_x = (player_x + eye_x * dist_to_wall) as i32;
                let test_y = (player_y + eye_y * dist_to_wall) as i32;

                // Is ray in bounds?
                if test_x < 0 || test_x >= MAP_X || test_y < 0 || test_y >= MAP_Y {
                    hit_wall = true;
                    dist_to_wall = MAX_DEPTH;
                } else {
                    // Ray is in bounds so check if it hits a wall
                    if (map[(test_x * MAP_X + test_y) as usize] == '#') {
                        hit_wall = true;
                    }
                }
            }


            let ceiling = ((win_size.y as f32 / 2.0) - (win_size.y as f32 / dist_to_wall)) as i32;
            let floor = win_size.y - ceiling;

            for y in 0..win_size.y {
                let pos = Vec2::xy(x, y);

                if y <= ceiling {
                    // Ceiling
                    pencil.draw_char(' ', pos);
                } else if y > ceiling && y <= floor {
                    // Wall
                    pencil.draw_char('#', pos);
                } else {
                    // Floor
                    pencil.draw_char('.', pos);
                }
            }
        }

    });
}