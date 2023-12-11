use std::fs;

fn is_all_same(arr: &Vec<i32>) -> bool {
    if arr.is_empty() {
        return true;
    }
    let first = arr[0];
    arr.iter().all(|&item| item == first)
}


fn main() {

		let file_path: String = "input.txt".to_string();
		let binding = fs::read_to_string(&file_path).unwrap();
		let lines = binding.lines();

		let mut total: i32 = 0;
		for line in lines {
			let numbers: Vec<i32> = line.split(" ")
			.map(|x| x.parse::<i32>().unwrap())
			.collect();

			let mut diff: Vec<Vec<i32>> = Vec::new();

			diff.push(numbers.clone());
			let mut counter = 0;

			loop {
				let mut temp: Vec<i32> = Vec::new();

				for index in 0..diff[counter].len()-1 {
					temp.push(diff[counter][index+1] - diff[counter][index]);
				}

				diff.push(temp.clone());

				if is_all_same(&temp) && temp[0] == 0 {
					break;
				}

				counter += 1;
			}

			let len = diff.len();
			diff[len-1].push(0);

			diff.reverse();

			for i in 1..len {
				let next = diff[i-1].iter().last().unwrap().clone();
				let current = diff[i].iter().last().unwrap().clone();
				diff[i].push(current+next);
			}

			diff.reverse();

			let a = diff[0].iter().last().unwrap().clone();
			total += a;
		}
		println!("total: {:?}", total);
}