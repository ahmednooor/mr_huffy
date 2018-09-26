

use mr_huffy::helpers::*;

pub fn freq_bytes_to_freq_map(freq_bytes: &mut Vec<u8>) -> Vec<(u8, u32)> {
    // let mut freq_bytes = freq_bytes.clone();
    let mut freq_map: Vec<(u8, u32)> = Vec::new();

    while freq_bytes.len() > 0 {
        freq_map.push((
            freq_bytes.remove(0),
            u8_arr_to_u32(
                [freq_bytes.remove(0),
                 freq_bytes.remove(0),
                 freq_bytes.remove(0),
                 freq_bytes.remove(0)]
            )
        ))
    }

    freq_map
}
