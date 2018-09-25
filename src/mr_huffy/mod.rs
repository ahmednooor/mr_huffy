mod helpers;

mod get_freq_map; use self::get_freq_map::*;
mod make_tree; use self::make_tree::*;
mod get_comrpessed_bits_map; use self::get_comrpessed_bits_map::*;
mod make_bitstring; use self::make_bitstring::*;
mod bitstring_to_encoded_bytes; use self::bitstring_to_encoded_bytes::*;
mod freq_map_to_bytes; use self::freq_map_to_bytes::*;
mod comb_freq_and_encoded_bytes; use self::comb_freq_and_encoded_bytes::*;

mod sep_freq_and_encoded_bytes; use self::sep_freq_and_encoded_bytes::*;
mod freq_bytes_to_freq_map; use self::freq_bytes_to_freq_map::*;
mod encoded_bytes_to_bitstring; use self::encoded_bytes_to_bitstring::*;
mod decoded_bytes_from_bitstring_and_tree; 
    use self::decoded_bytes_from_bitstring_and_tree::*;

use std::io;
use std::io::prelude::*;
use std::fs::File;


pub fn encode(input_bytes: &Vec<u8>) -> Vec<u8> {
    if input_bytes.len() < 2 { return input_bytes.clone(); }

    let freq_map = get_freq_map(&input_bytes);
    // println!("{:?}", freq_map);

    let tree = make_tree(&freq_map);
    // println!("{:?}", tree);

    let comrpessed_bits_map = get_comrpessed_bits_map(&tree);
    // println!("{:?}", comrpessed_bits_map);

    let compressed_bitstring = make_bitstring(&comrpessed_bits_map, 
                                              &input_bytes);
    // println!("{}", compressed_bitstring);

    let encoded_bytes = bitstring_to_encoded_bytes(&compressed_bitstring);
    // println!("{:?}", encoded_bytes);

    let freq_bytes = freq_map_to_bytes(&freq_map);
    // println!("{:?}", freq_bytes);

    let encoded_output_bytes = 
        comb_freq_and_encoded_bytes(&freq_bytes, 
                                    &encoded_bytes);
    // println!("{:?}", encoded_output_bytes);
    
    encoded_output_bytes
}

pub fn encode_file(input_file_path: &str, 
                   output_file_path: &str) -> (u8, String) {
    
    let input_bytes = match read_bytes(&input_file_path) {
        Err(_) => return (1, String::from(
                "[ERR 'encode_file']: Could Not Read Input File!"
            )),
        Ok(bytes) => bytes
    };
    let mut encoded_output_bytes = encode(&input_bytes);
    
    let is_outfile_written = 
        write_bytes(&output_file_path, &mut encoded_output_bytes);
    
    if is_outfile_written.is_err() {
        return (2, String::from(
                "[ERR 'encode_file']: Could Not Write Output File!"
            ));
    }

    (0, String::from("File Encoded Successfully!"))
}

pub fn decode(encoded_input_bytes: &Vec<u8>) -> Vec<u8> {
    if encoded_input_bytes.len() < 2 { return encoded_input_bytes.clone(); }
    
    let (freq_bytes, encoded_bytes) = 
        sep_freq_and_encoded_bytes(&encoded_input_bytes);
    // println!("{:?}", freq_bytes);
    // println!("{:?}", encoded_bytes);

    let freq_map = freq_bytes_to_freq_map(&freq_bytes);
    // println!("{:?}", freq_map);
    
    let tree = make_tree(&freq_map);
    // println!("{:#?}", tree);

    let compressed_bitstring = encoded_bytes_to_bitstring(&encoded_bytes);
    // println!("{}", compressed_bitstring);

    let decoded_bytes = 
        decoded_bytes_from_bitstring_and_tree(&compressed_bitstring, 
                                              &tree);
    // println!("{:?}", decoded_bytes);

    decoded_bytes
}

pub fn decode_file(input_file_path: &str, 
                   output_file_path: &str) -> (u8, String) {
    
    let input_bytes = match read_bytes(&input_file_path) {
        Err(_) => return (1, String::from(
                "[ERR 'decode_file']: Could Not Read Input File!"
            )),
        Ok(bytes) => bytes
    };
    let mut encoded_output_bytes = decode(&input_bytes);
    
    let is_outfile_written = 
        write_bytes(&output_file_path, &mut encoded_output_bytes);
    
    if is_outfile_written.is_err() {
        return (2, String::from(
                "[ERR 'decode_file']: Could Not Write Output File!"
            ));
    }

    (0, String::from("File Decoded Successfully!"))
}


pub fn read_bytes(file_path: &str) -> io::Result<Vec<u8>> {
    /* taken from,
    https://doc.rust-lang.org/std/io/trait.Read.html
    */
    let mut f = File::open(file_path)?;
    let mut buffer: Vec<u8> = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn write_bytes(file_path: &str, bytes: &mut Vec<u8>) -> io::Result<()> {
    /* taken from,
    https://doc.rust-lang.org/std/fs/struct.File.html
    */
    let mut f = File::create(file_path)?;

    // write the whole buffer
    f.write_all(bytes)?;
    Ok(())
}
