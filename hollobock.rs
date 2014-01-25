extern mod native;
extern mod rsfml;

use rsfml::system::{Clock, Vector2f};
use rsfml::window::{ContextSettings, VideoMode, event, Close };
use rsfml::graphics::{RenderWindow, Color};

use entity::{Entity};
use player::{Player};

pub mod world;
pub mod list;
pub mod entity;
pub mod player;

#[start]
fn start(argc: int, argv: **u8) -> int {
	native::start(argc, argv, main)
}

fn main() {
	let setting = ContextSettings::default();
	let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32), "Hollobock", Close, &setting) {
		Some(window) => window,
		None => fail!("Cannot create a new Render Window.")
	};

    let player = Player {
    	position: Vector2f::new(200., 200.),
    	rotation: 0.,
    	weapon_cooldown: 0.
    };

	let mut frame_timer = Clock::new();
    let mut world = Some(~world::World {
        entities: list::Cons(
            ~player as ~entity::Entity,
            ~list::Nil
        )
    });

	while window.is_open() {
		let dt = frame_timer.get_elapsed_time().as_seconds();
		frame_timer.restart();

		loop {
			match window.poll_event() {
				event::Closed => window.close(),
				event::NoEvent => break,
				_ => {}
			}
		}

		window.clear(&Color::new_RGB(0, 200, 200));

        match world {
            Some(w) => {
                world = Some(~world::World { entities : world::update(dt, &w.entities, list::Nil, &window) } );
                world::draw(&mut window, &w.entities);
            }
            None => ()
        }
		
		window.display()
	}
}