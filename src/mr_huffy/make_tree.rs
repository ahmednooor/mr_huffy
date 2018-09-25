

pub fn make_tree(freq_map: &Vec<(u8, u32)>) -> Tree {
    let mut tree: Vec<Tree> = Vec::new();
    
    for (byte, freq) in freq_map {
        tree.push(Tree {
            byte: Some(byte.clone()),
            freq: freq.clone(),
            left: None,
            right: None
        });
    }
    
    while tree.len() != 1 {
        let mut min_1_key: Option<usize> = None;
        let mut min_2_key: Option<usize> = None;
        let min_1_node: Tree;
        let min_2_node: Tree;

        for (index, _) in tree.iter().enumerate() {
            if min_1_key == None {
                min_1_key = Some(index);
            }
            if tree[index].freq < tree[min_1_key.unwrap()].freq {
                min_1_key = Some(index);
            }
        }

        min_1_node = tree.remove(min_1_key.unwrap());
        
        for (index, _) in tree.iter().enumerate() {
            if min_2_key == None {
                min_2_key = Some(index);
            }
            if tree[index].freq < tree[min_2_key.unwrap()].freq {
                min_2_key = Some(index);
            }
        }

        min_2_node = tree.remove(min_2_key.unwrap());
        
        tree.push(
            Tree {
                byte: None,
                freq: min_1_node.freq.clone() + min_2_node.freq.clone(),
                left: Some(Box::new(min_1_node)),
                right: Some(Box::new(min_2_node))
            }
        );
    }

    tree.into_iter().nth(0).unwrap()
}

#[derive(Debug)]
pub struct Tree {
    pub byte: Option<u8>,
    pub freq: u32,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>
}
