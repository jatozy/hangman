mod hangman;

use hangman::game::Game;
use slint::SharedString;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let logic = Rc::new(RefCell::new(None));

    let player1_logic = Rc::clone(&logic);
    ui.on_player1_finished({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let secret = ui.get_secret_word();
            *player1_logic.borrow_mut() = Some(Game::new(secret.as_str(), 3));

            if let Some(ref mut game) = *player1_logic.borrow_mut() {
                ui.set_solved_word(SharedString::from(game.get_solution()));
            }
        }
    });

    let player2_logic = Rc::clone(&logic);
    ui.on_player2_guess({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let guess = ui.get_guessed_letter();

            if let Some(ref mut game) = *player2_logic.borrow_mut() {
                game.guess_a_letter(guess.chars().next().unwrap());
                ui.set_user_errors(game.get_user_errors());
                ui.set_solved_word(SharedString::from(game.get_solution()));
                ui.set_game_finished(game.has_finished());
                ui.set_game_lost(game.is_game_over());
            }
        }
    });

    ui.run()?;

    Ok(())
}
