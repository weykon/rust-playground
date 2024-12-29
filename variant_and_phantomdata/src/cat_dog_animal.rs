use std::{borrow::Borrow, marker::PhantomData, ops::Deref};

trait Animal {}
struct Dog;
struct Cat;
impl Animal for Dog {}
impl Animal for Cat {}

struct AnimalShelter<T: Animal>(PhantomData<T>); // 协变
struct AnimalProcessor<T: ?Sized>(PhantomData<fn(T)>); // 逆变
struct TemporaryShelter<'a, T: Animal> {
    animal: &'a mut T,
}

// 接收shelter，次animal，接收是泛型，在协变下
fn shelter_animal<T: Animal + 'static>(shelter: AnimalShelter<T>, animal: &T) {
    // 将动物安置在收容所
    println!("Sheltering animal");
}

// 接收processor，次animal，接收是动态的，逆变下
// porcessor 逆变， animal &mut 不变
// 所以对于这个函数，原本dyn的动物是猫狗等的子类型，但是这里的phantomdata说明了逆变的情况，所以在原本如果可以协变的情况下，传入的类型 Sub <: Super 的， 也就是可以传一个Sub的， 但是现在限定了逆变，那么只能传 Super， 所以后面这里我写的传的是一个general_processor，而不是具体的猫狗的processor
// 然后这里的&mut 是对应于T的传入处理是不变的，所以传入是什么，处理就是什么，那么现在设置的是 dyn Animal， 是一个trait object，也就是对于一个类型实例的引用和虚表函数指针。
fn process_animal(processor: &AnimalProcessor<dyn Animal>, animal: &mut dyn Animal) {
    // 处理动物
    println!("Processing animal");
    let p = animal.deref();
    // let p = animal.deref();
    // let p = animal.deref
}

pub fn run() {
    let dog_shelter = AnimalShelter::<Dog>(PhantomData);
    let cat_shelter = AnimalShelter::<Cat>(PhantomData);

    let general_processor = AnimalProcessor::<dyn Animal>(PhantomData);

    let mut my_dog = Dog;
    let mut my_cat = Cat;

    shelter_animal(dog_shelter, &my_dog);
    shelter_animal(cat_shelter, &my_cat);

    process_animal(&general_processor, &mut my_dog);
    process_animal(&general_processor, &mut my_cat);

    let day_shelter_dog = TemporaryShelter {
        animal: &mut my_dog,
    };
    let day_shelter_cat = TemporaryShelter {
        animal: &mut my_cat,
    };
}
