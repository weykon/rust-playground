& vs ref  & 与 ref
& denotes that your pattern expects a reference to an object. Hence & is a part of said pattern: &Foo matches different objects than Foo does.
& 表示您的模式需要对对象的引用。因此 & 是所述模式的一部分： &Foo 与 Foo 匹配不同的对象。

ref indicates that you want a reference to an unpacked value. It is not matched against: Foo(ref foo) matches the same objects as Foo(foo).
ref 表示您想要引用未打包的值。它不匹配： Foo(ref foo) 与 Foo(foo) 匹配相同的对象。
