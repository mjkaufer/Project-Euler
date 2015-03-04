#![feature(old_io)]
#![feature(old_path)]

use std::old_io::File;

fn main(){

    let data = Path::new("../data/8.dat");

    let mut file = match File::open(&data) {
        
        Err(why) => panic!("Error opening {}", why.desc),
        Ok(file) => file,
    };

    let hugeNumberString: String = match file.read_to_string() {
        Err(why) => panic!("couldn't read file, {}", why.desc),
        Ok(string) => string,
    };

    let hugeNumber = hugeNumberString.as_slice();


	let mut index = 0;
	let mut max: u64 = 0;

	let step = 13;

	println!("{}", multiply("123"));

	while index < hugeNumber.len() - step{
		

		let multSum = multiply(&(hugeNumber.to_string())[index..index+step]);

		max = if multSum > max{
			multSum
		} else {
			max
		};

		index+=1;	
		
	}

	println!("{}", max);	
    
}


fn parseInt(num: &str) -> u32{
	// std::str::FromStr::from_str(num).unwrap()
	num.parse().unwrap()

}

fn multiply(num: &str) -> u64{
	let mut multSum: u64 = 1;

	for c in num.chars(){
		let mut temp: String = String::new();
		temp.push(c);

		multSum *= (parseInt(temp.as_slice()) as u64);
	}

	multSum
}
