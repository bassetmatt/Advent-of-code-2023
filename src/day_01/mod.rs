use std::vec;

use crate::read_file;

fn replace_string_by_digit(line: &str) -> String {
	let digits_text: Vec<&str> = vec![
		"one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
	];
	let mut output = String::new();
	let mut digits_progress = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
	for c in line.chars() {
		if c.is_numeric() {
			output.push(c);
			digits_progress = vec![0, 0, 0, 0, 0, 0, 0, 0, 0]; // Reset
			continue;
		}
		for (id, digit) in digits_text.iter().enumerate() {
			if digit.len() > digits_progress[id] {
				if digit.chars().nth(digits_progress[id]).unwrap() == c {
					digits_progress[id] += 1;
					if digit.len() == digits_progress[id] {
						output.push(char::from_digit(1 + id as u32, 10).unwrap());
						digits_progress[id] = 0;
						continue;
					}
				} else if digit.starts_with(c) {
					digits_progress[id] = 1;
				} else {
					digits_progress[id] = 0;
				}
			}
		}
	}
	output
}

pub fn run() {
	test();
	let input = read_file("src/day_01/input");
	let mut sum = 0;
	for line in input.split('\n') {
		let line = replace_string_by_digit(line);
		let mut first_num = None;
		let mut last_num = None;
		for c in line.chars() {
			if c.is_numeric() {
				if first_num.is_none() {
					first_num = Some(c as u32 - '0' as u32);
				}
				last_num = Some(c as u32 - '0' as u32);
			}
		}
		sum += first_num.unwrap_or(0) * 10 + last_num.unwrap_or(0);
	}
	println!("Sum: {}", sum);
	assert_eq!(sum, 54885)
}

pub fn test() {
	let input = read_file("src/day_01/input_ex");
	let mut result = vec![];
	for line in input.split('\n') {
		let line = replace_string_by_digit(line);
		result.push(line.parse::<i32>().unwrap_or(0));
	}
	assert_eq!(result[0], 1);
	assert_eq!(result[1], 2);
	assert_eq!(result[2], 3);
	assert_eq!(result[3], 4);
	assert_eq!(result[4], 5);
	assert_eq!(result[5], 6);
	assert_eq!(result[6], 7);
	assert_eq!(result[7], 8);
	assert_eq!(result[8], 9);
	assert_eq!(result[9], 21);
	assert_eq!(result[10], 82);
	assert_eq!(result[11], 98);
	assert_eq!(result[12], 83);
	assert_eq!(result[13], 98);
	assert_eq!(result[14], 8);
	assert_eq!(result[15], 11);
}
