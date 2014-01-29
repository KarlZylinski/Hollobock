use rsfml::system::Vector2f;
use rsfml::graphics::RenderWindow;

use entity::{Entity, EntityUpdateResult};
use input::Input;
use world::World;
use enemy::Enemy;
use std::rand;
use std::rand::Rng;

pub struct EnemySpawner {
	time_since_spawn: f32
}

impl EnemySpawner {
	pub fn new() -> EnemySpawner {
		EnemySpawner {
			time_since_spawn: 0.
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
		let new_entities = if(self.time_since_spawn > 1.) {
			~[
				~Enemy {
					position: Vector2f::new(800.0f32 * rand(), 600.0f32 * rand()),
	    			rotation: 0.,
				} as ~Entity,
				~EnemySpawner {
					time_since_spawn: dt
				} as ~Entity
			]
		} else {
			~[
				~EnemySpawner {
					time_since_spawn: self.time_since_spawn + dt
				} as ~Entity
			]
		};

		return EntityUpdateResult { new_entities: new_entities };
	}


	fn draw(&self, _window: &mut RenderWindow) {		
	}

	fn clone(&self) -> ~Entity {
		return ~EnemySpawner { time_since_spawn: self.time_since_spawn } as ~Entity;
	}
}
