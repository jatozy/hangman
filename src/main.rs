mod hangman;

use hangman::hangman::Game;
use std::io;
use std::io::Write;

const MAXIMUM_ERRORS: usize = 3;

fn main() {
    print_text("Please enter the secret word: ");

    let secret_word = read_user_input();
    println!("The secret is >>{secret_word}<<");

    let mut game = Game::new(&secret_word, MAXIMUM_ERRORS);

    while game.has_finished() == false {
        println!("{game}");
        print_text("Guess a letter: ");
        let guess = read_user_input();
        let letter = guess.chars().nth(0).unwrap();
        game.guess_a_letter(letter);
    }

    println!("{game}");
}

fn print_text(text_to_display: &str) {
    print!("{text_to_display}");
    io::stdout().flush().unwrap();
}

fn read_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect(
        "Failed to read the input. \
        Please enter a valid word with small letters from a to z.",
    );
    return user_input.trim().to_string();
}
