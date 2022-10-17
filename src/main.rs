use std::io::{self, Write};

use crossterm::style::{Stylize, Color};
use rand::seq::SliceRandom;
use rand::thread_rng;

const MAX_GUESSES: usize = 6;

fn main() {
    let word = random_word();

    for _ in 0..MAX_GUESSES {
        let guess = read_guess();

        print_guess(&guess, word);

        if guess == word {
            println!("You WIN!");
            return;
        }
    }

    println!("You lose :(");
    println!("The word was '{}'", word);
}

fn read_guess() -> String {
    loop {
        print!("Your guess: ");
        io::stdout().flush().ok();

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).unwrap();

        let guess = guess.trim().to_ascii_lowercase();

        if guess.chars().any(|char| !char.is_alphabetic()) {
            println!("guess must only contain alphabetic characters");
            continue
        }

        if guess.len() != 5 {
            println!("guess must be 5 characters long");
            continue
        }

        return guess;
    }
}

fn print_guess(guess: &str, word: &str) {
    let states = calculate_states(guess, word);

    for (char, state) in states {
        let color = match state {
            State::Correct => Color::Green,
            State::WrongPosition => Color::Yellow,
            State::Wrong => Color::Red,
        };

        print!("{} ", char.with(color).underlined());
    }

    println!();
}

fn random_word() -> &'static str {
    words()
        .choose(&mut thread_rng())
        .expect("word list is empty :(")
}

fn words() -> Vec<&'static str> {
    include_str!("../words.txt")
        .lines()
        .filter(|word| word.is_ascii())
        .filter(|word| word.len() == 5)
        .filter(|word| word.chars().all(|c| c.is_ascii_lowercase()))
        .collect()
}

enum State {
    Correct,
    WrongPosition,
    Wrong,
}

fn calculate_states(guess: &str, word: &str) -> Vec<(char, State)> {
    guess
        .chars()
        .zip(word.chars())
        .map(|(guess, actual)| {
            if guess == actual {
                (guess, State::Correct)
            } else if word.contains(guess) {
                (guess, State::WrongPosition)
            } else {
                (guess, State::Wrong)
            }
        })
        .collect()
}
