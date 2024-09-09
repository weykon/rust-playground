fn look() {
    let num: Option<u8> = Some(1);

    match num {
        Some(v) => println!("{v}"),
        None => {}
    };
}
enum MyOption<T> {
    Some(T),
    None,
}
impl<T> MyOption<T> {
    // 这是Option时候的unwrap
    fn unwrap(self) -> T {
        match self {
            MyOption::Some(val) => val,
            MyOption::None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

fn unwrap_or<T>(option: Option<T>, default: T) -> T {
    match option {
        None => default,
        Some(value) => value,
    }
}
fn and_then<F, T, A>(option: Option<T>, f: F) -> Option<A>
where
    F: FnOnce(T) -> Option<A>,
{
    match option {
        None => None,
        Some(value) => f(value),
    }
}

// unwrap 在对应 Option 和 Result 都是不同的实现。
