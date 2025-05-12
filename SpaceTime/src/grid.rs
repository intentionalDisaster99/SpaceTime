/*
 *
 * File Name: 		grid.rs
 * Brief:			Provides the grid that shows the actual spacetime bending with mass
 * 
 * Status:			In progress
 * 
 */

use crate::planets::*;

use kiss3d::nalgebra::{Point3, distance};
use kiss3d::window::Window;


pub struct Grid {
	spacing: f32,
	color: Point3<f32>,
	step_size: usize,
}

impl Grid {

	// A function to calculate the z value at a certain location
	pub fn get_z_value(planets: &Vec<Planet>, position: Point3<f32>) -> f32 {

		/*
		 * I couldn't find much about any actual equations to use here, so I am going to 
		 * play with the gravity equation and try to figure it out from there
		 */

		// Adding in the mass proportional to the distance squared
		let mut output: f32 = 0.0;
		
		// Iterating for each planet
		for planet in planets.iter() {
			
			// Finding the distance 
			let planet_pos = Point3::new(planet.position.x, planet.position.y, planet.position.z);
			let dist: f32 = distance(&position, &planet_pos);

			output += planet.mass / (dist * dist); 
		}
		
		// Negative so that it goes down
		-output

	}

	// Draws the actual grid itself
	pub fn draw_grid(&self, window: &mut Window, planets: &Vec<Planet>, max_dimensions: (usize, usize), center: Point3<f32>) {

			
		// We need to save the last values we were at so
		let mut last_upper_point: Point3<f32> = Point3::new(-(max_dimensions.0 as f32), 0.0, Self::get_z_value(planets, Point3::new(max_dimensions.0 as f32, 0.0 as f32, 0.0)));
		let mut last_lower_point: Point3<f32> = Point3::new(-(max_dimensions.0 as f32), -self.spacing, Self::get_z_value(planets, Point3::new(max_dimensions.0 as f32,  -(self.spacing as f32), 0.0)));

		// Drawing the horizontal lines
		// Note that this isn't perfect, it rounds a few things, but I don't think that will be too huge a deal
		for y in (0..max_dimensions.1).step_by(self.spacing as usize) {

			// Iterating for the x values
			for x in (-(max_dimensions.0 as isize)..(max_dimensions.0 as isize)).step_by(self.step_size) {

				// The location of this point
				let temp: Point3<f32> =  Point3::new(x as f32, y as f32, 0.0);
				let b: Point3<f32> = Point3::new(x as f32, y as f32, Self::get_z_value(planets, temp));

				// Drawing the sub line for the upper one
				window.draw_line(&last_upper_point, &b, &self.color);

				// Saving this point
				last_upper_point = b;


				// Skipping if the y is zero because there's no reason to draw it twice
				if y == 0 { continue; }

				// Drawing the sub line for the lower one

			}

		}

	}


}








