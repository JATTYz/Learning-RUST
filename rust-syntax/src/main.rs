fn use_string(s: &String) {

    println!("{} : world", s);
}

fn use_number (number: u32) {

}

fn borrow_string(s: &String) {
    println!("{}",s);
}
fn main() {
    let mut s = String::from("hello");
    let borrow = s.get(0..5);
    borrow_string(&s);
    // borrow_string(&mut s);
    // borrow_string(&mut s);
    // let number: u32 = 123;
    // use_number(number);
    use_string(&s);
    print!("{}", s);
}
