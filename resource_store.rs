use std::rc::Rc;
use std::cell::RefCell;
use std::hashmap::HashMap;
use std::vec;
use std::io::File;

use extra::json;
use extra::json::{Json, List, Object, String};

use rsfml::graphics::Texture;
use rsfml::graphics::Sprite;
use rsfml::system::Vector2f;

use entity::renderer::Renderer;
use entity::sprite_renderer::SpriteRenderer;
use entity::Entity;
use entity::player::Player;


pub struct ResourceStore {
    textures: HashMap<~str, Option<Rc<RefCell<Texture>>>>,
    entities: HashMap<~str, ~Entity>
}

impl ResourceStore {
    pub fn new() -> ResourceStore {
        ResourceStore {
            textures: HashMap::new(),
            entities: HashMap::new()
        }
    }

    pub fn load_level(&mut self, filename: ~str) -> ~[~Entity:] {
        let level_json_str = File::open(&Path::new(filename)).read_to_end();
        let level_json = match json::from_str(level_json_str.to_str()) {
            Ok(json) => json,
            Err(e) => fail!("Failed to load level.")
        };

        return match level_json {
            json::Object(~o) => self.load_entities(o),
            _ => ~[]
        }
    }

    pub fn load_texture(&mut self, filename: ~str) -> Option<Rc<RefCell<Texture>>> {
        self.textures.find_or_insert_with(filename, |filename_to_load_from|
            Texture::new_from_file(*filename_to_load_from).map(|new_texture|
                 Rc::new(RefCell::new(new_texture))
            )   
        ).take()
    }

    fn load_entities(&mut self, o: json::Object) -> ~[~Entity:] {
        let no_entities: json::List = ~[];
        let entities_json = o.find(&~"entities").map_or(&no_entities, |e| match e {
            &json::List(ref l) => l,
            _ => &no_entities
        });

        let mut new_entities: ~[~Entity:] = ~[];

        for entity_json in entities_json.iter() {
            let entity_obj = match entity_json { &json::Object(ref o) => Some(o), _ => None };
            
            match self.load_entity(entity_obj) {
                Some(e) => new_entities = vec::append_one(new_entities, e),
                None => {}
            };
        }

        new_entities
    }

    fn load_entity(&mut self, object: Option<&~json::Object>) -> Option<~Entity:> {
        object.map_or(None, |o| {
            let entity_type = o.find(&~"type").as_ref().map_or(None, |&t| match t { &json::String(ref s) => Some(s), _ => None } );
            let renderer = self.load_renderer(o.find(&~"renderer"));

            return match entity_type {
                Some(et) => {
                    if et == &~"player" {
                        return match ResourceStore::load_vector2f(o.find(&~"position")) {
                            Some(v) => Some(~Player::new(v, renderer) as ~Entity:),
                            None => None
                        };
                    }

                    None
                },
                None => None
            };
        })
    }

    fn load_renderer(&mut self, object: Option<&json::Json>) -> Option<~Renderer:> {
        object.as_ref().map_or(None, |&r| {
            match r {
                &json::Object(ref o) => { 
                    let renderer_type = o.find(&~"type").as_ref().map_or(None, |&t| match t { &String(ref t) => Some(t), _ => None }) ;
                    
                    renderer_type.map_or(None, |t| {
                        if t == &~"sprite_renderer" {
                            let texture = o.find(&~"texture").as_ref().map_or(None, |&t| match t { &String(ref t) => self.load_texture(t.clone()), _ => None });

                            return texture.map_or(None, |t|
                                Sprite::new_with_texture(t).map(|s|
                                    ~SpriteRenderer::new(s) as ~Renderer:
                                )
                            );
                        }

                        None
                    })
                },
                _ => None
            }
        })
    }

    fn load_vector2f(vector_json: Option<&json::Json>) -> Option<Vector2f> {
        vector_json.as_ref().map_or(None, |&vector| {
            match vector {
                &List(ref l) => {
                    match (&l[0], &l[1]) {
                        (&json::Number(ref x), &json::Number(ref y)) => Some(Vector2f::new(x.clone() as f32, y.clone() as f32)),
                        _ => None
                    }
                },
                _ => None
            }
        })
    }
}
