use std::fmt::Debug;

fn debug <T: Debug> (t: T) {  // T: Debug + Sized
     println!("{:?}",t);
}
// 但是该函数拥有传递给它的任何值的所有权，这有点烦人，所以我将函数更改为仅采用引用

fn main() { 
    debug("my str");
    dbg("my str"); // 这里报错，需要下面加 + ?Sized
}

fn dbg<T: Debug + ?Sized>(t: &T) { // T: Debug + Sized
    println!("{:?}", t);
}

