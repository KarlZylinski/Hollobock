extern mod rsfml;

use rsfml::graphics::{RenderWindow};
use super::list;
use super::entity;

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