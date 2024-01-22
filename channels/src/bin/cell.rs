fn main() {
    use std::cell::RefCell;

    let refcell_value = RefCell::new(String::from("Hello"));
    {
        let mut borrowed_value = refcell_value.borrow_mut();
        borrowed_value.push_str(", world!");
    }

    let borrowed_value = refcell_value.borrow();
    println!("{}", borrowed_value);

    // let mut s = String::from("Hello");
    // let p1 = &mut s;
    // p1.push_str(" world !!");
    // println!("{}",p1);
    // let p2 = &s;
    // println!("{}",p2);
}