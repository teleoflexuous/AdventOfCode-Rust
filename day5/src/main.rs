use std::env;
use std::fs;

// Part 2 solution
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    // seeds: 79 14 55 13
    let first_line = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1];
    let mut seeds: Vec<(i128, i128)> = Vec::new();
    // seeds: (79, 79+14=93), (55, 55+13=68)
    // Iterate over every second element in first_line
    for (index, value) in first_line.split(" ").enumerate() {
        if index % 2 == 0 {
            // Push a tuple of (value, value + next_value)
            seeds.push((
                value.parse::<i128>().unwrap(),
                first_line.split(" ").collect::<Vec<&str>>()[index + 1]
                    .parse::<i128>()
                    .unwrap()
                    + value.parse::<i128>().unwrap(),
            ));
        }
    }
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
    let mut cases_checked: Vec<&str> = Vec::new();
    for mapping in mappings {
        let mut total_range_mapped: i128 = 0;
        current_values.sort_by(|a, b| a.0.cmp(&b.0));
        println!("Current values: {:?}", current_values);
        let mut new_ranges: Vec<(i128, i128)> = Vec::new();
        println!("New ranges: {:?}", new_ranges);
        for values_range in current_values.clone() {
            println!("Values range: {:?}", values_range);
            let value_start_point = values_range.0;
            let value_end_point = values_range.1;
            // Sort mappings by 2nd value
            let mut sorted_mapping = mapping.clone();
            sorted_mapping.sort_by(|a, b| a[1].cmp(&b[1]));
            println!("Sorted mapping: {:?}", sorted_mapping);
            let mut range_mapped: i128 = 0;
            for (index, single_mapping) in sorted_mapping.iter().enumerate() {
                let current_start_point = value_start_point + range_mapped;
                let mapping_start_point = single_mapping[1];
                let mapping_length = single_mapping[2];
                let mapping_end_point = mapping_start_point + mapping_length;
                let mapping_destination_point = single_mapping[0];

                println!("Current start point: {}", current_start_point);

                let mut case: &str = "";
                // Case 1: Range is to the left of mapping
                if value_end_point < mapping_start_point {
                    case = "1";
                    if !cases_checked.contains(&case) {
                        println!("CHECK HERE")
                    }
                    cases_checked.push(case);
                    new_ranges.push((current_start_point, value_end_point));
                    range_mapped += value_end_point - current_start_point;
                    println!("Value start point: {}, value end point: {}, mapping start point: {}, mapping end point: {}, mapping length: {}, mapping destination point: {}, current start point: {}, case: {}", value_start_point, value_end_point, mapping_start_point, mapping_end_point, mapping_length, mapping_destination_point, current_start_point, case);
                    println!(
                        "Case no. {}, it was {}last mapping",
                        case,
                        if index == sorted_mapping.len() - 1 {
                            ""
                        } else {
                            "not "
                        }
                    );
                    println!("New ranges in inner loop: {:?}", new_ranges);
                    break;
                }
                // Case 2: Range starts to the left of mapping, but ends inside mapping
                else if current_start_point < mapping_start_point
                    && value_end_point <= mapping_end_point
                {
                    case = "2";
                    if !cases_checked.contains(&case) {
                        println!("CHECK HERE")
                    }
                    cases_checked.push(case);
                    new_ranges.push((current_start_point, mapping_start_point));
                    let mapped_length = value_end_point - mapping_start_point;
                    new_ranges.push((
                        mapping_destination_point,
                        mapping_destination_point + mapped_length,
                    ));
                    range_mapped += value_end_point - current_start_point;
                    println!("Value start point: {}, value end point: {}, mapping start point: {}, mapping end point: {}, mapping length: {}, mapping destination point: {}, current start point: {}, case: {}", value_start_point, value_end_point, mapping_start_point, mapping_end_point, mapping_length, mapping_destination_point, current_start_point, case);
                    println!(
                        "Case no. {}, it was {}last mapping",
                        case,
                        if index == sorted_mapping.len() - 1 {
                            ""
                        } else {
                            "not "
                        }
                    );
                    println!("New ranges in inner loop: {:?}", new_ranges);
                    break;
                }
                // Case 3: Range starts to the left of mapping, but ends to the right of mapping
                else if current_start_point < mapping_start_point
                    && value_end_point > mapping_end_point
                {
                    case = "3";
                    if index != sorted_mapping.len() - 1 {
                        if !cases_checked.contains(&case) {
                            println!("CHECK HERE")
                        }
                        cases_checked.push(case);
                    }
                    new_ranges.push((current_start_point, mapping_start_point));
                    new_ranges.push((
                        mapping_destination_point,
                        mapping_destination_point + mapping_length,
                    ));
                    range_mapped += mapping_length;
                    if index == sorted_mapping.len() - 1 {
                        case = "3.1";
                        if !cases_checked.contains(&case) {
                            println!("CHECK HERE")
                        }
                        cases_checked.push(case);
                        new_ranges.push((mapping_start_point + mapping_length, value_end_point));
                        range_mapped += value_end_point - mapping_start_point - mapping_length;
                    }
                }
                // Case 4: Range starts inside mapping, but ends inside mapping
                else if current_start_point >= mapping_start_point
                    && value_end_point <= mapping_end_point
                    && value_end_point >= mapping_start_point
                {
                    case = "4";
                    if index != sorted_mapping.len() - 1 {
                        if !cases_checked.contains(&case) {
                            println!("CHECK HERE")
                        }
                        cases_checked.push(case);
                    }
                    let current_mapping_start_point =
                        mapping_destination_point + current_start_point - mapping_start_point;
                    new_ranges.push((
                        current_mapping_start_point,
                        current_mapping_start_point + value_end_point - current_start_point,
                    ));
                    range_mapped += value_end_point - current_start_point;
                    println!("Value start point: {}, value end point: {}, mapping start point: {}, mapping end point: {}, mapping length: {}, mapping destination point: {}, current start point: {}, case: {}", value_start_point, value_end_point, mapping_start_point, mapping_end_point, mapping_length, mapping_destination_point, current_start_point, case);
                    println!(
                        "Case no. {}, it was {}last mapping",
                        case,
                        if index == sorted_mapping.len() - 1 {
                            ""
                        } else {
                            "not "
                        }
                    );
                    println!("New ranges in inner loop: {:?}", new_ranges);
                    break;
                }
                // Case 5: Range starts inside mapping, but ends to the right of mapping
                else if current_start_point >= mapping_start_point
                    && value_end_point > mapping_end_point
                    && current_start_point <= mapping_end_point
                {
                    case = "5";
                    if index != sorted_mapping.len() - 1 {
                        if !cases_checked.contains(&case) {
                            println!("CHECK HERE")
                        }
                        cases_checked.push(case);
                    }
                    let current_mapping_start_point =
                        mapping_destination_point + current_start_point - mapping_start_point;
                    let length_inside_mapping = mapping_end_point - current_start_point;
                    new_ranges.push((
                        current_mapping_start_point,
                        current_mapping_start_point + length_inside_mapping,
                    ));
                    range_mapped += (current_mapping_start_point + length_inside_mapping)
                        - current_mapping_start_point;
                    println!("range mapped: {}, current_mapping_start_point: {}, length_inside_mapping: {}", range_mapped, current_mapping_start_point, length_inside_mapping);
                    if index == sorted_mapping.len() - 1 {
                        case = "5.1";
                        if !cases_checked.contains(&case) {
                            println!("CHECK HERE")
                        }
                        new_ranges.push((current_start_point + mapping_length, value_end_point));
                        range_mapped += value_end_point - current_start_point - mapping_length;
                    }
                }
                // Case 6: Range is to the right of mapping
                else if current_start_point > mapping_end_point {
                    case = "6";
                    if index != sorted_mapping.len() - 1 {
                        if !cases_checked.contains(&case) {
                            println!("CHECK HERE")
                        }
                        cases_checked.push(case);
                    }
                    if index == sorted_mapping.len() - 1 {
                        case = "6.1";
                        if !cases_checked.contains(&case) {
                            println!("CHECK HERE")
                        }
                        cases_checked.push(case);
                        new_ranges.push((current_start_point, value_end_point));
                        range_mapped += value_end_point - current_start_point;
                    }
                }
                // Check if case was checked
                println!("Value start point: {}, value end point: {}, mapping start point: {}, mapping end point: {}, mapping length: {}, mapping destination point: {}, current start point: {}, case: {}", value_start_point, value_end_point, mapping_start_point, mapping_end_point, mapping_length, mapping_destination_point, current_start_point, case);
                println!(
                    "Case no. {}, it was {}last mapping",
                    case,
                    if index == sorted_mapping.len() - 1 {
                        ""
                    } else {
                        "not "
                    }
                );
                println!("New ranges in inner loop: {:?}", new_ranges);
            }
            println!("Range mapped: {}", range_mapped);
            total_range_mapped += range_mapped;
        }
        println!("Total range mapped: {}", total_range_mapped);
        println!("New ranges: {:?}", new_ranges);
        current_values = new_ranges.clone();
        // Wait for input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // break;
    }
    // println!("Current values: {:?}", current_values);
    // Find the lowest value among first values of the pairs
    let lowest_value = current_values.iter().map(|x| x.0).min().unwrap();
    fs::write(output_file, lowest_value.to_string()).expect("Unable to write file");
}

