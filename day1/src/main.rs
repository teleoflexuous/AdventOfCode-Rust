use std::env;
use std::fs;

// Part 2 solution

fn check_line(line: &str) -> i32 {
    let spelled_numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut partial_answer: String = String::new();
    let mut answer: i32 = 0;
    'line_loop: for (char_index, char) in line.chars().enumerate() {
        if char.is_numeric() {
            partial_answer.push(char);
            break 'line_loop;
        }
        for (i, spelled_number) in spelled_numbers.iter().enumerate() {
            if char == spelled_number.chars().nth(0).unwrap() && char_index + spelled_number.len() <= line.len() {
                if &&line[char_index..char_index + spelled_number.len()] == spelled_number {
                    partial_answer.push_str(&(i + 1).to_string());
                    break 'line_loop;
                }
            }
        }
    }
    'reverse_line_loop: for (rev_char_index, char) in line.chars().rev().enumerate() {
        let char_index = line.len() - rev_char_index - 1;
        if char.is_numeric() {
            partial_answer.push(char);
            break 'reverse_line_loop;
        }
        for (i, spelled_number) in spelled_numbers.iter().enumerate() {
            if char == spelled_number.chars().nth(0).unwrap() && char_index + spelled_number.len() <= line.len() {
                if &&line[char_index..char_index + spelled_number.len()] == spelled_number {
                    partial_answer.push_str(&(i + 1).to_string());
                    break 'reverse_line_loop;
                }
            }
        }
    }
    answer += partial_answer.parse::<i32>().unwrap();
    return answer;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut answer: i32 = 0;
    for line in contents.lines() {
        answer += check_line(line);
    }
    fs::write(output_file, answer.to_string()).expect("Something went wrong writing the file");
}

// Part 1 solution
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let input_file = &args[1];
//     let output_file = &args[2];
//     let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
//     let mut answer: i32 = 0;
//     for line in contents.lines() {
//         let mut line_answer = String::new();
//         for char in line.chars() {
//             if char.is_numeric() {
//                 line_answer.push(char);
//                 break;
//             }
//         }
//         for char in line.chars().rev() {
//             if char.is_numeric() {
//                 line_answer.push(char);
//                 break;
//             }
//         }
//         answer += line_answer.parse::<i32>().unwrap();

//     }
//     fs::write(output_file, answer.to_string()).expect("Something went wrong writing the file");
// }
