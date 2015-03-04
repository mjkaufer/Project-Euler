fn main(){

	let mut count: u32 = 1;
	let mut num: u64 = 1;
	let mut primes: Vec<u64> = Vec::new();
	primes.push(2);
	
	// for i in 3..50{
	// 	let b: bool = vectorIsPrime(i, &primes);
	// 	if b{
	// 		primes.push(i);
	// 	}
	// 	println!("{},{}", i,b);

	// }

	while count < 10001{
		num += 2;
		if vectorIsPrime(num, &primes){
			count+=1;
			primes.push(num);
		}
	}

	println!("10,001st prime is {}",num);

}

fn vectorIsPrime(num: u64, p: &[u64]) -> bool{

	for &i in p{
		if num > i && num % i == 0{
			return false;
		}
	}

	true

}