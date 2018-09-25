

pub fn get_freq_map(input_bytes_vec: &Vec<u8>) -> Vec<(u8, u32)> {
    let mut freq_map: Vec<(u8, u32)> = Vec::new();

    for &byte in input_bytes_vec.iter() {
        let mut idx: Option<usize> = None;
        for (index, item) in freq_map.iter().enumerate() {
            if item.0 == byte {
                idx = Some(index);
            }
        }
        if idx.is_none() {
            freq_map.push((byte.clone(), 1));
        } else {
            freq_map[idx.unwrap()].1 += 1;
        }
    }

    freq_map
}
