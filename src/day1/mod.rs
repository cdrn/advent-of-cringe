use std::collections::{HashMap, HashSet};

use fancy_regex::Regex;

use crate::utils::read_file_to_string;

pub fn run() {
    let input_as_string = read_file_to_string("inputs/day1.txt").unwrap();
    let split_array_on_newline = input_as_string.split("\n");
    let mut all_numbers: Vec<i32> = Vec::new();
    for (_i, line) in split_array_on_newline.enumerate() {
        let (first_number, last_number) = extract_first_and_last_number_with_numerals(line);
        let full_number_as_string = format!("{}{}", first_number, last_number);
        println!("line: {}", line);

        let full_number_as_int: i32 = full_number_as_string.parse().unwrap();
        println!(
            "first number {}, last number {}, full number {}",
            first_number, last_number, full_number_as_int
        );
        all_numbers.push(full_number_as_int)
    }
    let sum_of_numbers = all_numbers.iter().sum::<i32>();
    println!("Sum of all numbers: {}", sum_of_numbers);
}

fn numeral_to_int(input_number: &str) -> String {
    let number_map: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .into_iter()
    .collect();

    if number_map.contains_key(input_number) {
        return number_map.get(input_number).unwrap().to_string();
    }

    return input_number.to_string();
}

fn extract_first_and_last_number(line: &str) -> (String, String) {
    let mut first_number = "".to_string();
    let mut last_number = "".to_string();

    let all_digits: HashSet<char> = "0123456789".chars().collect();

    // Order n baybee
    for (_i, character) in line.chars().enumerate() {
        if all_digits.contains(&character) {
            if first_number == "" {
                first_number = character.to_string();
            }
            last_number = character.to_string();
        }
    }

    return (first_number, last_number);
}

fn extract_first_and_last_number_with_numerals(line: &str) -> (String, String) {
    // println!("extracting line... {}", line);
    let mut first_number = "".to_string();
    let mut last_number = "".to_string();

    let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9))")
        .unwrap();

    let matches: Vec<_> = re.find_iter(line).filter_map(Result::ok).collect();
    // println!("matches: {:?}", matches);
    let first_match = matches.first();
    let last_match = matches.last();

    print!("first match: {:?}", first_match.as_ref().unwrap());

    first_number = match first_match {
        Some(m) => numeral_to_int(m.as_str()),
        None => "".to_string(),
    };

    last_number = match last_match {
        Some(m) => numeral_to_int(m.as_str()),
        None => "".to_string(),
    };

    return (first_number, last_number);
}
