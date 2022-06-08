mod encoder;
use clap::Parser;
use std::fs::{read, write};
//use std::str;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	///flag to decode instead of encode
	#[clap(short, long)]
	decode: bool,
	///path to file to encode or decode
    #[clap(short, long)]
    path: Option<String>,
	///path to file to print output to
	#[clap(short, long)]
    outfile: Option<String>,
	///to use string instead of path
	#[clap(short, long)]
	string: Option<String>,
	///to not use default base 64 conversion table
	#[clap(short, long)]
	language: Option<String>
}

fn main() {
	let cli = Cli::parse();
	let mut input: Vec<u8> = Vec::new();
	let mut lang_file: String = String::from("");
	let mut out_file: String = String::from("./out.txt");

	if let Some(path) = cli.path.as_deref() {
		input = read(path).unwrap();
	} else if let Some(string) = cli.string.as_deref() {
		input = string.as_bytes().to_vec();
	}

	if let Some(outpath) = cli.path.as_deref() {
		out_file = outpath.to_string();
	}
	if let Some(language) = cli.language.as_deref() {
		lang_file = language.to_string();
	}

	if cli.decode {
		let output = encoder::decode(std::str::from_utf8(&input).unwrap().to_string(), lang_file);
		match write(out_file, output) {
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
		match write(out_file, encode) {
			Ok(()) => {
				println!("DONE")
			},
			Err(..) => {
				println!("encode err")
			}
		}
	}
}
/* 
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
	println!("{}", encoded);
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
 */