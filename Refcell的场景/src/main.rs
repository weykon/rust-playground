use std::cell::RefCell;
use std::collections::HashMap;

struct Library {
    books: HashMap<String, RefCell<Book>>,
}

struct Book {
    title: String,
    available: bool,
}

impl Library {
    fn new() -> Library {
        let mut books = HashMap::new();
        books.insert(
            "The Rust Programming Language".to_string(),
            RefCell::new(Book {
                title: "The Rust Programming Language".to_string(),
                available: true,
            }),
        );
        Library { books }
    }

    // 借出书籍
    fn borrow_book(&self, title: &str) -> Result<(), String> {
        if let Some(book) = self.books.get(title) {
            if book.borrow().available {
                book.borrow_mut().available = false;
                Ok(())
            } else {
                Err("Book is already borrowed".to_string())
            }
        } else {
            Err("Book not found".to_string())
        }
    }

    // 归还书籍
    fn return_book(&self, title: &str) -> Result<(), String> {
        if let Some(book) = self.books.get(title) {
            if !book.borrow().available {
                book.borrow_mut().available = true;
                Ok(())
            } else {
                Err("Book was not borrowed".to_string())
            }
        } else {
            Err("Book not found".to_string())
        }
    }
}

fn main() {
    let library = Library::new();
    match library.borrow_book("The Rust Programming Language") {
        Ok(_) => println!("Book borrowed successfully"),
        Err(e) => println!("{}", e),
    }
}
