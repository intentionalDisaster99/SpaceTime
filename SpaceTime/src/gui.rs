/*
 *
 * File Name: 		gui.rs
 * Brief:			Handles the creation and upkeep of the gui and display
 * 
 * Status:			Working, being updated
 * 
 */

use crate::planets::*;
use crate::config::*;


use nalgebra::{Vector3, distance, Point3};
use kiss3d::nalgebra::{Translation3};
use kiss3d::window::Window;
use kiss3d::light::Light;

piotafdsa

pub struct GUI {

	pub window: Window,

	// All of the planets that we are going to be rendering
	pub planets: Vec<Planet>,

}



impl GUI {

	// Starts a new screen with nothing in it
	pub fn new(title: &str) -> Self {

		let mut window = Window::new(title);
		window.set_light(Light::StickToCamera);

		Self { window: window, planets: Vec::<Planet>::new() }

	}

	pub fn get_window(&mut self) -> &mut Window {
		&mut self.window
	}

	// A function to take in a planet and add it to the gui
	pub fn add_planet(&mut self, mass: f32, radius: f32) -> &mut Planet {
		self.planets.push(Planet::new(&mut self.window, mass, radius));
		let index: usize = self.planets.len()-1 as usize;
		&mut self.planets[index]
	}

	// Updates all of the planets including the gravitational force between each
	pub fn update_all_planets(&mut self) -> &mut Self {

		// * There are some easy improvements to this algorithm
		// * I just don't have as much experience with Rust ownership


		// Adds in gravitational force
		for i in 0..self.planets.len() {

			// Skipping the sun so that it stays in the middle
			if i == 0 { continue; }

			for j in 0..self.planets.len() {

				// Not adding force towards itself
				// if i == j { continue; }

				// Finding the magnitude
				let point1 = Point3::from(self.planets[i].position * Constants::VISUAL_SCALE_FACTOR);				
				let point2 = Point3::from(self.planets[j].position * Constants::VISUAL_SCALE_FACTOR);
				let dist = distance(&point1, &point2);
				if dist < 1e-5 {
					continue; // Skip force calculations when they are very close to each other
				}
				let mag = Constants::G * self.planets[i].mass / (dist*dist) * self.planets[j].mass;

				// Finding the direction
				let direction_vector = self.planets[j].position - self.planets[i].position;
				if direction_vector.magnitude_squared() < 1e-10 {
				    continue; // Skip force calculation for overlapping planets
				}
				let direction = direction_vector.normalize();

				// Adding in the force
				self.planets[i].force(direction, mag); 

			}

		}

		// Updates each planet
		for planet in self.planets.iter_mut() {
			planet.update().draw().draw_path(&mut self.window);
		}

		self
	}


}




