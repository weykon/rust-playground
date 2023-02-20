use std::cell::Cell;


// https://zhuanlan.zhihu.com/p/384388192
// 一个可修改的内存位置
// 例子1
struct SomeStruct {
    regular_field: u8,
    special_field: Cell<u8>,
}
fn main() {
    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: Cell::new(1),
    };
    let new_value = 100;
    // 错误：my_struct 是不可变的
    // my_struct.regular_field = new_value;

    // 可行: 尽管 my_struct 是不可变的，special_field是一个Cell,Cell总是可以被修改
    my_struct.special_field.set(new_value);
    assert_eq!(my_struct.special_field.get(), new_value);
}


// Cell如何通过共享引用来修改的数据：
// 在unsafe block里面得到Cell的可变引用，
// 然后用mem::repleace与新值做替换。