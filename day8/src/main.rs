use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    // input file format:
    // RL
    // 
    // AAA = (BBB, CCC)
    // BBB = (DDD, EEE)
    // CCC = (ZZZ, GGG)
    // DDD = (DDD, DDD)
    // EEE = (EEE, EEE)
    // GGG = (GGG, GGG)
    // ZZZ = (ZZZ, ZZZ)
    let left_right_instruction: &str = lines[0].trim_end_matches("\r");
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines[2..].iter() {
        // Split into {"AAA": ("BBB", "CCC")}
        let mut split_line = line.split(" = ");
        let left = split_line.next().unwrap();
        let right = split_line.next().unwrap();
        let mut split_right = right.split(", ");
        let right_left = split_right.next().unwrap();
        let right_left = right_left.trim_start_matches("(");
        let right_right = split_right.next().unwrap().trim_end_matches("\r");
        let right_right = right_right.trim_end_matches(")");
        nodes.insert(left, (right_left, right_right));        
    }
    // AAA
    let start_node_key = "AAA";
    let start_node = nodes.get(start_node_key);
    let mut i: i32 = 0;
    let mut current_node_key = start_node_key;
    let mut current_node = start_node;
    // Until we reach 'ZZZ' key(not value)
    while current_node_key != "ZZZ" {
        for left_right in left_right_instruction.chars() {
            if left_right == 'R' {
                current_node_key = current_node.unwrap().1;
                current_node = nodes.get(current_node_key);
            } else if left_right == 'L' {
                current_node_key = current_node.unwrap().0;
                current_node = nodes.get(current_node_key);
            }
        i += 1;
        }
        if i % 1000 == 0 {
            println!("{} steps", i);
        }
    }
    println!("{} steps", i);
    fs::write(output_file, i.to_string()).expect("Unable to write file");
}
