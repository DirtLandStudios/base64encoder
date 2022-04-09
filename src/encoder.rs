use bitvec::prelude::*; //import bitvec lib

pub fn encode(input: Vec<u8>) {
	let bits: BitVec<u8> = BitVec::<u8, Lsb0>::from_slice(&input);
	let (charvalues, paddingvalues) = sextet_split(bits);
	let padding = "=".repeat(paddingvalues);
	let encoded = value_to_chars(charvalues) + &padding;
	println!("{}", encoded);
}
//splits bitvec into 6bit then shoves them into u8
fn sextet_split(mut bits: BitVec<u8>) -> (Vec<u8>, usize) {
	//because 8 is not a multiple of 6, some multiples of 8 (number of bits) are not multiples of 6, 
	//so we add 0s at the end to make it divisible by both
	//check how much padding is needed, and add it
	let padding: usize = (bits.len() % 6).try_into().unwrap();
	for _x in 0..padding {
		bits.push(false);
	}
	//split, then shove into u8
	let mut bits_slice: Vec<u8> = Vec::new();
	//amount of sextets we have
	let sextets: usize = (bits.len() / 6).try_into().unwrap();
	//slice bits into sextets, change to u8 (char values) and shove char values into a vec
	for x in 0..sextets {
		let u = &bits[x..(x + 5)];
		bits_slice.push(u.load::<u8>());
	}
	return (bits_slice, padding);
}

fn value_to_chars(charvalues: Vec<u8>) -> String {
	let language: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_owned();
	let dictionary: Vec<char> = language[..].chars().collect();
	assert_eq!(dictionary.len(), 64);
	let mut encode: Vec<char> = Vec::new();
	for i in charvalues {
		encode.push(dictionary[i as usize]);
	}
	return encode.iter().cloned().collect::<String>();
}