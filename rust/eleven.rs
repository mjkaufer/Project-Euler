#![feature(old_io)]
#![feature(old_path)]

use std::old_io::File;
use std::cmp;

static DIM: usize = 20;
static LENGTH: usize = 4;

fn main(){


	let mut vec2d: Vec<Vec<usize>> = Vec::new();

	let data = Path::new("../data/11.dat");

    let mut file = match File::open(&data) {
        Err(why) => panic!("err, {}", why.desc),
        Ok(file) => file,
    };

    let rawData: String = match file.read_to_string() {
        Err(why) => panic!("err, {}", why.desc),
        Ok(string) => string,
    };

    let mut splitVector: Vec<&str> = rawData.split(" ").collect();//only works if input has no breaks, only spaces to separate numbers

	for r in 0..DIM{
		let mut rowVec: Vec<usize> = Vec::new();

		for c in 0..DIM{

			let ch: &str = splitVector.pop().unwrap().trim();
			
			rowVec.insert(0, parseInt(ch));
		}

		vec2d.insert(0, rowVec);
	}

	let mut max: u32 = 0;

	for r in 0..DIM{
		for c in 0..DIM{
			max = cmp::max(max, maxMult(r, c, vec2d.as_slice()));
		}
	}

	println!("{:?}", vec2d);

	println!("{}", max);


}

fn parseInt(num: &str) -> usize{
	// println!("{:?}", num);
	num.parse().unwrap()

}

fn maxMult(r: usize, c: usize, vec2d: &[Vec<usize>]) -> u32 {
	cmp::max(
		diag(r, c, vec2d),
		cmp::max(
			hori(r, c, vec2d),
			cmp::max(
				vert(r, c, vec2d),
				revd(r, c, vec2d)
			)
		)
	)
}

fn diag(r: usize, c: usize, vec2d: &[Vec<usize>]) -> u32 {
	if r + LENGTH > DIM || c + LENGTH > DIM{
		return 0;
	}
	let mut p: u32 = 1;

	for i in 0..LENGTH{
		p *= (vec2d[r+i][c+i] as u32);
	}

	p
}

fn revd(r: usize, c: usize, vec2d: &[Vec<usize>]) -> u32 {

	if r + LENGTH > DIM || (c as i32 - LENGTH as i32) < -1{//had to cast to i32 because u's are always positive - was causing annoying bug
		return 0;
	}

	let mut p: u32 = 1;

	for i in 0..LENGTH{
		p *= (vec2d[r+i][c-i] as u32);
	}

	p
}

fn hori(r: usize, c: usize, vec2d: &[Vec<usize>]) -> u32 {
	if c + LENGTH > DIM{
		return 0;
	}
	let mut p: u32 = 1;

	for i in 0..LENGTH{
		p *= (vec2d[r][c+i] as u32);
	}

	p
}

fn vert(r: usize, c: usize, vec2d: &[Vec<usize>]) -> u32 {
	if r + LENGTH > DIM{
		return 0;
	}
	let mut p: u32 = 1;

	for i in 0..LENGTH{
		p *= (vec2d[r+i][c] as u32);
	}

	p
}