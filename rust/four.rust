use std::num;

fn main(){

	//this way is kinda brute forcing it

	let mut maxPal = 0;

	for x in std::iter::range_step_inclusive(999, 100, -1){
		for y in std::iter::range_step_inclusive(999, 100, -1){
			if isPalindrome( ( x*y ).to_string() ) && (x*y) > maxPal{
				maxPal = x*y;
			}
		}
	}

	println!("{}", maxPal);

}

fn isPalindrome(num: String) -> bool{
	if num.len() == 1{
		return true;
	}

	let lastChar = &num[num.len() - 1..];
	let firstChar = &num[0..1];

	if firstChar != lastChar{
		return false;
	}

	if num.len() == 2{
		return true;
	}

	isPalindrome((&num[1..num.len() - 1]).to_string())
}