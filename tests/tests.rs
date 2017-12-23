extern crate smith_waterman;
fn default_weights() -> smith_waterman::MatcherWeights {
    smith_waterman::MatcherWeights {
        gap_penalty : -5,
        pattern_skip_penalty : -10,
        first_letter_bonus : 3,
        match_bonus: 1
    }
}

#[test]
fn it_works() {
    let m = smith_waterman::Matcher::from("aoeu", default_weights());
    assert_eq!(m.score(), 0);
}

#[test]
fn matching() {
    let bts = "aoeu".as_bytes();
    for (i, b) in bts.into_iter().enumerate() {
        println!("=== {}", *b as char);
        let mut m = smith_waterman::Matcher::from("aoeu", default_weights());
        m.add_pchar(*b);
        if i == 0  {
            assert_eq!(m.score(), 1 + 3);
        } else if  i == 3 {
            assert_eq!(m.score(), 1 + 3);
        } else {
            assert_eq!(m.score(), 1);
        }

    }
}

#[test]
fn matching2chars() {
    let bts = "aoeu".as_bytes();
    for i in 0..bts.len() {
        for j in i + 1..bts.len() {
            let first_skip = if (j - i) > 1 { 1 } else { 0 };
            let start = if i==0 { 3 } else {0 };
            let end = if j==3 { 3 } else {0 };

            let mut m = smith_waterman::Matcher::from("aoeu", default_weights());
            println!("*** add {}", bts[i] as char);
            m.add_pchar(bts[i]);
            println!("add {}", bts[j] as char);
            m.add_pchar(bts[j]);
            println!(
                "i {}, j {}, fs {}, b {} {}",
                i,
                j,
                first_skip,
                start,
                end
            );
            assert_eq!(
                m.score(),
                (-5) * first_skip + 2 +  start + end
            );
        }
    }
}

#[test]
fn not_matching() {
    let mut m = smith_waterman::Matcher::from("aoeu", default_weights());
    m.add_pchar('x' as u8);
    println!("score {}", m.score());
    assert_eq!(m.score(), -10);
}

#[test]
fn not_greedy() {
    let weights = smith_waterman::MatcherWeights {
        gap_penalty : -5,
        pattern_skip_penalty : -10,
        first_letter_bonus : 3,
        match_bonus: 1
    };
    let mut m = smith_waterman::Matcher::from("anao", weights);
    for x in "ao".as_bytes() {
        m.add_pchar(*x);
    }
    assert_eq!(m.score(), 2 + 3);
}

#[test]
fn word_boundary() {
    let mut m = smith_waterman::Matcher::from("ht ao", default_weights());
    m.add_pchar('a' as u8);
    assert_eq!(m.score(), 3 + 1);
}

#[test]
fn remove_char() {
    let mut m = smith_waterman::Matcher::from("aoeu", default_weights());
    m.add_pchar('a' as u8);
    m.add_pchar('e' as u8);
    assert_eq!(m.score(), 3+1 + -5 + 1);
    m.remove_pchar();
    assert_eq!(m.score(), 3+1);
    m.add_pchar('o' as u8);
    assert_eq!(m.score(), 3+1+1);
}
