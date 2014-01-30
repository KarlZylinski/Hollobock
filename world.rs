use rsfml::graphics::RenderWindow;
use entity::Entity;
use input::Input;
use std::vec;

pub struct World {
	entities: ~[~Entity:]
}

impl World {		
	pub fn update(&self, dt: f32, input: &Input) -> World {
		let mut new_entities: ~[~Entity:] = ~[];
		
		for entity in self.entities.iter() {
			let update_result = entity.update(dt, self, input);
			new_entities = vec::append(new_entities, update_result.new_entities);
		}

		return World {
			entities: new_entities
		};
	}

	pub fn draw(&self, window: &mut RenderWindow) {
		for entity in self.entities.iter() {
			entity.draw(window);
		}
	}
}
