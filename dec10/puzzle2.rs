use std::fs;
use std::collections::HashMap;

fn get_nbrs(map: Vec<Vec<char>>, i: i32, j: i32, rows: i32, columns: i32) -> Vec<(i32, i32)> {
	let mut output: Vec<(i32, i32)> = Vec::new();

	let mut lookup_table: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
	lookup_table.insert('|', vec![(1, 0), (-1, 0)]);
	lookup_table.insert('-', vec![(0, 1), (0, -1)]);
	lookup_table.insert('L', vec![(0, 1), (-1, 0)]);
	lookup_table.insert('J', vec![(0, -1), (-1, 0)]);
	lookup_table.insert('7', vec![(1, 0), (0, -1)]);
	lookup_table.insert('F',  vec![(1, 0), (0, 1)]);
	lookup_table.insert('.', vec![(0, 0)]);

	if map[i as usize][j as usize] == 'S' {
		let directions_to_check = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
		for (vert, hor) in directions_to_check {
			let ii = i + vert;
			let jj = j + hor;

			if !(0 <= ii && ii < rows && 0 <= jj && jj < columns) {
				continue; }

			let temp = lookup_table.get(&map[ii as usize][jj as usize]).unwrap();

			for i in temp {
				if i.0 == 0 && i.1 == 0 {
					continue;
				}

				if !(0 <= ii+i.0 && ii+i.0 < rows && 0 <= jj+i.1 && jj+i.1 < columns) {
				continue; 
				}

				if map[(ii+i.0) as usize][(jj + i.1) as usize] == 'S' {
					output.push((vert, hor));
				}
			}
	}
		return output;
		
	} else {
		output.extend(lookup_table.get(&map[i as usize][j as usize]).unwrap());
		return output;
	}


}

fn main() {
	let file_path: String = "sample5.txt".to_string();
	let binding = fs::read_to_string(&file_path).unwrap();
	let lines = binding.lines();

	let mut map: Vec<Vec<char>> = Vec::new();
	let mut start: (i32, i32) = (0, 0);

	for line in lines {
		let mut row: Vec<char> = Vec::new();
		for x in line.chars() {
			row.push(x);

			if x == 'S' {
				start.0 = map.len() as i32;
				start.1 = row.len() as i32 -1;
			}
		}

		map.push(row);
	}

	let rows = map.len() as i32;
	let columns = map[0].len() as i32;
	let mut new_map = map.clone();
	let mut nodes_searched: Vec<(i32, i32)> = Vec::new();
	let mut nodes_queue: Vec<((i32, i32), i32)> = Vec::new();
	nodes_queue.push((start, 0));

	let mut dists: HashMap<(i32, i32), i32> = HashMap::new();

	while nodes_queue.len() > 0 {
		let (current, dist) = nodes_queue.remove(0);

		dists.insert(current, dist);
		nodes_searched.push(current);
		new_map[current.0 as usize][current.1 as usize] = '0';

		let nbrs = get_nbrs(map.clone(), current.0, current.1, rows, columns);

		'outer: for nbr in nbrs {
			
			for node in &nodes_searched {
				if *node == (current.0+nbr.0, current.1+nbr.1) {
					continue 'outer;
				}
			}
			nodes_queue.push(((current.0+nbr.0, current.1+nbr.1), dist+1));
		}
	}


	let mut inside = 0;
	let mut crossed;
	for i in 0..rows {
		'inner: for j in 0..columns{
			let current = (i, j);

			for nd in &nodes_searched {
				if current == *nd {
					continue 'inner;
				}
			}

				crossed = 0;

				for m in 0..columns-j {
					if !nodes_searched.contains(&(current.0, current.1+m)) {
						continue;
					}


					for l in vec!['|', 'J', 'L'] {
						if map[current.0 as usize][(current.1+m) as usize] == l {
							crossed += 1;
							break;
						}
					}
				}

				if (crossed % 2) != 0 {
						new_map[current.0 as usize][current.1 as usize] = '1';
						inside += 1;
				}

			}
	}


	println!("{:?}", inside);
}