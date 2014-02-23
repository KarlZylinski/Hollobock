use rsfml::graphics::{FloatRect, RenderWindow};
use entity::Entity;
use input::Input;
use std::vec;

pub struct World {
    entities: ~[Entity],
    bounds: FloatRect
}

impl World {
    pub fn new(entities: ~[Entity], bounds: FloatRect) -> World {
        World {
            entities: entities.clone(),
            bounds: bounds
        }
    }

    pub fn update(&self, dt: f32, input: &Input) -> World {
        let mut new_entities: ~[Entity] = ~[];
        let mut events: ~[Event] = ~[];
        
        for entity in self.entities.iter() {
            let update_result = entity.update(dt, self, input);
            new_entities = vec::append(new_entities, update_result.new_entities);
            events = vec::append(events, update_result.events)
        }

        WorldUpdateResult {
            world: World::new(new_entities, self.bounds),
            events: events
        }
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        for entity in self.entities.iter() {
            entity.draw(window);
        }
    }

    pub fn clone(&self) -> World {
        World {
            entities: self.entities.clone(),
            bounds: self.bounds
        }
    }
}

struct WorldUpdateResult {
    world: World,
    events: ~[Event]
}