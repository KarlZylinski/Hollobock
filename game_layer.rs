use layer::{Layer, LayerUpdateResult};
use rsfml::system::Vector2f;
use rsfml::graphics::RenderWindow;
use world::World;
use player::Player;
use entity::Entity;
use enemy::Enemy;
use input::Input;

pub struct GameLayer {
	world: World
}

impl GameLayer {	
	pub fn new() -> GameLayer {
		let player = Player {
	    	position: Vector2f::new(200., 200.),
	    	rotation: 0.,
	    	weapon_cooldown: 0.
	    };

	    let enemy = Enemy {
	    	position: Vector2f::new(500., 500.),
	    	rotation: 0.
	    };

		GameLayer {
			world: World {
				entities: ~[
					~player as ~Entity,
					~enemy as ~Entity
				]
			}
		}
	}
}

impl Layer for GameLayer {
	fn update(&self, dt: f32, input: &Input) -> LayerUpdateResult {
		let new_world = self.world.update(dt, input);
		
		let new_game_layer = ~GameLayer {
			world: new_world
		};

		return LayerUpdateResult {
			new_layers: ~[ new_game_layer as ~Layer ]
		};
	}

	fn draw(&self, window: &mut RenderWindow) {		
		self.world.draw(window);
	}

	fn clone(&self) -> ~Layer {
		~GameLayer {
			world: World { entities: self.world.entities.clone() }
		} as ~Layer
	}
}
