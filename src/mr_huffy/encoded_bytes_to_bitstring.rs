

use mr_huffy::helpers::*;

pub fn encoded_bytes_to_bitstring(encoded_bytes: &Vec<u8>) -> String {
    let mut bitstring = String::new();
    
    for byte in encoded_bytes {
        bitstring.push_str(
            u8_to_bitstring(byte.clone()).as_str()
        );
    }

    bitstring = remove_offset_from_bitstring(&bitstring);
    bitstring
}
