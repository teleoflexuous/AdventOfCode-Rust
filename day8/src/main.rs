use std::collections::HashMap;
use std::env;
use std::fs;

// Part 2

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let left_right_instruction: &str = lines[0].trim_end_matches("\r");
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines[2..].iter() {
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
    let start_nodes: HashMap<&str, (&str, &str)> = nodes
        .iter()
        .filter(|(key, _)| key.ends_with("A"))
        .map(|(key, value)| (*key, *value))
        .collect();

    let mut paths: Vec<i32> = Vec::new();

    for (node_key, node) in start_nodes.iter() {
        println!("Starting at node: {}", node_key);
        let mut i: i32 = 0;
        let mut current_node_key = node_key;
        let mut current_node = nodes.get(current_node_key);
        while !current_node_key.ends_with("Z") {
            for left_right in left_right_instruction.chars() {
                if left_right == 'L' {
                    current_node_key = &current_node.unwrap().0;
                    current_node = nodes.get(current_node_key);

                } else if left_right == 'R' {
                    current_node_key = &current_node.unwrap().1;
                    current_node = nodes.get(current_node_key);
                }
                i += 1;
            }
        }
        paths.push(i);
    }
    let mut output = String::new();
    for path in paths.iter() {
        output.push_str(&path.to_string());
        output.push_str("\n");
    }
    fs::write(output_file, output).expect("Unable to write file");
    // Calculate LCM of all paths. It doesn't have to work, but it does.
}

// Part 1
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let input_file = &args[1];
//     let output_file = &args[2];
//     let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
//     let lines: Vec<&str> = contents.split("\n").collect();

//     let left_right_instruction: &str = lines[0].trim_end_matches("\r");
//     let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
//     for line in lines[2..].iter() {
//         let mut split_line = line.split(" = ");
//         let left = split_line.next().unwrap();
//         let right = split_line.next().unwrap();
//         let mut split_right = right.split(", ");
//         let right_left = split_right.next().unwrap();
//         let right_left = right_left.trim_start_matches("(");
//         let right_right = split_right.next().unwrap().trim_end_matches("\r");
//         let right_right = right_right.trim_end_matches(")");
//         nodes.insert(left, (right_left, right_right));
//     }
//     let start_node_key = "AAA";
//     let start_node = nodes.get(start_node_key);
//     let mut i: i32 = 0;
//     let mut current_node_key = start_node_key;
//     let mut current_node = start_node;

//     while current_node_key != "ZZZ" {
//         for left_right in left_right_instruction.chars() {
//             if left_right == 'R' {
//                 current_node_key = current_node.unwrap().1;
//                 current_node = nodes.get(current_node_key);
//             } else if left_right == 'L' {
//                 current_node_key = current_node.unwrap().0;
//                 current_node = nodes.get(current_node_key);
//             }
//         i += 1;
//         }
//         if i % 1000 == 0 {
//             println!("{} steps", i);
//         }
//     }
//     println!("{} steps", i);
//     fs::write(output_file, i.to_string()).expect("Unable to write file");
// }
