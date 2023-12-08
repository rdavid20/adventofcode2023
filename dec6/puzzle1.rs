use std::fs;

fn parse_line(input: &str) -> Vec<i32> {
	let mut times = input.split(":").last();
	let mut times_vec: Vec<i32> = Vec::new();

	match times {
		Some(x) => {
			let temp = x.split(" ");
			for i in temp {
				if i.len() > 0 {
					times_vec.push(i.parse::<i32>().unwrap());
				}
			}
		}
		None => ()
	}
	times_vec
}


fn main() {
	let file_path: String = "sample.txt".to_string();
	let binding = fs::read_to_string(&file_path).unwrap();
	let mut lines = binding.lines();

	let mut times = lines.next().unwrap();
	let mut distances = lines.next().unwrap();

	let mut times_vec: Vec<i32> = parse_line(times);
	let mut dist_vec: Vec<i32> = parse_line(distances);

	let mut len = times_vec.len();

	let mut total_ways = 1;
	let start_speed = 0;

	for race_id in 0..len {
		let mut ways = 0;
		let record = &dist_vec[race_id];
		println!("Previous record is: {:?}", record);
			let time = times_vec[race_id];
			let mut speed = start_speed;
			for i in 0..time {
				let time_left = time-i;
				let distance_possible = speed*time_left;
				if distance_possible > *record {
					ways += 1;
				}
				speed += 1;
			}
		total_ways *= ways;
		println!("Possible ways to beat {record}: {:?}", ways);
	}
	println!("Total ways possible: {:?}", total_ways);

}