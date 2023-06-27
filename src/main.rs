mod option;
mod result;
mod generic_blanket_impls;
mod dynamic_size_type;
mod cell;
mod borrow_and_ref;
mod r#async;
mod r#iterator;

fn main() {
    borrow_and_ref::main();
}

