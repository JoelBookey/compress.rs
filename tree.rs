use bit_vec::BitVec;
use std::collections::BinaryHeap;
use std::collections::HashMap;


pub fn pretty_print(node: &HuffmanTree, i: u16) {
    if let &HuffmanTree::Leaf(c, w) = node {
        println!("{:^1$}{c}: {w}", ' ', i as usize, c = c as char);
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
    Node(Option<u16>, Box<HuffmanTree>, Box<HuffmanTree>),
    Leaf(char, Option<u16>),
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
        let mut weights: HashMap<char, u16> = HashMap::new();
        for c in input.chars() {
            if let std::collections::hash_map::Entry::Vacant(e) = weights.entry(c) {
                e.insert(1);
            } else {
                *weights.get_mut(&c).unwrap() += 1;
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
                    Some(l_weight + r_weight),
                    Box::new(left),
                    Box::new(right),
                ))
            } else {
                queue.push(HuffmanTree::Node(
                    Some(l_weight + r_weight),
                    Box::new(right),
                    Box::new(left),
                ))
            }
        }

        let left = queue.pop().unwrap();
        let right = queue.pop().unwrap();
        let weight = left.weight() + right.weight();
        HuffmanTree::Node(Some(weight), Box::new(left), Box::new(right))

    }

    pub fn reconstruct(v: &mut BitVec) -> Self {
        //let len = pop_byte(&mut BitVec).unwrap();
        match v.pop().unwrap() {
            true => Self::Leaf(pop_byte(v).unwrap() as char, None),
            false => Self::Node(None, Box::new(Self::reconstruct(v)), Box::new(Self::reconstruct(v)))
        }
    }

    pub fn deconstructed(&self) -> BitVec {
        let mut vec = BitVec::new();
        //vec.push(self.get_lookup_table().len() as u8);
        self.deconstruct(&mut vec);
        vec.iter().rev().collect()
    }

    fn deconstruct(&self, v: &mut Vec) {
        match self {
            HuffmanTree::Node(_w, l, r) => {
                v.push(false);
                l.deconstruct(v);
                r.deconstruct(v);
            }
            HuffmanTree::Leaf(_c, _w) => {
                v.push(true);
                v.append(BitVec::from_bytes(&[rev_byte(c)]));
            }
        }
    }

    fn weight(&self) -> u16 {
        match self {
            HuffmanTree::Node(_w, l, r) => l.weight() + r.weight(),
            HuffmanTree::Leaf(_, w) => *(w.expect("no weight set")),
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
}

    


fn pop_byte(v: &mut BitVec) -> Option<u8> {
    v.to_bytes().last().map(|val| {
        for i in 0..8 {
            let _ = v.pop();
        }
        *val
    })
}

fn rev_byte(byte: u8) -> u8 {
    BitVec::from_bytes(&[byte]).iter().rev().collect::<BitVec<u32>>().to_bytes().get(0).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pop_byte() {
        let vec = BitVec::from_bytes(&[0b10011001, 0b11111111]);
        assert_eq!(pop_byte(&mut vec), 255);
    }
    
    #[test]
    fn test_rev_byte() {
        assert_eq!(rev_byte(b11110000), b00001111);
    }
}
