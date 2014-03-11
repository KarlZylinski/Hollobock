use rsfml::graphics::RenderWindow;
use input::Input;
use event::Event;

pub mod game_layer;
pub mod gui_layer;

pub enum Layer {
    GameLayer(~game_layer::GameLayerStruct),
    GuiLayer(~gui_layer::GuiLayerStruct)
}

pub trait LayerTrait {
    fn update(&mut self, dt: f32, input: &Input) -> ~[Event];
    fn draw(&self, window: &mut RenderWindow);
    fn clone(&self) -> Layer;
}

impl Layer {
    fn as_layer_trait<'r>(&'r self) -> &'r LayerTrait: {
        match self {
            &GameLayer(~ref l) => l as &LayerTrait:,
            &GuiLayer(~ref l) => l as &LayerTrait:
        }
    }

    fn as_mut_layer_trait<'r>(&'r mut self) -> &'r mut LayerTrait: {
        match self {
            &GameLayer(~ref mut l) => l as &mut LayerTrait:,
            &GuiLayer(~ref mut l) => l as &mut LayerTrait:
        }
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> ~[Event] {
    	self.as_mut_layer_trait().update(dt, input)
    }

    pub fn draw(&self, window: &mut RenderWindow) {
    	self.as_layer_trait().draw(window);
    }

    pub fn clone(&self) -> Layer {
        self.as_layer_trait().clone()
    }
}

impl Clone for Layer {
    fn clone(&self) -> Layer {
        return self.clone();
    }
}
