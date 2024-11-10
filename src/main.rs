/*
    INFO: This program is basically a 1:1 translation of OLC's tutorial on doing this in C++
        - https://youtu.be/xW8skO7MFYw
*/

use std::f32::consts::PI;

use player::Player;
use ruscii::app::{App, State};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{KeyEvent, Key};

mod player;
mod renderer;
mod map;


const MAX_DEPTH: f32 = 16.0;
const FOV: f32 = PI / 3.0;




fn main() {
    let mut app = App::default();

    ////////////////////////////////////////////////////////////////
    ////  -----------------  Initialization  -----------------  ////
    ////////////////////////////////////////////////////////////////

    let mut player = Player::new(4.0, 4.0, 0.0, 0.1, 0.1);

    let mut show_stats = false;

    
    app.run(|app_state: &mut State, window: &mut Window| {

        let win_size = window.size();

        ////////////////////////////////////////////////////////////////
        ////  ---------------------  Update  ---------------------  ////
        ////////////////////////////////////////////////////////////////

        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::F1) => show_stats = !show_stats,
                _ => (),
            }
        }

        player.process_keys_down(app_state.keyboard().get_keys_down());




        ////////////////////////////////////////////////////////////////
        ////  ----------------------  Draw  ----------------------  ////
        ////////////////////////////////////////////////////////////////
        
        let mut pencil = Pencil::new(window.canvas_mut());
        renderer::raycast(&mut pencil, &mut player, &win_size, MAX_DEPTH, FOV);

        // Show Stats
        if show_stats {
            let stats_rect_set = RectCharset::double_lines();
            let rect_pos = Vec2::xy(0, 0);
            let rect_size = Vec2::xy(19, 6); // 19x5

            pencil
                .draw_filled_rect(' ', rect_pos, rect_size)
                .draw_rect(&stats_rect_set, rect_pos, rect_size)
                .draw_text(&format!("Player X: {:.2}", player.get_x()), Vec2::xy(1, 1))
                .draw_text(&format!("Player y: {:.2}", player.get_y()), Vec2::xy(1, 2))
                .draw_text(&format!("Player A: {:.2}", player.get_a().to_degrees()), Vec2::xy(1, 3))
                .draw_text(&format!("Win Size: {:.2}x{:.2}", win_size.x, win_size.y), Vec2::xy(1, 4));
        }
        
    });
}