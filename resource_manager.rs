use rsfml::graphics::{RenderWindow, RectangleShape, Texture};
use std::rc::Rc;
use std::cell::RefCell;

pub struct ResourceManager {
	textures: ~[Rc<Texture>]
}

impl ResourceManager {
	pub fn new() -> ResourceManager {
		ResourceManager {
			textures: ~[]
		}
	}

    pub fn load_texture(&mut self, filename: &str) -> Rc<RefCell<Texture>> {
    	let new_texture = match Texture::new_from_file(filename) {
    		Some(new_texture) => new_texture,
    		None => fail!("Could not load texture")
    	};

    	return Rc::new(RefCell::new(new_texture));
    }
}
