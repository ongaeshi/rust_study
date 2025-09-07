use std::rc::Rc;

struct Node {
    data: i32,
    child: Option<Rc<Node>>
}

fn print_link(start_node: Rc<Node>) {
    let mut p = start_node;
    loop {
        println!("{}", p.data);
        if p.child.is_none() {
            break;
        }
        // p = p.child.unwrap(); //=> cannot move out of `Rc`
        p = Rc::clone(p.child.as_ref().unwrap());
    }
}

fn main() {
    let node3 = Rc::new(Node {
        data: 3,
        child: None,
    });
    let node1 = Rc::new(Node {
        data: 1,
        child: Some(Rc::clone(&node3)),
    });
    let node2 = Rc::new(Node {
        data: 2,
        child: Some(Rc::clone(&node3)),
    });

    println!("link from node1");
    print_link(node1);

    println!("link from node2");
    print_link(node2);
}
