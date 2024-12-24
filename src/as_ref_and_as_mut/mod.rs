struct WrapValue {
    pub value: Option<i32>,
}

/// 1 用 'as_ref' 将 Option<i32> 转换为 Option<&i32>
fn main1() {
    let opt_string: Option<String> =
        Some(String::from("hello"));
    let opt_str: Option<&str> = opt_string
        .as_ref()
        .map(|s| s.as_str());
}

/// 2 用 `as_ref` 将 `&String` 转换为 `&str`
fn main2() {
    let s = String::from("hello");
    let string_ref: &String = &s;
    let str_ref: &str = string_ref.as_ref();
}

fn main3() {
    let mut val = String::from("hello");

    if let Some(s) = Some(&mut val).as_mut()
    {
        s.push_str(" world");
    }

    println!("{}", val);
}

#[derive(Debug)]
struct Person {
    name: String,
}
fn process_name(name: &str) {
    println!("Processing name : {} ", name);
}

fn main4() {
    let person = Person {
        name: String::from("Alice"),
    };
    let person_ref = &person;

    process_name(person_ref.name.as_ref());
}

/// Self-taught

#[derive(Debug)]
struct Book {
    id: u32,
    title: String,
    author: String,
    pages: u32,
    price: f64,
}

#[derive(Debug)]
struct BookInfo<'a> {
    id: u32,
    title: &'a str,
}

impl Book {
    fn as_info(&self) -> BookInfo {
        BookInfo {
            id: self.id,
            title: &self.title,
        }
    }
}

#[derive(Debug)]
struct Product {
    id: u32,
    name: String,
    description: String,
    price: f64,
    stock: u32,
    category: String,
}

#[derive(Debug)]
struct ProductPreview<'a> {
    id: u32,
    name: &'a str,
    price: f64,
}

impl Product {
    fn as_preview(&self) -> ProductPreview {
        ProductPreview {
            id: self.id,
            name: &self.name,
            price: self.price,
        }
    }
}

// 以上是rust对于关系上的as字样的用法。

// deref 的话就是一种反推的过程
// 比如BookInfo回到Book

mod deref_example {
    use std::ops::Deref;

    #[derive(Debug)]
    struct Book {
        id: u32,
        title: String,
        content: String,
    }

    #[derive(Debug)]
    struct BookRef {
        book: Book,
    }

    impl Deref for BookRef {
        type Target = Book;

        fn deref(&self) -> &Self::Target {
            &self.book
        }
    }
    fn main() {
        let book = Book {
            id: 1,
            title: String::from(
                "Rust Programming",
            ),
            content: String::from(
                "anything;",
            ),
        };
        let book_ref = BookRef { book };

        // 通过解引用可以直接访问 Book 的字段
        println!("书名: {}", book_ref.title); // 自动解引用
        println!("ID: {}", book_ref.id);
        // 自动解引用
    }
}

mod deref_why {
    use std::ops::{Add, Deref};

    // 更重要的是讨论deref的由来
    struct MyBox<T: Copy + Clone>(T);

    impl<T> MyBox<T>
    where
        T: Copy + Clone,
    {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
        fn as_ref(&self) -> &T {
            &self
        }
    }

    impl<T: Add<Output = T>> Add for &MyBox<T>
    where
        T: Copy + Clone,
    {
        type Output = MyBox<T>;
        fn add(
            self,
            other: &MyBox<T>,
        ) -> Self::Output {
            MyBox::new(
                *(self.deref())
                    + *(other.deref()),
            )
        }
    }
    impl<T> Add for MyBox<T>
    where
        T: Copy + Clone + Add<Output = T>,
    {
        type Output = MyBox<T>;
        fn add(
            self,
            other: MyBox<T>,
        ) -> Self::Output {
            MyBox::new(self.0 + other.0)
        }
    }
    impl<T> Deref for MyBox<T>
    where
        T: Copy + Clone,
    {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    fn main() {
        // 假设目前我需要去装一个我i32, 那么在deref的操作下，可以拿到i32的值
        let x_box = MyBox(5);
        let y_box = MyBox(4);

        let sum = &x_box + &y_box;
        let eq_sum = (*(x_box.deref()))
            + (*(y_box.deref()));
        let same_as_but_take_sum =
            x_box + y_box;
    }
}

mod my_option {
    pub enum MyOption<T> {
        Some(T),
        None,
    }

    // 如果一个Option值去as_ref，那么需要考虑当前的Option是否有这个值
    impl<T> MyOption<T> {
        fn as_ref(&self) -> MyOption<&T> {
            match self {
                MyOption::Some(
                    ref value,
                ) => MyOption::Some(value),
                MyOption::None => {
                    MyOption::None
                }
            }
        }
    }
}

mod complex_option_set {
    use super::my_option::MyOption;
    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
    }

    impl<T> MyOption<T> {
        fn as_mut(
            &mut self,
        ) -> MyOption<&mut T> {
            match self {
                MyOption::Some(
                    ref mut value,
                ) => MyOption::Some(value),
                MyOption::None => {
                    MyOption::None
                }
            }
        }
    }
}
