use layer::{Layer, LayerUpdateResult};
use rsfml::system::Vector2f;
use rsfml::graphics::{RenderWindow, RectangleShape, Color};
use input::Input;

pub struct GuiLayer;

impl GuiLayer {	
	pub fn new() -> GuiLayer {
		GuiLayer
	}
}

impl Layer for GuiLayer {
	fn update(&self, _dt: f32, _input: &Input) -> LayerUpdateResult {
		LayerUpdateResult {
			new_layers: ~[~GuiLayer as ~Layer]
		}
	}

	fn draw(&self, window: &mut RenderWindow) {		
		let mut rectangle = match RectangleShape::new() {
			Some(rectangle) => rectangle,
			None() => fail!("Error, cannot create rectangle.")
		};

		rectangle.set_fill_color(&Color::green());
		rectangle.set_size(&Vector2f::new(120., 10.));
		rectangle.set_position(&Vector2f::new(10.,10.));

		window.draw(&rectangle);
	}

	fn clone(&self) -> ~Layer {
		~GuiLayer as ~Layer
	}
}
