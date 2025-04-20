mod hangman;

use hangman::game::Game;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let logic = Rc::new(RefCell::new(None));
    let logic_clone = Rc::clone(&logic);

    ui.on_start_guessing({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let secret = ui.get_secret_word();
            *logic_clone.borrow_mut() = Some(Game::new(secret.as_str(), 3));
            println!("Start a new Game with the secret >>{secret}<< and >>3<< trys.");
            println!(
                "Has finished >>{}<<",
                logic_clone.take().unwrap().has_finished()
            );
        }
    });

    ui.run()?;

    Ok(())
}
