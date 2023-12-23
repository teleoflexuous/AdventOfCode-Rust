use std::env;
use std::fs;

// Part 2

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    // Lines:
    // Time:      7  15   30
    // Distance:  9  40  200
    let time: i128 = lines[0].split(":").collect::<Vec<&str>>()[1]
        .chars()
        .filter_map(|x| x.to_digit(10))
        .fold(0, |acc, x| acc * 10 + x as i128);
    let distance: i128 = lines[1].split(":").collect::<Vec<&str>>()[1]
        .chars()
        .filter_map(|x| x.to_digit(10))
        .fold(0, |acc, x| acc * 10 + x as i128);

    let mut answer: i128 = 0;
    for miliseconds_count in 0..time {
        let speed = miliseconds_count;
        let possible_distance = speed * (time - miliseconds_count);
        if possible_distance > distance {
            answer+=1;
        }
    }
    let answer_string = answer.to_string();
    fs::write(output_file, answer_string).expect("Something went wrong writing the file");

}

// Part 1

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let input_file = &args[1];
//     let output_file = &args[2];
//     let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
//     let lines: Vec<&str> = contents.split("\n").collect();
//     let times: Vec<i32> = lines[0]
//         .split_whitespace()
//         .filter_map(|x| x.parse::<i32>().ok())
//         .collect();
//     let distances: Vec<i32> = lines[1]
//         .split_whitespace()
//         .filter_map(|x| x.parse::<i32>().ok())
//         .collect();
//     let mut time_distance_pair: Vec<(i32, i32)> = Vec::new();
//     for i in 0..times.len() {
//         time_distance_pair.push((times[i], distances[i]));
//     }
//     let mut answers: Vec<Vec<i32>> = Vec::new();
//     for (race_time, top_distance) in time_distance_pair {
//         let mut winning_waits: Vec<i32> = Vec::new();
//         for miliseconds_count in 0..race_time {
//             let speed = miliseconds_count;
//             let possible_distance = speed * (race_time-miliseconds_count);
//             if possible_distance > top_distance {
//                 winning_waits.push(miliseconds_count);
//             }
//         }
//         answers.push(winning_waits);
//     }
//     let mut answer: i32 = 1;
//     for answer_set in answers {
//         answer *= answer_set.len() as i32;
//     }
//     let answer_string = answer.to_string();
//     fs::write(output_file, answer_string).expect("Something went wrong writing the file");
// }
