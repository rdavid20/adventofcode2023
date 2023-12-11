use std::fs;
use std::convert::TryInto;
 
fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

fn parse_line(input: &str) -> u64 {
	let mut times = input.split(":").last();
	let mut times_vec: Vec<u64> = Vec::new();

	match times {
		Some(x) => {
			let output = remove_whitespace(x);
			output.parse::<u64>().unwrap()
		}
		None => 0
	}
}


fn main() {
	let file_path: String = "input.txt".to_string();
	let binding = fs::read_to_string(&file_path).unwrap();
	let mut lines = binding.lines();

	let mut times = lines.next().unwrap();
	let mut distances = lines.next().unwrap();

	let mut times_vec: u64 = parse_line(times);
	let mut dist_vec: u64 = parse_line(distances);

	let mut total_ways = 1;
	let start_speed = 0;


	let mut ways = 0;
	let record = dist_vec;
	println!("Previous record is: {:?}", record);
	let time = times_vec;
	let mut speed = start_speed;
	for i in 0..time {
		let time_left = time-i;
		let distance_possible = speed*time_left;
		if distance_possible > record {
			ways += 1;
		}
		speed += 1;
	}
	println!("Possible ways to beat {record}: {:?}", ways);


}