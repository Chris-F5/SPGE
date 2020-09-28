use super::entity::Entity;

pub trait ComponentStorage<ComponentType>
where
    ComponentType: Component,
{
    fn get_component(&self, entity: Entity) -> &ComponentType;
    fn attach_component(&mut self, entity: Entity, component: ComponentType);
    fn remove_component(&mut self, entity: Entity);
}

pub trait Component {}
impl<T> Component for T where T: Debug {}
