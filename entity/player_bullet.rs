use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, RectangleShape, FloatRect};

use input::Input;
use entity::world::World;
use entity::PlayerBullet;
use entity::{Entity, Enemy, EntityUpdateResult};
use entity::EntityTrait;

pub struct PlayerBulletStruct {
    position: Vector2f,
    direction: Vector2f,
    velocity: f32,
    health: int
}

fn intersecting_with_enemy(bullet: &PlayerBulletStruct, world: &World) -> bool {
    let mut enemies = world.entities.iter().filter_map(|e|
        match *e {
            Enemy(ref enemy) => Some(enemy),
            _ => None
        }
    );

    for enemy in enemies {
        let empty_rect = FloatRect::new(0.0,0.0,0.0,0.0);

        if FloatRect::intersects(
            &bullet.rect().get_global_bounds(),
            &enemy.renderer.as_ref().map_or(empty_rect, |r| r.bounds()),
            &empty_rect){
            
            return true;
        }
    }

    return false;
}

fn inside_world(bullet: &PlayerBulletStruct, world: &World) -> bool {
    let empty_rect = FloatRect::new(0.0,0.0,0.0,0.0);

    return FloatRect::intersects(
        &bullet.rect().get_global_bounds(),
        &world.bounds,
        &empty_rect);
}

impl PlayerBulletStruct {
    pub fn rect(&self) -> RectangleShape {
        let mut rectangle = match RectangleShape::new() {
            Some(rectangle) => rectangle,
            None() => fail!("Error, cannot create rectangle.")
        };

        let size = Vector2f::new(10., 10.);
        let origin = size * 0.5f32;
        
        rectangle.set_size(&size);
        rectangle.set_origin(&origin);
        rectangle.set_position(&self.position);

        return rectangle;
    }

    pub fn new(position: &Vector2f, direction: &Vector2f, velocity: f32, health: int) -> PlayerBulletStruct {
        PlayerBulletStruct {
            position: position.clone(),
            direction: direction.clone(),
            velocity: velocity,
            health: health
        }
    }
}

impl EntityTrait for PlayerBulletStruct {
    fn update(&self, dt: f32, world: &World, _input: &Input) -> EntityUpdateResult {
        let intersecting_with_enemy = intersecting_with_enemy(self, world);
        let inside_world = inside_world(self, world);

        let new_health = if intersecting_with_enemy || !inside_world {
            self.health - 1
        } else {
            self.health
        };

        let new_entities = if new_health > 0 {
            ~[PlayerBullet(~PlayerBulletStruct::new(
                &(self.position + self.direction * self.velocity * dt),
                &self.direction, 
                self.velocity,
                new_health
            ))]
        } else {
            ~[]
        };

        return EntityUpdateResult { new_entities: new_entities, events: ~[] };
    }

    fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.rect());
    }

    fn position(&self) -> Vector2f
    {
        self.position
    }
    
    fn clone(&self) -> Entity {
        return PlayerBullet(~PlayerBulletStruct {
            position: self.position.clone(),
            direction: self.direction.clone(),
            velocity: self.velocity,
            health: self.health
        });
    }
}