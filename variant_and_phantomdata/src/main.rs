use std::marker::PhantomData;
mod cat_dog_animal;
mod negative_try;
mod again;
mod basic;

fn main() {
    println!("Hello, why and how phantom data!");
    let _: Invariant<&'static str> = Invariant(PhantomData);
    let _: Covariant<&'static str> = Covariant(PhantomData);
    let _: Contravariant<&'static str> = Contravariant(PhantomData);

    negative_try::run();
}

struct Foo<T>(PhantomData<T>); // 协变
struct Bar<T>(PhantomData<*mut T>); // 不变
struct Baz<T>(PhantomData<fn(T)>); // 逆变

struct Invariant<T>(PhantomData<*mut T>);
struct Covariant<T>(PhantomData<T>);
struct Contravariant<T>(PhantomData<fn(T)>);
