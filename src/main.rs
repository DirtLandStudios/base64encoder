use bitvec::prelude::*;
fn main() {

}
//splits bitvec into 6bit then shoves them into u8
fn sextet_split(mut bits: BitVec) -> Vec<u8>{
	//check how much padding is needed, and add it
	let padding: u8 = (bits.len() % 6).try_into().unwrap();
	let mut x = 0;
	while x <= padding {
		bits.push(false);
		x += 1;
	}
	//split, then shove into u8
	let mut bits_slice: Vec<u8> = Vec::new();
	let sextets: u8 = (bits.len() / 6).try_into().unwrap();
	x = 0;
	while x <= sextets {
		//let u = BitSlice::<_, Lsb0>::from_slice(&bits[x..(x+6)]);
		let u = &bits[x as usize..(x as usize + 5)];
		bits_slice.push(u.load::<u8>());
		x += 1;
	}
	return bits_slice;
}