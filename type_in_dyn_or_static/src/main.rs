fn main() {
    println!("Hello, world!");
}

trait Animal {}
struct StaticGenericStruct<T: Animal>(T);
struct DynamicStruct(dyn Animal);
// 以上是两种不同的类型，一个是静态泛型，一个是动态类型
// 静态的时候，是从编译时期就确定具体类型，且T大小已知。会进行单态化，即编译时期就确定具体类型,为每个类型生成一个特定的代码

// 使用 trait 对象，运行时动态分发
// 大小在编译时未知（unsized）
// 需要通过指针（如 Box、& 或 *mut）来处理
// 允许在运行时处理不同的具体类型
// 有一些运行时开销
