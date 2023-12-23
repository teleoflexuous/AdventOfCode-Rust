use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs;

// Part 2

fn joker_friendly_get_hand_type<'a>(
    hand: &'a str,
    hand_type_order: &Vec<&'a str>,
    card_order: &'a str,
) -> &'a str {
    let mut best_hand_type = "singles";
    let mut best_hand: String = String::new();
    let mut joker_count = 0;
    for c in hand.chars() {
        if c == 'J' {
            joker_count += 1;
        }
    }
    println!("Hand: {}, joker_count: {}", hand, joker_count);
    let mut hand = hand.replace("J", "");
    for _ in 0..joker_count {
        hand.push('J');
    }
    let mut replacements: Vec<String> = Vec::new();

    for perm in card_order.chars().combinations_with_replacement(joker_count) {
        let mut replacement = String::new();
        for c in perm {
            replacement.push(c);
        }
        replacements.push(replacement);
    }
    
    println!("Replacements: {:?}", replacements);
    for replacement in replacements.iter() {
        for combo in replacement.split(",") {
            let mut new_hand = hand.clone();
            for _ in 0..joker_count {
                new_hand.pop();
            }
            new_hand.push_str(combo);
            let hand_type = get_hand_type(new_hand.clone());
            if hand_type_order
                .iter()
                .position(|&r| r == hand_type)
                .unwrap()
                < hand_type_order
                    .iter()
                    .position(|&r| r == best_hand_type)
                    .unwrap()
            {
                best_hand_type = hand_type;
                println!("New best hand: {}, best hand type: {}", new_hand, best_hand_type);
                best_hand = new_hand;
            }
        }
    }
    println!("Original hand: {}, best hand: {}, best hand type: {}", hand, best_hand, best_hand_type);
    return best_hand_type;
}

fn get_hand_type(hand: String) -> &'static str {
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
        return "singles";
    } else if char_count[0].1 == 5 {
        return "fives";
    } else if char_count[0].1 == 4 {
        return "quads";
    } else if char_count[0].1 == 3 {
        if char_count[1].1 == 2 {
            return "full_houses";
        } else {
            return "triples";
        }
    } else if char_count[0].1 == 2 {
        if char_count[1].1 == 2 {
            return "two_pairs";
        } else {
            return "pairs";
        }
    }
    panic!("Invalid hand type");
}

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
    let card_order = "AKQT98765432J";
    let hand_type_order = vec![
        "fives",
        "quads",
        "full_houses",
        "triples",
        "two_pairs",
        "pairs",
        "singles",
    ];

    for (hand, bid) in hand_bid_pairs {
        let hand_type = joker_friendly_get_hand_type(hand, &hand_type_order, card_order);
        match hand_type {
            "singles" => singles.push((hand, bid)),
            "fives" => fives.push((hand, bid)),
            "quads" => quads.push((hand, bid)),
            "full_houses" => full_houses.push((hand, bid)),
            "triples" => triples.push((hand, bid)),
            "two_pairs" => two_pairs.push((hand, bid)),
            "pairs" => pairs.push((hand, bid)),
            _ => panic!("Invalid hand type"),
        }
        println!("{}: {}", hand, hand_type);
    }
    let mut groups: HashMap<&str, Vec<(&str, i32)>> = HashMap::new();
    groups.insert("fives", fives);
    groups.insert("quads", quads);
    groups.insert("full_houses", full_houses);
    groups.insert("triples", triples);
    groups.insert("two_pairs", two_pairs);
    groups.insert("pairs", pairs);
    groups.insert("singles", singles);

    for (_hand_type, group) in groups.iter_mut() {
        group.sort_by(|a, b| {
            let mut a_chars = a.0.chars();
            let mut b_chars = b.0.chars();
            loop {
                let a_char = a_chars.next().unwrap();
                let b_char = b_chars.next().unwrap();
                if a_char == b_char {
                    continue;
                }
                let a_index = card_order.find(a_char).unwrap();
                let b_index = card_order.find(b_char).unwrap();
                return a_index.cmp(&b_index);
            }
        });
    }
    let mut sorted_groups: Vec<(&str, i32)> = Vec::new();
    for hand_type in hand_type_order {
        let group = groups.get(hand_type).unwrap();
        for (hand, bid) in group {
            sorted_groups.push((hand, *bid));
        }
    }
    let mut value = 0;
    let mut i = 1;
    println!("Sorted groups: {:?}\n", sorted_groups);
    for (hand, bid) in sorted_groups.iter().rev() {
        value += bid * i;
        i += 1;
    }
    println!("Total value: {}", value);
    fs::write(output_file, value.to_string()).expect("Unable to write file");
}

