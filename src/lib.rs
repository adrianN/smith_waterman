#![allow(dead_code)]

mod path_match {
    use std::cmp::max;

    #[derive(Clone)]
    struct TableEntry {
        score: isize,
    }

    impl TableEntry {
        fn new() -> TableEntry {
            TableEntry { score: 0 }
        }

        fn get_score(&self) -> isize {
            self.score
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
            println!("Get {} {}", p, t);
            &self.data[t + p * self.text_len]
        }

        fn get_mut<'a>(&'a mut self, p: usize, t: usize) -> &'a mut TableEntry {
            println!("Get mut {} {}", p, t);
            &mut self.data[t + p * self.text_len]
        }

        fn add_row(&mut self) {
            for i in 0..self.text_len {
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
            for (i, b) in self.text.as_bytes().into_iter().enumerate() {
                let i = i+1;
                println!("i {} b {}", i, *b as char);
                let pattern_skip = (1..self.pattern_length + 1)
                    .map(|x| self.table.get(self.pattern_length - x, i).get_score() - x as isize)
                    .max()
                    .unwrap_or(-1);
                println!("ps {}", pattern_skip);
                let text_skip = (1..i)
                    .map(|x| {
                        self.table.get(self.pattern_length - 1, i - x).get_score() - x as isize
                    })
                    .max()
                    .unwrap_or(-1);
                println!("ts {}", text_skip);
                let matching = if k == *b {
                    self.table.get(self.pattern_length - 1, i - 1).get_score()
                } else {
                    -1
                };
                self.table.get_mut(self.pattern_length, i).score =
                    max(max(pattern_skip, text_skip), matching);
            }
        }

        pub fn remove_pchar(&mut self) {
            self.pattern_length -= 1;
        }

        pub fn score(&self) -> isize {
            self.table.get(self.pattern_length, self.text.len()).score
        }
    }
}

#[cfg(test)]
mod tests {
    use path_match;
    #[test]
    fn it_works() {
        let m = path_match::Matcher::new("aoeu");
        assert!(m.score() == 0);
    }

    #[test]
    fn matching() {
        let mut m = path_match::Matcher::new("aoeu");
        m.add_pchar('a' as u8);
        assert!(m.score() == 2);
    }
}
