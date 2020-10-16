use crate::storage::entity_storage::EntityInnerStorage;

pub trait EntityComponent: Sized {
    type Storage: EntityInnerStorage<Self>;
}
