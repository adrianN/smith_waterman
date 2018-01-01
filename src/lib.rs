#![allow(dead_code)]
#![deny(missing_docs)]

//! This crate implements a version of the Smith Waterman algorithm for
//! sequence alignment. It works on arrays of u8.
//!
//! The crate provides a `Matcher` struct that does the work and keeps track
//! of the state. You provide a &str to create that struct and then you can
//! add or remove pattern characters using the `add_pchar` and `remove_pchar`
//! methods. You can query the score with the `score` function.
//!
//! To tune the behavior of the algorithm you can provide weights to change
//! the bonuses and penalties for matching or not matching a character. 
//! See the documentation of the `MatcherWeights` struct.

#[derive(Clone, Debug)]
struct TableEntry {
    match_count: isize,
    streak: isize,
    gaps: isize,
    pattern_skip: isize,
    word_boundary: isize,
}

/// For tuning the algorithm.
#[derive(Default)]
pub struct MatcherWeights {
    /// Penalty for leaving a gap. Makes the algorithm prefer matching consecutive characters
    /// Unmatched prefix and suffix are free.
    /// Default -5
    pub gap_penalty: isize,
    /// Penalty for skipping a character of the pattern.
    /// Default -10
    pub pattern_skip_penalty: isize,
    /// Points for matching two characters
    /// Default 1
    pub match_bonus: isize,
    /// Points for matching a character after -, _, \, /, Space
    /// This is additional to the points for matching
    /// Default 3
    pub first_letter_bonus: isize,
}
const BOUNDARIES: [u8; 5] = [45, 95, 92, 47, 32];

impl MatcherWeights {
    /// Make a new MatcherWeight with the default weights.
    pub fn new() -> MatcherWeights {
        MatcherWeights {
            gap_penalty: -5,
            pattern_skip_penalty: -10,
            match_bonus: 1,
            first_letter_bonus: 3,
        }
    }
}

impl TableEntry {
    fn new() -> TableEntry {
        TableEntry {
            match_count: 0,
            streak: 0,
            gaps: 0,
            pattern_skip: 0,
            word_boundary: 0,
        }
    }

    // boundary_match==True <-> previous character is a word boundary
    fn match_key(&self, boundary_match: bool) -> TableEntry {
        TableEntry {
            match_count: self.match_count + 1,
            streak: self.streak + 1,
            word_boundary: self.word_boundary + if boundary_match { 1 } else { 0 },
            ..*self
        }
    }

    fn skip_pattern(&self) -> TableEntry {
        TableEntry {
            pattern_skip: self.pattern_skip + 1,
            ..*self
        }
    }

    fn skip_text(&self) -> TableEntry {
        TableEntry {
            streak: 0,
            // the unmatched prefix doesnt' contribute a gap because
            // streak==0
            gaps: self.gaps + if self.streak != 0 { 1 } else { 0 },
            ..*self
        }
    }

    // (Count*Weight for all scoring features, streak).
    fn get_score(&self, w: &MatcherWeights) -> (isize, isize) {
        (
            w.gap_penalty * self.gaps + w.pattern_skip_penalty * self.pattern_skip
                + w.match_bonus * self.match_count
                + w.first_letter_bonus * self.word_boundary,
            if self.streak > 1 { self.streak } else { 0 },
        )
    }
}

struct Table {
    data: Vec<TableEntry>,
    text_len: usize,
}

impl Table {
    fn new(text_len: usize) -> Table {
        Table {
            data: vec![TableEntry::new(); 1 * (1 + text_len)],
            text_len: text_len,
        }
    }

    fn get(&self, p: usize, t: usize) -> &TableEntry {
        &self.data[t + p * self.text_len]
    }

    fn ensure_row(&mut self) {
        self.data.reserve(self.text_len);
    }

    fn pop_row(&mut self) {
        for _ in 0..self.text_len {
            self.data.pop();
        }
    }

    fn add(&mut self, entry: TableEntry) {
        self.data.push(entry);
    }
}

/// This keeps track of the matcher state
/// It consumes memory proportional to text length * pattern length
pub struct Matcher<'a> {
    text: &'a str,
    pattern_length: usize,
    table: Table,
    weights: MatcherWeights,
}

impl<'a> Matcher<'a> {
    /// Make an empty matcher with default weights
    pub fn new(text: &'a str) -> Matcher<'a> {
        Matcher::from(text, MatcherWeights::new())
    }

    /// Make an empty matcher with custom weights
    pub fn from(text: &'a str, weights: MatcherWeights) -> Matcher<'a> {
        Matcher {
            text: text,
            pattern_length: 0,
            table: Table::new(text.len()),
            weights: weights,
        }
    }

    /// Add a pattern character
    pub fn add_pchar(&mut self, k: u8) {
        self.pattern_length += 1;
        let mut last_b: Option<&u8> = None;
        self.table.ensure_row();
        let text_chars = self.text.as_bytes().into_iter().enumerate();
        for (i, b) in text_chars {
            let i = i + 1;
            // We could save some copies by changing the score function
            // so that we don't have to create new structs to get the
            // scores for skipping things.
            let pattern_skip = Some(self.table.get(self.pattern_length - 1, i).skip_pattern());
            let text_skip = if i >= 2 {
                Some(self.table.get(self.pattern_length, i - 1).skip_text())
            } else {
                None
            };
            let matching = if k == *b {
                let boundary_match = match last_b {
                    Some(x) => BOUNDARIES.into_iter().any(|y| *x == *y),
                    None => true,
                };
                Some(
                    self.table
                        .get(self.pattern_length - 1, i - 1)
                        .match_key(boundary_match),
                )
            } else {
                None
            };
            let r: TableEntry = matching
                .into_iter()
                .chain(text_skip)
                .chain(pattern_skip)
                .max_by_key(|x| x.get_score(&self.weights))
                .unwrap();
            self.table.add(r);
            last_b = Some(b);
        }
    }

    /// Remove the last pattern character
    pub fn remove_pchar(&mut self) {
        if self.pattern_length > 0 {
            self.pattern_length -= 1;
            self.table.pop_row();
        }
    }

    /// Score for current pattern
    pub fn score(&self) -> isize {
        let mut te = TableEntry {
            ..*self.table.get(self.pattern_length, self.text.len())
        };
        // the gap at the end shouldn't count
        if te.streak == 0 && te.gaps > 0 {
            te.gaps -= 1;
        }
        te.get_score(&self.weights).0
    }
}
