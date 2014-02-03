use layer::{Layer, LayerUpdateResult};
use rsfml::graphics::RenderWindow;
use input::Input;
use resource_store::ResourceStore;
use entity::world::World;
use entity::Entity;
use entity::renderer::Renderer;

pub struct GameLayer {
    world: World
}

impl GameLayer {    
    pub fn new(resource_store: &mut ResourceStore) -> GameLayer {
        GameLayer {
            world: World {
                entities: resource_store.load_level(~"level.json")
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
