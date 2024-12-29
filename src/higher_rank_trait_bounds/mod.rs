use std::fmt::Display;
// 相当于接收任意生命周期的参数，对于实时运行时候的参数的生命周期作为一种变体
struct MySimpleFormatter;

trait Formatter {
    fn format<T: Display>(
        &self,
        value: T,
    ) -> String;
}

impl Formatter for MySimpleFormatter {
    fn format<T: Display>(
        &self,
        value: T,
    ) -> String {
        format!("Value: {}", value)
    }
}

fn apply_formatter<F>(
    formatter: F,
) -> impl for<'a >Fn(&'a str) -> String
where
    F: Formatter,
{
    move |s| formatter.format(s)
}

fn run() {
    let formatter = MySimpleFormatter;
    let f = apply_formatter(formatter);
    let s1 = "hello";
    let s2 = String::from("World");
    println!("{}", f(s1));
    println!("{}", f(&s2));
    {
        let s3 = "hello";
        println!("{}", f(s3));
    }
}
