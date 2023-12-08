use std::fs;
use std::collections::HashMap;

fn main() {
	let file_path: String = "input.txt".to_string();
	let binding = fs::read_to_string(&file_path).unwrap();
	let mut lines = binding.lines();

	let mut map: HashMap<String, (String, String)> = HashMap::new();

	let mut directions = lines.next().unwrap();
	println!("Directions: {:?}", directions);
	lines.next();

	let start: String = "AAA".to_string();
	let end: String = "ZZZ".to_string();

	for line in lines {
		let temp = line.split(" ").collect::<Vec<&str>>();

		let mut l = temp[2].chars();
		l.next();
		l.next_back();

		let mut r = temp[3].chars();
		r.next_back();

		map.insert(temp[0].to_string(), (l.as_str().to_string(), r.as_str().to_string()));
	}

	let mut current: String = start;
	let mut steps: i32 = 0;
	loop {
		for direction in directions.chars() {
			steps += 1;
			let options = map.get(&current).unwrap();

			if direction == 'R' {
				current = options.1.clone();
			} else {
				current = options.0.clone();
			}

		}

		if current == "ZZZ".to_string() {
			break;
		}

	}

	println!("Found ZZZ with {:?} steps", steps);


}