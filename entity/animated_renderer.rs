use std::rc::Rc;
use std::cell::RefCell;

use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, FloatRect, IntRect, Texture};
use rsfml::graphics::rc::Sprite;

use entity::renderer::Renderer;

pub struct AnimatedRenderer {
    sprite: Sprite,
    frames: i32,
    frame: i32,
    time_this_frame: f32,
    time_per_frame: f32
}

impl AnimatedRenderer {
    pub fn new(texture: Rc<RefCell<Texture>>, frames: i32, time_per_frame: f32) -> Option<AnimatedRenderer> {
        let texture_size = texture.borrow().with(|t| t.get_size());

        match Sprite::new_with_texture(texture) {
            Some(mut s) => {
                let rect = IntRect::new(0, 0, (texture_size.x as i32) / frames, texture_size.y as i32);

                s.set_texture_rect(&rect);

                Some(AnimatedRenderer {
                    sprite: s,
                    frames: frames,
                    frame: 0,
                    time_per_frame: time_per_frame,
                    time_this_frame: 0.0
                })
            },
            None => None
        }
    }

    fn get_next_frame(&self) -> i32 {
        let next = self.frame + 1;

        if next >= self.frames {
            0
        } else {
            next
        }
    }

    fn update_animation(&self, dt: f32) -> (IntRect, i32, f32) {
        let (frame, time_this_frame) = if self.time_this_frame + dt > self.time_per_frame {
            (self.get_next_frame(), 0.0)
        } else {
            (self.frame, self.time_this_frame + dt)
        };

        let texture_rect = self.sprite.get_texture_rect();
        (IntRect::new(texture_rect.width * frame, 0, texture_rect.width, texture_rect.height), frame, time_this_frame)
    }
}

impl Renderer for AnimatedRenderer {
     fn update(&self, position: &Vector2f, rotation: f32, dt: f32) -> Option<~Renderer:> {
        self.sprite.clone().map(|mut new_sprite: Sprite| {
            new_sprite.set_position(position);
            new_sprite.set_rotation(rotation);

            let sprite_center = Vector2f::new(new_sprite.get_local_bounds().width * 0.5, new_sprite.get_local_bounds().height * 0.5);
            new_sprite.set_origin(&sprite_center);

            let (texture_rect, frame, time_this_frame) = self.update_animation(dt);
            new_sprite.set_texture_rect(&texture_rect);

            ~AnimatedRenderer {
                sprite: new_sprite,
                frames: self.frames,
                frame: frame,
                time_per_frame: self.time_per_frame,
                time_this_frame: time_this_frame
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
            ~AnimatedRenderer {
                sprite: new_sprite,
                frames: self.frames,
                frame: self.frame,
                time_per_frame: self.time_per_frame,
                time_this_frame: self.time_this_frame
            } as ~Renderer:
        )
    }
}
