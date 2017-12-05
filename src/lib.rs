#![allow(dead_code)]

mod path_match {
    use std::cmp::max;

    #[derive(Clone)]
    struct TableEntry {
        score: isize,
    }

    impl TableEntry {
        fn new() -> TableEntry {
            TableEntry { score: -100 }
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
            let mut t = Table {
                data: vec![TableEntry::new(); 1 * (1 + text_len)],
                text_len: text_len,
            };
            for i in 0..t.data.len() {
                t.data[i].score = -(i as isize);
            }
            t
        }

        fn get<'a>(&'a self, p: usize, t: usize) -> &'a TableEntry {
            println!("Get {} {} -> {}", p, t, self.data[t + p * self.text_len].get_score());
            &self.data[t + p * self.text_len]
        }

        fn get_mut<'a>(&'a mut self, p: usize, t: usize) -> &'a mut TableEntry {
            println!("Get {} {} -> {}", p, t, self.data[t + p * self.text_len].get_score());
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
            for (i, b) in self.text.as_bytes().into_iter().enumerate() {
                let i = i+1;
                println!("i {} b {}", i, *b as char);
                let pattern_skip = (1..self.pattern_length + 1)
                    .map(|x| self.table.get(self.pattern_length - x, i).get_score() - x as isize)
                    .max()
                    .unwrap_or(-10000);
                println!("ps {}", pattern_skip);
                let text_skip = (1..i)
                    .map(|x| {
                        self.table.get(self.pattern_length , i - x).get_score() - x as isize
                    })
                    .max()
                    .unwrap_or(-10000);
                println!("ts {}", text_skip);
                let matching = if k == *b {
                    println!("match");
                    self.table.get(self.pattern_length - 1, i - 1).get_score()

                } else {
                    -10000
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
        println!("score {}", m.score());
        assert!(m.score() == -4);
    }

    #[test]
    fn matching() {
        let bts = "aoeu".as_bytes();
        for b in bts {
            let mut m = path_match::Matcher::new("aoeu");
            m.add_pchar(*b);
            println!("score {}", m.score());
            assert!(m.score() == -3);
        }
    }

    #[test]
    fn matching2chars() {
        let bts = "aoeu".as_bytes();
        for i in 0..bts.len() {
            for j in i+1..bts.len() {
                let mut m = path_match::Matcher::new("aoeu");
                m.add_pchar(bts[i]);
                m.add_pchar(bts[j]);
                println!("score {}",m.score());
                assert!(m.score() == -2);
            }
        }
    }
}
