use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut score: i32 = 0;
    let mut cards: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        let winning_numbers: Vec<i32> = parts[0].split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect();
        let drawn_numbers: Vec<i32> = parts[1].split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect();
        cards.push((winning_numbers, drawn_numbers));
    }
    for card in cards {
        let mut score_card: i32 = 0;
        for number in card.0 {
            if card.1.contains(&number) {
                score_card += 1;
            }
        }
        if score_card>0 {
            score += 1*2i32.pow((score_card-1).try_into().unwrap());
        }
    }
    fs::write(output_file, score.to_string()).expect("Unable to write file");
}
