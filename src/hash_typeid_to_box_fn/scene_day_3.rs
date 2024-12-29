use std::{
    any::{Any, TypeId},
    cell::RefCell, marker::PhantomData,
};

// 初定结构，是保存所有impl 了ready的struct，这些struct的data保存在hash_data,用typeid来做标记，这样可以在后面的函数中，通过typeid来找到对应的struct。
// 而对于self.hash_data的修改，还不清楚具体过程怎么样才比较合理。
use std::collections::HashMap;

// 定义 Scene 结构
struct Scene {
    hash_data: HashMap<TypeId, Box<dyn Any>>,
    functions: Vec<
        Box<
            dyn FnMut(
                &mut HashMap<
                    TypeId,
                    Box<dyn Any>,
                >,
            ),
        >,
    >,
}

// Ready 特征，用于用户实现
trait Ready {
    fn ready(
        &mut self,
        data: &mut HashMap<
            TypeId,
            Box<dyn Any>,
        >,
    );
}

impl Scene {
    fn new() -> Self {
        Scene {
            hash_data: HashMap::new(),
            functions: Vec::new(),
        }
    }

    // 添加准备函数
    fn add_ready<
        T: Ready + Default + 'static,
    >(
        &mut self,
        mut component: T,
    ) {
        self.hash_data.insert(
            TypeId::of::<T>(),
            Box::new(T::default()),
        );
        self.functions.push(Box::new(
            move |data| {
                println!("data: {:?}", data);
                component.ready(data);
            },
        ));
    }

    // 按顺序执行所有ready函数
    fn render(&mut self) {
        for function in &mut self.functions {
            function(&mut self.hash_data);
        }
    }
}

// 示例用户实现
#[derive(Debug, Default)]
struct UserComponent {
    value: i32,
}

impl Ready for UserComponent {
    fn ready(
        &mut self,
        data: &mut HashMap<
            TypeId,
            Box<dyn Any>,
        >,
    ) {
        // 可以从 hash_data 中获取并修改数据
        if let Some(value) = data
            .get_mut(&TypeId::of::<Self>())
        {
            if let Some(v) =
                value.downcast_mut::<Self>()
            {
                println!("get downcast mut : {:?}", v);
                v.value = self.value;
            }
        }
    }
}

pub fn main() {
    let mut scene = Scene::new();

    // 添加组件
    scene.add_ready(UserComponent {
        value: 5,
    });
    let get = scene
        .hash_data
        .get(&TypeId::of::<UserComponent>());
    if let Some(value) = get {
        if let Some(user_component) = value.downcast_ref::<UserComponent>() {
            println!("get: UserComponent {{ value: {} }}", user_component.value);
        }
    }
    println!("get: {:?}", get);
    scene.add_ready(UserComponent {
        value: 10,
    });

    let get = scene
        .hash_data
        .get(&TypeId::of::<UserComponent>());
    if let Some(value) = get {
        if let Some(user_component) = value.downcast_ref::<UserComponent>() {
            println!("get: UserComponent {{ value: {} }}", user_component.value);
        }
    }
    println!("get: {:?}", get);
    // 执行更新
    scene.render();
}


// 然后给任意已经实现ready的添加一下拓展trait，去补齐一下一个中间Box到hashdata上
trait Setup2HashBox: Ready{
    fn get_res(&mut self);
}
struct SceneBridge<T>{
    scene: Box<Scene>,
    _marker: PhantomData<T>
}
impl<T> SceneBridge<T> {
    fn combind() {

    }
}
impl <T> Setup2HashBox for T where T: Ready {
    fn get_res(&mut self) {

    }
}
