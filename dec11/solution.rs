use std::fs;
use std::cmp;

fn calc_sum(pairs: &Vec<((i32, i32),(i32, i32))>, rows: &Vec<usize>, cols: &Vec<usize>, scale: i32) -> u64{
	let mut total: u64 = 0;
	for (start, end) in pairs {
		let new_start = start;
		let new_end = end;
		let mut times_1 = 0;

		for i in rows {
			if ((*i as i32) > cmp::min(new_start.0, new_end.0)) && ((*i as i32) < cmp::max(new_start.0, new_end.0)){
				times_1 += 1;
			}
		}

		for i in cols {
			if ((*i as i32) > cmp::min(new_start.1, new_end.1)) && ((*i as i32) < cmp::max(new_start.1, new_end.1)) {
				times_1+= 1;
			}
		}
		let add_rows = (scale-1)*times_1;
		let man_dist = i32::abs(new_end.0-new_start.0) + add_rows + i32::abs(new_end.1-new_start.1);
		total += man_dist as u64;
	}
	total

}


fn main() {
	let file_path: String = "input.txt".to_string();
	let binding = fs::read_to_string(&file_path).unwrap();
	let lines = binding.lines();

	let mut map: Vec<Vec<char>> = Vec::new();

	for line in lines {
		let row = line.chars().collect();
		map.push(row);
	}

	let row_count = map.len();
	let column_count = map[0].len();
	let mut empty_rows = Vec::new();
	let mut empty_columns = Vec::new();

	for i in 0..row_count {
		if map[i].iter().min() == map[i].iter().max() {
			empty_rows.push(i);
		}
	}

	for i in 0..column_count {
		let mut temp = Vec::new();
		for row in &map {
			temp.push(row[i]);
		}
		if temp.iter().min() == temp.iter().max() {
			empty_columns.push(i);
		}
	}

	let mut galaxies: Vec<(i32, i32)> = Vec::new();
	for i in 0..row_count {
		for j in 0..column_count {
			if map[i as usize][j as usize] == '#' {
				galaxies.push((i as i32, j as i32));
			}
		}
	}

	let galaxy_count = galaxies.len();
	let mut pairs: Vec<((i32, i32),(i32, i32))> = Vec::new();
	for i in 0..galaxy_count {
		for j in i..galaxy_count {
			if i == j {
				continue;
			}
			pairs.push((galaxies[i], galaxies[j]));
		}
	}

	let task1 = calc_sum(&pairs, &empty_rows, &empty_columns, 2);
	println!("Task 1: {:?}", task1);

	let task2 = calc_sum(&pairs, &empty_rows, &empty_columns, 1000000);
	println!("Task 2: {:?}", task2);

}