/*
 *
 * File Name: 		planets.rs
 * Brief:			Provides the structs for the main rust file
 * 
 * Progress:		In Progress
 * 
*/

// The planet struct, though of course this includes all celestial bodies in space
pub struct Planet {

	// Positional members
	position: Vector2,
	velocity: Vector2,

	// Characteristic members
	mass: u64,
	radius: u64,
	color: color, // TODO Switch this to be a texture


}


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


