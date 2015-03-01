fn main(){

	println!("{}", squareOfSums(100) - sumOfSquares(100));
}

fn sumOfSquares(num: u32) -> u64{
	let mut sum: u64 = 0;
	for i in 1..(num+1){
		sum+=(i*i) as u64;
	}

	sum
}

fn squareOfSums(num: u32) -> u64{
	let sum: u64 = (num * (num + 1) / 2) as u64;
	
	sum * sum	
}