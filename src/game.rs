use super::wordlist::WordSet;

pub enum Difficulty {
	VeryEasy,
	Easy,
	Average,
	Hard,
	VeryHard,
}

pub struct Game {
	turns: u8,
	password: String,
	difficulty: Difficulty,
	word_list: WordSet,
}

impl Game {
	pub fn new(turns: u8, difficulty: Difficulty) -> Game {}
}
