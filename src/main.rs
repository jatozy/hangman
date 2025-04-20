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
    let logic_clone = Rc::clone(&logic);

    ui.on_player1_finished({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let secret = ui.get_secret_word();
            *logic_clone.borrow_mut() = Some(Game::new(secret.as_str(), 3));
            ui.set_solved_word(secret);
        }
    });

    ui.on_player2_guess({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let guess = ui.get_guessed_letter();
            println!("Guessed letter >>{guess}<<")
        }
    });

    ui.run()?;

    Ok(())
}
