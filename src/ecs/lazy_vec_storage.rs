use super::{component_storage::Component, component_storage::ComponentStorage, entity::Entity};
use std::collections::{hash_map::HashMap, linked_list::LinkedList};

pub struct LazyVecStorage<ComponentType> {
    components: Vec<ComponentType>,
    entity_map: HashMap<usize, usize>,
    isolated_unallocated_indices: LinkedList<usize>,
}
impl<ComponentType> LazyVecStorage<ComponentType> {
    fn new() -> LazyVecStorage<ComponentType> {
        LazyVecStorage {
            components: Vec::with_capacity(16),
            entity_map: HashMap::with_capacity(16),
            isolated_unallocated_indices: LinkedList::new(),
        }
    }
}
impl<ComponentType> ComponentStorage<ComponentType> for LazyVecStorage<ComponentType>
where
    ComponentType: Component,
{
    fn remove_component(&mut self, entity: Entity) {
        if let Some(component_index) = self.entity_map.remove(&entity.id) {
            if component_index == self.components.len() - 1 {
                self.components.pop();
            } else {
                self.isolated_unallocated_indices.push_back(component_index);
            }
        } else {
            panic!("The entity does not have this component type attached.");
        }
    }
    fn attach_component(&mut self, entity: Entity, component: ComponentType) {
        if let Some(index) = self.isolated_unallocated_indices.pop_front() {
            self.entity_map.insert(entity.id, index);
            self.components[index] = component;
        } else {
            self.entity_map.insert(entity.id, self.components.len());
            self.components.push(component);
        }
    }
    fn get_component(&self, entity: Entity) -> &ComponentType {
        if let Some(component_index) = self.entity_map.get(&entity.id) {
            return &self.components[*component_index];
        } else {
            panic!("The entity does not have this component type attached.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Component, ComponentStorage, Entity, LazyVecStorage};

    struct TestComponent {
        id: i8,
    }
    //impl Component for TestComponent {}

    #[test]
    fn component_pool() {
        let mut pool = LazyVecStorage::<TestComponent>::new();
        let c1 = TestComponent { id: 1 };
        let c2 = TestComponent { id: 2 };
        let c3 = TestComponent { id: 3 };
        let e1 = Entity { id: 1 };
        let e2 = Entity { id: 2 };
        let e3 = Entity { id: 3 };

        pool.attach_component(e1, c1);
        assert_eq!(pool.get_component(e1).id, 1);

        pool.attach_component(e2, c2);
        assert_eq!(pool.get_component(e1).id, 1);
        assert_eq!(pool.get_component(e2).id, 2);
        assert_eq!(pool.entity_map.len(), 2);

        pool.remove_component(e1);
        assert_eq!(pool.get_component(e2).id, 2);
        assert_eq!(pool.isolated_unallocated_indices.len(), 1);
        assert_eq!(pool.entity_map.len(), 1);

        pool.attach_component(e3, c3);
        assert_eq!(pool.get_component(e3).id, 3);
        assert_eq!(pool.components.len(), 2);
        assert_eq!(pool.isolated_unallocated_indices.len(), 0);
        assert_eq!(pool.entity_map.len(), 2);
    }
}
