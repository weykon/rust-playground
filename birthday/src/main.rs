use chrono::{Datelike, NaiveDate};

struct Person {
    birthday: String,
    age: i32,
}

trait Birthday {
    fn birthday(&mut self);
}
impl Person {
    fn new(time: String) -> Person {
        let birth_date = NaiveDate::parse_from_str("2000-01-01", "%Y-%m-%d").unwrap();
        let now = chrono::Local::now().date().naive_local();
        let age = now.year() - birth_date.year();
        Person {
            birthday: time,
            age,
        }
    }
}
impl Birthday for Person {
    fn birthday(&mut self) {
        println!("Happy Birthday!");
        self.age += 1;
        println!("You are now {} years old.", self.age);
    }
}

fn main() {
    let mut me = Person::new("2000-01-01".to_string());

    me.birthday();
}
