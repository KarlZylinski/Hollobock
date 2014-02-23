use rsfml::system::Vector2f;
use rsfml::graphics::RenderWindow;

use layer::{Layer, LayerUpdateResult};
use input::Input;
use resource_store::ResourceStore;
use gui::bar::Bar;

pub struct GuiLayer {
    health_bar: Option<Bar>
}

impl EventPropagator for GuiLayer {
    fn propagate_event(&mut self, event: &Event) -> bool {
        match event {
            UpdateHealthBar { health } => {
                self.health_bar.set_target(health);
                true
            },
            _ => false
        }
    }
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
}

impl Layer for GuiLayer {
    fn update(&self, dt: f32, _input: &Input) -> LayerUpdateResult {
        LayerUpdateResult {
            new_layers: ~[
                ~GuiLayer {
                    health_bar: self.health_bar.as_ref().map(|hb| { hb.update(dt) })
                } as ~Layer:
            ],
            events: ~[]
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
