#![warn(dead_code)]
use bit_vec::BitVec;
use std::collections::BinaryHeap;
use std::collections::HashMap;


pub fn pretty_print(node: &HuffmanTree, i: u16) {
    if let &HuffmanTree::Leaf(c, w) = node {
        println!("{:^1$}{c}: {w}", ' ', i as usize);
    } else if let HuffmanTree::Node(w, l, r) = node {
        println!("{:^1$}{w}", ' ', i as usize);
        pretty_print(l, i + 1);
        pretty_print(r, i + 1);
    }
}


const LEFT: bool = false;
const RIGHT: bool = true;

#[derive(Debug, PartialEq, Eq)]
pub enum HuffmanTree {
    Node(u16, Box<HuffmanTree>, Box<HuffmanTree>),
    Leaf(u8, u16),
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
    
    pub fn from_str(input: &str) -> Self {
        let mut weights: HashMap<u8, u16> = HashMap::new();
        for c in input.chars() {
            if let std::collections::hash_map::Entry::Vacant(e) = weights.entry(c as u8) {
                e.insert(1);
            } else {
                *weights.get_mut(&(c as u8)).unwrap() += 1;
            }
        }

        let mut queue = BinaryHeap::new();
        weights
            .iter()
            .for_each(|(key, val)| queue.push(HuffmanTree::Leaf(*key, *val)));


        while queue.len() > 2 {
            let left = queue.pop().unwrap();
            let right = queue.pop().unwrap();
            let l_weight = left.weight();
            let r_weight = right.weight();
            if l_weight < r_weight {
                queue.push(HuffmanTree::Node(
                    l_weight + r_weight,
                    Box::new(left),
                    Box::new(right),
                ))
            } else {
                queue.push(HuffmanTree::Node(
                    l_weight + r_weight,
                    Box::new(right),
                    Box::new(left),
                ))
            }
        }

        let left = queue.pop().unwrap();
        let right = queue.pop().unwrap();
        let weight = left.weight() + right.weight();
        HuffmanTree::Node(weight, Box::new(left), Box::new(right))

    }

    pub fn from_bitvec(v: BitVec) -> Self {
        HuffmanTree::Leaf(b'9', 10)
    }

    fn weight(&self) -> u16 {
        match self {
            HuffmanTree::Node(_w, l, r) => l.weight() + r.weight(),
            HuffmanTree::Leaf(_, w) => *w,
        }
    }

    pub fn get_lookup_table(&self) -> HashMap<u8, BitVec> {
        let mut out = HashMap::new();
        let _ = self.get_lookup_table_inner(&mut out, &BitVec::new());
        out
    }
    fn get_lookup_table_inner(
        &self,
        table: &mut HashMap<u8, BitVec>,
        prev: &BitVec,
    ) -> Option<u8> {
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
            output.append(&mut table.get(&(c as u8)).expect("u8 not in table").clone());
        }
        let r = 8 - (output.len() % 8);
        let mut output2 = BitVec::from_bytes(&[r as u8]);
        output2.append(&mut output);
        (0..r).for_each(|_| output2.push(false));
        output2
    }

    pub fn get_u8(&self, route: BitVec) -> Option<u8> {
        self.get_u8_inner(route.iter().rev().collect())
    }

    fn get_u8_inner(&self, mut route: BitVec) -> Option<u8> {
        match self {
            HuffmanTree::Leaf(c, _w) => Some(*c),
            HuffmanTree::Node(_w, l, r) => match route.pop() {
                Some(LEFT) => l.get_u8_inner(route),
                Some(RIGHT) => r.get_u8_inner(route),
                None => None,
            },
        }
    }

    pub fn decode_bits(&self, mut input: BitVec) -> String {
        let r = {
            let bytes = input.to_bytes();
            *bytes.first().unwrap()
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
            while self.get_u8(current_c.clone()).is_none() {
                if input.is_empty() {
                    return output;
                }
                current_c.push(input.pop().unwrap());
            }
            output.push(self.get_u8(current_c.clone()).unwrap() as char);
        }
    }
}

