use std::fs;
use std::convert::TryInto;

fn vec_to_num(input: &Vec<u64>) -> u64 {
    let mut temp_num: u64 = 0;
    let mut len: u32 = input.len().try_into().unwrap();
    for i in 0..input.len() {
        len -= 1;
        temp_num += input[i]*10_u64.pow(len);
    }
    temp_num
}
 
fn get_seeds(input: &String) -> Vec<u64> {
	let mut seeds: Vec<u64> = Vec::new();
	let mut string_as_chars = input.chars().peekable();
	let mut temp: Vec<u64> = Vec::new();

	loop {
		let digit = string_as_chars.next().unwrap();
		let next = string_as_chars.peek();

		if digit <= '9' && digit >= '0' {
			temp.push(digit as u64 - 0x30);
		} 

		match next {
			Some(x) => {
				if x == &' ' && temp.len() > 0 {
					seeds.push(vec_to_num(&temp));
					temp.clear();
				}
			}
			None => {
				seeds.push(vec_to_num(&temp));
				break; 
			}
		}
	}
	seeds
}

fn convert(target: &mut Vec<u64>, map: &Vec<Vec<u64>>) -> Vec<u64> {
	let mut output: Vec<u64> = Vec::new(); 
	let len = target.len();
	let map_len = map.len();

	for i in 0..len {
		let mut pushed: bool = false;
		let value = target[i];
		for j in 0..map_len {
			let dest = map[j][0];
			let start = map[j][1];
			let range = map[j][2];

			if value >= start && value < (start+range) {
				output.push(dest+(value-start));
				pushed = true;
				break;
			}
		}
		if pushed == false {
			output.push(value);
		}
	}
	output
}




fn main() {
	let file_path: String = "input.txt".to_string();

	let binding = fs::read_to_string(&file_path).unwrap();
	let mut lines = binding.lines();
	
	// Get Seeds
	let first_line = lines.next().unwrap();
	let mut seeds: Vec<u64> = get_seeds(&first_line.to_string());

	let mut test = lines.clone();
	let mut numbers: Vec<Vec<u64>> = Vec::new();
	for line in test {
		if line.to_string().is_empty() == false {
			let temp = get_seeds(&line.to_string());
			if temp.len() > 1 {
				numbers.push(temp);
				}
			} else {
				seeds = convert(&mut seeds, &numbers);
				numbers.clear();
			}
	}
	seeds = convert(&mut seeds, &numbers);

	let mut min: u64 = seeds[0];

	for i in 0..seeds.len() {
		if seeds[i] < min {
			min = seeds[i];
		}
	}
	println!("{:?}", min);

}


