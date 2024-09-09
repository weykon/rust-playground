enum MyResult<T, E> {
    Ok(T),
    Err(E),
}
impl<T, E: ::std::fmt::Debug> MyResult<T, E> {
    fn unwrap(self) -> T {
        match self {
            MyResult::Ok(val) => val,
            MyResult::Err(err) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
        }
    }
}

// ? - 故障时返回Err    象