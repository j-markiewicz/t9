use std::{borrow::Cow, sync::OnceLock};

use tracing::instrument;

pub const WORDS_EN: &str = include_str!("./words-en.txt");
pub const WORDS_PL: &str = include_str!("./words-pl.txt");
static WORDMAP_EN: OnceLock<WordMap> = OnceLock::new();
static WORDMAP_PL: OnceLock<WordMap> = OnceLock::new();

/// Initialize dictionaries
pub fn init() {
	let _ = WORDMAP_EN.get_or_init(|| WordMap::new(WORDS_EN.lines()));
	let _ = WORDMAP_PL.get_or_init(|| WordMap::new(WORDS_PL.lines()));
}

/// A map of words
#[derive(Debug, Clone, Default)]
struct WordMap {
	punc: Option<Box<WordMap>>,
	abc: Option<Box<WordMap>>,
	def: Option<Box<WordMap>>,
	ghi: Option<Box<WordMap>>,
	jkl: Option<Box<WordMap>>,
	mno: Option<Box<WordMap>>,
	pqrs: Option<Box<WordMap>>,
	tuv: Option<Box<WordMap>>,
	wxyz: Option<Box<WordMap>>,
	content: Vec<&'static str>,
}

impl WordMap {
	fn new(words: impl Iterator<Item = &'static str>) -> Self {
		let mut res = Self::default();

		for word in words {
			let mut map = &mut res;
			let chars = word.chars().map(Character::from_char);

			if chars.clone().any(|c| c.is_none()) {
				continue;
			}

			for char in chars.map(|c| c.unwrap()) {
				let next = map.get_next_mut(char);

				if next.is_none() {
					*next = Some(Box::default());
				}

				map = next.as_deref_mut().unwrap();
				map.content.push(word);
			}
		}

		res
	}

	fn get_next(&self, char: Character) -> &Option<Box<WordMap>> {
		match char {
			Character::Punctuation => &self.punc,
			Character::Abc => &self.abc,
			Character::Def => &self.def,
			Character::Ghi => &self.ghi,
			Character::Jkl => &self.jkl,
			Character::Mno => &self.mno,
			Character::Pqrs => &self.pqrs,
			Character::Tuv => &self.tuv,
			Character::Wxyz => &self.wxyz,
		}
	}

	fn get_next_mut(&mut self, char: Character) -> &mut Option<Box<WordMap>> {
		match char {
			Character::Punctuation => &mut self.punc,
			Character::Abc => &mut self.abc,
			Character::Def => &mut self.def,
			Character::Ghi => &mut self.ghi,
			Character::Jkl => &mut self.jkl,
			Character::Mno => &mut self.mno,
			Character::Pqrs => &mut self.pqrs,
			Character::Tuv => &mut self.tuv,
			Character::Wxyz => &mut self.wxyz,
		}
	}
}

/// A T9 input character
#[derive(Debug, Clone, Copy)]
pub enum Input {
	/// A word character (letter or punctuation)
	Word(Character),
	/// Go to the next suggestion (`*`, UI-only)
	Next,
	/// Space (`0`)
	Space,
	/// Remove the previous input (`#`)
	Backspace,
}

impl TryFrom<char> for Input {
	type Error = InvalidCharacter;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'1'..='9' => Character::try_from(value).map(Self::Word),
			'*' => Ok(Self::Next),
			'0' => Ok(Self::Space),
			'#' => Ok(Self::Backspace),
			char => Err(Self::Error::NotT9(char)),
		}
	}
}

/// A T9 word character
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Character {
	Punctuation = 1,
	Abc = 2,
	Def = 3,
	Ghi = 4,
	Jkl = 5,
	Mno = 6,
	Pqrs = 7,
	Tuv = 8,
	Wxyz = 9,
}

impl Character {
	pub fn chars(self, lang: Language) -> &'static [char] {
		match lang {
			Language::En => match self {
				Self::Punctuation => &[',', '.', '!', '?', '\'', '-', '&', '1'],
				Self::Abc => &['a', 'b', 'c', '2'],
				Self::Def => &['d', 'e', 'f', '3'],
				Self::Ghi => &['g', 'h', 'i', '4'],
				Self::Jkl => &['j', 'k', 'l', '5'],
				Self::Mno => &['m', 'n', 'o', '6'],
				Self::Pqrs => &['p', 'q', 'r', 's', '7'],
				Self::Tuv => &['t', 'u', 'v', '8'],
				Self::Wxyz => &['w', 'x', 'y', 'z', '9', '0'],
			},
			Language::Pl => match self {
				Self::Punctuation => &[',', '.', '!', '?', '\'', '-', '&', '1'],
				Self::Abc => &['a', 'b', 'c', 'ą', 'ć', '2'],
				Self::Def => &['d', 'e', 'f', 'ę', '3'],
				Self::Ghi => &['g', 'h', 'i', '4'],
				Self::Jkl => &['j', 'k', 'l', 'ł', '5'],
				Self::Mno => &['m', 'n', 'o', 'ń', 'ó', '6'],
				Self::Pqrs => &['p', 'q', 'r', 's', 'ś', '7'],
				Self::Tuv => &['t', 'u', 'v', '8'],
				Self::Wxyz => &['w', 'x', 'y', 'z', 'ż', 'ź', '9', '0'],
			},
		}
	}

	pub fn all_chars(self) -> &'static [char] {
		match self {
			Self::Punctuation => &[',', '.', '!', '?', '\'', '-', '&', '1'],
			Self::Abc => &['a', 'b', 'c', 'ą', 'ć', '2'],
			Self::Def => &['d', 'e', 'f', 'ę', '3'],
			Self::Ghi => &['g', 'h', 'i', '4'],
			Self::Jkl => &['j', 'k', 'l', 'ł', '5'],
			Self::Mno => &['m', 'n', 'o', 'ń', 'ó', '6'],
			Self::Pqrs => &['p', 'q', 'r', 's', 'ś', '7'],
			Self::Tuv => &['t', 'u', 'v', '8'],
			Self::Wxyz => &['w', 'x', 'y', 'z', 'ż', 'ź', '9', '0'],
		}
	}

	pub fn from_char(char: char) -> Option<Self> {
		match char {
			',' | '.' | '!' | '?' | '\'' | '-' | '&' | '1' => Some(Self::Punctuation),
			'a' | 'b' | 'c' | 'ą' | 'ć' | '2' => Some(Self::Abc),
			'd' | 'e' | 'f' | 'ę' | '3' => Some(Self::Def),
			'g' | 'h' | 'i' | '4' => Some(Self::Ghi),
			'j' | 'k' | 'l' | 'ł' | '5' => Some(Self::Jkl),
			'm' | 'n' | 'o' | 'ń' | 'ó' | '6' => Some(Self::Mno),
			'p' | 'q' | 'r' | 's' | 'ś' | '7' => Some(Self::Pqrs),
			't' | 'u' | 'v' | '8' => Some(Self::Tuv),
			'w' | 'x' | 'y' | 'z' | 'ż' | 'ź' | '9' | '0' => Some(Self::Wxyz),
			_ => None,
		}
	}
}

