use rsfml::graphics::Texture;
use std::rc::Rc;
use std::cell::RefCell;
use std::hashmap::HashMap;

pub struct ResourceStore {
	textures: HashMap<~str, Option<Rc<RefCell<Texture>>>>
}

impl ResourceStore {
	pub fn new() -> ResourceStore {
		ResourceStore {
			textures: HashMap::new()
		}
	}

    pub fn load_texture(&mut self, filename: ~str) -> Option<Rc<RefCell<Texture>>> {
        self.textures.find_or_insert_with(filename, |filename_to_load_from|
            Texture::new_from_file(*filename_to_load_from).map(|new_texture|
                 Rc::new(RefCell::new(new_texture))
            )   
        ).take()
    }
}
