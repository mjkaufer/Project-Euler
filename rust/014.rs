use std::collections::HashMap;

fn main(){

	let mut collatz: HashMap<u32, u32> = HashMap::new();

	collatz.insert(1u32,1u32);

	let mut maxLength: u32 = 0;
	let mut maxVal: u32 = 0;

	for i in 1..1_000_000{
		// println!("Starting {}", i);
		let length = collatzLength(i, &collatz);

		if length > maxLength{
			maxLength = length;
			maxVal = i;
		}

		collatz.insert(i, length);

	}

	println!("Max chain length under 1,000,000 at i={}", maxVal);

}

// fn collatzLength(mut num: u32, map: &HashMap<u32, u32>) -> u32{//8.099

// 	let mut length: u32 = 0;

// 	while !map.contains_key(&num){
// 		length += 1;

// 		if num % 2 == 0{
// 			num /= 2;
// 		} else {
// 			num = 3 * num + 1;
// 		}

// 	}

// 	length + *map.get(&num).unwrap()
// }


fn collatzLength(mut num: u32, map: &HashMap<u32, u32>) -> u32{//7.137

	if map.contains_key(&num){
		return *map.get(&num).unwrap();
	}

	if num % 2 == 0{

		return collatzLength(num / 2, map) + 1;

	} else {

		return collatzLength(num * 3 + 1, map) + 1;

	}

}