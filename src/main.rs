use bitvec::prelude::*; //import bitvec lib

fn main() {
	let args: Vec<String> = std::env::args().collect();
	match &*args[0].to_owned() {
		"-p" | "--path" => {
			let file: Vec<u8> = std::fs::read(&args[1]).unwrap();
			let bits: BitVec<u8> = BitVec::<u8, Lsb0>::from_slice(&file[..]); //incase we need to add spliting due to too big variables
			let (charvalues, padding) = sextet_split(bits);
		},
		"-h" | "--help" => {
			println!("-p | --path (path to file) \n or string to encode");
		},
		_ => {
			panic!(); //until I come up with something better (string to encode)
		}
	}

}
//splits bitvec into 6bit then shoves them into u8
fn sextet_split(mut bits: BitVec<u8>) -> (Vec<u8>, u8) {
	//because 8 is not a multiple of 6, some multiples of 8 (number of bits) are not multiples of 6, 
	//so we add 0s at the end to make it divisible by both

	//check how much padding is needed, and add it
	let padding: u8 = (bits.len() % 6).try_into().unwrap();
	let mut x = 0;
	while x <= padding {
		bits.push(false);
		x += 1;
	}

	//split, then shove into u8
	let mut bits_slice: Vec<u8> = Vec::new();
	//amount of sextets we have
	let sextets: u8 = (bits.len() / 6).try_into().unwrap();
	//slice bits into sextets, change to u8 (char values) and shove char values into a vec
	x = 0;
	while x <= sextets {
		let u = &bits[x as usize..(x as usize + 5)];
		bits_slice.push(u.load::<u8>());
		x += 1;
	}
	return (bits_slice, padding);
}
/* //not yet implemented
fn value_to_chars(charvalues: Vec<u8>) -> String {

}
*/