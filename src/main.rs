mod encoder;

fn main() {
	let mut args: Vec<String> = std::env::args().collect();
	args.resize(3, "".to_string());
	match &*args[1].to_owned() {
		"-p" | "--path" => {
			let file: Vec<u8> = std::fs::read(&args[2]).unwrap();
			encoder::encode(file);
			//let bits: BitVec<u8> = BitVec::<u8, Lsb0>::from_slice(&file[..]); //incase we need to add spliting due to too big variables
		},
		"-s" | "--string" => {
			//let bits: BitVec<u8> = BitVec::<u8, Lsb0>::from_slice(&args[2].as_bytes());
			encoder::encode(args[2].as_bytes().to_vec())
		},
		"-h" | "--help" => {
			println!("-p | --path (path to file) \n-s | --string (string to encode)");
		},
		"test" => {
			let s = [0];
			encoder::encode(s.to_vec());
			//let a: BitVec<u8> = BitVec::<u8, Lsb0>::from_slice(&s);
		},
		_ => {
			println!("-h for help");
		}
	}

}


