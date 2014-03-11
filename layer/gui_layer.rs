use std::rc::Rc;
use std::cell::RefCell;
use event::{Event, EventHandler, PlayerHealthChanged};

use rsfml::system::Vector2f;
use rsfml::graphics::RenderWindow;

use layer::{Layer, LayerTrait, GuiLayer};
use input::Input;
use resource_store::ResourceStore;
use gui::bar::Bar;

pub struct GuiEventHandler {
    gui: Rc<RefCell<Layer>>
}

impl EventHandler for GuiEventHandler {
    fn handle(&mut self, event: Event) -> bool {
        return match event {
            PlayerHealthChanged(health) => {
                self.gui.borrow().try_borrow_mut().map(|mut gui| {
                    match gui.get() {
                        &GuiLayer(~ ref mut gui_struct) => gui_struct.set_health(health),
                        _ => {}
                    };
                });
                true
            }
        }
    }
}

pub struct GuiLayerStruct {
    health_bar: Option<Bar>
}

impl GuiLayerStruct {
    pub fn new(rs: &mut ResourceStore) -> GuiLayerStruct {
        let health_bar = rs.load_texture(~"health_bar.png").map(|t| {
            Bar::new(100., 100., t, &Vector2f::new(10., 10.), &Vector2f::new(100., 1.))
        });

        GuiLayerStruct {
            health_bar: health_bar
        }
    }

    pub fn set_health(&mut self, health: u8) {
        self.health_bar.as_mut().map(|h| { h.set_target(health as f32) });
    }
}

impl LayerTrait for GuiLayerStruct {
    fn update(&mut self, dt: f32, _input: &Input) -> ~[Event] {
        self.health_bar = self.health_bar.as_ref().map(|hb| { hb.update(dt) });

        ~[]
    }

    fn draw(&self, window: &mut RenderWindow) {
        self.health_bar.as_ref().map(|hb| { hb.draw(window) });
    }

    fn clone(&self) -> Layer {
        GuiLayer(~GuiLayerStruct {
            health_bar: self.health_bar.as_ref().map(|hb| { hb.clone() })
        })
    }
}
