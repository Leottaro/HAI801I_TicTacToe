use hai801i_tictactoe::tictactoe::{O, PERSONNE, TicTacToe, X};
use macroquad::color;
use macroquad::input::{MouseButton, is_mouse_button_pressed, mouse_position};
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
use macroquad::text::draw_text;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};

pub async fn display_tictactoe(game: &TicTacToe) {
    clear_background(color::WHITE);
    let cell_size = screen_width().min(screen_height()) / game.get_n() as f32;

    let (mouse_x, mouse_y) = mouse_position();
    let hovered_cell_x = (mouse_x / cell_size) as usize;
    let hovered_cell_y = (mouse_y / cell_size) as usize;

    for i in 0..game.get_n() {
        for j in 0..game.get_n() {
            let x = j as f32 * cell_size;
            let y = i as f32 * cell_size;

            if !game.is_over() && j == hovered_cell_x && i == hovered_cell_y {
                draw_rectangle(x, y, cell_size, cell_size, color::LIGHTGRAY);
            }
            draw_rectangle_lines(x, y, cell_size, cell_size, 2.0, color::BLACK);

            let case = game.get_case(j, i);
            if case != PERSONNE {
                let text = match case {
                    X => "X",
                    O => "O",
                    _ => "ERREUR",
                };

                let color = match case {
                    X => color::GRAY,
                    O => color::BLUE,
                    _ => color::RED,
                };

                draw_text(
                    text,
                    x + cell_size / 2.0 - 10.0,
                    y + cell_size / 2.0 + 10.0,
                    40.0,
                    color,
                );
            }
        }
    }

    next_frame().await;
}

#[macroquad::main("TicTacToe")]
async fn main() {
    let mut game = TicTacToe::new(4);
    let mut bot_plays_first = false;
    loop {
        let cell_size = screen_width().min(screen_height()) / game.get_n() as f32;

        let (mouse_x, mouse_y) = mouse_position();
        let hovered_cell_x = (mouse_x / cell_size) as usize;
        let hovered_cell_y = (mouse_y / cell_size) as usize;
        let is_mouse_pressed = is_mouse_button_pressed(MouseButton::Left);

        if is_mouse_pressed {
            if game.is_over() {
                game.reset();
                bot_plays_first = !bot_plays_first;
                if bot_plays_first {
                    game.bot_play();
                }
            } else {
                game.play(hovered_cell_x, hovered_cell_y);
                game.bot_play();
                if game.is_over() {
                    println!("Game Over! Winner: {:?}", game.get_winner());
                }
            }
        }

        display_tictactoe(&game).await;
    }
}
