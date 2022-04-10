//use std::io::{stdin, Read};
use bitvec::prelude::*;
//base64: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
pub fn encode(input: Vec<u8>, lang: String) {
    //stdin().read_to_end(&mut s).unwrap();
    let bits: BitVec<u8, Msb0> = BitVec::<u8, Msb0>::from_vec(input);
    let charvalues = sextet_split(bits);
    let encoded = value_to_chars(charvalues, get_language(lang));
    println!("{}", encoded);
}

/// Splits bitvec into 6-bit chunks then shoves them into `u8`.
fn sextet_split(mut bits: BitVec<u8, Msb0>) -> Vec<u8> {
    // Check how much input padding is needed, and add it.
    let padding: usize = (6 - bits.len() % 6) % 6;
    for _ in 0..padding {
        bits.push(false);
    }

    // Split, then shove into `u8`.
    let mut bits_slice: Vec<u8> = Vec::new();
    // Amount of bits we have.
    let nbits: usize = bits.len();
    // Slice bits into sextets, then change to `u8` (char values) and shove char values into a vec.
    for x in (0..nbits).step_by(6) {
        let u: &BitSlice<u8, Msb0> = &bits[x..x + 6];
        let v: u8 = u.load_be::<u8>();
        bits_slice.push(v);
    }
    bits_slice
}

/// Takes a list of 6-bit values to a base-64 string.
fn value_to_chars(charvalues: Vec<u8>, lang: String) -> String {
    // Check how much output padding is needed.
    let padding: usize = (4 - charvalues.len() % 4) % 4;

    let language: String = lang.to_owned();
    let dictionary: Vec<char> = language.chars().collect();
    assert_eq!(dictionary.len(), 64);
    let mut encode: String = String::new();
    for i in charvalues {
        assert!(i < 0x40);
        encode.push(dictionary[i as usize]);
    }

    // Add output padding.
    for _ in 0..padding {
        encode.push('=');
    }

    encode
}

fn get_language(file: String) -> String {
    match std::fs::read_to_string(file) {
        Err(_) => {
            let base64: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string();
            return base64;
        },
        Ok(lang) => {
            return lang;
        }
    }
}