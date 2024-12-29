use std::{
    any::{Any, TypeId},
    collections::HashMap, marker::PhantomData,
};

struct Res(HashMap<TypeId, Box<dyn Any>>);
// box 放类型
// 类型可以跟统一trait

impl Res {
    // 添加类型
    fn insert<T: 'static>(
        &mut self,
        value: T,
    ) {
        self.0.insert(
            TypeId::of::<T>(),
            Box::new(value),
        );
    }
    // // 用T绑定约束 无法new的同时有self的storage，所以用一个桥梁
    // fn new<T: NewScene>() -> Self {
    //     T::setup();
    // }
    fn get<T: 'static>(&self) -> Option<&T> {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
    }
}

// - 主要用途：
//   - 告诉编译器这个泛型参数 T 是有意义的
//   - 帮助实现类型安全和所有权语义
//   - 避免编译器警告未使用的泛型参数
// 我们原本的目的就是用一个约束捆绑一下用户的struct的，用户的T
struct BridgeForScene<T: NewScene> {
    storage: Res,
    _phantom: std::marker::PhantomData<T>,
}
impl<T> BridgeForScene<T>
where
    T: NewScene,
{
    fn new() -> Self {
        let mut res = Res(HashMap::new());
        T::setup(&mut res);
        Self {
            storage:res,
            _phantom: PhantomData,
        }
    }
}

trait NewScene {
    fn setup(res: &mut Res) {
        res.insert(DataCache::default());
    }
}
#[derive(Default)]
struct DataCache;

struct Scene1;
impl NewScene for Scene1 {
    fn setup(res: &mut Res) {
        res.insert(DataCache::default());
    }
}
pub fn run () {
    let scene = BridgeForScene::<Scene1>::new();

}
