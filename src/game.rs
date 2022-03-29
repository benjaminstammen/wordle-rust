use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use dialoguer::console;
use dialoguer::console::StyledObject;
use tabled::{builder::Builder, Style};

pub struct Counter<T: Clone + Eq + Hash> {
    counts: HashMap<T, i32>,
}

impl<T: Clone + Display + Eq + Hash> Counter<T> {
    fn new() -> Counter<T> {
        Counter {
            counts: HashMap::new(),
        }
    }

    pub fn add_vec<I>(&mut self, collection: I)
    where
        I: Iterator<Item = T> {
        for object in collection {
            let count = self.counts.entry(object.clone()).or_insert(0);
            *count += 1;
        }
    }

    pub fn clone_counts(&self) -> HashMap<T, i32> {
        return self.counts.clone();
    }
}

pub enum GameState {
    InProgress,
    Won,
    Lost,
}

pub struct Grid {
    pub word: String,
    pub max_guesses: i32,
    pub guesses: Vec<String>,
    pub known_chars: HashSet<char>,
    pub close_chars: HashSet<char>,
    pub unused_chars: HashSet<char>,
    char_counts: Counter<char>,
    styled_guesses: Vec<Vec<StyledObject<char>>>,
}

impl Grid {

    pub fn new(word: String) -> Grid {
        let mut grid = Grid {
            word,
            max_guesses: 5,
            guesses: Vec::new(),
            known_chars: HashSet::new(),
            close_chars: HashSet::new(),
            unused_chars: HashSet::new(),
            char_counts: Counter::new(),
            styled_guesses: Vec::new(),
        };
        grid.char_counts.add_vec(grid.word.chars());
        return grid
    }

    // TODO: push styled chars onto array to save processing?
    pub fn guess(&mut self, guess: &str) -> GameState {
        self.guesses.push(String::from(guess));
        self.compute_renderables(guess);

        if self.word == guess {
            return GameState::Won
        } else if self.guesses.len() >= self.max_guesses as usize {
            return GameState::Lost
        }
        return GameState::InProgress
    }

    pub fn print(&self) {
        let mut table_builder = Builder::default()
            .set_header(1..=self.word.chars().count());
        for row in self.styled_guesses.iter() {
            table_builder = table_builder.add_row(row)
        }
        for _ in self.styled_guesses.len()..self.max_guesses as usize {
            table_builder = table_builder.add_row([0; 0])
        }

        println!("{}", table_builder.build().with(Style::modern()));
    }

    //
    fn compute_renderables(&mut self, guess: &str) {
        let mut char_counts = self.char_counts.clone_counts();
        let mut row_vec: Vec<StyledObject<char>> = Vec::new();

        let style_blue = console::Style::new().blue().bold();
        let style_green = console::Style::new().green().bold();
        let style_yellow = console::Style::new().yellow().bold();
        let style_unused = console::Style::new().bold();

        // "green" and "unused" pass
        for (i, char) in guess.chars().enumerate() {
            if self.word.chars().nth(i).unwrap() == char {
                // Add to known characters
                self.known_chars.insert(char);

                row_vec.push(style_green.apply_to(char));
                let count = char_counts.get_mut(&char).unwrap();
                *count -= 1;
            } else {
                // Add to unused chars for the time being
                self.unused_chars.insert(char);

                row_vec.push(style_unused.apply_to(char))
            }
        }
        // "yellow" pass
        for (i, char) in guess.chars().enumerate() {
            if self.word.chars().nth(i).unwrap() != char && char_counts.contains_key(&char) {
                let count = char_counts.get_mut(&char).unwrap();
                if *count > 0 {
                    // Add to known chars for the time being
                    self.close_chars.insert(char);

                    *count -= 1;
                    row_vec[i] = style_yellow.apply_to(char);
                }
            }
        }
        // "blue" pass
        //
        // Not in default wordle, added to indicate a letter that must have more
        // instances in the word in order to complete the puzzle
        for (i , char) in guess.chars().enumerate() {
            if self.word.chars().nth(i).unwrap() == char {
                let count = char_counts.get_mut(&char).unwrap();
                if *count > 0 {
                    row_vec[i] = style_blue.apply_to(char);
                }
            }
        }
        // Set differences
        self.close_chars = &self.close_chars.clone() - &self.known_chars.clone();
        self.unused_chars = &self.unused_chars.clone() - (&self.known_chars.union(&self.close_chars.clone()).copied().collect());
        self.styled_guesses.push(row_vec);
    }
}