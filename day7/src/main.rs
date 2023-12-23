use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut hand_bid_pairs: Vec<(&str, i32)> = Vec::new();
    for line in lines {
        let mut split_line: Vec<&str> = line.split(" ").collect();
        split_line[1] = split_line[1].trim_end_matches('\r');
        hand_bid_pairs.push((split_line[0], split_line[1].parse::<i32>().unwrap()));
    }
    
    let mut singles: Vec<(&str, i32)> = Vec::new();
    let mut pairs: Vec<(&str, i32)> = Vec::new();
    let mut two_pairs: Vec<(&str, i32)> = Vec::new();
    let mut triples: Vec<(&str, i32)> = Vec::new();
    let mut full_houses: Vec<(&str, i32)> = Vec::new();
    let mut quads: Vec<(&str, i32)> = Vec::new();
    let mut fives: Vec<(&str, i32)> = Vec::new();
    for (hand, bid) in hand_bid_pairs {
        let mut char_count: Vec<(char, i32)> = Vec::new();
        for c in hand.chars() {
            let mut found = false;
            for (char, count) in &mut char_count {
                if *char == c {
                    *count += 1;
                    found = true;
                    break;
                }
            }
            if !found {
                char_count.push((c, 1));
            }
        }
        char_count.sort_by(|a, b| b.1.cmp(&a.1));
        if char_count[0].1 == 1 {
            singles.push((hand, bid));
        } else if char_count[0].1 == 5 {
            fives.push((hand, bid));
        } else if char_count[0].1 == 4 {
            quads.push((hand, bid));
        } else if char_count[0].1 == 3 {
            if char_count[1].1 == 2 {
                full_houses.push((hand, bid));
            } else {
                triples.push((hand, bid));
            }
        } else if char_count[0].1 == 2 {
            if char_count[1].1 == 2 {
                two_pairs.push((hand, bid));
            } else {
                pairs.push((hand, bid));
            }
        }
    }

    let order = "AKQJT98765432";
    let mut groups = vec![&mut fives, &mut quads, &mut full_houses, &mut triples, &mut two_pairs, &mut pairs, &mut singles];
    for group in groups.iter_mut() {
        group.sort_by(|a, b| {
            let mut a_chars = a.0.chars();
            let mut b_chars = b.0.chars();
            loop {
                let a_char = a_chars.next().unwrap();
                let b_char = b_chars.next().unwrap();
                if a_char == b_char {
                    continue;
                }
                let a_index = order.find(a_char).unwrap();
                let b_index = order.find(b_char).unwrap();
                return a_index.cmp(&b_index);
            }
        });
    }

    let mut value = 0;
    let mut i = 1;
    for group in groups.iter().rev() {
        for (hand, bid) in group.iter().rev() {
            println!("{}: {}", hand, bid);
            value += bid * i;
            i += 1;
        }
    }
    println!("Total value: {}", value);
    fs::write(output_file, value.to_string()).expect("Unable to write file");
}