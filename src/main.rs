use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::any::Any;

fn main() {
    let input = String::from("abcdefghijklmnopqrstuvwxyz now I know my abc next time will you sing with me");
    let q = get_queue::<Leaf, Leaf>(&input);
    assert!(Leaf::new('3', 5) > Leaf::new('s', 2));  
    let tree = generate_tree(q);
    
}

fn get_queue<A, B>(input: &String) -> BinaryHeap<HeapValue<A, B>> 
    where A: Child + Ord + PartialOrd + Eq + PartialEq, 
        B: Child + Ord + PartialOrd + Eq + PartialEq 
{
    let mut weights: HashMap<char, u32> = HashMap::new();
    for c in input.chars() {
        if weights.contains_key(&c) {
            *weights.get_mut(&c).unwrap() += 1; 
        } else {
            weights.insert(c, 1);
        }
    }
    
    let mut q = BinaryHeap::new();
    weights.iter().for_each(|(key, val)| q.push(HeapValue::Leaf(Leaf::new(*key, *val))));
    q
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Leaf {
    symbol: char,
    weight: u32,
}

impl std::cmp::PartialOrd for Leaf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Leaf {
    fn new(symbol: char, weight: u32) -> Self {
        Self { symbol, weight }
    }


}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Node<L, R> where L: Child, R: Child {
    weight: u32,
    left: Box<L>,
    right: Box<R>,
}

trait Child {
    fn weight(&self) -> u32;
    fn as_any(&self) -> &dyn Any;
}

impl<L, R> Child for Node<L, R> where L: Child, R: Child {
    fn weight(&self) -> u32 {
        self.left.weight() + self.right.weight()   
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Child for Leaf {
    fn weight(&self) -> u32 {
        self.weight
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<L, R> Node<L, R> where L: Child, R: Child {
    fn new(left: Box<L>, right: Box<R>) -> Node<L, R> {
        let mut n = Node { weight: 0, left: left, right: right };
        n.set_weight();
        n
    }

    fn set_weight(&mut self) {
        let mut new_w: u32 = 0;
        new_w += self.left.weight();
        new_w += self.right.weight();
    }

}


#[derive(Ord, Eq, PartialEq, PartialOrd)]
enum HeapValue<L, R> where L: Child, R: Child {
    Leaf(Leaf),
    Node(Node<L, R>),
}


fn generate_tree<A, B, L, R>(mut leaves: BinaryHeap<HeapValue<A, B>> ) -> Node<L, R> 
    where L: Child, R: Child,
        A: Child + Ord + PartialOrd + Eq + PartialEq, 
        B: Child + Ord + PartialOrd + Eq + PartialEq 
    {
    if leaves.len() < 2 {
        panic!("leaves len was less than two");
    }
    while leaves.len() > 2 {
        let right = leaves.pop().unwrap();
        let left = leaves.pop().unwrap();
        let tree = HeapValue::Node(Node::new(Box::new(left), Box::new(right)));
        leaves.push(tree)
    }


    let mut right = unwrap_heapval(leaves.pop().unwrap());
    if let Some(l) = right.as_any().downcast_ref::<Leaf>() {
        right = l;
    } else {
        right = right.as_any().downcast_ref::<Node>().unwrap()
    }
    let left = unwrap_heapval(leaves.pop().unwrap());
    Node::new(left, right)

    
} 

fn unwrap_heapval<A, B>(val: HeapValue<A, B>) -> Box<dyn Child> where A: Child, B: Child {
    let weird = match val {
        HeapValue::Leaf(l) => Box::new(l) as Box<dyn Child>,
        HeapValue::Node(n) => Box::new(n) as Box<dyn Child>,
    };

}


// if len > 1
// make first two nodes into sub tree
//        4
//       / \
//      A   B
//
// join with previous subtree
//              
//          ABCD
//         /    \
//        /      \ 
//       A+B     C+D
//       / \     / \
//      A   B   C   D
//
//  or 
//
//      ABCDEF
//     /      \
//    EF      ABCD
//
//  or 
//
//      ABCDEFGH
//     /        \
//   EFGH      ABCD
//
//  if len == 1 
//
//      ABCDEFGHI
//     /         \
//   EFGHI      ABCD
//  /     \
// I       EFGH 
