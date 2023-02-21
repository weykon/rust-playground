同步
多线程
异步

而异步函数是惰性的，只有遇到.await关键字才会执行

Rust异步的核心其实是Future，Future是由异步计算或函数产生的单一最终值。Rust的异步函数都会返回Future，Future基本上就是代表着延迟的计算。

trait Future { 

    fn poll () -> Poll

}

有程序不断调用future的poll保持更新，是异步执行器来执行。
异步执行器管理Future集合。加async就是就这段函数
异步执行器知道它返回带有Future的，执行其poll
