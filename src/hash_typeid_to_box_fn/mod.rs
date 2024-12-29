use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

// the title is wrong, here is testing the hash
// typeid and store the type's instance data
#[derive(Default)]
struct Res {
    storage: HashMap<TypeId, Box<dyn Any>>,
}

impl Res {
    fn insert<T: 'static>(
        &mut self,
        value: T,
    ) {
        self.storage.insert(
            TypeId::of::<T>(),
            Box::new(value),
        );
    }
}
#[derive(Default)]
struct Wrapper {
    value: i32,
}
fn run() {
    let mut res = Res::default();
    res.insert(Wrapper::default());
}

//////
struct Same<T> {
    value: T,
}
impl Same<i32> {}
impl Same<i64> {}

pub mod scene;
pub mod scene_day_2;
pub mod  scene_day_3;
pub  mod scene_day_4;
fn input_string_or_refstr(
    string: impl Into<String>,
) {
    println!(
        "It's OK to input &str or String"
    );
}
fn test_previous() {
    input_string_or_refstr("&str");
    input_string_or_refstr(
        "String".to_string(),
    );
}

pub mod test;
