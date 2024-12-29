use std::marker::PhantomData;

struct Dog {} // 具体类型

trait Animal {
    fn eat() {}
} // trait 对象 抽象类型
type BeSonFromAnimal = dyn Animal; // 抽象类型

// 关于这个过程是实现了trait的类型的引用，是trait object(&dyn Trait) 的子类型
impl Animal for Dog {}

struct AnimalShelter<T: Animal>(
    PhantomData<fn(T)>,
);
impl<T: Animal> AnimalShelter<T> {
    fn admit(&mut self, animal: T) {}
}
impl Animal for Cat{}
struct Cat{}
fn test() {
    let mut dog_shelter: AnimalShelter<Dog> =
        AnimalShelter(PhantomData);
    let mut cat_shelter: AnimalShelter<Cat> = AnimalShelter(PhantomData);
    dog_shelter.admit(Dog{});
    // dog_shelter.admit(Cat{}); 这里就会错误
    cat_shelter.admit(Cat{});
}

// 参考使用场景下，类型差别是否有关系，如果是协变，
// 那么在标记中 <T> 同样的协变
// 在标记中 <fn<T>> 就是相反去逆变
