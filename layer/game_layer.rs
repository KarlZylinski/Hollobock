use layer::{LayerTrait, GameLayer, Layer};
use rsfml::graphics::{FloatRect, RenderWindow};
use input::Input;
use resource_store::ResourceStore;
use entity::world::{WorldUpdateResult, World};
use entity::renderer::Renderer;
use event::Event;

pub struct GameLayerStruct {
    world: World
}

impl GameLayerStruct {    
    pub fn new(resource_store: &mut ResourceStore) -> GameLayerStruct {
        GameLayerStruct {
            world: World::new(
                resource_store.load_level(~"level.json"),
                FloatRect::new(0.0, 0.0, 800.0, 600.0)
            )
        }
    }
}

impl LayerTrait for GameLayerStruct {
    fn update(&mut self, dt: f32, input: &Input) -> ~[Event] {
        let world_update_result = self.world.update(dt, input);

        match world_update_result {
            WorldUpdateResult { world, events } => {
                self.world = world;
                return events;
            }
        }
    }

    fn draw(&self, window: &mut RenderWindow) {     
        self.world.draw(window);
    }

    fn clone(&self) -> Layer {
        GameLayer(~GameLayerStruct {
            world: self.world.clone()
        })
    }
}
