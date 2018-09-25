

use std::collections::HashMap;
use mr_huffy::make_tree::*;

pub fn get_comrpessed_bits_map(tree: &Tree) -> HashMap<u8, String> {
    let mut comrpessed_bits_map: HashMap<u8, String> = HashMap::new();

    fn iterate_tree(tree: &Tree, 
                    bits: &mut String, 
                    bits_map: &mut HashMap<u8, String>) {
        if tree.left.is_none() && tree.right.is_none() && tree.byte != None {
            bits_map.insert(tree.byte.unwrap().clone(), bits.clone());
            return
        }
        
        if !tree.left.is_none() {
            bits.push('0');
            let left_child = match tree.left {
                Some(ref x) => { x },
                None => { tree }
            };
            iterate_tree(left_child, bits, bits_map);
            bits.pop();
        }
        
        if !tree.right.is_none() {
            bits.push('1');
            let right_child = match tree.right {
                Some(ref x) => { x },
                None => { tree }
            };
            iterate_tree(right_child, bits, bits_map);
            bits.pop();
        }
    }

    iterate_tree(tree, &mut String::new(), &mut comrpessed_bits_map);

    comrpessed_bits_map
}
