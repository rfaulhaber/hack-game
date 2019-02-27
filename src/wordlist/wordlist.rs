extern crate rand;

use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{BufReader, Cursor, Read};

pub struct WordSet {
	words_by_len: Vec<Vec<String>>,
	len_ids: HashMap<usize, usize>,
}

impl WordSet {
	pub fn from_reader(reader: &mut std::io::Read) -> WordSet {
		let buf = BufReader::new(reader);

		let mut words_by_len: Vec<Vec<String>> = Vec::new();
		let mut len_ids: HashMap<usize, usize> = HashMap::new();

		let mut current_index = 0;

		for (i, line) in buf.lines().enumerate() {
			let line_str = line.unwrap();
			let line_len = line_str.len();

			if words_by_len.is_empty() {
				words_by_len.push(vec![line_str]);
			} else {
				// thank you clippy
				let index = len_ids.entry(line_len).or_insert_with(|| {
					let next_index = current_index + 1;
					current_index
				});

				words_by_len[*index].push(line_str);
			}
		}

		println!("words_by_len: {:?}", words_by_len);
		println!("len_ids: {:?}", len_ids);

		WordSet {
			words_by_len,
			len_ids,
		}
	}

	pub fn get_word_list(&self, word_len: usize, list_len: usize) -> Vec<String> {
		let mut rng = &mut rand::thread_rng();

		let key = self.len_ids[&word_len];
		let words_of_len = &self.words_by_len[key];

		words_of_len
			.choose_multiple(&mut rng, list_len)
			.cloned()
			.collect()
	}

	// TODO get "similar" words by some metric
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_word_list_returns_random_list() {
		let mut words = Cursor::new(vec!["a", "as", "asd", "asdf", "b", "be", "bee"].join("\n"));

		let set = WordSet::from_reader(&mut words);

		let result = set.get_word_list(2, 2);

		println!("result: {:?}", result);

		assert_eq!(result.len(), 2);
		assert!(result.contains(&String::from("be")));
		assert!(result.contains(&String::from("as")));
	}
}