// Part 1 solution
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let input_file = &args[1];
//     let output_file = &args[2];
//     let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
//     let mut lines = contents.lines();
//     let first_line = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1];
//     let seeds = first_line
//         .split(" ")
//         .map(|x| x.parse::<i128>().unwrap())
//         .collect::<Vec<i128>>();

//     let mut current_values = seeds.clone();

//     let mut mappings: Vec<Vec<Vec<i128>>> = Vec::new();
//     let mut current_mapping: Vec<Vec<i128>> = Vec::new();
//     for line in lines.skip(1) {
//         if line.chars().any(|c| c.is_alphabetic()) {
//             mappings.push(current_mapping.clone());
//             current_mapping = Vec::new();
//             continue;
//         } else if line.is_empty() {
//             continue;
//         } else {
//             let values = line
//                 .split(" ")
//                 .map(|x| x.parse::<i128>().unwrap())
//                 .collect::<Vec<i128>>();
//             current_mapping.push(values.clone());
//         }
//     }

//     mappings.push(current_mapping.clone());
//     mappings.remove(0);
//     for mapping in mappings {
//         let mut new_values = vec![-1; current_values.len()];
//         for single_mapping in mapping {
//             for (index, value) in current_values.iter().enumerate() {
//                 if new_values[index] != -1 {
//                     continue;
//                 }
//                 if &single_mapping[1] <= value && value <= &(single_mapping[1] + single_mapping[2])
//                 {
//                     new_values[index] = single_mapping[0] + value - single_mapping[1];
//                 }
//             }
//         }
//         for (index, value) in new_values.clone().iter().enumerate() {
//             if *value == -1 {
//                 new_values[index] = current_values[index];
//             }
//         }
//         current_values = new_values;
//     }
//     let lowest_value = current_values.iter().min().unwrap();
//     fs::write(output_file, lowest_value.to_string()).expect("Unable to write file");
// }
