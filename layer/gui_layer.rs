use std::rc::Rc;
use std::cell::RefCell;
use event::{Event, EventHandler, PlayerHealthChanged};

use rsfml::system::Vector2f;
use rsfml::graphics::RenderWindow;

use layer::{Layer, LayerUpdateResult};
use input::Input;
use resource_store::ResourceStore;
use gui::bar::Bar;

pub struct GuiEventHandler {
    gui: Rc<RefCell<GuiLayer>>
}

impl EventHandler for GuiEventHandler {
    fn handle_event(&mut self, event: Event) -> bool {
        return match event {
            PlayerHealthChanged(health) => {
                self.gui.borrow().try_borrow_mut().map(|mut gui| {
                    gui.get().set_health(health);
                });
                true
            }
        }
    }
}

pub struct GuiLayer {
    health_bar: Option<Bar>
}

impl GuiLayer {
    pub fn new(rs: &mut ResourceStore) -> GuiLayer {
        let health_bar = rs.load_texture(~"health_bar.png").map(|t| {
            Bar::new(100., 100., t, &Vector2f::new(10., 10.), &Vector2f::new(100., 1.))
        });

        GuiLayer {
            health_bar: health_bar
        }
    }

    pub fn set_health(&mut self, health: u8) {
        self.health_bar.as_mut().map(|h| { h.set_target(health as f32) });
    }
}

impl Layer for GuiLayer {
    fn update(&self, dt: f32, _input: &Input) -> LayerUpdateResult {
        LayerUpdateResult {
            new_layers: ~[
                ~GuiLayer {
                    health_bar: self.health_bar.as_ref().map(|hb| { hb.update(dt) })
                } as ~Layer:
            ]
        }
    }

    fn draw(&self, window: &mut RenderWindow) {
        self.health_bar.as_ref().map(|hb| { hb.draw(window) });
    }

    fn clone(&self) -> ~Layer: {
        ~GuiLayer {
            health_bar: self.health_bar.as_ref().map(|hb| { hb.clone() })
        } as ~Layer:
    }
}
