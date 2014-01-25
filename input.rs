use rsfml::window::{keyboard, mouse};
use rsfml::system::Vector2i;
use std::vec;

pub struct Input {
	keys_pressed: ~[bool],
	keys_released: ~[bool],
	keys_held: ~[bool],
	mouse_position: Vector2i,
	mouse_position_previous_frame: Vector2i,
	mouse_buttons_pressed: ~[bool],
	mouse_buttons_released: ~[bool],
	mouse_buttons_held: ~[bool],
}

impl Input {
	pub fn init(mouse_position: Vector2i) -> Input {
		return Input {
	    	keys_pressed: vec::from_elem(120, false),
			keys_released: vec::from_elem(120, false),
			keys_held: vec::from_elem(120, false),
			mouse_position: mouse_position,
			mouse_position_previous_frame: mouse_position,
			mouse_buttons_pressed: vec::from_elem(10, false),
			mouse_buttons_released: vec::from_elem(10, false),
			mouse_buttons_held: vec::from_elem(10, false)
    	};
	}

	pub fn init_from_previous_frame(from_previous_frame: &Input) -> Input {
		return Input {
	    	keys_pressed: vec::from_elem(120, false),
			keys_released: vec::from_elem(120, false),
			keys_held: from_previous_frame.keys_held.clone(),
			mouse_position: from_previous_frame.mouse_position,
			mouse_position_previous_frame: from_previous_frame.mouse_position,
			mouse_buttons_pressed: vec::from_elem(10, false),
			mouse_buttons_released: vec::from_elem(10, false),
			mouse_buttons_held: from_previous_frame.mouse_buttons_held.clone()
    	};
	}

	pub fn set_key_pressed(&mut self, key: keyboard::Key) {
		if self.keys_held[key as int] == true {
			return;
		}

		self.keys_pressed[key as int] = true;
		self.keys_held[key as int] = true;
	}

	pub fn key_pressed(&self, key: keyboard::Key) -> bool {
		return self.keys_pressed[key as int];
	}

	pub fn set_key_released(&mut self, key: keyboard::Key) {
		self.keys_released[key as int] = true;
		self.keys_held[key as int] = false;
	}

	pub fn key_released(&self, key: keyboard::Key) -> bool {
		return self.keys_released[key as int];
	}

	pub fn key_held(&self, key: keyboard::Key) -> bool {
		return self.keys_held[key as int];
	}

	pub fn set_mouse_button_pressed(&mut self, button: mouse::MouseButton) {
		if self.mouse_buttons_held[button as int] == true {
			return;
		}

		self.mouse_buttons_pressed[button as int] = true;
		self.mouse_buttons_held[button as int] = true;
	}

	pub fn mouse_button_pressed(&self, button: mouse::MouseButton) -> bool {
		return self.mouse_buttons_pressed[button as int];
	}

	pub fn set_mouse_button_released(&mut self, button: mouse::MouseButton) {
		self.mouse_buttons_released[button as int] = true;
		self.mouse_buttons_held[button as int] = false;
	}

	pub fn mouse_button_released(&self, button: mouse::MouseButton) -> bool {
		return self.mouse_buttons_released[button as int];
	}

	pub fn mouse_button_held(&self, button: mouse::MouseButton) -> bool {
		return self.mouse_buttons_held[button as int];
	}

	pub fn mouse_delta(&self) -> Vector2i {
		return self.mouse_position - self.mouse_position_previous_frame;
	}
}