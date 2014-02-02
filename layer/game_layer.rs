use layer::{Layer, LayerUpdateResult};
use rsfml::system::Vector2f;
use rsfml::graphics::{RenderWindow, Sprite};
use input::Input;
use resource_store::ResourceStore;
use entity::world::World;
use entity::Entity;
use entity::player::Player;
use entity::enemy_spawner::EnemySpawner;
use entity::sprite_renderer::SpriteRenderer;

pub struct GameLayer {
	world: World
}

impl GameLayer {	
	pub fn new(resource_store: &mut ResourceStore) -> GameLayer {

		let player = Player {
	    	position: Vector2f::new(200., 200.),
	    	rotation: 0.,
	    	renderer: resource_store.load_texture(~"player.png").map_or(None,|t| -> Option<SpriteRenderer> {
	    		Sprite::new_with_texture(t).map(|s| -> SpriteRenderer {
	    			SpriteRenderer::new(s)
	    		})
	    	}),
	    	weapon_cooldown: 0.
	    };

	    let enemy_spawner = EnemySpawner::new(
	    	resource_store.load_texture(~"enemy.png").map_or(None,|t| -> Option<SpriteRenderer> {
	    		Sprite::new_with_texture(t).map(|s| -> SpriteRenderer {
	    			SpriteRenderer::new(s)
	    		})
	    	})
		);

		GameLayer {
			world: World {
				entities: ~[
					~player as ~Entity:,
					~enemy_spawner as ~Entity:
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
