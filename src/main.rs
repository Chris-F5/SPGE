mod ecs;

fn main() {
    println!("Hello, world!");

    let mut s = TestSystem {};
    s.run_now();
}

use ecs::system::System;
use ecs::system::SystemData;

impl SystemData for i8 {
    fn fetch() -> Self {
        7
    }
}

impl SystemData for i32 {
    fn fetch() -> Self {
        999
    }
}

pub struct TestSystem {}
impl System for TestSystem {
    type SystemData = (i32, i8);
    fn run(&mut self, (a, b): Self::SystemData) {
        println!("{} {}", a, b)
    }
    fn run_now(&mut self) {
        let data = Self::SystemData::fetch();
        self.run(data);
    }
}
