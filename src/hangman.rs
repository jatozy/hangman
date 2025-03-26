pub mod hangman {
    use std::fmt;

    pub struct Game {
        maximum_errors: usize,
        happened_errors: usize,
        secret: String,
        solution: String,
    }

    impl Game {
        pub fn new(secret: &str, maximum_errors: usize) -> Game {
            Game {
                maximum_errors: maximum_errors,
                happened_errors: 0,
                secret: secret.to_string(),
                solution: "_".repeat(secret.len()),
            }
        }

        pub fn guess_a_letter(&mut self, guessed_letter: char) {
            let positions = self.get_positions_of_letter(guessed_letter);
            if positions.is_empty() {
                self.increment_errors();
            } else {
                self.reveal_letters_of_solution(guessed_letter, positions)
            }
        }

        pub fn has_finished(&self) -> bool {
            self.is_game_over() || self.player_has_won()
        }

        fn is_game_over(&self) -> bool {
            self.happened_errors == self.maximum_errors
        }

        fn player_has_won(&self) -> bool {
            self.secret == self.solution
        }

        fn get_positions_of_letter(&self, searched_letter: char) -> Vec<usize> {
            let mut positions: Vec<usize> = Vec::new();

            for (index, letter) in self.secret.chars().enumerate() {
                if letter == searched_letter {
                    positions.push(index);
                }
            }

            return positions;
        }

        fn increment_errors(&mut self) {
            self.happened_errors += 1;
        }

        fn reveal_letters_of_solution(&mut self, letter: char, positions: Vec<usize>) {
            for i in positions {
                self.solution
                    .replace_range(i..i + 1, letter.to_string().as_str());
            }
        }

        fn game_status(&self) -> &str {
            if self.is_game_over() {
                "GAME OVER!!!"
            } else if self.has_finished() {
                "CONGRATULATIONS, YOU WON"
            } else {
                ""
            }
        }
    }

    impl fmt::Display for Game {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Errors {} of {}\n\
                Solution \"{}\"\n\
                {}",
                self.happened_errors,
                self.maximum_errors,
                self.solution,
                self.game_status()
            )
        }
    }
}
