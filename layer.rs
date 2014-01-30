use rsfml::graphics::RenderWindow;
use input::Input;

pub trait Layer {
	fn update(&self, dt: f32, input: &Input) -> LayerUpdateResult;
	fn draw(&self, window: &mut RenderWindow);
	fn clone(&self) -> ~Layer:;
}

impl Clone for ~Layer: {
	fn clone(&self) -> ~Layer: {
		return self.clone();
	}
}

pub struct LayerUpdateResult {
	new_layers: ~[~Layer:]
}
