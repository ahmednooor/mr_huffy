mod mr_huffy;

fn main() {
    let input_file_path = 
        String::from("C:\\Users\\Ahmed Noor\\Desktop\\rust_playground\\\
                      mr_huffy\\src\\test_files\\sample_1.txt");
    let output_file_path = 
        String::from("C:\\Users\\Ahmed Noor\\Desktop\\rust_playground\\\
                      mr_huffy\\src\\test_files\\encoded_sample_1.txt.hc1");
    let decoded_output_file_path = 
        String::from("C:\\Users\\Ahmed Noor\\Desktop\\rust_playground\\\
                      mr_huffy\\src\\test_files\\decoded_sample_1.txt");
        
    let _ = test_file(&input_file_path, 
                      &output_file_path, 
                      &decoded_output_file_path);
    
    
    let input_file_path = 
        String::from("C:\\Users\\Ahmed Noor\\Desktop\\rust_playground\\\
                      mr_huffy\\src\\test_files\\sample_2.png");
    let output_file_path = 
        String::from("C:\\Users\\Ahmed Noor\\Desktop\\rust_playground\\\
                      mr_huffy\\src\\test_files\\encoded_sample_2.png.hc1");
    let decoded_output_file_path = 
        String::from("C:\\Users\\Ahmed Noor\\Desktop\\rust_playground\\\
                      mr_huffy\\src\\test_files\\decoded_sample_2.png");
        
    let _ = test_file(&input_file_path, 
                      &output_file_path, 
                      &decoded_output_file_path);
}

fn test_file(input_file_path: &str, 
             output_file_path: &str, 
             decoded_output_file_path: &str) -> bool {
    
    use std::time::{SystemTime, UNIX_EPOCH};

    println!("---------------------------------------------------------------");
    println!("Testing File: '{}'", input_file_path);
    println!("");

    let et1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let is_encoded = 
        mr_huffy::encode_file(input_file_path, output_file_path);
    let et2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    if is_encoded.0 == 0 {
        println!("Encoding Succcessful!");
        println!("Time Took: \t{:?}", et2 - et1);
    } else {
        println!("Encoding Failed!");
        return false;
    }

    let dt1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let is_decoded = 
        mr_huffy::decode_file(output_file_path, decoded_output_file_path);
    let dt2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    if is_decoded.0 == 0 {
        println!("Decoding Succcessful!");
        println!("Time Took: \t{:?}", dt2 - dt1);
    } else {
        println!("Decoding Failed!");
        return false;
    }

    let input_bytes = read_bytes(input_file_path).unwrap();
    let encoded_bytes = read_bytes(output_file_path).unwrap();
    let decoded_bytes = read_bytes(decoded_output_file_path).unwrap();

    let is_input_and_decoded_same = input_bytes == decoded_bytes;
    println!("");
    println!("Tests:");
    println!("funcs: ['encode_file', 'decode_file']: \t{}", is_input_and_decoded_same);
    // println!("Input And Decoded Bytes Same? \t\t{}", is_input_and_decoded_same);
    
    let perc_compressed: f64 = 
        (100f64 - 
            ( ( encoded_bytes.len() as f64 / input_bytes.len() as f64 ) 
            * 100f64 )
        ) as f64;
    
    println!("");
    println!("Stats:");
    println!("Input File Size: \t {} bytes", input_bytes.len());
    println!("Encoded File Size: \t {} bytes", encoded_bytes.len());
    println!("Decoded File Size: \t {} bytes", decoded_bytes.len());
    println!("Percentage Compressed: \t {:.2} %", perc_compressed);
    println!("---------------------------------------------------------------");

    true
}

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_bytes(file_path: &str) -> io::Result<Vec<u8>> {
    /* taken from,
    https://doc.rust-lang.org/std/io/trait.Read.html
    */
    let mut f = File::open(file_path)?;
    let mut buffer: Vec<u8> = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}
