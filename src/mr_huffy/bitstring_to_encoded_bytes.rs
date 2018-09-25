

use mr_huffy::helpers::*;

pub fn bitstring_to_encoded_bytes(bitstring: &String) -> Vec<u8> {
    let offsetted_bitstring = add_offset_to_bitstring(bitstring);
    // println!("{}", offsetted_bitstring);

    let mut bytes_vector: Vec<u8> = Vec::new();
    let mut index: usize = 0;
    
    while index + 8 <= offsetted_bitstring.len() {
        bytes_vector.push(
            u8_from_bitstring(&offsetted_bitstring[index..index+8])
        );
        index += 8;
    }

    bytes_vector
}
