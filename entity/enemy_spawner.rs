use rsfml::system::Vector2f;
use rsfml::graphics::RenderWindow;

use input::Input;
use std::rand;
use std::rand::Rng;
use entity::world::World;
use entity::enemy::Enemy;
use entity::{Entity, EntityUpdateResult};
use entity::sprite_renderer::SpriteRenderer;

pub struct EnemySpawner {
	time_since_spawn: f32,
	renderer: Option<SpriteRenderer>
}

impl EnemySpawner {
	pub fn new(renderer: Option<SpriteRenderer>) -> EnemySpawner {
		EnemySpawner {
			time_since_spawn: 0.,
			renderer: renderer
		}
	}
}

fn rand() -> f32 {
	let mut rng = rand::rng();
	
	if rng.gen() { 
		return rng.gen::<f32>();
	}

	return 0.;
}

impl Entity for EnemySpawner {
	fn update(&self, dt: f32, _world: &World, _input: &Input) -> EntityUpdateResult {
		let new_entities = if self.time_since_spawn > 1. {
			~[
				~Enemy {
					position: Vector2f::new(800.0f32 * rand(), 600.0f32 * rand()),
	    			rotation: 0.,
	    			renderer: self.renderer.as_ref().map_or(None, |r| -> Option<SpriteRenderer> { r.clone() })
				} as ~Entity:,
				~EnemySpawner {
					time_since_spawn: dt,
					renderer: self.renderer.as_ref().map_or(None, |r| -> Option<SpriteRenderer> { r.clone() }),
				} as ~Entity:
			]
		} else {
			~[
				~EnemySpawner {
					time_since_spawn: self.time_since_spawn + dt,
					renderer: self.renderer.as_ref().map_or(None, |r| -> Option<SpriteRenderer> { r.clone() }),
				} as ~Entity:
			]
		};

		return EntityUpdateResult { new_entities: new_entities };
	}


	fn draw(&self, _window: &mut RenderWindow) {		
	}

	fn clone(&self) -> ~Entity: {
		return ~EnemySpawner {
			time_since_spawn: self.time_since_spawn,
			renderer: self.renderer.as_ref().map_or(None, |r| -> Option<SpriteRenderer> { r.clone() }),
		} as ~Entity:;
	}
}
