use super::entity::Entity;

pub trait ComponentStorage<ComponentType> {
    fn get_component(&self, entity: Entity) -> &ComponentType;
    fn attach_component(&mut self, entity: Entity, component: ComponentType);
    fn remove_component(&mut self, entity: Entity);
}
