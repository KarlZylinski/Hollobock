use rsfml::graphics::{RenderWindow};
use list;
use list::{List, Cons};
use entity;
use input::{Input};

pub struct World {
	entities: List<~entity::Entity>
}

pub fn update(dt: f32, entities: &List<~entity::Entity>, new_entities: List<~entity::Entity>, input: &Input) -> List<~entity::Entity> {
	match entities {        
		&Cons(ref x, ~ref next) => {
			let update_result = x.update(dt, input);
			match(update_result.new_entities) {
				list::Nil => return update(dt, next, new_entities, input),
				_ => return update(dt, next, list::concat(new_entities, update_result.new_entities), input)
			}
			
		},
		_ => { return new_entities; }
	}
}

pub fn draw(window: &mut RenderWindow, entities: &List<~entity::Entity>) {
	match entities {        
		&Cons(ref x, ~ref next) => {
			x.draw(window);
			draw(window, next);
		},
		_ => {}
	}
}