use std::env;
use std::fs;

// Part 1 solution
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut numbers: Vec<i32> = Vec::new();
    let mut char_matrix: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let mut char_line: Vec<char> = Vec::new();
        for char in line.chars() {
            char_line.push(char);
        }
        char_matrix.push(char_line);
    }
    for (line_index, line) in char_matrix.iter().enumerate() {
        for (char_index, char) in line.iter().enumerate() {
            if char.is_numeric() || char == &'.' {
                continue;
            } else {
                // Look for numeric characters in the surrounding 8 squares
                let mut checked_squares: Vec<(usize, usize)> = Vec::new();
                for i in 0..3 {
                    let square_line_index: usize = line_index + i - 1 as usize;
                    let line: Vec<char> = char_matrix[square_line_index].clone();
                    for j in 0..3{
                        let square_char_index: usize = char_index + j - 1 as usize;
                        if checked_squares.contains(&(square_line_index, square_char_index)) {
                            continue;
                        } else {
                            checked_squares.push((square_line_index, square_char_index));
                        }
                        let char = line[square_char_index];
                        if char.is_numeric() {
                            let mut number: String = String::new();
                            let mut left_char_index: usize = square_char_index;
                            let mut right_char_index: usize = square_char_index;
                            while left_char_index > 0 && line[left_char_index - 1].is_numeric() {
                                left_char_index -= 1;
                                checked_squares.push((square_line_index, left_char_index));
                            }
                            while right_char_index < line.len() - 1 && line[right_char_index + 1].is_numeric() {
                                right_char_index += 1;
                                checked_squares.push((square_line_index, right_char_index));
                            }
                            for i in left_char_index..right_char_index + 1 {
                                number.push(line[i]);
                            }
                            println!("Found number: {}", number);
                            numbers.push(number.parse::<i32>().unwrap());
                        }
                    }
                    println!("Checked squares: {:?}", checked_squares)
                }
            }
        }
    }
    let mut answer: i32 = 0;
    for number in numbers {
        answer += number;
    }
    fs::write(output_file, answer.to_string()).expect("Something went wrong writing the file");
}
