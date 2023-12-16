use std::env;
use std::fs;

struct Game {
    number: i32,
    green: i32,
    blue: i32,
    red: i32,
}


// Part 2 solution

// Part 1 solution
fn main() {
    let valid_game: Game = Game {
        number: 0,
        red: 12,
        green: 13,
        blue: 14,
    };
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut valid_games = Vec::new();
    for line in contents.lines() {
        // Split line on :
        // Remove 5 first characters from the first part
        // Set game_number to the leftover of the first part
        let parts: Vec<&str> = line.split(": ").collect();
        let game_number = parts[0][5..].parse::<i32>().unwrap();
        // Split second part on "; " into as many draws, as there are
        // For each draw, split on ", "
        // For each color, check if it is valid
        let draws: Vec<&str> = parts[1].split("; ").collect();
        println!("Game {} has {} draws", game_number, draws.len());
        let mut valid = true;
        'draw_loop: for draw in draws {
            let draw_parts: Vec<&str> = draw.split(", ").collect();
            for part in draw_parts.iter() {
                let cube: Vec<&str> = part.split(" ").collect();
                let color = cube[1].chars().last().unwrap();
                let number = cube[0].parse::<i32>().unwrap();
                match color {
                    'n' => {
                        if number > valid_game.green {
                            println!(
                                "Game {} is invalid, because of a green cubes count: {}",
                                game_number, number
                            );
                            valid = false;
                            break 'draw_loop;
                        }
                    }
                    'e' => {
                        if number > valid_game.blue {
                            println!(
                                "Game {} is invalid, because of a blue cubes count: {}",
                                game_number, number
                            );
                            valid = false;
                            break 'draw_loop;
                        }
                    }
                    'd' => {
                        if number > valid_game.red {
                            println!(
                                "Game {} is invalid, because of a red cubes count: {}",
                                game_number, number
                            );
                            valid = false;
                            break 'draw_loop;
                        }
                    }
                    _ => println!("Something went wrong with the color"),
                }
            }
        }
        println!("Game {} is valid", game_number);
        if valid {
            valid_games.push(game_number);
        }
    }
    // Add all valid games numbers together
    println!("Valid games: {:?}", valid_games);
    let answer: i32 = valid_games.iter().sum();
    fs::write(output_file, answer.to_string()).expect("Something went wrong writing the file");
}
