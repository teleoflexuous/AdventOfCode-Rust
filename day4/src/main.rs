use std::collections::HashMap;
use std::env;
use std::fs;

// Part 2 solution

fn check_cards(scores: &mut HashMap<i32, i32>, owned_cards: &mut Vec<i32>) -> u128 {
    let mut cards_count: u128 = owned_cards.len().try_into().unwrap();
    let mut new_owned_cards: Vec<i32> = Vec::new();
    for card in owned_cards {
        let mut score_card: i32 = 0;
        for i in 1..=*scores.get(card).unwrap() {
            new_owned_cards.push(*card + i);
        }
    }
    if new_owned_cards.len() == 0 {
        return cards_count;
    }
    cards_count += check_cards(scores, &mut new_owned_cards);
    cards_count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut scores: HashMap<i32, i32> = HashMap::new();
    let mut owned_cards: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        let card_number: i32 = parts[0].split(':').collect::<Vec<&str>>()[0]
            .split_whitespace()
            .collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let winning_numbers: Vec<i32> = parts[0]
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        let drawn_numbers: Vec<i32> = parts[1]
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        scores.insert(card_number, 0);
        for number in winning_numbers {
            if drawn_numbers.contains(&number) {
                *scores.get_mut(&card_number).unwrap() += 1;
            }
        }
        owned_cards.push(card_number);
    }
    let answer = check_cards(&mut scores, &mut owned_cards);
    fs::write(output_file, answer.to_string()).expect("Unable to write file");
}

// Part 1 solution
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let input_file = &args[1];
//     let output_file = &args[2];
//     let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
//     let mut score: i32 = 0;
//     let mut cards: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();
//     for line in contents.lines() {
//         let parts: Vec<&str> = line.split('|').collect();
//         let winning_numbers: Vec<i32> = parts[0].split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect();
//         let drawn_numbers: Vec<i32> = parts[1].split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect();
//         cards.push((winning_numbers, drawn_numbers));
//     }
//     for card in cards {
//         let mut score_card: i32 = 0;
//         for number in card.0 {
//             if card.1.contains(&number) {
//                 score_card += 1;
//             }
//         }
//         if score_card>0 {
//             score += 1*2i32.pow((score_card-1).try_into().unwrap());
//         }
//     }
//     fs::write(output_file, score.to_string()).expect("Unable to write file");
// }
