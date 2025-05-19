/*
 *
 * File Name: 		planets.rs
 * Brief:			Provides the structs for the main rust file
 * 
 * Status:			Functional
 * 
*/

use crate::config::Constants;

use kiss3d::window::Window;
use kiss3d::scene::SceneNode;
use kiss3d::nalgebra::{Translation3, Point3, Vector3};

// The planet struct, though of course this includes all celestial bodies in space
pub struct Planet {

	// Positional members
	pub position: Vector3<f32>,
	pub velocity: Vector3<f32>,
	acceleration: Vector3<f32>,

	// Characteristic members
	pub mass: f32,
	pub radius: f32,
	color: Vector3<f32>, // TODO Switch this to be a texture

	// Rendering Members
	object: SceneNode,

	// The locations for the path
	pub orbit_path: Vec<Vector3<f32>>,
	path_color: Vector3<f32>,

}



impl Planet {

	// A function to create a new planet
	pub fn new(window: &mut Window, mass: f32, radius: f32) -> Self {
		Self {
			position: Vector3::<f32>::new(0.0, 0.0, 0.0),
			velocity: Vector3::<f32>::new(0.0, 0.0, 0.0),
			acceleration: Vector3::<f32>::new(0.0, 0.0, 0.0),
			mass: mass,
			radius: radius,
			color: Vector3::<f32>::new(0.0, 0.0, 0.0),
			object: window.add_sphere(radius / Constants::VISUAL_SCALE_FACTOR * 5.0), // Times 5 to make it easier to see
			orbit_path: Vec::<Vector3<f32>>::new(),
			path_color: Vector3::<f32>::new(0.5, 0.5, 1.0), // Defaults to light blue
		}
	}

	// A function to initialize the planet on the window
	pub fn init(&mut self, window: &mut Window) -> &mut Self {
		self.object = window.add_sphere(self.radius);
		self.object.set_color(self.color.x, self.color.y, self.color.z);

		self
	}

	// A function to draw the planet
	pub fn draw(&mut self) -> &mut Self {
		
		// Drawing the planet itself
		let new_pos: Translation3<f32> = Translation3::<f32>::new(self.position.x, self.position.y, self.position.z);
		self.object.set_local_translation(new_pos);

		self
	}

	// Draws the path that the planet has taken thus far
	pub fn draw_path(&self, window: &mut Window) {
		// Drawing the orbital path of the planet
		for i in 0..(self.orbit_path.len() - 1) {
			let p1 = Point3::new(self.orbit_path[i].x, self.orbit_path[i].y, self.orbit_path[i].z);
			let p2 = Point3::new(self.orbit_path[i + 1].x, self.orbit_path[i + 1].y, self.orbit_path[i + 1].z);
			window.draw_line(&p1, &p2, &(Point3::new(self.path_color.x, self.path_color.y, self.path_color.z))); 
		}
	}

	// A function to update the position of the planet
	pub fn update(&mut self) -> &mut Self{

		// Changing the velocity depending on the acceleration
		self.velocity += self.acceleration;

		// Moving it
		self.position += self.velocity * Constants::DELTA_TIME;

		// Clearing the acceleration for the next force application
		self.acceleration = Vector3::<f32>::new(0.0, 0.0, 0.0);

		// Adding to the path
		self.orbit_path.push(self.position);

		// Limiting the length so that it doesn't get too intensive
		if self.orbit_path.len() > Constants::MAX_PATH_LENGTH {
			self.orbit_path.remove(0); 
		}

		self
	}

	// Applies a force to this planet in a certain direction
	pub fn force(&mut self, direction: Vector3<f32>, magnitude: f32) -> &mut Self {
		let dir = direction.normalize();
		self.acceleration += (dir * magnitude) / self.mass;
		self
	}

	// Moves the planet to the specified coordinates without changing the velocity
	pub fn move_to(&mut self, new_pos: Vector3<f32>) -> &mut Self {
		self.position = new_pos;
		let translation = Translation3::new(new_pos.x, new_pos.y, new_pos.z);
		self.object.set_local_translation(translation);
		self
	}

	// Adds in a specific velocity to the planet
	pub fn add_velocity(&mut self, added_vel: Vector3<f32>) -> &mut Self {
		self.velocity += added_vel;
		self
	}

	// Changes the color from the default white to be another color 
	pub fn set_color(&mut self, color: Vector3<f32>) -> &mut Self {
		self.color = color;
		self.object.set_color(color.x, color.y, color.z);
		self
	}

	// Changes the color of the path from a light blue to a different color
	pub fn set_path_color(&mut self, color: Vector3<f32>) -> &mut Self {
		self.path_color = color;
		self
	}

}





