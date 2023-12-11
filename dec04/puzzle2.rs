use std::fs;
use std::convert::TryInto;
use std::collections::HashMap;

fn calc_score_for_line(line: &String) -> (i32, i32) {
		let mut score: i32 = 0;
		let mut test_bool: bool = false;
		let mut winning_numbers = Vec::new();
		let mut temp = Vec::new();
		let mut string_as_chars = line.chars().peekable();
		let mut matches = 0;
		loop {
			let digit = string_as_chars.next().unwrap();
			let next = string_as_chars.peek();

			if digit <= '9' && digit >= '0' {
				temp.push(digit as i32 - 0x30);
			}
			else if digit == '|' {
					test_bool = true;
				}
			else if digit == ':' {
				temp.clear();
			}
			if digit == ' ' || next.is_none() {
				let mut temp_num: i32 = 0;
				let mut len: u32 = temp.len().try_into().unwrap();
				for k in 0..temp.len(){
					len -= 1;
					temp_num += temp[k]*10_i32.pow(len);
				}

				if test_bool == false {
					winning_numbers.push(temp_num);
				} else if temp_num > 0 {
					for i in &winning_numbers {
						if temp_num == *i {
							matches += 1;
							if score == 0 {
								score += 1;
							} else {
								score = score * 2;
							}
						}
					}
				}
				temp.clear();
			}

			if next.is_none() {
					break;
			}
		}
		return (score, matches);
}

fn main() {
	let file_path: String = "input.txt".to_string();
	let mut line_id = 1; 
	let mut copies: HashMap<i32, i32> = HashMap::new();
	let mut total: i32 = 0;

	for line in fs::read_to_string(&file_path).unwrap().lines() {
		//println!("{line_id}");
		let (score, matches) = calc_score_for_line(&line.to_string());

		let runs = &copies.get(&line_id);
		let mut iterations: i32 = 1;

		match &copies.get(&line_id) {
			Some(x) => {
				println!("{x}");
				iterations += *x;
			}
			None => {iterations = 1;
			}
		}
		total += iterations;

		for i in 1..matches+1 {
			let test = &line_id + i;
			match &copies.get(&test) {
				Some(x) => {
					&copies.insert(&line_id + i, *x+iterations);
				}
				None => {
					&copies.insert(&line_id + i, iterations);
				}
			}
		}
		line_id += 1;
	}

	println!("{:?}", total);

}