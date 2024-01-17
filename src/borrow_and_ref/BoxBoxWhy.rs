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

// 更新： 是对一些类型大小未知的情况下，使用Box来解决编译时大小未知的问题。
// 还有几个二三个可用的场景
// 只在乎他的位置而不在乎他的大小

// 现在我举一个放在堆上的用于表示trait对象的例子
trait Animal {
    fn name(&self) -> &'static str;
}
struct Dog;
impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}
struct Cat;
impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }
}
// 写一个函数处理他们的名字的，那么如果传一个(animal: Animal)进去，那么就会报错，因为trait是一个动态大小的类型
// 所以我们可以用Box来解决这个问题
fn print_name(animal: Box<dyn Animal>) {
    println!("You're a {}", animal.name());
}
fn a_trait_with_box_usage (){
    print_name(Box::new(Dog));
    print_name(Box::new(Cat));
}