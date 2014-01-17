extern mod native;
extern mod rsfml;

use rsfml::system::{Clock, Vector2f};
use rsfml::window::{ContextSettings, VideoMode, event, Close, keyboard };
use rsfml::graphics::{RenderWindow, RectangleShape, Color};

mod world;
mod entity;

#[start]
fn start(argc: int, argv: **u8) -> int {
	native::start(argc, argv, main)
}

fn main () -> () {
	let setting = ContextSettings::default();
	let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32), "SFML Example", Close, &setting) {
		Some(window) => window,
		None => fail!("Cannot create a new Render Window.")
	};

	let world = world::World {
		entities: ~world::list::Cons(
			world::entity::Entity { position: Vector2f::new(200., 200.) },
			~world::list::Cons(
				world::entity::Entity { position: Vector2f::new(50., 50.) },
				~world::list::Nil
			)
		)
	};

	let mut frame_timer = Clock::new();

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
		world::draw(&mut window, world.entities);
		window.display()
	}
}