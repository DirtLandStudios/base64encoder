//use std::io::{stdin, Read};
use bitvec::prelude::*;
//base64: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
pub fn encode(input: Vec<u8>, lang: String) -> String {
    //stdin().read_to_end(&mut s).unwrap();
    let bits: BitVec<u8, Msb0> = BitVec::<u8, Msb0>::from_vec(input);
    let charvalues = sextet_split(bits);
    let encoded: String = value_to_chars(charvalues, get_language(lang));
    return encoded;
}

pub fn decode(input: String, lang: String) -> Vec<u8> {
    let (values, padding) = chars_to_values(input.to_owned().chars().collect(), get_language(lang));
    let bits: BitVec<u8, Msb0> = BitVec::<u8, Msb0>::from_vec(values);
    let output = sextet_resplit(bits, padding);
    return output;
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
    return bits_slice;
}

fn sextet_resplit(bits: BitVec<u8, Msb0>, padding: usize) -> Vec<u8> {
    let mut bits_slice: BitVec<u8, Msb0> = BitVec::new();
    // Amount of bits we have.
    let nbits: usize = bits.len();
    //get the important 6 of each byte, and shove into vec
    for x in (0..nbits).step_by(8) {
        let u: &BitSlice<u8, Msb0> = &bits[x..x + 6];
        bits_slice.extend_from_bitslice(u);
    }
    // remove padding
    bits_slice.drain((bits.len() - padding)..bits.len());
    assert_eq!(bits_slice.len() % 8, 0);
    
    return bits_slice.into_vec();
}

/// Takes a list of 6-bit values to a base-64 string.
fn value_to_chars(charvalues: Vec<u8>, dictionary: Vec<char>) -> String {
    // Check how much output padding is needed.
    let padding: usize = (4 - charvalues.len() % 4) % 4;
    let mut encode: String = String::new();
    for i in charvalues {
        assert!(i < 0x40); //0x40 is 64 in hex
        encode.push(dictionary[i as usize]);
    }

    // Add output padding.
    for _ in 0..padding {
        encode.push('=');
    }

    return encode;
}

fn get_language(file: String) -> Vec<char> {
    let dictionary: Vec<char>;
    match std::fs::read_to_string(file) {
        Err(_) => {
            let base64: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                .to_string()
                .to_owned();
            dictionary = base64
                            .chars()
                            .collect();
        },
        Ok(lang) => {
            dictionary = lang
                            .to_owned()
                            .chars()
                            .collect();
            assert_eq!(dictionary.len(), 64);
        }
    }
    return dictionary;
}

fn chars_to_values(input: Vec<char>, dictionary:Vec<char>) -> (Vec<u8>, usize) {
    let mut padding = 0;
    let mut ouput: Vec<u8> = Vec::new();
    for x in input {
        if x != '=' {
            let pos: u8 = dictionary.iter().position(|&f| f == x).unwrap() as u8;
            ouput.push(pos);
        }
        else {
            padding += 2;
        }
    }
    return (ouput, padding);
}
