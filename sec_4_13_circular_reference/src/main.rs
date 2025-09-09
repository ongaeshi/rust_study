use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    data: i32,
    child: Option<Rc<RefCell<Node>>>
}

fn main() {
    // for i in 0..10_000_000 {
    loop {
        let node1 = Rc::new(RefCell::new(Node {
            data: 1,
            child: None,
        }));
        let node2 = Rc::new(RefCell::new(Node {
            data: 2,
            child: None,
        }));
        let node3 = Rc::new(RefCell::new(Node {
            data: 3,
            child: None,
        }));
    
        // // これはOK
        // node1.borrow_mut().child = Some(Rc::clone(&node3));
        // node2.borrow_mut().child = Some(Rc::clone(&node3));

        // 以下のように書くと循環参照になってしまいメモリが解放されなくなってしまう。
        node1.borrow_mut().child = Some(Rc::clone(&node2));
        node2.borrow_mut().child = Some(Rc::clone(&node1));
    }
}
