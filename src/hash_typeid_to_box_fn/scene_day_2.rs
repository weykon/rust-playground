use std::{
    any::{Any, TypeId},
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    marker::PhantomData,
    ops::Deref,
    rc::Rc,
};

// scene 通义下， 用res hashmap存过程中所有产生的给paint的数据
// ready trait是给struct出来各种ready阶段的数据
// hashmap去保存typeid的box数据
// 从语义上使用，
//
// struct VertexReady{ lots_of_data: Vec<f32> }
// impl Ready for VertexReady {}
// Scene::new("Boid").add_ready(VertexReady::default())
//                   .add_ready(VertexReady::default())
//                   .add_paint(VertexPaint::default())
//                   .done();

pub struct Scene {
    res: HashMap<TypeId, Box<dyn Any>>,
    pub name: String,
    readys: Vec<TypeId>,
    passes: Vec<TypeId>,
}
pub type Gfx = i32;
pub trait Ready {
    fn ready(&mut self, gfx: &Gfx);
}
pub trait Paint {
    fn paint(&mut self, gfx: &Gfx);
}
impl Scene {
    pub fn new(
        name: impl Into<String>,
    ) -> Self {
        Scene {
            res: HashMap::new(),
            name: name.into(),
            readys: Vec::new(),
            passes: Vec::new(),
        }
    }
    pub fn add_ready<T: Ready + 'static>(
        &mut self,
        ready: T,
    ) -> &mut Self {
        println!("add_ready ");
        self.res.insert(
            TypeId::of::<T>(),
            Box::new(ready),
        );
        let typeid = TypeId::of::<T>();
        println!("the typeid : {:?}", typeid);
        self.readys.push(TypeId::of::<T>());
        self
    }
    pub fn add_paint<T: Paint + 'static>(
        &mut self,
        paint: T,
    ) -> &mut Self {
        println!("add_print");
        self.res.insert(
            TypeId::of::<T>(),
            Box::new(paint),
        );
        self
    }
    pub fn get<T: 'static>(
        &self,
    ) -> Option<&T> {
        self.res
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
    }
    pub fn get_mut<T: 'static>(
        &mut self,
    ) -> Option<&mut T> {
        self.res
            .get_mut(&TypeId::of::<T>())
            .and_then(|b| b.downcast_mut())
    }
    pub fn run(&mut self, gfx: &Gfx) {
        for typeid in &self.readys {
            println!("iter readys: {:?}", typeid);
            if let Some(ready) =
                self.res.get_mut(typeid)
            {
                if let Some(ready) = ready.downcast_mut::<&mut dyn Ready>() {
                    ready.ready(gfx);
                }
            }
        }
        for typeid in &self.passes {
            println!("iter passes: {:?}", typeid);
            if let Some(paint) =
                self.res.get_mut(typeid)
            {
                if let Some(paint) = paint.downcast_mut::<&mut dyn Paint>() {
                    paint.paint(gfx);
                }
            }
        }
    }
}
