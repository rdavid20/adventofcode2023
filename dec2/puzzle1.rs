use std::fs;

fn check_line(test: &String ) -> i32 {
	let mut string_as_char = test.chars().peekable();
	let max_red: i32 = 12; 
	let max_green: i32 = 13;
	let max_blue: i32 = 14;
	let mut reds: i32 = 0;
	let mut greens: i32 = 0;
	let mut blues: i32 = 0;
	let mut output = "".to_string();

	// Get game number
	loop {
		let current = string_as_char.next().unwrap();
		let next = string_as_char.peek();

		match next {
			Some(x) => {
				if x == &':' {
					output += &current.to_string();
					break;
				} else if current >= '0' && current <= '9' {
					output += &current.to_string();
				}
			}
			None => break,
		}
	}
	println!("game number: {output}");

	let mut temp: String = "".to_string();
	let mut color: String = "".to_string();
	// Get colors
	loop {
		let current = string_as_char.next().unwrap();

		if current >= '0' && current <= '9' {
			temp += &current.to_string();
		} else if current >= 'a' && current <= 'z' {
			color += &current.to_string();
		}

		let next = string_as_char.peek();
		match next {
			Some(x) => {
				if x == &',' || x == &';'{
					let value = temp.parse::<i32>().unwrap();
					match color.as_str() {
						"red" => {
							if value > reds {
								reds = value;
							}
						}
						"green" => {
							if value > greens {
								greens = value;
							}
						}
						"blue" => {
							if value > blues {
								blues = value;
							}
						}
						_ => break,
					}
					temp = "".to_string();
					color = "".to_string();
				}	
			}
			None => {
				let value = temp.parse::<i32>().unwrap();
					match color.as_str() {
						"red" => {
							if value > reds {
								reds = value;
							}
						}
						"green" => {
							if value > greens {
								greens = value;
							}
						}
						"blue" => {
							if value > blues {
								blues = value;
							}
						}
						_ => break,
					}
					temp = "".to_string();
					color = "".to_string();
				break;
			}
		}
	}

	// Return game number if possible
	if reds <= max_red && greens <= max_green && blues <= max_blue {
		return output.parse::<i32>().unwrap();
	}
	else {
		return 0;
	}
}


fn main() {
	let file_path: String = "input.txt".to_string();
	let mut total: i32 = 0;

	for line in fs::read_to_string(file_path).unwrap().lines() {
		let hey = check_line(&line.to_string());
		total += hey;
	}
	println!("{total}");

	
}