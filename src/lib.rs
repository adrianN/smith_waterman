#![allow(dead_code)]

#[derive(Clone, Copy, Debug)]
struct TableEntry {
    match_score: isize,
    streak: isize,
    gaps: isize,
    pattern_skip: isize,
    word_boundary: isize,
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

    fn get_score(&self) -> isize {
        -5 * self.gaps + (-1) * 10 * self.pattern_skip + self.match_score
            + self.word_boundary * 3
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
        println!(
            "Get {} {} -> {}",
            p,
            t,
            self.data[t + p * self.text_len].get_score()
        );
        &self.data[t + p * self.text_len]
    }

    fn get_mut<'a>(&'a mut self, p: usize, t: usize) -> &'a mut TableEntry {
        println!(
            "Get {} {} -> {}",
            p,
            t,
            self.data[t + p * self.text_len].get_score()
        );
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
}

impl<'a> Matcher<'a> {
    pub fn new(text: &'a str) -> Matcher<'a> {
        Matcher {
            text: text,
            pattern_length: 0,
            table: Table::new(text.len()),
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
                .max_by_key(|x| x.get_score());
            println!("ps {:?}", pattern_skip);
            let text_skip = (1..i)
                .map(|x| self.table.get(self.pattern_length, i - x).skip_text())
                .max_by_key(|x| x.get_score());
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
                .max_by_key(|x| x.get_score())
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
            .get_score()
    }
}
