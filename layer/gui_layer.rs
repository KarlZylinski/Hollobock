use std::rc::Rc;
use std::cell::RefCell;

use rsfml::system::Vector2f;
use rsfml::graphics::{RenderWindow, Texture};
use rsfml::graphics::rc::Sprite;

use layer::{Layer, LayerUpdateResult};
use input::Input;
use resource_store::ResourceStore;

pub struct GuiLayer {
    health_bar_texture: Option<Rc<RefCell<Texture>>>
}

impl GuiLayer {    
    pub fn new(rs: &mut ResourceStore) -> GuiLayer {
        GuiLayer {
            health_bar_texture: rs.load_texture(~"health_bar.png")
        }
    }
}

impl Layer for GuiLayer {
    fn update(&self, _dt: f32, _input: &Input) -> LayerUpdateResult {
        LayerUpdateResult {
            new_layers: ~[self.clone()]
        }
    }

    fn draw(&self, window: &mut RenderWindow) {
        self.health_bar_texture.clone().map(|t| {
            Sprite::new_with_texture(t).map(|mut s| {
                s.set_position(&Vector2f::new(10.,10.));
                s.set_scale(&Vector2f::new(100., 1.));
                window.draw(&s);
            });
        });
    }

    fn clone(&self) -> ~Layer: {
        ~GuiLayer {
            health_bar_texture: self.health_bar_texture.clone()
        } as ~Layer:
    }
}
