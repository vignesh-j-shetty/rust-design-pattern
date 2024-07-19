use std::cell::RefCell;

#[derive(Debug)]
struct Node<'a> {
    val: RefCell<i32>,
    adjacent: Vec<& 'a Node<'a>>
}

fn add_one(a: &Node) {
    let mut r = a.val.borrow_mut();
    *r = 320;
    for adj in a.adjacent.iter() {
        add_one(adj);
    }
}

fn main() {
    let b = Node {
        val: RefCell::new(2),
        adjacent:vec![]
    };
    let c = Node  {
        val: RefCell::new(3),
        adjacent: vec![]
    };
    let a = Node {
        val: RefCell::new(1),
        adjacent: vec![&b, &c]
    };

    add_one(&a);
    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}