

pub fn u32_from_bitstring(bitstring: &str) -> u32 {
    let bitstring = String::from(bitstring)
        .chars().rev().collect::<String>();
    
    let mut power: u64 = 1;
    let mut return_int: u32 = 0;

    for bit in bitstring.chars() {
        if bit == '1' {
            return_int += power as u32;
        }
        power = power * 2;
    }

    return_int
}

pub fn u8_from_bitstring(bitstring: &str) -> u8 {
    let bitstring = String::from(bitstring)
        .chars().rev().collect::<String>();
    
    let mut power: u32 = 1;
    let mut return_int: u8 = 0;

    for bit in bitstring.chars() {
        if bit == '1' {
            return_int += power as u8;
        }
        power = power * 2;
    }

    return_int
}

pub fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    /* taken from,
    https://users.rust-lang.org/t/how-to-serialize-a-u32-into-byte-array/986/5
    */

    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;

    [b1, b2, b3, b4]
}

pub fn remove_offset_from_bitstring(bitstring: &String) -> String {
    let mut offset_index: usize = 0;
    
    for (index, ch) in bitstring.chars().enumerate() {
        if ch == '1' {
            offset_index = index + 1;
            break;
        }
    }

    let actual_bitstring: String;

    if offset_index < bitstring.len() {
        actual_bitstring = String::from(bitstring[offset_index..].to_owned());
    } else {
        actual_bitstring = String::from(bitstring[..].to_owned());
    }

    actual_bitstring
}

pub fn add_offset_to_bitstring(bitstring: &String) -> String {
    let mut bitstring = String::from("1".to_owned() + bitstring);
    let offset: u8 = (8 - bitstring.len() % 8) as u8;
    let mut offset_bits = String::new();
    
    if offset > 0 && offset < 8 {
        for _ in 0..offset {
            offset_bits.push('0');
        }
    }
    
    bitstring = String::from(offset_bits + &bitstring);
    
    bitstring
}

pub fn u8_to_bitstring(integer: u8) -> String {
    let mut bitstring = format!("{:b}", integer)
        .chars().rev().collect::<String>();

    let offset = {
        8 - bitstring.len()
    };

    for _ in 0..offset {
        bitstring.push('0')
    }
    
    bitstring = bitstring.chars().rev().collect::<String>();
    bitstring
}

pub fn u8_arr_to_u32(u8_arr: [u8; 4]) -> u32 {
    let mut u32_bitstring = String::new();

    for integer in u8_arr.iter() {
        u32_bitstring.push_str(
            u8_to_bitstring(integer.clone()).as_str()
        )
    }

    let u32int: u32 = u32_from_bitstring(&u32_bitstring[..]);
    u32int
}

// fn __u8_to_bitstring(integer: u8) -> String {
//     let mut integer = integer;
//     let mut bitstring = String::new();
    
//     while integer > 1 {
//         bitstring.push_str(
//             (integer % 2).to_string().as_str()
//         );
//         integer = (integer / 2) as u8;
//     }
//     bitstring.push_str(
//         integer.to_string().as_str()
//     );
    
//     let offset = {
//         8 - bitstring.len()
//     };

//     for _ in 0..offset {
//         bitstring.push('0')
//     }
    
//     bitstring = bitstring.chars().rev().collect::<String>();
//     bitstring
// }
