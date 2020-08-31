use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
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

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // overflow cycle
    // println!("a next item = {:?}", a.tail());
}

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });

//     println!(
//         "leaf strong = {}, weak = {}",
//         Rc::strong_count(&leaf),
//         Rc::weak_count(&leaf)
//     );

//     //println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

//     {
//         let branch = Rc::new(Node {
//             value: 5,
//             parent: RefCell::new(Weak::new()),
//             children: RefCell::new(vec![Rc::clone(&leaf)]),
//         });

//         *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

//         println!(
//             "branch strong = {}, weak = {}",
//             Rc::strong_count(&branch),
//             Rc::weak_count(&branch)
//         );
//         println!(
//             "leaf strong = {}, weak = {}",
//             Rc::strong_count(&leaf),
//             Rc::weak_count(&leaf)
//         );
//     }

//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//     println!(
//         "leaf strong = {}, weak = {}",
//         Rc::strong_count(&leaf),
//         Rc::weak_count(&leaf)
//     );
//     let value = Rc::new(RefCell::new(5));

//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
//     let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

//     *value.borrow_mut() += 10;

//     println!("a after = {:?}", a);
//     println!("b after = {:?}", b);
//     println!("c after = {:?}", c);
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let _b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let _c = Cons(8, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }

// pub trait Messenger {
//     fn send(&self, msg: &str);
// }

// pub struct LimitTracker<'a, T: 'a + Messenger> {
//     messenger: &'a T,
//     value: usize,
//     max: usize,
// }

// impl<'a, T> LimitTracker<'a, T>
// where
//     T: Messenger,
// {
//     pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value: usize) {
//         self.value = value;
//         let percentage_of_max = self.value as f64 / self.max as f64;

//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error: You are over your quota!");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger
//                 .send("Urgent warning: You've used up over 90% off your quota!");
//         } else if percentage_of_max >= 0.75 {
//             self.messenger
//                 .send("Warning: You've used up over 75% of your quota!");
//         }
//     }
// }

// fn main() {}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::cell::RefCell;

//     struct MockMessenger {
//         sent_messages: RefCell<Vec<String>>,
//     }

//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: RefCell::new(vec![]),
//             }
//         }
//     }

//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             self.sent_messages.borrow_mut().push(String::from(message));
//             //     let mut one_borrow = self.sent_messages.borrow_mut();
//             //     let mut two_borrow = self.sent_messages.borrow_mut();
//             //     one_borrow.push(String::from(message));
//             //     two_borrow.push(String::from(message));
//         }
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
//         limit_tracker.set_value(80);

//         assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
//     }
// }
