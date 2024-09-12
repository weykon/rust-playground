pub fn main() {
    let a_num = 10;
    std::thread::spawn(move || {
        wrap(&a_num);
    });
}
fn wrap(num: &i32) {
    println!("num is {}", num);
    let x: i32 = 1;
    match x {
        ref y => println!("{}", *y),
    }
}
