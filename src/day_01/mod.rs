use crate::read_file;

pub fn run() {
	let input = read_file("src/day_01/input");
	let mut sum = 0;
	for line in input.split('\n') {
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
}
