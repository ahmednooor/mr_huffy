

use mr_huffy::make_tree::*;

pub fn decoded_bytes_from_bitstring_and_tree(bitstring: &String, 
                                         tree: &Tree) -> Vec<u8> {
    let mut actual_bytes: Vec<u8> = Vec::new();
    let mut current_node = tree;

    for bit in bitstring.chars() {
        if bit == '0' && !current_node.left.is_none() {

            current_node = match current_node.left {
                Some(ref x) => { x },
                None => { current_node }
            };

        }
        if bit == '1' && !current_node.right.is_none() {
            
            current_node = match current_node.right {
                Some(ref x) => { x },
                None => { current_node }
            };

        }
        if !current_node.byte.is_none() {
            
            actual_bytes.push(current_node.byte.unwrap());            
            current_node = tree;

        }
    }

    actual_bytes
}
