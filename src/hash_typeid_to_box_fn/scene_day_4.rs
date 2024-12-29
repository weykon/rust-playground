use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

use super::scene_day_2::Scene;

struct Scene<T> {
    hash_data: HashMap<TypeId, Box<dyn Any>>,
    readys: Vec<TypeId>,
    _marker: PhantomData<T>,
}
trait Ready: 'static {
    fn ready(&mut self);
}
impl<T> Scene<T>
where
    T: Default + 'static + Ready,
{
    fn add_ready(&mut self) {
        self.hash_data.insert(
            TypeId::of::<T>(),
            Box::new(T::default()),
        );
        self.readys.push(TypeId::of::<T>());
    }
    fn run(&mut self) {
        self.readys.iter().for_each(|typeid| {
            if let Some(data) = self.hash_data.get_mut(typeid) {
                if let Some( ready_component) = data.downcast_mut::<Box<dyn Ready>>() {
                    ready_component.ready();
                }
            }
        });
    }
}
#[derive(Default)]
struct VertexReady {
    lots_of_data: Vec<f32>,
}
impl Ready for VertexReady {
    fn ready(&mut self) {
        self.lots_of_data = vec![1.0];
    }
}
trait GetRes: Ready {
    fn get_res(
        &self,
    ) -> &HashMap<TypeId, Box<dyn Any>>;
}
impl<T: Ready> GetRes for T
where
    T: Default + 'static + Ready,
{
    fn get_res(
        &self,
    ) -> &HashMap<TypeId, Box<dyn Any>> {
        &self.hash_data
    }
}
