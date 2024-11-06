1. Rust 中的型变

在 Rust 中,型变主要体现在 __生命周期参数__ 和 __指针类型__ 上:

• 生命周期通常是协变的
• 引用 &T 和 Box 对 T 是协变的
• 可变引用 &mut T 对 T 是不变的
• 原始指针 *const T 对 T 是协变的, *mut T 对 T 是不变的
• __函数指针对返回值是协变的,对参数是逆变的__

2. PhantomData 和型变

PhantomData 是一个零大小类型,用于在类型系统中标记泛型参数 T 的使用,而不实际存储 T 类型的值。它的型变性质取决于如何使用:

• PhantomData 默认是协变的
• PhantomData<*const T> 是协变的
• PhantomData<*mut T> 是不变的
• PhantomData<fn(T)> 对 T 是逆变的
• PhantomData<fn() -> T> 对 T 是协变的



---


在 Rust 中,使用 PhantomData 的特定形式确实是为了明确指定类型参数的型变性质。这些形式基本上是固定的模式,用来表示特定的型变关系。让我们详细解释一下:


> 1.协变 (Covariant)固定形式: PhantomData<T>
> ```rs
> struct Covariant<T>(PhantomData<T>);
> ```
> 这是最简单的形式,表示 T 是协变的。

> 2.逆变 (Contravariant)固定形式: PhantomData<fn(T)>
> ```rs
> struct Contravariant<T>(PhantomData<fn(T)>);
> ```
> 这种形式利用了函数参数的逆变性质。


> 3.不变 (Invariant)固定形式: PhantomData<*mut T> 或 > PhantomData<&'a mut T>
> ```rs
> struct Invariant<T>(PhantomData<*mut T>);
> ```
> 这利用了可变指针的不变性质。

> 4.协变生命周期固定形式: PhantomData<&'a ()>
> ```rs
> struct CovariantLifetime<'a>(PhantomData<&'a ()>);
> ```
> 这种形式用于表示生命周期参数的协变。

> 5.逆变生命周期固定形式: PhantomData<fn(&'a ())>
> ```rs
> struct ContravariantLifetime<'a>(PhantomData<fn(&> 'a ())>);
> ```
> 这种形式利用了函数参数中生命周期的逆变性质。


这些固定形式的写法确实是为了明确指定特定的型变关系。它们是 Rust 类型系统的一部分,编译器会根据这些形式来推断和检查型变性质。