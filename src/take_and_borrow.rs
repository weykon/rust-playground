pub fn main() {
    println!("take and borrow ; ");

    let mut just_option_take_the_own = Some(1);

    let let_me_take_it = just_option_take_the_own.take();

    println!("what the source now : {:?}", just_option_take_the_own);
    println!("and here took : {:?}", let_me_take_it);

    // 他是从option中拿走值
}
