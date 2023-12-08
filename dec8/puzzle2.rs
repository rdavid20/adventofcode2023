use std::fs;
use std::collections::HashMap;

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}


fn main() {
	let file_path: String = "input.txt".to_string();
	let binding = fs::read_to_string(&file_path).unwrap();
	let mut lines = binding.lines();

	let mut map: HashMap<String, (String, String)> = HashMap::new();

	let directions = lines.next().unwrap();
	println!("Directions: {:?}", directions);
	lines.next();

	let mut start_nodes: Vec<String> = Vec::new();

	for line in lines {
		let temp = line.split(" ").collect::<Vec<&str>>();

		if temp[0].chars().last().unwrap() == 'A' {
			start_nodes.push(temp[0].to_string());
		} 

		let mut l = temp[2].chars();
		l.next();
		l.next_back();

		let mut r = temp[3].chars();
		r.next_back();

		map.insert(temp[0].to_string(), (l.as_str().to_string(), r.as_str().to_string()));
	}

	println!("start nodes:");
	for i in &start_nodes {
		println!("{:?}", i);
	}

	let mut current: Vec<String> = start_nodes;
	let len = current.len();
	let mut steps: i32 = 0;
	let mut paths_min: [usize; 6] = [0, 0, 0, 0, 0, 0];
	let mut found: bool;

	loop {
		for direction in directions.chars() {
			steps+=1;
			for i in 0..len {
				let options = map.get(&current[i]).unwrap();
				if direction == 'R' {
					current[i] = options.1.clone();
				} else {
					current[i]  = options.0.clone();
				}
			}
		}

		found = true;
		for i in 0..len {
			if current[i].chars().last().unwrap() == 'Z' && paths_min[i] == 0 {
				paths_min[i] = steps as usize;
				found = false;
			} else if paths_min[i] == 0 {
				found = false;
			}
		}

		if found == true {
			break;
		}

	}

	let solution = lcm(&paths_min);

	println!("final nodes: ");
	for i in current {
		println!("{:?}", i);
	}

	println!("Found all nodes ending with Z in {:?} steps", solution);
 

}