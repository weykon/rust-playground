trait Animal { }
struct Dog { }
impl Animal for Dog {}
// dog 类型实现了animal trait， dog类型的引用就自动成为这个trait object 的子类型


// 要从多视角去看trait的不同作用，trait是一种多方法下的同一个名字的操作
// 所以需要更全面的去看待trait的实现
//
// 1. 作为行为约束
// 2. 作为类型约束，在泛型中使用
// 3. 作为关联类型的容器
// 4.作为标记trait（零大小类型）
// .......

// trait object
struct TraitObject {
    data: *mut (),
    vtable: *mut VTable,
}
struct VTable {
    destructor: fn(*mut ()),
    size:usize,
    align:usize,
    methods: [fn()]
}
