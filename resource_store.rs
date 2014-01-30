use rsfml::graphics::Texture;
use std::rc::Rc;
use std::cell::RefCell;
use std::hashmap::HashMap;

pub struct ResourceStore {
	textures: HashMap<~str, Rc<RefCell<Texture>>>
}

impl ResourceStore {
	pub fn new() -> ResourceStore {
		ResourceStore {
			textures: HashMap::new()
		}
	}

    pub fn load_texture(&mut self, filename: ~str) -> Rc<RefCell<Texture>> {
        return match self.textures.find_copy(&filename) {
            Some(texture) => texture,
            None => {
                let new_texture = match Texture::new_from_file(filename) {
                    Some(new_texture) => new_texture,
                    None => fail!("Could not load texture.")
                };

                Rc::new(RefCell::new(new_texture))
            }
        }    	
    }
}
