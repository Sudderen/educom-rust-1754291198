// use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use List::{Cons, Nil};


// use crate::List::{Cons, Nil};


// struct MyBox<T>(T);

// impl<T>  MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     } 
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// struct CustomSmartPointer {
//     data:String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self){
//         println!("Dropping CustomSmartPointer with data `{}!`", self.data);
//     }
// }

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "Leaf strong = {}, weak = {}", 
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
        "Branch strong = {}, weak = {}", 
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
        );

        println!(
        "Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
        );
    }

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // let a = 5;
    // let b = &mut a;

    // let mut c = 10;
    // let d = &c;
    // *d = 20;

    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Box::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));
    // let b = Box::new(5);
    // println!("b = {}", b);
    // let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *(y.deref()));

//     let m = MyBox::new(String::from("Rust");
//     hello(&m);
// }

// fn hello(name: &str) {
//     println!("Hello, {}", name);

    // let c = CustomSmartPointer {
    //     data: String::from("My Stuff"),
    // };
    // let d = CustomSmartPointer {
    //     data: String::from("Other Stuff"),
    // };
    // println!("CustomSmartPointers created.");
    // let c = CustomSmartPointer {
    //     data: String::from("Some Data"),
    // };

    // println!("CustomSmartPointer created");
    // drop(c);
    // println!("CustomSmartPointer dropped before the end of main.");
}