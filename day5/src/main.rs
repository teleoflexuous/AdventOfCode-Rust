use std::env;
use std::fs;


// Part 1 solution
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let first_line = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1];
    let seeds = first_line
        .split(" ")
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let mut current_values = seeds.clone();

    let mut mappings: Vec<Vec<Vec<i128>>> = Vec::new();
    let mut current_mapping: Vec<Vec<i128>> = Vec::new();
    for line in lines.skip(1) {
        if line.chars().any(|c| c.is_alphabetic()) {
            mappings.push(current_mapping.clone());
            current_mapping = Vec::new();
            continue;
        } else if line.is_empty() {
            continue;
        } else {
            let values = line
                .split(" ")
                .map(|x| x.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            current_mapping.push(values.clone());
        }
    }

    mappings.push(current_mapping.clone());
    mappings.remove(0);
    for mapping in mappings {
        let mut new_values = vec![-1; current_values.len()];
        for single_mapping in mapping {
            for (index, value) in current_values.iter().enumerate() {
                if new_values[index] != -1 {
                    continue;
                }
                if &single_mapping[1] <= value && value <= &(single_mapping[1] + single_mapping[2])
                {
                    new_values[index] = single_mapping[0] + value - single_mapping[1];
                }
            }
        }
        for (index, value) in new_values.clone().iter().enumerate() {
            if *value == -1 {
                new_values[index] = current_values[index];
            }
        }
        current_values = new_values;
    }
    let lowest_value = current_values.iter().min().unwrap();
    fs::write(output_file, lowest_value.to_string()).expect("Unable to write file");
}
