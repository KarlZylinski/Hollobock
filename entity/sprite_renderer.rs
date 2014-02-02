use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, Sprite};

pub struct SpriteRenderer {
	sprite: Sprite
}

impl SpriteRenderer {
	pub fn new(sprite: Sprite) -> SpriteRenderer {
		SpriteRenderer {
			sprite: sprite
		}
	}

	pub fn update(&self, position: &Vector2f, rotation: f32) -> Option<SpriteRenderer> {
		self.sprite.clone().map(|mut new_sprite: Sprite| {
				new_sprite.set_position(position);
				new_sprite.set_rotation(rotation);

				let sprite_center = Vector2f::new(new_sprite.get_local_bounds().width * 0.5, new_sprite.get_local_bounds().height * 0.5);
				new_sprite.set_origin(&sprite_center);

				SpriteRenderer {
					sprite: new_sprite
				}
			}
		)
	}

	pub fn draw(&self, window: &mut RenderWindow) {
		window.draw(&self.sprite);
	}

	pub fn clone(&self) -> Option<SpriteRenderer> {
		self.sprite.clone().map(|new_sprite| 
			SpriteRenderer {
				sprite: new_sprite
			}
		)
	}
}
