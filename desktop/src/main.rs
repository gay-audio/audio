use desktop::{components::Component, event::Update};

pub struct Tester;
impl Component for Tester {
    type Event = Update;
    fn update(&mut self, event: &Self::Event) {
        println!("Thing happened: {event:?}");
    }
}

pub trait Widget {
    fn render(&self);
}

pub struct State {}

fn main() {
    let mut component = Tester;
}
