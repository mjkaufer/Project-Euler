#![feature(old_io)]
#![feature(old_path)]

use std::old_io::File;
use std::cmp;
use std::iter::range_step_inclusive;


fn main(){
	let mut bins: Vec<u16> = Vec::new();



	let data = Path::new("../data/13.dat");

    let mut file = match File::open(&data) {
        Err(why) => panic!("err, {}", why.desc),
        Ok(file) => file,
    };

    let rawData: String = match file.read_to_string() {
        Err(why) => panic!("err, {}", why.desc),
        Ok(string) => string,
    };

    let mut splitVector: Vec<&str> = rawData.split("\n").collect();//only works if input has no breaks, only spaces to separate numbers

    for i in 0..toNumVec(splitVector[0]).len(){
    	bins.push(0);
    }


    for i in 0..splitVector.len(){

    	let numVec: Vec<u16> = toNumVec(splitVector[i]);

    	for j in 0..numVec.len(){
    		bins[j] += numVec[j];
    	}
    }


	for index in std::iter::range_step(((bins.len() - 1) as i32), 0, -1){
		let i = index as usize;
		
		bins[i - 1] += (bins[i] / 10);
		bins[i] = bins[i] % 10;
		
    	
	}

	println!("-----");

	let mut answer = "".to_string();

    for i in 0..bins.len(){
    	answer = answer + bins[i].to_string().as_slice();
    }

    println!("Sum is {}", answer);

    println!("First 10 digits are {}", &answer.as_slice()[0..10]);

}

fn parseInt(num: &str) -> u16{
	// println!("{:?}", num);
	num.parse().unwrap()

}

fn toNumVec(numStr: &str) -> Vec<u16> {//something like "1234" outputs vector [1,2,3,4]
	let strVec: Vec<&str> = numStr.trim().split("").collect();

	let mut numOutput: Vec<u16> = Vec::new();

	for i in strVec{
		if i != ""{
			numOutput.push(parseInt(i));	
		}
	}

	numOutput
}