
use std::collections::HashMap;

pub fn make_bitstring(comrpessed_bits_map: &HashMap<u8, String>, 
                      input_bytes: &Vec<u8>) -> String {
    let mut bitstring = String::new();

    for byte in input_bytes {
        bitstring.push_str(
            comrpessed_bits_map.get(byte).unwrap()
        );
    }

    bitstring
}
