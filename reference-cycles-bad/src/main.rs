use crate::List::{Cons, Nil};
use std::cell::{Cell, Ref, RefCell};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    // RefCell to modify nodes and vector of each node shares ownership with children
    // which are also nodes
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
    // parent should own its children, children should not own parent, that would cause strong count to not reduce to 0 = stackoverflow
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
// Rc<T>::clone increases strong count of Rc<T>, Rc<T> instance is only cleaned up once strong_count reaches 0 of of scope.

// use Rc::downgrade to get pointer of Weak<T> , this doesnt take ownership

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "1leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "2branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "3leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("4leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "5leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}


// recap
// Box<T> known size, points to heap
// RC<T> keeps track of number of references on heap
// RefCell<T> with interior mutability, take immutable outter value and mutate it inside scopes.
// Weak<T> change reference to not count on Rc<T> type to 0
 