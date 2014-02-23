pub enum Event {
    UpdateHealthBar {
        health: f32
    }
}

trait EventPropagator {
	fn propagate_event(&mut self, event: &Event) -> bool;
}