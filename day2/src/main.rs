use std::env;
use std::fs;

struct Game {
    number: i32,
    green: i32,
    blue: i32,
    red: i32,
    power: i32,
    // Add method for calculating power(gree*red*blue)
}

impl Game {
    fn calculate_power(&mut self) {
        self.power = self.green * self.blue * self.red;
    }
}

// Part 2 solution
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let mut answer: i32 = 0;
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let draws: Vec<&str> = parts[1].split("; ").collect();
        let mut game: Game = Game {
            number: parts[0][5..].parse::<i32>().unwrap(),
            red: 0,
            green: 0,
            blue: 0,
            power: 0,
        };
        for draw in draws {
            let draw_parts: Vec<&str> = draw.split(", ").collect();
            for part in draw_parts.iter() {
                let cube: Vec<&str> = part.split(" ").collect();
                let color = cube[1].chars().last().unwrap();
                let number = cube[0].parse::<i32>().unwrap();
                match color {
                    'n' => {
                        if number > game.green {
                            game.green = number;
                        }
                    }
                    'e' => {
                        if number > game.blue {
                            game.blue = number;
                        }
                    }
                    'd' => {
                        if number > game.red {
                            game.red = number;
                        }
                    }
                    _ => println!("Something went wrong with the color"),
                }
            }
        }
        game.calculate_power();
        answer += game.power;
    }
    fs::write(output_file, answer.to_string()).expect("Something went wrong writing the file");
}


// Part 1 solution
// fn main() {
//     let valid_game: Game = Game {
//         number: 0,
//         red: 12,
//         green: 13,
//         blue: 14,
//     };
//     let args: Vec<String> = env::args().collect();
//     let input_file = &args[1];
//     let output_file = &args[2];
//     let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
//     let mut valid_games = Vec::new();
//     for line in contents.lines() {
//         let parts: Vec<&str> = line.split(": ").collect();
//         let game_number = parts[0][5..].parse::<i32>().unwrap();
//         let draws: Vec<&str> = parts[1].split("; ").collect();
//         println!("Game {} has {} draws", game_number, draws.len());
//         let mut valid = true;
//         'draw_loop: for draw in draws {
//             let draw_parts: Vec<&str> = draw.split(", ").collect();
//             for part in draw_parts.iter() {
//                 let cube: Vec<&str> = part.split(" ").collect();
//                 let color = cube[1].chars().last().unwrap();
//                 let number = cube[0].parse::<i32>().unwrap();
//                 match color {
//                     'n' => {
//                         if number > valid_game.green {
//                             valid = false;
//                             break 'draw_loop;
//                         }
//                     }
//                     'e' => {
//                         if number > valid_game.blue {
//                             valid = false;
//                             break 'draw_loop;
//                         }
//                     }
//                     'd' => {
//                         if number > valid_game.red {
//                             valid = false;
//                             break 'draw_loop;
//                         }
//                     }
//                     _ => println!("Something went wrong with the color"),
//                 }
//             }
//         }
//         println!("Game {} is valid", game_number);
//         if valid {
//             valid_games.push(game_number);
//         }
//     }
//     println!("Valid games: {:?}", valid_games);
//     let answer: i32 = valid_games.iter().sum();
//     fs::write(output_file, answer.to_string()).expect("Something went wrong writing the file");
// }
