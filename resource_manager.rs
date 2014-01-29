use rsfml::graphics::{RenderWindow, RectangleShape, Texture};
use std::gc::Gc;

pub struct ResourceManager {
	textures: ~[Gc<Texture>]
}

impl ResourceManager {
	pub fn new() -> ResourceManager {
		ResourceManager {
			textures: ~[]
		}
	}

    pub fn load_texture(&mut self, filename: &str) -> Gc<Texture> {
    	let new_texture = match Texture::new_from_file(filename) {
    		Some(new_texture) => new_texture,
    		None => fail!("Could not load texture")
    	};

    	return Gc::new(new_texture);
    }
}
