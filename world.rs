extern mod rsfml;

use rsfml::graphics::{RenderWindow};

pub mod entity;
pub mod list;

pub struct World {
	entities: ~list::List<entity::Entity>
}

pub fn draw(window: &mut RenderWindow, entity: &list::List<entity::Entity>) {
	match entity {        
		&list::Cons(x, ~ref next) => {
			entity::draw(window, &x);
			draw(window, next);
		},
		_ => {}
	}
}