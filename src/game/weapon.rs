trait Weapon {
    fn fire();
    fn reload();
    fn re_compose();
}
trait ZoomComp {
    fn cast_heavy(&self);
}
struct RedPoint {
    zoom_scale: u32,
    heavy: u32,
}
struct Holosight {
    zoom_scale: u32,
    heavy: u32,
}
struct AK {
    damage: u32,
    capacity: u32,
    heavy: u32,
    zoom_comp: Box<dyn ZoomComp>,
}
impl ZoomComp for RedPoint {
    fn cast_heavy(&self) {}
}

impl ZoomComp for Holosight {
    fn cast_heavy(&self) {}
}


// 在这个 Box 中成功加入的dyn trait， 是需要把trait作为了trait对象才可以这样用。
// 在这个trait底下实现的函数需要满足对于成为trait对象的条件，是不允许静态方法，因为我一开始没有在case_heavy加self
// 现在加了self，作为了trait对象来处理动态分发。
//
// 后面的内容是可以命题为： 动态分发和静态分发。
//
// 静态分发是我们之前在其他语言中常见的泛型
//
