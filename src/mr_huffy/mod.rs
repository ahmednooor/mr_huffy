mod helpers; use self::helpers::*;

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


pub fn encode(input_bytes: &mut Vec<u8>) -> Vec<u8> {
    // use std::time::{SystemTime, UNIX_EPOCH};
    // let et1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // let et2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // println!("[]Time Took: \t{:?}", et2 - et1);

    if input_bytes.len() < 2 { return input_bytes.clone(); }

    let freq_map = get_freq_map(&input_bytes);

    let tree = make_tree(&freq_map);    

    let comrpessed_bits_map = get_comrpessed_bits_map(&tree);

    let compressed_bitstring = make_bitstring(&comrpessed_bits_map, 
                                              &input_bytes);
    let encoded_bytes = bitstring_to_encoded_bytes(compressed_bitstring);

    let freq_bytes = freq_map_to_bytes(&freq_map);

    let encoded_output_bytes = 
        comb_freq_and_encoded_bytes(&freq_bytes, 
                                    &encoded_bytes);

    // let mut decoded_bytes = decode(&mut encoded_output_bytes.clone());
    // println!("['encode', 'decode']: {}", input_bytes == (&mut decoded_bytes));
    
    encoded_output_bytes
}


pub fn encode_file(input_file_path: &str, 
                   output_file_path: &str) -> (u8, String) {
    
    let mut input_bytes = match read_bytes(&input_file_path) {
        Err(_) => return (1, String::from(
                "[ERR 'encode_file']: Could Not Read Input File!"
            )),
        Ok(bytes) => bytes
    };
    let mut encoded_output_bytes = encode(&mut input_bytes);
    
    let is_outfile_written = 
        write_bytes(&output_file_path, &mut encoded_output_bytes);
    
    if is_outfile_written.is_err() {
        return (2, String::from(
                "[ERR 'encode_file']: Could Not Write Output File!"
            ));
    }

    (0, String::from("File Encoded Successfully!"))
}


pub fn decode(encoded_input_bytes: &mut Vec<u8>) -> Vec<u8> {
    // use std::time::{SystemTime, UNIX_EPOCH};
    // let et1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // let et2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // println!("[]Time Took: \t{:?}", et2 - et1);

    if encoded_input_bytes.len() < 2 { return encoded_input_bytes.clone(); }
    
    let (mut freq_bytes, encoded_bytes) = 
        sep_freq_and_encoded_bytes(encoded_input_bytes);
    
    let freq_map = freq_bytes_to_freq_map(&mut freq_bytes);
    
    let tree = make_tree(&freq_map);
    
    let compressed_bitstring = encoded_bytes_to_bitstring(&encoded_bytes);
    
    let decoded_bytes = 
        decoded_bytes_from_bitstring_and_tree(&compressed_bitstring, 
                                              &tree);
    decoded_bytes
}


pub fn decode_file(input_file_path: &str, 
                   output_file_path: &str) -> (u8, String) {
    
    let mut input_bytes = match read_bytes(&input_file_path) {
        Err(_) => return (1, String::from(
                "[ERR 'decode_file']: Could Not Read Input File!"
            )),
        Ok(bytes) => bytes
    };
    let mut encoded_output_bytes = decode(&mut input_bytes);
    
    let is_outfile_written = 
        write_bytes(&output_file_path, &mut encoded_output_bytes);
    
    if is_outfile_written.is_err() {
        return (2, String::from(
                "[ERR 'decode_file']: Could Not Write Output File!"
            ));
    }

    (0, String::from("File Decoded Successfully!"))
}
