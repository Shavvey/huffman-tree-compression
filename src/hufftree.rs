use crate::minheap::{self, MinHeap, Node, Subtree};
use std::collections::hash_map::HashMap;

pub struct HuffTree {
    root: Subtree, // we can describe the tree as just a collection of subtrees
}

impl HuffTree {
    pub fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    // return len of entire tree, uses recursive call of subtree len() method
    pub fn len(&self) -> usize {
        self.root.len()
    }

    // print every node inside the huffman tree,
    // uses an pre-order traversal (parent-left-right)
    pub fn print(&self) {
        self.root.print();
    }

    // function to build the hufftree from
    // bare min_heap, we do just by inserting
    // intermediates nodes such that each node is put inside
    // one of these intermediate nodes
    pub fn build(min_heap: &mut MinHeap) -> Self {
        while min_heap.size != 1 {
            let left = min_heap.extract_min().unwrap();
            let right = min_heap.extract_min().unwrap();
            let mut node = Node::new('$', left.count + right.count);
            node.left = Subtree {
                root: Option::Some(Box::new(left)),
            };
            node.right = Subtree {
                root: Option::Some(Box::new(right)),
            };
            min_heap.insert(node);
        }
        let root_node = min_heap.extract_min().unwrap();
        Self {
            root: Subtree {
                root: Option::Some(Box::new(root_node)),
            },
        }
    }

    // creates a very simple string representation of the hufftree
    pub fn to_string(&self) -> String {
        self.root.to_string()
    }

    // print out the codes for all characters encountered in the text file
    pub fn print_codes(&self) {
        let tree_path: [u8; minheap::MAX_TREE_HEIGHT] = [0; minheap::MAX_TREE_HEIGHT];
        self.root.print_codes(tree_path, 0);
    }

    // get the item using it code (stored as a string of 0s and 1s for now)
    pub fn get_item(&self, bit_string: String) {
        self.root.get_item(bit_string);
    }

    // build the huffman tree just using the textfile
    pub fn from_file(file_name: &str) -> HuffTree {
        let mut min_heap = MinHeap::create_from_file(file_name);
        HuffTree::build(&mut min_heap)
    }

    // create a mapping of each character to its given bitstring
    pub fn to_map(&self) -> HashMap<char, String> {
        self.root.to_map()
    }
}
