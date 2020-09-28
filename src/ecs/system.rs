use super::world::World;

pub trait System {
    type SystemData: SystemData;
    fn run(&mut self, system_data: Self::SystemData);
    fn run_now(&mut self) {
        let data = Self::SystemData::fetch();
        self.run(data);
    }
}

pub trait SystemData {
    fn setup(world: &mut World) {}
    fn fetch() -> Self;
}

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: SystemData),+> SystemData for ($($name,)+)
        {
            fn setup(world: &mut World){
                $( <$name as SystemData>::setup(world); )*
            }
            fn fetch()->Self{
                ( $( <$name as SystemData>::fetch(), )* )
            }
        }
    };
}

tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
