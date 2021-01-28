use crate::rust_learn::cons_list::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[test]
fn test() {
    let mut list = &Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //只是遍历这个list就写不出来,好痛苦>_<
    /*loop {
        let b = match list {
            Cons(n, mut l) => {
                println!("{}", n);
                list = l.borrow_mut();
                //list = l.as_ref();
                break;
                true
            }
            Nil => {
                false
            }
        };
        if !b { break; }
    }*/
}