impl TryFrom<char> for Character {
	type Error = InvalidCharacter;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'1' => Ok(Self::Punctuation),
			'2' => Ok(Self::Abc),
			'3' => Ok(Self::Def),
			'4' => Ok(Self::Ghi),
			'5' => Ok(Self::Jkl),
			'6' => Ok(Self::Mno),
			'7' => Ok(Self::Pqrs),
			'8' => Ok(Self::Tuv),
			'9' => Ok(Self::Wxyz),
			char @ ('*' | '0' | '#') => Err(Self::Error::NotWord(char)),
			char => Err(Self::Error::NotT9(char)),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum InvalidCharacter {
	/// The character is not a valid T9 input character
	NotT9(char),
	/// The character is a T9 "special" character, i.e. `0` (space), `*` (next),
	/// or `#` (backspace)
	NotWord(char),
}

#[derive(Debug, Clone, Copy, Default)]
pub enum InputMode {
	/// Multi-tap mode
	#[default]
	Multitap,
	/// T9 mode
	T9,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Language {
	/// English
	#[default]
	En,
	/// Polski
	Pl,
}

/// Decode a multi-tap input without any dictionary lookup
fn multitap_decode(input: &[Character], lang: Language) -> String {
	let mut buf = String::new();
	let mut current = (None, 0);

	for &char in input {
		if current.0 == Some(char) {
			current.1 += 1;
		} else {
			if let Some(cur) = current.0 {
				let chars = cur.chars(lang);
				buf.push(chars[current.1 % chars.len()]);
			}

			current.1 = 0;
			current.0 = Some(char);
		}
	}

	if let Some(cur) = current.0 {
		let chars = cur.chars(lang);
		buf.push(chars[current.1 % chars.len()]);
	}

	buf
}

/// Return the top 3 matching words assuming multi-tap input
#[instrument]
pub fn multitap(input: &[Character], lang: Language) -> [Cow<'static, str>; 3] {
	if input.is_empty() {
		return [
			Cow::Borrowed(""),
			Cow::Borrowed(":-)"),
			Cow::Borrowed(":-("),
		];
	}

	let buf = multitap_decode(input, lang);

	let mut words = match lang {
		Language::En => WORDS_EN,
		Language::Pl => WORDS_PL,
	}
	.lines()
	.filter(|w| w.starts_with(&buf) && w != &buf);

	let sug = (
		words.next().unwrap_or_default(),
		words.next().unwrap_or_default(),
	);

	[Cow::Owned(buf), Cow::Borrowed(sug.0), Cow::Borrowed(sug.1)]
}

/// Return the top 3 matching words assuming T9 input
#[instrument]
pub fn t9(input: &[Character], lang: Language) -> [Cow<'static, str>; 3] {
	if input.is_empty() {
		return [
			Cow::Borrowed(""),
			Cow::Borrowed(":-)"),
			Cow::Borrowed(":-("),
		];
	}

	let mut words = Some(match lang {
		Language::En => WORDMAP_EN.get().expect("dictionary not initialized"),
		Language::Pl => WORDMAP_PL.get().expect("dictionary not initialized"),
	});

	for &char in input {
		if let Some(dict) = words {
			words = dict.get_next(char).as_deref();
		} else {
			break;
		}
	}

	let mut words = words
		.map(|w| w.content.as_slice())
		.unwrap_or_default()
		.iter()
		.copied();

	if let Some(first) = words.next() {
		[
			Cow::Borrowed(first),
			Cow::Borrowed(words.next().unwrap_or_default()),
			Cow::Borrowed(words.next().unwrap_or_default()),
		]
	} else {
		[
			Cow::Owned(multitap_decode(input, lang)),
			Cow::Borrowed(":-)"),
			Cow::Borrowed(":-("),
		]
	}
}
