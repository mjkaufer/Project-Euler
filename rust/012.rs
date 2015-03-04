use std::num::Float;

fn main(){


	let mut triangleIndex: u32 = 1;
	let mut triangleSum: u32 = 1;

	while factors(triangleSum) <= 500{
		triangleIndex+=1;
		triangleSum+=triangleIndex;
	}

	println!("First triangle number with over 500 factors is {}", triangleSum);

}

fn floorSqrt(num: u32) -> u32{//returns the floor of the square root - u64 because 32 yields overflow :(
	(num as f32).sqrt() as u32
}

fn factors(triangleSum: u32) -> u32 {
	let mut f: u32 = 0;// 1 and triangleSum

	let sqrt = floorSqrt(triangleSum);

	for i in 1..sqrt+1{//+1 so it's inclusive
		if triangleSum % i == 0{
			f+=2;
		}
	}

	if sqrt * sqrt == triangleSum{//triangleSum is square
		f-=1;
	}

	f
}