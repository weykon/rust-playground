https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md#sizedness


## 讨论在动态大小类型时的内容


?Sized 可以发音为“optionally sized”或“maybe sized”，并将其添加到类型参数的边界允许类型调整大小或取消大小
?Sized 通常被称为“扩大边界”或“松弛边界”，因为它放松而不是限制类型参数
?Sized 是 Rust 中唯一的松弛边界