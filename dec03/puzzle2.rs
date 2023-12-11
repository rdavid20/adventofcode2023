use std::fs;
use std::convert::TryInto;
use std::collections::HashMap;

fn main() {
	let file_path: String = "input.txt".to_string();

	// Get size of array
	let mut width = 0;
	let mut height = 0;
	for line in fs::read_to_string(&file_path).unwrap().lines() {
		height+= 1;

		if height == 1 {
			for _digit in line.chars() {
				width += 1;
			}
		}
	}
	let mut array = vec![vec![0; width]; height];
	println!("{height}, {width}");

	// Put chars into array
	let mut h_count = 0;
	let mut w_count;
	for line in fs::read_to_string(file_path).unwrap().lines() {
		w_count = 0;
		for digit in line.chars() {
			array[h_count][w_count] = digit as i32 - 0x30;
			w_count += 1;
		}
		h_count+=1;
	}	

	let mut total: i32 = 0;
	// Get numbers
	let mut n_vec = Vec::new();
	let mut gear: bool = false; 
	let mut gear_location: [i32; 2] = [-1, -1];
	let mut gear_numbers: HashMap<[i32; 2], i32> = HashMap::new();
	for i in 0..height {
		for j in 0..width {

			if array[i][j] >= 0 && array[i][j] <= 9 {
				n_vec.push(array[i][j]);
				let mut start_w: i32 = -1;
				let mut end_w: i32 = 1;
				let mut start_h: i32 = -1;
				let mut end_h: i32 = 1;

				if width-1 == j {
					end_w = 0;
				}
				if j == 0 {
					start_w = 0;
				}
				if height-1 == i {
					end_h = 0;
				}
				if i == 0 {
					start_h = 0;
				}

				for k in start_h+1..end_h+2 {
					for l in start_w+1..end_w+2 {
						if array[i+(k as usize)-1][j+(l as usize)-1] == -6 {
							gear = true;
							gear_location = [(i+(k as usize)-1) as i32, (j+(l as usize)-1) as i32];
						}
					}
				}
			}

			if (array[i][j] < 0 || array[i][j] > 9) || j == width-1 {
				let mut len: u32 = n_vec.len().try_into().unwrap();
				let mut temp_num: i32 = 0;
				for k in 0..n_vec.len(){
					len -= 1;
					temp_num += n_vec[k]*10_i32.pow(len);
				}
				if gear == true {
					match gear_numbers.get(&gear_location) {
						Some(x) => {
							println!("adding to sum {}", temp_num * x);
							total += temp_num * x;
						}
						None => {
							println!("adding to hashmap");
							gear_numbers.insert(gear_location, temp_num); 
							()
						}

					}
					println!("{temp_num}");
				}
				n_vec.clear();
				gear = false;
			}

		}
	}

	println!("{total}");
}
