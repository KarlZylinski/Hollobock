use std::rc::Rc;
use std::cell::RefCell;
use std::num::abs;

use rsfml::system::Vector2f;
use rsfml::graphics::{RenderWindow, Texture};
use rsfml::graphics::rc::Sprite;

use std::cmp::{min, max};

#[deriving(Clone)]
pub struct Bar {
	max: f32,
    current: f32,
    target: f32,
    change_rate_multiplier: f32,
    texture: Rc<RefCell<Texture>>,
    position: Vector2f,
    size: Vector2f
}

impl Bar {
	pub fn new(max: f32, current: f32, texture: Rc<RefCell<Texture>>, position: &Vector2f, size: &Vector2f) -> Bar {
		Bar {
			max: max,
			current: current,
			target: current,
            change_rate_multiplier: 1.0,
			texture: texture.clone(),
			position: position.clone(),
			size: size.clone()
		}
	}

	pub fn update(&self, dt: f32) -> Bar {
        let delta = self.target - self.current;

        let current = if abs(self.target - self.current) < 0.00001 {
        	self.target
        } else {
        	self.current + delta * dt * self.change_rate_multiplier
        };

        Bar {
        	max: self.max,
        	current: current,
        	target: self.target,
            change_rate_multiplier: self.change_rate_multiplier,
        	texture: self.texture.clone(),
        	position: self.position.clone(),
        	size: self.size.clone()
        }
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        Sprite::new_with_texture(self.texture.clone()).map(|mut s| {
            s.set_position(&self.position);
            let size = Vector2f::new(self.size.x * self.current / self.max, self.size.y);
            s.set_scale(&size);
            window.draw(&s);
        });
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = max(min(target, self.max), 0.0);
    }
}