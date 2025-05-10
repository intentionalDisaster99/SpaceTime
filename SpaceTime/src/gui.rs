/*
 *
 * File Name: 		gui.rs
 * Brief:			Handles the creation and upkeep of the gui and display
 * 
 * Progress:		In Progress
 * 
 */

use kiss3d::nalgebra::{Translation3};
use kiss3d::window::Window;
use kiss3d::light::Light;


pub struct GUI {

	window: Window,

}



impl GUI {

	// Starts a new screen with nothing in it
	pub fn new(title: &str) -> Self {

		let mut window = Window::new(title);
		window.set_light(Light::StickToCamera);

		Self { window }

	}

	pub fn getWindow(&mut self) -> &mut Window {
		&mut self.window
	}
	


}




