/*
 *
 * File Name: 		planets.rs
 * Brief:			Provides the structs for the main rust file
 * 
 * Progress:		In Progress
 * 
*/

use kiss3d::window::Window;
use kiss3d::scene::SceneNode;
use kiss3d::nalgebra::{Translation3};
use nalgebra::Vector3;

// The planet struct, though of course this includes all celestial bodies in space
pub struct Planet {

	// Positional members
	position: Vector3<f32>,
	velocity: Vector3<f32>,

	// Characteristic members
	mass: u64,
	radius: u64,
	color: Vector3<f32>, // TODO Switch this to be a texture

	// Rendering Members
	object: SceneNode,

}



impl Planet {

	// A function to create a new planet
	pub fn new() -> Self {
		todo!();
	}

	// A function to initialize the planet on the window
	pub fn init(&mut self, window: &mut Window) -> &mut Self {
		self.object = window.add_sphere(self.radius as f32);
		self.object.set_color(self.color.x, self.color.y, self.color.z);

		self
	}

	// A function to draw the planet
	pub fn draw(&mut self) -> &mut Self {
		let newPos: Translation3<f32> = Translation3::<f32>::new(self.position.x, self.position.y, self.position.z);
		self.object.set_local_translation(newPos);
		self
	}

	// A function to update the position of the planet
	pub fn update(&mut self) -> &mut Self{
		todo!();
	}



}







// Kiss3d has some of these functions already, so I'll use them
/*

// A simple vector struct
pub struct Vector2<T> {
	x: T,
	y: T,
}

// Simple vector calculations
impl<T> Vector2<T> {

	// Simple constructor (or constructor adjacent)
	fn new(x: T, y: T) -> Self {
		Self { x, y } 
	}

	// Creates a new vector with only zeros
	fn zero() -> Self {
		Self { 0.0, 0 }
	}

	fn multiply(&mut self, other: Vector2) -> &mut Self {
		self.x *= other.x;
    	self.y *= other.y;
		self
	}

	fn multiply(one: Vector2, other: Vector2) -> Self {
		Self {
			one.x * two.x,
			two.y * two.y
		}
	}

	fn add(&mut self, other: Vector2) -> &mut Self {
		self.x += other.x;
    	self.y += other.y;
		self
	}

	fn add(one: Vector2, other: Vector2) -> Self {
		Self {
			one.x + two.x,
			two.y + two.y
		}
	}

	// Changes the length of the vector to be 1
    fn normalize(self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }


}


*/