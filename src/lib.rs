#![allow(dead_code)]

mod path_match {

    #[derive(Clone, Copy, Debug)]
    struct TableEntry {
        match_score: isize,
        streak: isize,
        gaps: isize,
        pattern_skip: isize,
    }

    impl TableEntry {
        fn new() -> TableEntry {
            TableEntry {
                match_score: 0,
                streak: 0,
                gaps: 0,
                pattern_skip: 0,
            }
        }

        fn match_key(&self) -> TableEntry {
            TableEntry {
                match_score: self.match_score + self.streak + 1,
                streak: self.streak + 1,
                gaps: self.gaps,
                pattern_skip: self.pattern_skip,
            }
        }

        fn skip_pattern(&self) -> TableEntry {
            TableEntry {
                match_score: self.match_score,
                streak: self.streak,
                gaps: self.gaps,
                pattern_skip: self.pattern_skip + 1,
            }
        }

        fn skip_text(&self) -> TableEntry {
            if self.streak == 0 {
                self.clone()
            } else {
                TableEntry {
                    match_score: self.match_score,
                    streak: 0,
                    gaps: self.gaps + 1,
                    pattern_skip: self.pattern_skip,
                }
            }
        }

        fn get_score(&self) -> isize {
            -5 * self.gaps + (-1) * 10 * self.pattern_skip + self.match_score
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
                    Some(self.table.get(self.pattern_length - 1, i - 1).match_key())
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
}

#[cfg(test)]
mod tests {
    use path_match;
    #[test]
    fn it_works() {
        let m = path_match::Matcher::new("aoeu");
        assert_eq!(m.score(), -5);
    }

    #[test]
    fn matching() {
        let bts = "aoeu".as_bytes();
        for (i, b) in bts.into_iter().enumerate() {
            println!("===");
            let mut m = path_match::Matcher::new("aoeu");
            m.add_pchar(*b);
            if i == 0 || i == 3 {
                assert_eq!(m.score(), -5 + 1);
            } else {
                assert_eq!(m.score(), -5 + 1 + -5);
            }
        }
    }

    #[test]
    fn matching2chars() {
        let bts = "aoeu".as_bytes();
        for i in 0..bts.len() {
            let first_skip = if i == 0 { 0 } else { 1 };
            for j in i + 1..bts.len() {
                let second_skip = if (j - i) > 1 { 1 } else { 0 };
                let third_skip = if j < 3 { 1 } else { 0 };
                let match_bonus = if (j - i) == 1 { 1 } else { 0 };

                let mut m = path_match::Matcher::new("aoeu");
                println!("*** add {}", bts[i] as char);
                m.add_pchar(bts[i]);
                println!("add {}", bts[j] as char);
                m.add_pchar(bts[j]);
                println!(
                    "i {}, j {}, fs {}, ss {}, ts {}, mb {}",
                    i,
                    j,
                    first_skip,
                    second_skip,
                    third_skip,
                    match_bonus
                );
                assert_eq!(
                    m.score(),
                    (-5) * (first_skip + second_skip + third_skip) + 2 + match_bonus
                );
            }
        }
    }

    #[test]
    fn not_matching() {
        let mut m = path_match::Matcher::new("aoeu");
        m.add_pchar('x' as u8);
        println!("score {}", m.score());
        assert_eq!(m.score(), -15);
    }
}
