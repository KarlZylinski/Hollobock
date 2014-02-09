use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, FloatRect};
use rsfml::graphics::rc::Sprite;

use entity::renderer::Renderer;

pub struct SpriteRenderer {
    sprite: Sprite
}

impl SpriteRenderer {
    pub fn new(sprite: Sprite) -> SpriteRenderer {
        SpriteRenderer {
            sprite: sprite
        }
    }
}

impl Renderer for SpriteRenderer {
     fn update(&self, position: &Vector2f, rotation: f32, _dt: f32) -> Option<~Renderer:> {
        self.sprite.clone().map(|mut new_sprite: Sprite| {
            new_sprite.set_position(position);
            new_sprite.set_rotation(rotation);

            let sprite_center = Vector2f::new(new_sprite.get_local_bounds().width * 0.5, new_sprite.get_local_bounds().height * 0.5);
            new_sprite.set_origin(&sprite_center);

            ~SpriteRenderer {
                sprite: new_sprite
            } as ~Renderer:
        })
    }

    fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.sprite);
    }

    fn bounds(&self) -> FloatRect {
        return self.sprite.get_global_bounds();
    }

    fn clone(&self) -> Option<~Renderer:> {
        self.sprite.clone().map(|new_sprite| 
            ~SpriteRenderer {
                sprite: new_sprite
            } as ~Renderer:
        )
    }
}
