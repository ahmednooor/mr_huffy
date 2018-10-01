

use mr_huffy::helpers::*;

pub fn bitstring_to_encoded_bytes(mut bitstring: String) -> Vec<u8> {
    bitstring = add_offset_to_bitstring(bitstring);

    let mut bytes_vector: Vec<u8> = Vec::new();
    let mut index: usize = 0;
    let bitstring_len = bitstring.len();
    while index + 8 <= bitstring_len {
        bytes_vector.push(
            u8_from_bitstring(&bitstring[index..index+8])
        );
        index += 8;
    }

    bytes_vector
}
