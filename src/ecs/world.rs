use std::any::TypeId;
use std::collections::hash_map::HashMap;

pub struct World {
    component_managers: HashMap<TypeId, Box<Resource>>,
}
trait Resource {}
