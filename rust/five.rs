fn main(){

	let mut counter: u64 = 20;

	while !evenlyDivisible(counter){
		counter+=20;
	}

	println!("{}",counter);


}


fn evenlyDivisible(num: u64) -> bool{//bool whether or not num is divisible by all numbers from 1-20; ignore 1 and 2
	//todo, optimize with GCD/LCM stuff
	for i in 11..21{
		if num % i != 0{
			return false;
		}
	}

	true
}