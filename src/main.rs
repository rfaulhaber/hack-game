extern crate termion;
mod game;
mod wordlist;

use game::{Difficulty, Game, GuessResult};
use std::fs::File;
use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::input::TermRead;
use wordlist::WordSet;

fn main() {
    let mut words_file = File::open("./enable1-filtered.txt").expect("could not find word file");

    let word_set = WordSet::from_reader(&mut words_file);

    let stdout = stdout();
    let mut stdout = stdout.lock();
    let stdin = stdin();
    let mut stdin = stdin.lock();

    write!(stdout, "{}", clear::All).unwrap();
    stdout.write_all(b"Difficulty?").unwrap();
    stdout.flush().unwrap();

    if let Some(diff_input) = stdin.read_line().unwrap() {
        let diff = match diff_input.to_lowercase().as_str() {
            "very easy" | "1" => Difficulty::VeryEasy,
            "easy" | "2" => Difficulty::Easy,
            "average" | "3" => Difficulty::Average,
            "hard" | "4" => Difficulty::Hard,
            "very hard" | "5" => Difficulty::VeryHard,
            _ => Difficulty::Average,
        };

        let mut game = Game::new(4 as u8, diff, &word_set);

        let word_list = game.get_word_list();

        writeln!(stdout, "{}", word_list.join("\n")).unwrap();

        while game.ongoing() {
            writeln!(stdout, "Guess? {} turns left", game.turns()).unwrap();

            if let Some(guess) = stdin.read_line().unwrap() {
                let result = game.guess(guess);

                match result {
                    GuessResult::Incorrect { matching, total } => {
                        writeln!(stdout, "{}/{} correct", matching, total).unwrap();
                    }
                    GuessResult::Correct => {
                        writeln!(stdout, "You win!").unwrap();
                    }
                    GuessResult::GameOver => {
                        writeln!(stdout, "You lose!").unwrap();
                    }
                }
            }
        }
    }
}
