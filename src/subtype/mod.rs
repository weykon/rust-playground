// Note: debug expects two parameters with the *same* lifetime
fn debug<'a>(a: &'a str, b: &'a str) {
    println!("a = {a:?} b = {b:?}");
}

pub fn main() {
    // 这里旧版本应该是会报错误的
    let hello: &'static str = "hello";
    {
        // 'b   .   'static <: 'b
        // let world = String::from("world");
        let world: String =
            String::from("world");
        let world = &world; // 'world has a shorter lifetime than 'static
        debug(hello, world);

        // 'long <: 'short
    }

    main2();
}

fn assign<T>(input: &mut T, val: T) {
    *input = val;
}
// 获取可变的T引用，

fn main2<'b>() {
    // variance  方差，不一致，协变性，逆变性等的概括
    // 从上面的 'static <: 'b ,更深一层是 &'static T <: &'b T

    // wrong
    // let mut hello: &'static str = "hello";
    // {
    //     let world = String::from("world");
    //     assign(&mut hello, &world);
    // //       rustc: `world` does not live long enough
    // //       borrowed value does not live long enough
    // }
    // println!("{hello}"); // use after free 😿

    // 过程： 定义的hello可变引用，在{}中，假设'b, 那么 'static <: 'b,  &'static T <: &'b T, 而assign的隐式的状态是 &mut 'b, 'b, 限定了input和val是一致，但是目前传入的显然不是了。
    // 这个显然不是，其实我一开始也不确定，而上面的例子去看，也是可以的 &'a ,&'a 也可以，但是目前的是mut的情况，所以在不可变的引用可能是已经优化了。

    // right
    let mut hello: &'static str = "hello";
    {
        let world = String::from("world");
        assign(&mut hello, &world);
    }
    println!("{hello}"); // use after free 😿


    // The type F's variance is how the subtyping of its inputs affects the subtyping of its outputs. There are three kinds of variance in Rust. Given two types Sub and Super, where Sub is a subtype of Super:
    // 对于F的变体下，影响的是输入的子类型如何影响输出的子类型，有三种变体，给定两个类型Sub和Super，其中Sub是Super的子类型：


    // https://doc.rust-lang.org/nomicon/subtyping.html#variance
    // 表中对于&mut T, 里的T是不变的，生命周期是协变，但是那个不是
    //
    //
    // fn debug<T: std::fmt::Debug>(a: T, b: T) {
        // println!("a = {a:?} b = {b:?}");
    // }
    // 对于这个函数来讲，a,b 是一样的T，&'a T是可以协变到'a的，所以可以表现执行为子类型化。所以当前的&'static str是&'b str 的子类型，所以是可以的。编译器决定&'static str可以变成为&'b str。
    //

    let hello: Box<&'static str> = Box::new("hello");
    let mut world: Box<&'b str>;
    world = hello;
}

mod function_pointer;
