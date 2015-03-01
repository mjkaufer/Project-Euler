fn main(){
	
	let mut firstfib = 0;
	let mut secondfib = 1;
	let bounds = 4000000;
	let mut sum = 0;


	while secondfib <= bounds{
		sum += if secondfib % 2 == 0{
			secondfib
		} else {
			0
		};

		let next = nextFib(firstfib, secondfib);
		firstfib = secondfib;

		secondfib = next;
	}
	println!("{}",sum);

}

fn nextFib(f: u32, s: u32) -> u32{//we could've scrapped this function altogether - I used it because I'm trying to learn rust and, thus, functions
	f + s
}