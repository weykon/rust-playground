/// 有时搞不懂Box的意义和使用场景？
///
/// Box的主要作用是在堆上分配数据，以避免在栈上占用过多的空间。在某些情况下，我们需要在编译时确定数据的大小，例如在定义结构体或枚举时。如果我们需要在运行时动态分配数据，则可以使用Box。
/// 具体而言，Box通常用于创建递归数据结构（如树）和在堆上分配大量数据时，以避免栈溢出。另外，如果我们需要将所有权从一个函数传递到另一个函数，并且数据的大小在编译时未知，则也可以使用Box。
/// 举个例子，假设我们需要在运行时从用户输入创建一个字符串，然后将其传递给一个函数进行处理。由于字符串的大小在编译时未知，我们可以使用Box来在堆上动态分配字符串：
fn process_string(s: Box<String>) {
    // 处理字符串
}

fn main() {
    let s = Box::new(String::from("hello"));
    process_string(s);
}
