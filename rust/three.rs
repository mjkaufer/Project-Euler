use std::iter;
use std::num::Float;
use std::iter::range_step_inclusive;

fn main(){
	println!("{}",largestPrimeFactor(600851475143));
}

fn floorSqrt(num: u64) -> u64 {//returns the floor of the square root - u64 because 32 yields overflow :(
	(num as f64).sqrt() as u64
}

fn largestPrimeFactor(num: u64) -> u64 {

	let start: u64 = floorSqrt(num);

	for f in 0..(start-1){

		let factor = start - f;//so we start by checking the larger values

		if num % factor == 0 && primeTest(factor){
			return factor;
		}

	}

	-1
}

fn primeTest(num: u64) -> bool {

	if num == 2{
		return true;
	} else if num < 2 || num % 2 == 0{
		return false;
	} else {
		for possibleFactor in range_step_inclusive(3, floorSqrt(num), 2){
			if num % possibleFactor == 0{
				return false;
			}
		}
	}

	true
}