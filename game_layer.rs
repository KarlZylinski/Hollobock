use layer::{Layer, LayerUpdateResult};
use rsfml::system::Vector2f;
use rsfml::graphics::{Sprite, RenderWindow, Sprite};
use world::World;
use player::Player;
use entity::Entity;
use input::Input;
use enemy_spawner::EnemySpawner;
use resource_manager::ResourceManager;

pub struct GameLayer {
	world: World
}

impl GameLayer {	
	pub fn new(resource_manager: &mut ResourceManager) -> GameLayer {
	    let player_tex = resource_manager.load_texture("player.png");

		let player = Player {
	    	position: Vector2f::new(200., 200.),
	    	rotation: 0.,
	    	sprite: match Sprite::new_with_texture(player_tex) {
	    		Some(sprite) => sprite,
	    		None => fail!("Failed to create sprite!")
	    	},
	    	weapon_cooldown: 0.
	    };

	    let enemy_spawner = EnemySpawner::new();

		GameLayer {
			world: World {
				entities: ~[
					~player as ~Entity:,
					~enemy_spawner as ~Entity
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
			new_layers: ~[ new_game_layer as ~Layer: ]
		};
	}

	fn draw(&self, window: &mut RenderWindow) {		
		self.world.draw(window);
	}

	fn clone(&self) -> ~Layer: {
		~GameLayer {
			world: World { entities: self.world.entities.clone() }
		} as ~Layer:
	}
}
