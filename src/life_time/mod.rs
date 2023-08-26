#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}

pub fn main() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    foo.share();
    // 上面两条代码调换位置，问题似乎是不能在同一作用域下不可变引用和可变引用出现交集调用
    println!("{:?}", loan);
}
