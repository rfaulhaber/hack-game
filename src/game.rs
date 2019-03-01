extern crate rand;
use super::wordlist::WordSet;
use rand::seq::SliceRandom;
use rand::Rng;

pub enum Difficulty {
	VeryEasy,
	Easy,
	Average,
	Hard,
	VeryHard,
}

#[derive(PartialEq, Debug)]
pub enum GuessResult {
	Correct,
	Incorrect { matching: u8, total: u8 },
	GameOver,
}

pub struct Game {
	turns: u8,
	password: String,
	word_list: Vec<String>,
}

impl Game {
	pub fn new(turns: u8, diff: Difficulty, word_set: &WordSet) -> Game {
		let word_len = get_word_len_by_difficulty(diff);
		let word_list = word_set.get_random_word_list(word_len as usize, 20);

		let mut rng = &mut rand::thread_rng();
		let password = word_list.choose(&mut rng).unwrap().to_owned();

		Game {
			turns,
			password,
			word_list,
		}
	}

	pub fn guess(&mut self, guess: String) -> GuessResult {
		if self.turns == 0 {
			return GuessResult::GameOver;
		} else if self.password == guess {
			self.turns = 0;
			return GuessResult::Correct;
		}

		let mut matching = 0;

		for (l, r) in guess.chars().zip(self.password.chars()) {
			if l == r {
				matching += 1;
			}
		}

		self.turns -= 1;

		GuessResult::Incorrect {
			matching: matching,
			total: self.password.len() as u8,
		}
	}

	pub fn get_word_list(&self) -> Vec<String> {
		self.word_list.clone()
	}

	pub fn ongoing(&self) -> bool {
		self.turns > 0
	}

	pub fn turns(&self) -> u8 {
		self.turns
	}
}

fn get_word_len_by_difficulty(diff: Difficulty) -> u8 {
	let mut rng = rand::thread_rng();

	match diff {
		Difficulty::VeryEasy => rng.gen_range(4, 5),
		Difficulty::Easy => rng.gen_range(6, 8),
		Difficulty::Average => rng.gen_range(9, 10),
		Difficulty::Hard => rng.gen_range(11, 12),
		Difficulty::VeryHard => rng.gen_range(13, 15),
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	use std::io::Cursor;

	#[test]
	fn guess_returns_results() {
		let mut words = Cursor::new(vec!["cats", "bats", "rats", "hats"].join("\n"));

		let set = WordSet::from_reader(&mut words);

		let mut game = Game {
			turns: 4,
			password: String::from("rats"),
			word_list: vec![
				String::from("cats"),
				String::from("bats"),
				String::from("hats"),
			],
		};

		let incorrect = game.guess(String::from("hats"));

		if let GuessResult::Incorrect { matching, total } = incorrect {
			assert_eq!(matching, 3);
			assert_eq!(total, 4);
		} else {
			panic!("shouldn't panic");
		}

		let correct = game.guess(String::from("rats"));

		assert_eq!(correct, GuessResult::Correct);

		let game_over = game.guess(String::from("what"));

		assert_eq!(game_over, GuessResult::GameOver);
	}
}
