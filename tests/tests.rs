extern crate path_match;
fn default_weights() -> path_match::MatcherWeights {
    path_match::MatcherWeights {
        gap_penalty : -5,
        pattern_skip_penalty : -10,
        first_letter_bonus : 3
    }
}

#[test]
fn it_works() {
    let m = path_match::Matcher::from("aoeu", default_weights());
    assert_eq!(m.score(), -5);
}

#[test]
fn matching() {
    let bts = "aoeu".as_bytes();
    for (i, b) in bts.into_iter().enumerate() {
        println!("===");
        let mut m = path_match::Matcher::from("aoeu", default_weights());
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

            let mut m = path_match::Matcher::from("aoeu", default_weights());
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
    let mut m = path_match::Matcher::from("aoeu", default_weights());
    m.add_pchar('x' as u8);
    println!("score {}", m.score());
    assert_eq!(m.score(), -15);
}

#[test]
fn not_greedy() {
    let mut m = path_match::Matcher::from("aoehtnaoeu", default_weights());
    for x in "aoeu".as_bytes() {
        m.add_pchar(*x);
    }
    assert_eq!(m.score(), -5 + 10);
}

#[test]
fn word_boundary() {
    let mut m = path_match::Matcher::from("ht ao", default_weights());
    m.add_pchar('a' as u8);
    assert_eq!(m.score(), -5 + 3 + 1 - 5);
}

