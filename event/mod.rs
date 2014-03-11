#[deriving(Clone)]
pub enum Event {
    PlayerHealthChanged(u8)
}

pub trait EventHandler {
    fn handle(&mut self, event: Event) -> bool;
}
