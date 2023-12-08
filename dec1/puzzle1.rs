use std::fs;

fn main() {
	let file_path: String = "input.txt".to_string();
	let mut total: i32 = 0;

	for line in fs::read_to_string(file_path).unwrap().lines() {
		let mut cleaned: String = "".to_string();
		let mut temp: char = '0'; 
		for digit in line.chars() {
			if digit >= '0' && digit <= '9' {
				if cleaned.len() == 0 {
					cleaned += &digit.to_string();
				}
				temp = digit; 
			}
		}
		cleaned += &temp.to_string();
		total += cleaned.parse::<i32>().unwrap();
	}
	println!("{total}");
}