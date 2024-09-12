# Subtraits & Supertraits

一个 Sub 一个Super

子集 和 超集

子 :

超 where

# Trait 对象

在 generic_blanket_impls 代码中， 
将动态的Box内面的数据类型用 dyn trait 指出来。

Trait 对象是没有大小的，所以它们必须总是在一个指针后面。

根据类型中dyn关键字的存在来区分具体类型和 trait 对象在类型级别上的区别。

不是所有的 trait 都可以被转成 trait 对象。当且仅当一个 trait 满足下面这些要求时，它才是对象安全的（object-safe）：

* trait 不要求Self: Sized
* trait 的所有方法都是对象安全的

当一个 trait 方法满足下面的要求时，该方法是对象安全的：

— 方法要求Self:Sized 或者
- 方法在其接收者位置仅使用一个Self类型
