pub enum Event {
    PlayerHealthChanged(u8)
}

pub trait EventHandler {
    fn handle_event(&mut self, event: Event) -> bool;
}
