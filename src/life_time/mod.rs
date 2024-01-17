use std::{borrow::{BorrowMut, Borrow}, cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}
struct Thing {}

pub fn main() {
    let mut foo = Foo;
    foo.share();
    let loan = foo.mutate_and_share();
    // 上面两条代码调换位置，问题似乎是不能在同一作用域下不可变引用和可变引用出现交集调用
    println!("{:?}", loan);

    let mut app = App {
        a_num: RefCell::new(32),
        a_thing: Rc::new(RefCell::new(Thing {})),
    };


    let  a_thing_ref = Rc::clone(&app.a_thing); 

}

struct App {
    a_num: RefCell<i32>,
    a_thing: Rc<RefCell<Thing>>,
}

impl Thing {
    fn do_thing(&self, app: &App) {
        println!(" in thing do_thing fn saying : {}", app.a_num.borrow());
    }

    pub fn will_mut_app(&self, app: &App) {
        *app.a_num.borrow_mut() = 32;
    }
}
