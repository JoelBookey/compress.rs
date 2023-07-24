use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::io::Write;
use bit_vec::BitVec;

fn main() -> Result<(), std::io::Error>{
    let input = std::fs::read_to_string("input.txt")?;
    let tree = generate_tree(&input);
   pretty_print(&tree, 0);
    let table = tree.get_lookup_table();
  //  println!("{:?}", table);
    let message = encode_message_from_table(&input, &table);
    //println!("{:?}", message);
    write_bits_to_file("output", &message)?;

    Ok(())
}


fn encode_message_from_table(input: &String, table: &HashMap<char, BitVec>) -> BitVec {
    let mut output = BitVec::new();
    for c in input.chars() {
        output.append(&mut table.get(&c).expect("char not in table").clone());
    }

    output
}

fn write_bits_to_file(f_name: &str, v: &BitVec) -> Result<(), std::io::Error>{
    let mut file = std::fs::File::create(f_name)?;
    file.write_all(v.to_bytes().as_slice())?;

    Ok(())
}

fn generate_tree(input: &String) -> HuffmanTree {
    make_tree(get_queue(input))
}

fn pretty_print(node: &HuffmanTree, i: u16) {
    if let &HuffmanTree::Leaf(c, w) = node {
        println!("{:^1$}{c}: {w}", ' ', i as usize);
    } else if let HuffmanTree::Node(w, l, r) = node {
        println!("{:^1$}{w}", ' ', i as usize);
        pretty_print(l, i + 1);
        pretty_print(r, i + 1);
    }
}

fn get_queue(input: &String) -> BinaryHeap<HuffmanTree>{
    let mut weights: HashMap<char, u16> = HashMap::new();
    for c in input.chars() {
        if weights.contains_key(&c) {
            *weights.get_mut(&c).unwrap() += 1; 
        } else {
            weights.insert(c, 1);
        }
    }
    
    let mut q = BinaryHeap::new();
    weights.iter().for_each(|(key, val)| q.push(HuffmanTree::Leaf(*key, *val)));
    q
}

const LEFT: bool = false;
const RIGHT: bool = true;

#[derive(Debug, PartialEq, Eq)]
enum HuffmanTree {
    Node(u16, Box<HuffmanTree>, Box<HuffmanTree>),
    Leaf(char, u16)
}

use std::cmp;
impl Ord for HuffmanTree {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let own_prob = self.weight();
        let other_prob = other.weight();

        if own_prob > other_prob {
            cmp::Ordering::Less
        } else if own_prob == other_prob {
            cmp::Ordering::Equal
        } else {
            cmp::Ordering::Greater
        }
    }
}

impl PartialOrd for HuffmanTree {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl HuffmanTree {
    fn weight(&self) -> u16 {
        match self {
            HuffmanTree::Node(_w, l, r) => l.weight() + r.weight(),
            HuffmanTree::Leaf(_, w) => *w
        }
    } 

    fn get_lookup_table(&self) -> HashMap<char, BitVec> {
        let mut out = HashMap::new();
        let _ = self.get_lookup_table_inner(&mut out, &BitVec::new());
        out
    }
    fn get_lookup_table_inner(&self, table: &mut HashMap<char, BitVec>, prev: &BitVec) -> Option<char> {
        if let HuffmanTree::Leaf(c, _w) = self {
            return Some(*c);
        } else if let HuffmanTree::Node(_weight, left, right) = self {
            let mut left_branch = prev.clone();
            left_branch.push(LEFT);
            if let Some(c) = left.get_lookup_table_inner(table, &left_branch) {
                table.insert(c, left_branch);
            }
            let mut right_branch = prev.clone();
            right_branch.push(RIGHT);
            if let Some(c) = right.get_lookup_table_inner(table, &right_branch) {
                table.insert(c, right_branch);
            }

        }
        None
    }
}

fn make_tree(mut heap: BinaryHeap<HuffmanTree>) -> HuffmanTree {
    while heap.len() > 2 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let l_weight = left.weight();
        let r_weight = right.weight();
        if l_weight < r_weight {
            heap.push(HuffmanTree::Node(l_weight + r_weight, Box::new(left), Box::new(right)))   
        } else {
           heap.push(HuffmanTree::Node(l_weight + r_weight, Box::new(right), Box::new(left)))   
        }
        

    }

    let left = heap.pop().unwrap();
    let right = heap.pop().unwrap();
    let weight = left.weight() + right.weight();
    HuffmanTree::Node(weight, Box::new(left), Box::new(right))  
}
