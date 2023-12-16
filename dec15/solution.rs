// Hash algorithm
use std::fs;

fn encode(input: &str) -> u8 {
	let mut output: u32 = 0;

	for i in input.chars() {
		output += i as u32;
		output *= 17;
		output = output % 256
	}
	output as u8
}


fn main() {
	let file_path: String = "sample.txt".to_string();
	let mut binding = fs::read_to_string(&file_path).unwrap();
	binding.pop();
	let steps: Vec<&str> = binding.split(",").collect();

	// Part 1
	let mut sum: u32 = 0;
	for i in &steps {
		let value = encode(i);
		sum += value as u32;
	}
	println!("Part 1: {:?}", sum);

}