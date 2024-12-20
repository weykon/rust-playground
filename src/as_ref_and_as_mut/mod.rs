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

   // 更重要的是讨论deref的由来
   struct MyBox<T>(T);
   impl<T> Deref for MyBox<T> {
         type Target = T;
         fn deref(&self) -> &Self::Target {
              &self.0
         }
   }
   fn main  () {
       // 假设目前我需要去装一个我i32, 那么在deref的操作下，可以拿到i32的值
       let x_box = MyBox(5);
       let y_box = MyBox(4);

       let sum = x_box + y_box;
       let eq_sum = (*(x_box.deref())) + (*(y_box.deref()));
   }
}
