mod r#async;
mod borrow_and_ref;
mod cell;
mod dynamic_size_type;
mod game;
mod generic_blanket_impls;
mod r#iterator;
mod life_time;
mod option;
mod result;
mod ref_keyword_usage;  
mod take_and_borrow;
mod learn_from_pure_words;
mod html_parser;
fn main() {
    life_time::main();
    take_and_borrow::main();
    learn_from_pure_words::main();
}
