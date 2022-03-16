use bitvec::prelude::*;
fn main() {

}
//splits bitvec into 6bit then shoves them into u8
fn sextet_split(bits: BitVec) -> Vec<u8>{
	//check how much padding is needed, and add it
	let padding = bits.len() % 6;
	for x in padding {
		bits.push(false);
	}
	//split, then shove into u8
	let bits_slice ::Vec<u8>;
	for x in bits.len()/6 {
		let u = BitSlice::<_, Lsb0>::from_slice(&bits[x..(x+6)]);
		bits_slice.push(u.load<u8>());
	}
	return bits_slice;
}