#![allow(dead_code)]
extern crate rand;

#[derive(Clone, Copy, Debug)]
struct TableEntry {
    match_count: isize,
    streak: isize,
    gaps: isize,
    //todo text_skip
    pattern_skip: isize,
    word_boundary: isize,
    id: u16,
}

pub struct MatcherWeights {
    pub gap_penalty: isize,
    pub pattern_skip_penalty: isize,
    pub match_bonus: isize,
    pub first_letter_bonus: isize,
}
const BOUNDARIES: [u8; 5] = [45, 95, 92, 47, 32];

impl MatcherWeights {
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
        let id = rand::random::<u16>();
        println!("() <- {}", id);
        TableEntry {
            match_count: 0,
            streak: 0,
            gaps: 0,
            pattern_skip: 0,
            word_boundary: 0,
            id: id,
        }
    }

    fn match_key(&self, boundary_match: bool) -> TableEntry {
        let id = rand::random::<u16>();
        println!("{} <- {} m", self.id, id);
        TableEntry {
            match_count: self.match_count + 1,
            streak: self.streak + 1,
            word_boundary: self.word_boundary + if boundary_match { 1 } else { 0 },
            id: id,
            ..*self
        }
    }

    fn skip_pattern(&self) -> TableEntry {
        let id = rand::random::<u16>();
        println!("{} <- {} sp", self.id, id);
        TableEntry {
            pattern_skip: self.pattern_skip + 1,
            id: id,
            ..*self
        }
    }

    fn skip_text(&self) -> TableEntry {
        let id = rand::random::<u16>();
        println!("{} <- {} st", self.id, id);
        TableEntry {
            streak: 0,
            gaps: self.gaps + if self.streak != 0 { 1 } else { 0 },
            id: id,
            ..*self
        }
    }

    fn get_score(&self, w: &MatcherWeights) -> (isize, isize) {
        (
            w.gap_penalty * self.gaps + w.pattern_skip_penalty * self.pattern_skip
                + w.match_bonus * self.match_count
                + w.first_letter_bonus * self.word_boundary,
            self.streak,
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

    fn get<'a>(&'a self, p: usize, t: usize) -> &'a TableEntry {
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

pub struct Matcher<'a> {
    text: &'a str,
    pattern_length: usize,
    table: Table,
    weights: MatcherWeights,
}

impl<'a> Matcher<'a> {
    pub fn new(text: &'a str) -> Matcher<'a> {
        Matcher::from(text, MatcherWeights::new())
    }

    pub fn from(text: &'a str, weights: MatcherWeights) -> Matcher<'a> {
        Matcher {
            text: text,
            pattern_length: 0,
            table: Table::new(text.len()),
            weights: weights,
        }
    }

    pub fn add_pchar(&mut self, k: u8) {
        self.pattern_length += 1;
        let mut last_b: Option<&u8> = None;
        self.table.ensure_row();
        for (i, b) in self.text.as_bytes().into_iter().enumerate() {
            println!("{} {}", k as char, &self.text[0..i + 1]);
            let i = i + 1;
            let pattern_skip = (1..self.pattern_length + 1)
                .map(|x| self.table.get(self.pattern_length - x, i).skip_pattern())
                .max_by_key(|x| x.get_score(&self.weights));
            let text_skip = (1..i)
                .map(|x| self.table.get(self.pattern_length, i - x).skip_text())
                .max_by_key(|x| x.get_score(&self.weights));
            let matching = if k == *b {
                let boundary_match = i == self.text.len() || match last_b {
                    Some(x) => BOUNDARIES.into_iter().any(|y| *x == *y),
                    None => true,
                };
                let te = self.table
                    .get(self.pattern_length - 1, i - 1)
                    .match_key(boundary_match);
                println!(
                    "{} {} {:?} {:?}",
                    k as char,
                    i,
                    te,
                    te.get_score(&self.weights)
                );
                Some(te)
            } else {
                None
            };
            let r: TableEntry = pattern_skip
                .into_iter()
                .chain(text_skip)
                .chain(matching)
                .max_by_key(|x| {
                    println!("> {:?}", x.get_score(&self.weights));
                    x.get_score(&self.weights)
                })
                .unwrap();
            println!(
                "add i {} p {} {:?} {:?}",
                i,
                self.pattern_length,
                r,
                r.get_score(&self.weights)
            );
            self.table.add(r);
            last_b = Some(b);
        }
    }

    pub fn remove_pchar(&mut self) {
        if self.pattern_length > 0 {
            self.pattern_length -= 1;
            self.table.pop_row();
        }
    }

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
