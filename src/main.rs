use std::fs;

mod day_01;

pub fn read_file(filename: &str) -> String {
	fs::read_to_string(filename).unwrap()
}

fn main() {
	let day = 1;

	match day {
		1 => day_01::run(),
		2 => unimplemented!("No solution yet"),
		3 => unimplemented!("No solution yet"),
		4 => unimplemented!("No solution yet"),
		5 => unimplemented!("No solution yet"),
		6 => unimplemented!("No solution yet"),
		7 => unimplemented!("No solution yet"),
		8 => unimplemented!("No solution yet"),
		9 => unimplemented!("No solution yet"),
		10 => unimplemented!("No solution yet"),
		11 => unimplemented!("No solution yet"),
		12 => unimplemented!("No solution yet"),
		13 => unimplemented!("No solution yet"),
		14 => unimplemented!("No solution yet"),
		15 => unimplemented!("No solution yet"),
		16 => unimplemented!("No solution yet"),
		17 => unimplemented!("No solution yet"),
		18 => unimplemented!("No solution yet"),
		19 => unimplemented!("No solution yet"),
		20 => unimplemented!("No solution yet"),
		21 => unimplemented!("No solution yet"),
		22 => unimplemented!("No solution yet"),
		23 => unimplemented!("No solution yet"),
		24 => unimplemented!("No solution yet"),
		25 => unimplemented!("No solution yet"),
		_ => unreachable!("Not a valid day"),
	}
}
