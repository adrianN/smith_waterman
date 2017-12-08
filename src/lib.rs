#![allow(dead_code)]

#[derive(Clone, Copy, Debug)]
struct TableEntry {
    match_score: isize,
    streak: isize,
    gaps: isize,
    pattern_skip: isize,
    word_boundary: isize,
}

pub struct MatcherWeights {
    pub gap_penalty: isize,
    pub pattern_skip_penalty: isize,
    pub first_letter_bonus: isize,
}

impl MatcherWeights {
    pub fn new() -> MatcherWeights {
        MatcherWeights {
            gap_penalty: -5,
            pattern_skip_penalty: -10,
            first_letter_bonus: 3,
        }
    }
}

impl TableEntry {
    fn new() -> TableEntry {
        TableEntry {
            match_score: 0,
            streak: 0,
            gaps: 0,
            pattern_skip: 0,
            word_boundary: 0,
        }
    }

    fn match_key(&self) -> TableEntry {
        TableEntry {
            match_score: self.match_score + self.streak + 1,
            streak: self.streak + 1,
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
        if self.streak == 0 {
            self.clone()
        } else {
            TableEntry {
                streak: 0,
                gaps: self.gaps + 1,
                ..*self
            }
        }
    }

    fn boundary_match(&self) -> TableEntry {
        TableEntry {
            word_boundary: self.word_boundary + 1,
            ..*self
        }
    }

    fn get_score(&self, w: &MatcherWeights) -> isize {
        w.gap_penalty * self.gaps + w.pattern_skip_penalty * self.pattern_skip + self.match_score
            + w.first_letter_bonus * self.word_boundary
    }
}

struct Table {
    data: Vec<TableEntry>,
    text_len: usize,
}

impl Table {
    fn new(text_len: usize) -> Table {
        let mut t = Table {
            data: vec![TableEntry::new(); 1 * (1 + text_len)],
            text_len: text_len,
        };
        for i in 1..t.data.len() {
            t.data[i].gaps = 1;
        }
        t
    }

    fn get<'a>(&'a self, p: usize, t: usize) -> &'a TableEntry {
        &self.data[t + p * self.text_len]
    }

    fn get_mut<'a>(&'a mut self, p: usize, t: usize) -> &'a mut TableEntry {
        &mut self.data[t + p * self.text_len]
    }

    fn add_row(&mut self) {
        for _i in 0..self.text_len {
            self.data.push(TableEntry::new());
        }
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
        self.table.add_row();
        let mut last_b: Option<&u8> = None;
        for (i, b) in self.text.as_bytes().into_iter().enumerate() {
            let i = i + 1;
            println!("i {} b {}", i, *b as char);
            let pattern_skip = (1..self.pattern_length + 1)
                .map(|x| self.table.get(self.pattern_length - x, i).skip_pattern())
                .max_by_key(|x| x.get_score(&self.weights));
            println!("ps {:?}", pattern_skip);
            let text_skip = (1..i)
                .map(|x| self.table.get(self.pattern_length, i - x).skip_text())
                .max_by_key(|x| x.get_score(&self.weights));
            println!("ts {:?}", text_skip);
            let matching = if k == *b {
                let m = self.table.get(self.pattern_length - 1, i - 1).match_key();
                let boundaries = "-_\\/ ".as_bytes();
                match last_b {
                    Some(x) => {
                        if boundaries.into_iter().any(|y| *x == *y) {
                            Some(m.boundary_match())
                        } else {
                            Some(m)
                        }
                    }
                    _ => Some(m),
                }
            } else {
                None
            };
            println!("match {:?}", matching);
            let r: TableEntry = pattern_skip
                .into_iter()
                .chain(text_skip)
                .chain(matching)
                .max_by_key(|x| x.get_score(&self.weights))
                .unwrap();
            println!("\x1b[31;1;4mstoring \x1b[0m{:?}", r);
            *self.table.get_mut(self.pattern_length, i) = r;
            last_b = Some(b);
        }
    }

    pub fn remove_pchar(&mut self) {
        self.pattern_length -= 1;
    }

    pub fn score(&self) -> isize {
        self.table
            .get(self.pattern_length, self.text.len())
            .get_score(&self.weights)
    }
}