// Part 1
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let input_file = &args[1];
//     let output_file = &args[2];
//     let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
//     let lines: Vec<&str> = contents.split("\n").collect();

//     let mut hand_bid_pairs: Vec<(&str, i32)> = Vec::new();
//     for line in lines {
//         let mut split_line: Vec<&str> = line.split(" ").collect();
//         split_line[1] = split_line[1].trim_end_matches('\r');
//         hand_bid_pairs.push((split_line[0], split_line[1].parse::<i32>().unwrap()));
//     }

//     let mut singles: Vec<(&str, i32)> = Vec::new();
//     let mut pairs: Vec<(&str, i32)> = Vec::new();
//     let mut two_pairs: Vec<(&str, i32)> = Vec::new();
//     let mut triples: Vec<(&str, i32)> = Vec::new();
//     let mut full_houses: Vec<(&str, i32)> = Vec::new();
//     let mut quads: Vec<(&str, i32)> = Vec::new();
//     let mut fives: Vec<(&str, i32)> = Vec::new();
//     for (hand, bid) in hand_bid_pairs {
//         let mut char_count: Vec<(char, i32)> = Vec::new();
//         for c in hand.chars() {
//             let mut found = false;
//             for (char, count) in &mut char_count {
//                 if *char == c {
//                     *count += 1;
//                     found = true;
//                     break;
//                 }
//             }
//             if !found {
//                 char_count.push((c, 1));
//             }
//         }
//         char_count.sort_by(|a, b| b.1.cmp(&a.1));
//         if char_count[0].1 == 1 {
//             singles.push((hand, bid));
//         } else if char_count[0].1 == 5 {
//             fives.push((hand, bid));
//         } else if char_count[0].1 == 4 {
//             quads.push((hand, bid));
//         } else if char_count[0].1 == 3 {
//             if char_count[1].1 == 2 {
//                 full_houses.push((hand, bid));
//             } else {
//                 triples.push((hand, bid));
//             }
//         } else if char_count[0].1 == 2 {
//             if char_count[1].1 == 2 {
//                 two_pairs.push((hand, bid));
//             } else {
//                 pairs.push((hand, bid));
//             }
//         }
//     }

//     let order = "AKQJT98765432";
//     let mut groups = vec![&mut fives, &mut quads, &mut full_houses, &mut triples, &mut two_pairs, &mut pairs, &mut singles];
//     for group in groups.iter_mut() {
//         group.sort_by(|a, b| {
//             let mut a_chars = a.0.chars();
//             let mut b_chars = b.0.chars();
//             loop {
//                 let a_char = a_chars.next().unwrap();
//                 let b_char = b_chars.next().unwrap();
//                 if a_char == b_char {
//                     continue;
//                 }
//                 let a_index = order.find(a_char).unwrap();
//                 let b_index = order.find(b_char).unwrap();
//                 return a_index.cmp(&b_index);
//             }
//         });
//     }

//     let mut value = 0;
//     let mut i = 1;
//     for group in groups.iter().rev() {
//         for (hand, bid) in group.iter().rev() {
//             println!("{}: {}", hand, bid);
//             value += bid * i;
//             i += 1;
//         }
//     }
//     println!("Total value: {}", value);
//     fs::write(output_file, value.to_string()).expect("Unable to write file");
// }
