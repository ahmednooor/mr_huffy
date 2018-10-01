

use mr_huffy::helpers::*;

pub fn freq_map_to_bytes(freq_map: &Vec<(u8, u32)>) -> Vec<u8> {
    let mut freq_bytes: Vec<u8> = Vec::new();

    for item in freq_map {
        let u32_byte_arr = u32_to_u8_arr(item.1);
        freq_bytes.push(item.0);
        freq_bytes.push(u32_byte_arr[0]);
        freq_bytes.push(u32_byte_arr[1]);
        freq_bytes.push(u32_byte_arr[2]);
        freq_bytes.push(u32_byte_arr[3]);
    }

    freq_bytes
}
