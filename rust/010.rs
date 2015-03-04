use std::num::Float;


fn main(){

	let mut num: u32 = 1;
	let mut primes: Vec<u32> = Vec::new();
	primes.push(2);

	while num < 2_000_000{
		num += 2;
		if (num - 1) % 10_000 == 0{
			println!("{}", num);	
		}
		
		if vectorIsPrime(num, &primes){
			primes.push(num);
		}
	}

	//now, all the primes below 2mil are stored in vec

	let mut sum: u64 = 0;

	for prime in primes{
		sum += (prime as u64);
	}

	println!("Sum of all primes under 2 million is {}",sum);

}

fn vectorIsPrime(num: u32, p: &[u32]) -> bool{

	if num > 5 && (num % 3 == 0 || num % 5 == 0){
		return false;
	}

	for &i in p{
		if num > i && num % i == 0{
			return false;
		} else if i > floorSqrt(num) {//checked the lower numbers already - speeds up significantly
			return true;
		}

	}

	true

}

fn floorSqrt(num: u32) -> u32 {
	(num as f32).sqrt() as u32
}