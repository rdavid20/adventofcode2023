use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
	FiveOfAKind = 1,
	FourOfAKind = 2,
	FullHouse = 3,
	ThreeOfAKind = 4,
	TwoPair = 5,
	OnePair = 6,
	HighCard = 7,
}

fn detect_hand_type(hand: &String) -> HandType {
	let mut hand_map: HashMap<String, i32> = HashMap::new();	
	println!("{:?}", hand);
	let mut jokers: i32 = 0;

	let binding = hand.clone();
	let mut temp_hand = binding.split("").collect::<Vec<&str>>();
	temp_hand.sort();

	for digit in temp_hand {
		let temp = digit.to_string();

		if temp == "1" {
			jokers += 1;
		}

		if temp.len() > 0 && temp != "1" {
			match hand_map.get(digit) {
				Some(x) => {
					hand_map.insert(temp, x+1);
				}
				None => {
					hand_map.insert(temp, 1+jokers);
				}
			}
		} 
	}

	for j in &hand_map {
		println!("{:?}", j);
	}

	let size = hand_map.keys().len();
	match size {
		5 => HandType::HighCard,
		4 => HandType::OnePair,
		3 => {
			for i in hand_map {
				if i.1 == 3 {
					return HandType::ThreeOfAKind;
				}
			}
			HandType::TwoPair
		}
		2 => {
			for i in hand_map {
				if i.1 == 4 {
					return HandType::FourOfAKind;
				}
			}
			HandType::FullHouse
		}
		1 => HandType::FiveOfAKind,
		_ => HandType::FiveOfAKind,
	}

}



fn main() {
	let file_path: String = "input.txt".to_string();
	let binding = fs::read_to_string(&file_path).unwrap();
	let lines = binding.lines();

	let mut pairs: HashMap<String, i32> = HashMap::new();
	let mut vectors: [Vec<String>; 7] = Default::default();


	for line in lines {
		let temp = line.split(" ").collect::<Vec<&str>>();
		let hand = temp[0]
		.replace('A', "E")
		.replace('K', "D")
		.replace('Q', "C")
		.replace('J', "1")
		.replace('T', "B").to_string();
		let bid = temp[1].parse::<i32>().unwrap();
		pairs.insert(hand.clone(), bid);

		match detect_hand_type(&hand) {
			x =>  {
				println!("{:?}", x);
				println!("");
				vectors[x as usize-1].push(hand);
			}
		}
	}

	let mut final_order = Vec::new();

	for j in 0..7 {
		vectors[j].sort();
		vectors[j].reverse();
	}

	for j in 0..7 {
		for i in &vectors[j] {
			final_order.push(i);
		}
	}

	final_order.reverse();

	let len = final_order.len();
	let mut total: i32 = 0;
	for i in 0..len {
		println!("{:?}", final_order[i]);
		let bid = pairs.get(final_order[i]).unwrap();
		println!("{:?} {}", bid, i as i32 +1);
		total += bid * (i as i32 +1);
	}

	println!("Total: {:?}", total);
}