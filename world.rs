use rsfml::graphics::{RenderWindow};
use list;
use list::{List, Cons};
use entity;
use input::{Input};

pub struct World {
	entities: List<~entity::Entity>
}

pub fn update(dt: f32, entities: &List<~entity::Entity>, input: &Input) -> List<~entity::Entity> {
	let mut entity_list = Some(entities);
	let mut new_entities = list::Nil;
	
	loop {
		match entity_list {
			Some(entity) => {
				match(entity) {
					&Cons(ref x, ~ref next) => {
						let update_result = x.update(dt, input);
						match(update_result.new_entities) {
							list::Nil => {},
							_ => new_entities = list::concat(new_entities, update_result.new_entities)
						}
						entity_list = Some(next);				
					},
					_ => entity_list = None
				}
			},
			None => break
		}
	}

	return new_entities;
}

pub fn draw(window: &mut RenderWindow, entities: &List<~entity::Entity>) {
	let mut entity_list = Some(entities);

	loop {
		match entity_list {
			Some(entity) => {
				match(entity) {
					&Cons(ref x, ~ref next) => {
						x.draw(window);
						entity_list = Some(next);
					},
					_ => entity_list = None
				}
			},
			None => break
		}
	}
}