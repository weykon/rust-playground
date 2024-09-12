// 先从中文意思一步步去描述场景中
// 学习

// Rc 是  不可变 下 多次使用 引用 时 的记录计数
// 这是避免复制数据的一种引用数据的方法

// 与日常的&对比， 是可以脱离和原始的借用规则和生命周期限制。

// 所以对于rc refcell 的组合，中文的描述大概可以是，创建多个不可变的引用，这些引用包裹的是对于这个类型的可变引用且仅有一个的时候，同时可以多个不可变脱离生命周期的审核的引用

use std::borrow::BorrowMut;

// 基础面 & &mut
struct App {
    num: i32,
    thing: Thing,
}
struct Thing {
    num: i32,
}
// 先用box来模拟一下
struct AppBox {
    num: Box<i32>,
    thing: Box<Thing>,
}
pub fn main() {
    let mut app_box = AppBox {
        num: Box::new(32),
        thing: Box::new(Thing { num: 32 }),
    };
    change(&mut app_box);
    println!("app_box.num = {}", app_box.num);
}

fn change(app: &mut AppBox) {
    *app.num = 48;
}
