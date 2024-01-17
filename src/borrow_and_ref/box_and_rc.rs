// 当疑惑与直接引用和box和rc这些的事情区别之间。
// BoxBoxWhy.rs 有一个简单的解答在关于使用Box的一种情况
// 现在这里说说关于box和rc的情况

// 首先他们有一个区别，Box是唯一所有权会持有或拿走，
// 而Rc可以通过克隆他的rc引用，并加以计数

use std::rc::Rc;

// Box
struct Config {
    name: String,
}
struct Server {
    // 其实这个除了使用Box来满足举例之外，config可以不使用Box，因为他的大小是已知的
    // 但是，我们直接使用引用的话，是使用的栈空间，而使用Box的话，是使用的堆空间
    // 大型结构体的时候，使用Box会更好一些
    // 不过其实直接使用引用也可以的，具体看代码的用途后续需不需要使用到Box
    config: Box<Config>,
}

// Rc
struct Book {
    name: String,
}
struct Borrower {
    name: String,
    borrowed_books: Vec<Rc<Book>>,
}
impl Borrower {
    fn new(name: String) -> Self {
        Borrower {
            name,
            borrowed_books: Vec::new(),
        }
    }
    fn borrow_book(&mut self, book_rc: Rc<Book>) {
        self.borrowed_books.push(book_rc);
    }
}
fn books_story() {
    let story_book = Rc::new(Book {
        name: String::from("The Book"),
    });
    let magazine_book = Rc::new(Book {
        name: String::from("The Magazine"),
    });
    let mut amy = Borrower::new("amy".to_owned());
    let mut bob = Borrower::new("bob".to_owned());

    amy.borrow_book(Rc::clone(&story_book));
    bob.borrow_book(Rc::clone(&magazine_book));
}
