// fn(T) -> U
// 为什么

// fn get_str() -> &'a str;
// fn get_static() -> &'static str;
// 产生一个， 至少'a ，至于实际中存在比'a更久多少并不重要，只要保证在'a的生命周期内就可以了

// fn store_ref(&'a str);
// with:
// fn store_static(&'static str);
// The first function can accept any string reference as long as it lives at least for 'a, but the second cannot accept a string reference that lives for any duration less than 'static, which would cause a conflict. Covariance doesn't work here. But if we flip it around, it actually does work! If we need a function that can handle &'static str, a function that can handle any reference lifetime will surely work fine.

mod how_transform {
    use std::cell::RefCell;

    thread_local! {
        pub static StaticVecs: RefCell<Vec<&'static str>> = RefCell::new(Vec::new());
    }

    /// saves the input given into a thread local `Vec<&'static str>`
    fn store(input: &'static str) {
        StaticVecs.with_borrow_mut(|v| {
            v.push(input)
        });
    }

    /// Calls the function with it's input (must have the same lifetime!)
    fn demo<'a>(
        input: &'a str,
        f: fn(&'a str),
    ) {
        f(input);
    }

    fn main() {
        demo("hello", store); // "hello" is 'static. Can call `store` fine

        {
            let smuggle =
                String::from("smuggle");

            // `&smuggle` is not static. If we were to call `store` with `&smuggle`,
            // we would have pushed an invalid lifetime into the `StaticVecs`.
            // Therefore, `fn(&'static str)` cannot be a subtype of `fn(&'a str)`
            demo(&smuggle, store);
        }

        // use after free 😿
        StaticVecs.with_borrow(|v| {
            println!("{v:?}")
        });

        // contravariance
        // fn foo (bar : Fn (&'a str) -> ()) {
        //      bar (""   /* 'a */)
        // }
        //
        // foo期待一个函数，内部带有一个期待'a的生命周期的东西，那么当调用使用
        // foo (fn (&'static str)){}
        // 的时候，'static 和 'a 是一致的，'static <: 'a， 'a 比static更长，那么这样调用无法满足foo期待的更长的生命周期。
        //
        // 那么如果 foo 期待的是 'static 的时候，你可以在foo调用传入比 'static 更短的生命周期，那么这样是可以的。
        //
        //
        // 如果单从函数的角度去看，我接受一个T，那么我对T的处理一定是逆变的，比如你给我一个关于T的变体处理的时候，我也不确定能接收到的是什么，假设你给我一个T的子类型，或者是某个子类型是T的参数。比如动物和狗猫，trait动物，狗猫实现了动物，那么猫狗是动物的子类型，子类型所能包含了父类所能做到的，而父类并不包含子类型所做到的。那么我接受一个动物，我不确定是猫还是狗，但是如果我接受一个猫，那么我就确定是猫了。这个过程中。我接受一个T，那么我对T的处理一定是逆变的。
        //
        //
        // 对于生命周期中
        // 'static <: 'a
        // &'static <: 'a
        //
        // 'static <: 'a
        // Fn(&'a T) <: Fn(&'static T)
    }
}
