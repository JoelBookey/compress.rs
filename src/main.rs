use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::any::Any;

fn main() {
    let input = String::from("abcdefghijklmnopqrstuvwxyz now I know my abc next time will you sing with me");
    let q = get_queue(&input);
    
}

fn get_queue(input: &String) -> BinaryHeap<Huffman>{
    let mut weights: HashMap<char, u16> = HashMap::new();
    for c in input.chars() {
        if weights.contains_key(&c) {
            *weights.get_mut(&c).unwrap() += 1; 
        } else {
            weights.insert(c, 0);
        }
    }
    
    let mut q = BinaryHeap::new();
    weights.iter().for_each(|(key, val)| q.push(Huffman::Leaf(*key, *val)));
    q
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Huffman {
    Node(Box<Huffman>, Box<Huffman>),
    Leaf(char, u16)
}
