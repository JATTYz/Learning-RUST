fn use_string(s: &String) {

    println!("{} : world", s);
}

fn borrow_string(s: &String) {
    println!("{}",s);
}
fn main() {
    let mut s = String::from("hello");
    let borrow = s.get(0..5);
    borrow_string(&s);
    use_string(&s);
    print!("{}", s);
}
