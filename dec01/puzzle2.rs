use std::fs;

fn check_if_string_is_number(input: &String) -> bool {
	match input.as_str() {
		"one" | "two" | "three" | "four"| 
		"five" | "six" | "seven" | "eight" | "nine" => true,
		_ => false,
	}
}

fn number_to_int(input: &String) -> char {
	match input.as_str() {
		"one" => '1',
		"two" => '2',
		"three" => '3',
		"four" => '4',
		"five" => '5', 
		"six" => '6', 
		"seven" => '7', 
		"eight" => '8',
		"nine" => '9',
		_ => '0',
	}	
}

fn line_parser(input: &String) -> i32 {
	let mut number_string = "".to_string();
	let mut backup: String = "".to_string();

	for i in 0..input.len()+1 {
		let mut out1 = input.clone();
		let out2 = out1.split_off(i);

		for j in 1..out2.len()+1 {
			let mut out4 = out2.clone();
			let _ = out4.split_off(j);

			if out4.len() == 1 {
				for i in out4.chars() {
					let temp = i as i32 - 0x30;
					if temp >= 0 && temp <= 9 {
						if number_string.len() == 0 {
							number_string += &out4;
						}

						backup = out4.clone();
					}
				}
			}

			if check_if_string_is_number(&out4) == true {
				let number = number_to_int(&out4);

				if number_string.len() == 0 {
					number_string += &number.to_string();
				}

				backup = number_to_int(&out4).to_string();
			}
		}
	}
	number_string += &backup;
	return number_string.parse::<i32>().unwrap();
}

fn main() {
	let file_path: String = "input.txt".to_string();
	let mut total: i32 = 0;
	for line in fs::read_to_string(file_path).unwrap().lines() {
		let value = line_parser(&line.to_string());
		total += value;
	}
	println!("{total}");
}