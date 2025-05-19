/*
 *
 * File Name: 		grid.rs
 * Brief:			Provides the grid that shows the actual spacetime bending with mass
 * 
 * Status:			Functional
 * 
 */

use crate::planets::*;

use kiss3d::nalgebra::{Point3, distance};
use kiss3d::window::Window;


pub struct Grid {
	spacing: f32,
	color: Point3<f32>,
	step_size: f32,
	scaling_factor: f32,
}

impl Grid {

	// Simple constructor
	pub fn new(spacing: f32, step_size: f32, scaling_factor: f32) -> Self {
		Self {
			spacing,
			step_size,
			color: Point3::new(1.0, 1.0, 1.0),
			scaling_factor,
		}
	}

	// A function to calculate the z value at a certain location
	pub fn get_z_value(planets: &Vec<Planet>, position: Point3<f32>, scaling_factor: f32) -> f32 {

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

			// Making sure there isn't a divide by zero error
			if dist < 1e-10 {
				continue
			}

			output += planet.mass / dist.powf(1.5); 
		}
		
		// Negative so that it goes down
		-output * scaling_factor

	}

	// Draws the actual grid itself
	pub fn draw_grid(&self, window: &mut Window, planets: &Vec<Planet>, number_of_gridlines: (usize, usize), center: Point3<f32>) {

		// Drawing the horizontal lines
		// Note that this isn't perfect, it rounds a few things, but I don't think that will be too huge a deal
		let mut i: f32 = 0.0;
		while i <= (number_of_gridlines.1 as f32) * self.spacing {

			// Shifting the y to match the center
			let upper_y = i as f32 + center.y;
			let lower_y = -i as f32 + center.y;

			// Saving the points we visit
			let mut last_upper_point: Point3<f32> = Point3::new(-self.spacing*(number_of_gridlines.0 as f32) + center.x, upper_y, Self::get_z_value(planets, Point3::new(-self.spacing*(number_of_gridlines.0 as f32) + center.x,  upper_y, 0.0), self.scaling_factor));
			let mut last_lower_point: Point3<f32> = Point3::new(-self.spacing*(number_of_gridlines.0 as f32) + center.x, lower_y, Self::get_z_value(planets, Point3::new(-self.spacing*(number_of_gridlines.0 as f32) + center.x,  lower_y, 0.0), self.scaling_factor));

			// Iterating for the x values
			let mut j: f32 = -self.spacing*(number_of_gridlines.0 as f32);
			while j <= (number_of_gridlines.0 as f32) * self.spacing {

				// Shifting the x to match the center
				let x = j + center.x;

				// The top

				// Figuring out where the next point is
				let mut here: Point3<f32> = Point3::new(x,upper_y,Self::get_z_value(planets, Point3::new(x,upper_y,0.0), self.scaling_factor));
  
  				// Drawing it
				window.draw_line(&last_upper_point, &here, &self.color);

				// Saving this point
				last_upper_point = here;

				// Skipping the bottom one because we don't need to draw twice if they are in the same spot
				if lower_y == upper_y {
					j += self.step_size;
					continue;
				}

				// The bottom

				// Figuring out where the next point is
				here = Point3::new(x,lower_y,Self::get_z_value(planets, Point3::new(x,lower_y,0.0), self.scaling_factor));

				// Drawing it
				window.draw_line(&last_lower_point, &here, &self.color);

				// Saving this point
				last_lower_point = here;

				// Incrementing
				j += self.step_size;

			}

			// Incrementing
			i += self.spacing;

		}

		// Now for the lines parallel to the y 
		i = 0.0;
		while i <= (number_of_gridlines.0 as f32) * self.spacing {

			// Shifting the y to match the center
			let upper_x = i as f32 + center.x;
			let lower_x = -i as f32 + center.x;

			// Saving the points we visit
			let mut last_upper_point: Point3<f32> = Point3::new(upper_x, -self.spacing*(number_of_gridlines.1 as f32)+center.y, Self::get_z_value(planets, Point3::new(upper_x, -self.spacing*(number_of_gridlines.1 as f32)+center.y, 0.0), self.scaling_factor));
			let mut last_lower_point:Point3<f32> = Point3::new(lower_x, -self.spacing*(number_of_gridlines.1 as f32)+center.y, Self::get_z_value(planets, Point3::new(lower_x, -self.spacing*(number_of_gridlines.1 as f32)+center.y, 0.0), self.scaling_factor)); 

			// Iterating for the y values
			let mut j: f32 = -self.spacing*(number_of_gridlines.1 as f32);
			while j <= (number_of_gridlines.1 as f32) * self.spacing {

				// Shifting the y to match the center
				let y = j + center.y;

				// The top

				// Figuring out where the next point is
				let mut here: Point3<f32> = Point3::new(upper_x, y,Self::get_z_value(planets, Point3::new(upper_x, y,0.0), self.scaling_factor));

				// Drawing it
				window.draw_line(&last_upper_point, &here, &self.color);

				// Saving this point
				last_upper_point = here;

				// Skipping the bottom one because we don't need to draw twice if they are in the same spot
				if lower_x == upper_x {
					j += self.step_size;
					continue;
				}

				// The bottom

				// Figuring out where the next point is
				here = Point3::new(lower_x,y,Self::get_z_value(planets, Point3::new(lower_x,y,0.0), self.scaling_factor));

				// Drawing it
				window.draw_line(&last_lower_point, &here, &self.color);

				// Saving this point
				last_lower_point = here;

				// Incrementing
				j += self.step_size;

			}

			// Incrementing
			i += self.spacing;

		}

	}


}








