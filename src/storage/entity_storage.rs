pub trait EntityInnerStorage<T> {
    fn get_mut(&mut self, id: u32) -> &mut Self;
    fn get(&self, id: u32) -> &Self;
}
