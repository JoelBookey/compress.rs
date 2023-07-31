#![warn(dead_code)]
use bit_vec::BitVec;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn generate_tree(input: &str) -> HuffmanTree {
    make_tree(get_queue(input))
}

pub fn pretty_print(node: &HuffmanTree, i: u16) {
    if let &HuffmanTree::Leaf(c, w) = node {
        println!("{:^1$}{c}: {w}", ' ', i as usize);
    } else if let HuffmanTree::Node(w, l, r) = node {
        println!("{:^1$}{w}", ' ', i as usize);
        pretty_print(l, i + 1);
        pretty_print(r, i + 1);
    }
}

fn get_queue(input: &str) -> BinaryHeap<HuffmanTree> {
    let mut weights: HashMap<char, u16> = HashMap::new();
    for c in input.chars() {
        if weights.contains_key(&c) {
            *weights.get_mut(&c).unwrap() += 1;
        } else {
            weights.insert(c, 1);
        }
    }

    let mut q = BinaryHeap::new();
    weights
        .iter()
        .for_each(|(key, val)| q.push(HuffmanTree::Leaf(*key, *val)));
    q
}

const LEFT: bool = false;
const RIGHT: bool = true;

#[derive(Debug, PartialEq, Eq)]
pub enum HuffmanTree {
    Node(u16, Box<HuffmanTree>, Box<HuffmanTree>),
    Leaf(char, u16),
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
            HuffmanTree::Leaf(_, w) => *w,
        }
    }

    pub fn get_lookup_table(&self) -> HashMap<char, BitVec> {
        let mut out = HashMap::new();
        let _ = self.get_lookup_table_inner(&mut out, &BitVec::new());
        out
    }
    fn get_lookup_table_inner(
        &self,
        table: &mut HashMap<char, BitVec>,
        prev: &BitVec,
    ) -> Option<char> {
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
    pub fn encode_message(&self, input: &str) -> BitVec {
        let table = self.get_lookup_table();
        let mut output = BitVec::new();
        for c in input.chars() {
            output.append(&mut table.get(&c).expect("char not in table").clone());
        }
        let r = 8 - (output.len() % 8);
        let mut output2 = BitVec::from_bytes(&[r as u8]);
        output2.append(&mut output);
        (0..r).for_each(|_| output2.push(false));
        output2
    }

    pub fn get_char(&self, route: BitVec) -> Option<char> {
        self.get_char_inner(route.iter().rev().collect())
    }

    fn get_char_inner(&self, mut route: BitVec) -> Option<char> {
        match self {
            HuffmanTree::Leaf(c, _w) => Some(*c),
            HuffmanTree::Node(_w, l, r) => match route.pop() {
                Some(LEFT) => l.get_char_inner(route),
                Some(RIGHT) => r.get_char_inner(route),
                None => None,
            },
        }
    }

    pub fn decode_bits(&self, mut input: BitVec) -> String {
        let r = {
            let bytes = input.to_bytes();
            *bytes.get(0).unwrap()
        };

        for _ in 0..r as usize {
            let _ = input.pop();
        }

        let mut output = String::new();
        let mut input: BitVec = input.iter().rev().collect();

        for _ in 0..8 {
            input.pop();
        }

        loop {
            let mut current_c = BitVec::new();
            while self.get_char(current_c.clone()).is_none() {
                if input.is_empty() {
                    return output;
                }
                current_c.push(input.pop().unwrap());
            }
            output.push(self.get_char(current_c.clone()).unwrap());
        }
    }
}

fn make_tree(mut heap: BinaryHeap<HuffmanTree>) -> HuffmanTree {
    while heap.len() > 2 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let l_weight = left.weight();
        let r_weight = right.weight();
        if l_weight < r_weight {
            heap.push(HuffmanTree::Node(
                l_weight + r_weight,
                Box::new(left),
                Box::new(right),
            ))
        } else {
            heap.push(HuffmanTree::Node(
                l_weight + r_weight,
                Box::new(right),
                Box::new(left),
            ))
        }
    }

    let left = heap.pop().unwrap();
    let right = heap.pop().unwrap();
    let weight = left.weight() + right.weight();
    HuffmanTree::Node(weight, Box::new(left), Box::new(right))
}