use std::num::Float;


fn main(){

	let max: u32 = 1000;
	//a^2 + b^2 + sqrt(a^2 + b^2) == 1000
	//one side can't be bigger than sum of others
	'outer: for a in 1..max/3{
		for b in 1..max/2{
			if a + b + floorSqrt(a*a + b*b) == max && pythagoreanSquare(a, b){
				println!("a={},b={},c={}",a,b,sqrt(a*a+b*b));
				println!("sum={}",a+b+floorSqrt(a*a+b*b));
				println!("multiplied={}",a*b*floorSqrt(a*a+b*b));
				break 'outer;
			}
		}
	}


}

fn pythagoreanSquare(a: u32, b:u32) -> bool{
	(floorSqrt(a*a + b*b) as f32) == sqrt(a*a + b*b)
}

fn floorSqrt(num: u32) -> u32 {//returns the floor of the square root - u64 because 32 yields overflow :(
	(num as f32).sqrt() as u32
}

fn sqrt(num: u32) -> f32 {//returns the floor of the square root - u64 because 32 yields overflow :(
	(num as f32).sqrt() as f32
}