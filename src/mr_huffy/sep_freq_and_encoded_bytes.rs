

use mr_huffy::helpers::*;

pub fn sep_freq_and_encoded_bytes(input_bytes: &mut Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    // let mut input_bytes = input_bytes.clone();

    let freq_bytes_len: u32 = u8_arr_to_u32(
        [input_bytes.remove(0),
         input_bytes.remove(0),
         input_bytes.remove(0),
         input_bytes.remove(0)]
    );

    let freq_bytes: Vec<u8> = 
        input_bytes[..(freq_bytes_len as usize)].to_vec();
    
    let encoded_bytes: Vec<u8> = 
        input_bytes[(freq_bytes_len as usize)..].to_vec();

    (freq_bytes, encoded_bytes)
}
