struct Point<T> {
    x: T,
    y: T,
}
impl Point<i32> {
    fn read(&self) {
        println!("impl Point in Kind of i32: x={}, y={}", &self.x, &self.y);
    }
}

fn main() {
    println!("Hello, world!");
    let p = Point { x: 1, y: 2 };
    p.read();
}
// 这里使用的关键点是，从Point的值类型多种的开始，从分散的类型当中，需要铺设到比较全的实现当中。
// 但这是一个数据字段的分割流向是一致的情况下使用。
