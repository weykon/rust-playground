use std::{ops::Deref, rc::Rc};

pub fn main() {
    let a = 9;

    // ref
    i_want_to_ref(&a);

    // borrow
    let b = &a;

    rc_check();
}

fn i_want_to_ref(a: &i32) -> &i32 {
    println!("{}", &a);
    &a
}

// 对Rc的疑问
fn rc_check() {
    let source: i32 = 4;

    // _a 和 _b 属不属于构成了对source的共享引用呢
    let _a = &source;
    let _b = &source;
    // GPT说属于。
    
    // 那么Rc就是多了个对引用的计数功能。
    let a: Rc<i32> = Rc::new(source);
    let b: Rc<i32> = Rc::new(source);

    take_owner(a);
    take_owner(b);
}

fn take_owner(a: Rc<i32>) {
    println!("{}", a.deref());
}
