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
