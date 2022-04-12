mod encoder;
use clap::Parser;
use std::fs::{read, write};
//use std::str;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	#[clap(short, long)]
	decode: bool,
    #[clap(short, long)]
    path: Option<String>,
	#[clap(short, long)]
	string: Option<String>,
	#[clap(short, long)]
	language: Option<String>
}

fn main() {
	test();
/* 	let cli = Cli::parse();
	let mut input: Vec<u8> = Vec::new();
	let mut lang_file: String = String::from("");
	
	if let Some(path) = cli.path.as_deref() {
		input = read(path).unwrap();
	}
	else if let Some(string) = cli.string.as_deref() {
		input = string.as_bytes().to_vec();
	}
	if let Some(language) = cli.language.as_deref() {
		lang_file = language.to_string();
	}
	if cli.decode {
		let output = encoder::decode(str::from_utf8(&input).unwrap().to_string(), lang_file);
		match write("./out.txt", output) {
			Ok(()) => {
				println!("DONE")
			},
			Err(..) => {
				println!("decode err")
			}
		}
	}
	else {
		let encode = encoder::encode(input, lang_file);
		println!("{}", encode);
	} */
}

fn test() {
	let input = read("./a").unwrap();
	match write("./in.txt", &input) {
		Ok(()) => {
			println!("DONE")
		},
		Err(..) => {
			println!("decode err")
		}
	}
	let encoded = encoder::encode(input, "".to_string());
	let decoded = encoder::decode(encoded, "".to_string());
	match write("./out.txt", decoded) {
		Ok(()) => {
			println!("DONE")
		},
		Err(..) => {
			println!("decode err")
		}
	}
}
/* fn main() {
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
			encoder::encode(args[2].as_bytes().to_vec());
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

} */


