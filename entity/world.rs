use rsfml::graphics::{FloatRect, RenderWindow};
use entity::Entity;
use input::Input;
use std::vec;
use event::Event;

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

    pub fn update(&self, dt: f32, input: &Input) -> WorldUpdateResult {
        let mut new_entities: ~[Entity] = ~[];
        let mut new_events: ~[Event] = ~[];
        
        for entity in self.entities.iter() {
            let update_result = entity.update(dt, self, input);
            new_entities = vec::append(new_entities, update_result.new_entities);
            new_events = vec::append(new_events, update_result.events);
        }

        WorldUpdateResult {
            world: World::new(new_entities, self.bounds),
            events: new_events
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

pub struct WorldUpdateResult {
    world: World,
    events: ~[Event]
}
