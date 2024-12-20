use std::marker::PhantomData;

trait Animal {}
struct Dog;
struct Cat;
impl Animal for Dog {}
impl Animal for Cat {}

// 使用 *mut T 使得 AnimalShelter 对 T 是不变的
struct AnimalShelter<T: ?Sized + Animal + 'static>(PhantomData<fn(T)>);

fn accept_any_animal_shelter(shelter: AnimalShelter<dyn Animal>) {
    // 处理任何类型的动物收容所
}

pub fn run() {
    let dog_shelter: AnimalShelter<Dog> = AnimalShelter::<Dog>(PhantomData);
    accept_any_animal_shelter(dog_shelter); // 这行会编译错误

    let general_processor = AnimalProcessor::<Dog>(PhantomData);
    process_specific_animal(general_processor); // 这行会编译错误
}

// 使用 T 而不是 fn(T) 使得 AnimalProcessor 对 T 是协变的
struct AnimalProcessor<T: Animal>(PhantomData<T>);

fn process_specific_animal(processor: AnimalProcessor<Dog>) {
    // 处理特定类型（狗）的处理器
}
