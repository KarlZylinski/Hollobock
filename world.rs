use rsfml::graphics::{RenderWindow};
use list;
use entity;

pub struct World {
	entities: list::List<~entity::Entity>
}


pub fn update(dt: f32, entities: &list::List<~entity::Entity>, new_entities: list::List<~entity::Entity>, window: &RenderWindow) -> list::List<~entity::Entity> {
	match entities {        
		&list::Cons(ref x, ~ref next) => {
			let update_result = x.update(dt, window);
			match(update_result.new_entities) {
				list::Nil => return update(dt, next, new_entities, window),
				_ => return update(dt, next, list::concat(new_entities, update_result.new_entities), window)
			}
			
		},
		_ => { return new_entities; }
	}
}

pub fn draw(window: &mut RenderWindow, entities: &list::List<~entity::Entity>) {
	match entities {        
		&list::Cons(ref x, ~ref next) => {
			x.draw(window);
			draw(window, next);
		},
		_ => {}
	}
}