use std::cell::{RefCell,Cell};
#[derive(Debug)]
struct Person {
    pub name : RefCell<String>,
    pub age : Cell<i32>,
}

impl Person {
    pub fn new(name : String,age : i32) -> Self {
        Person {name : RefCell::new(name),age : Cell::new(age)}
    }
}

// Cell and RefCell are used for Interior mutability.

fn main() {
    let p = Person::new("SHIVAM".to_owned(),23);
    {
        let mut t = p.name.borrow_mut();
        t.push_str(" VARSHNEY !!");
        p.age.set(24);
    }
    println!("{:?}",p);
    // use std::cell::RefCell;
    // use std::cell::Cell;

    // let refcell_value = RefCell::new(String::from("Hello"));
    // {
    //     let mut borrowed_value = refcell_value.borrow_mut();
    //     borrowed_value.push_str(", world!");
    // }

    // let borrowed_value = refcell_value.borrow();
    // println!("{}", borrowed_value);

    // let mut s = String::from("Hello");
    // let p1 = &mut s;
    // p1.push_str(" world !!");
    // println!("{}",p1);
    // let p2 = &s;
    // println!("{}",p2);
    // let temp = Cell::new(10);
    // let val = temp.get();
    // temp.set(val+12);
    
}



// 6 = 5+4
// 5 = 4+3
// 4 = 3+2
// 2 = 1+0

 // 3 = 2
 // 4 = 3
 // 5 = 5

