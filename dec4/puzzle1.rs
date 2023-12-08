use std::fs;
use std::convert::TryInto;

fn main (){
	let file_path: String = "input.txt".to_string();

	let mut total: i32 = 0;

	for line in fs::read_to_string(&file_path).unwrap().lines() {
		let mut score: i32 = 0;
		let mut test_bool: bool = false;
		let mut winning_numbers = Vec::new();
		let mut temp = Vec::new();
		let mut string_as_chars = line.chars().peekable();

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
		//println!("{score}");
		total += score;

	}
	println!("{total}");

}