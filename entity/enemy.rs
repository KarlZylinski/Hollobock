use std::f32;

use rsfml::system::Vector2f;
use rsfml::graphics::{RenderWindow, FloatRect};

use input::Input;
use math;
use std::iter::Iterator;
use entity::{Entity, Player, EntityUpdateResult};
use entity::world::World;
use entity::PlayerBullet;
use entity::Enemy;
use entity::EntityTrait;
use entity::renderer::Renderer;

pub struct EnemyStruct {
    position: Vector2f,
    rotation: f32,
    renderer: Option<~Renderer:>,
    health: int
}

fn intersecting_with_bullet(enemy: &EnemyStruct, world: &World) -> bool {
    let mut bullets = world.entities.iter().filter_map(|e|
        match *e {
            PlayerBullet(ref pb) => Some(pb),
            _ => None
        }
    );

    for bullet in bullets {
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

impl EnemyStruct {
    pub fn new(position: &Vector2f, rotation: f32, renderer: Option<~Renderer:>, health: int) -> EnemyStruct {
        EnemyStruct {
            position: position.clone(),
            rotation: rotation,
            renderer: renderer,
            health: health
        }
    }
}

impl EntityTrait for EnemyStruct {   
    fn update(&self, dt: f32, world: &World, _input: &Input) -> EntityUpdateResult {
        let player_option = world.entities.iter().find(|e| {
            match *e {
                &Player(_) => true,
                _ => false
            }
        });

        let player = match player_option {
            Some(player) => player,
            None => fail!("No player found in world.")
        };

        let dist_to_player = player.position() - self.position;
        let direction = math::normalize(dist_to_player);

        let speed = if math::length(&dist_to_player) < 50. {
            0.0f32
        } else {
            100.0f32
        };

        let new_position = self.position + direction * speed * dt;
        let new_rotation = f32::atan2(direction.y, direction.x).to_degrees();

        let intersecting_with_bullet = intersecting_with_bullet(self, world);

        let new_health = if intersecting_with_bullet {
            self.health - 1
        } else {
            self.health
        };

        let new_entities = if new_health > 0 {
            ~[Enemy(~EnemyStruct::new(
                &new_position,
                new_rotation,
                self.renderer.as_ref().map_or(None, |r| r.update(&new_position, new_rotation, dt)),
                new_health
            ))]
        } else {
            ~[]
        };

        EntityUpdateResult {
            new_entities: new_entities
        }
    }
    
    fn position(&self) -> Vector2f
    {
        self.position
    }
    
    fn draw(&self, window: &mut RenderWindow) {
        self.renderer.as_ref().map(|r| r.draw(window));
    }

    fn clone(&self) -> Entity {
        Enemy(~EnemyStruct {
            position: self.position.clone(),
            rotation: self.rotation,
            renderer: self.renderer.as_ref().map_or(None, |r| r.clone()),
            health: self.health
        })
    }
}
