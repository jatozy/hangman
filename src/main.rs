use std::io;
use std::io::Write;

fn main() {
    print_text("Please enter the secret word: ");

    const ERRORS_BEFORE_GAME_OVER: usize = 3;
    let mut errors_happened: usize = 0;

    let secret_word = read_user_input();
    println!("The secret is >>{secret_word}<<");

    let mut solution = "_".repeat(secret_word.len());

    loop {
        println!("Errors: {errors_happened} of {ERRORS_BEFORE_GAME_OVER}");
        println!("Solution: {solution}");

        print_text("Guess a letter: ");
        let guess = read_user_input();
        let letter = guess.chars().nth(0).unwrap();

        let positions = get_positions_of_letter(&secret_word, letter);
        if positions.is_empty() {
            errors_happened += 1;
            println!("The letter \"{guess}\" does not exist in the word");
        } else {
            println!("The letter \"{guess}\" exists in the word");
            for i in positions {
                solution.replace_range(i..i + 1, guess.as_str());
            }
        }

        if solution == secret_word {
            println!("CONGRATULATIONS!!! The word is \"{secret_word}\"");
            break;
        }
        if errors_happened == ERRORS_BEFORE_GAME_OVER {
            println!("GAME OVER!!! The word was \"{secret_word}\"");
            break;
        }
    }
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

fn get_positions_of_letter(text: &str, searched_letter: char) -> Vec<usize> {
    let mut positions: Vec<usize> = Vec::new();

    for (index, letter) in text.chars().enumerate() {
        if letter == searched_letter {
            positions.push(index);
        }
    }

    return positions;
}
