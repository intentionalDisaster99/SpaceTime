/*
 *
 * File Name: 		config.rs
 * Brief:			Provides some global constants for the entirety of the project
 * 
 * Status:			Functional
 * 
 */


pub struct Constants {}


impl Constants {
	pub const DELTA_TIME: f32 = 0.0001;
	pub const G: f32 = 6.67382e-11;
	pub const VISUAL_SCALE_FACTOR: f32 = 10e10;
	pub const MAX_PATH_LENGTH: usize = 1000;
}