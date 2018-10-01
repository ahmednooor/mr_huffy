

use mr_huffy::helpers::*;

pub fn comb_freq_and_encoded_bytes(freq_bytes: &Vec<u8>, 
                                   encoded_bytes: &Vec<u8>) -> Vec<u8> {
    let mut final_output_bytes: Vec<u8> = Vec::new();
    let freq_bytes_len = u32_to_u8_arr(freq_bytes.len() as u32);

    final_output_bytes.push(freq_bytes_len[0]);
    final_output_bytes.push(freq_bytes_len[1]);
    final_output_bytes.push(freq_bytes_len[2]);
    final_output_bytes.push(freq_bytes_len[3]);

    for byte in freq_bytes {
        final_output_bytes.push(byte.clone());
    }
    for byte in encoded_bytes {
        final_output_bytes.push(byte.clone());
    }

    final_output_bytes
}
