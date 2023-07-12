// fn main2() {
//     let b = Box::new(5);

//     println!("b = {}", b);
// }

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main3() {
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

// // 15.2. Treating Smart Pointers Like Regular References with the Deref Trait
// use std::ops::Deref;

// struct MyBox<T>(T);

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn main4() {
//     let x = 2;
//     let y = Box::new(x);

//     assert_eq!(x, 5);
//     assert_eq!(*y, 5);
// }

// 15.4. Rc<T>, the Reference Counted Smart Pointer
// allows to share ownership of some data

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main2() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

// 15.5. RefCell<T> and the Interior Mutability Pattern
