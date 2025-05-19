/*
 *
 * File Name: 		main.rs
 * Brief:			Creates a simple visualization that shows planets and the bending of spacetime
 * 
 * Status: 			Working
 * 
*/
mod config;
mod gui;
mod planets;
mod grid;

use gui::*;
use planets::*;
use config::Constants;
use grid::*;

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation3, Point3, Vector3};

// ! Note that there are multiple versions of nalgebra in this project
// ! This will need to be fixed asap

// Todo Change all of the colors to be Point3 so that I don't have to convert things
// Todo incorporate the grid into the GUI struct

fn main() {

	// Let the program exit gracefully
	std::panic::set_hook(Box::new(|info| {
        eprintln!("Application encountered an error: {}", info);
        std::process::exit(1); // Exit gracefully with a non-zero status code
    }));

	let mut gui: GUI = GUI::new("Planets");
	add_in_planets(&mut gui);

	let mut grid: Grid = Grid::new(0.2, 0.005, 1e-33);

	// let window: &mut Window = gui.get_window();
    while gui.window.render() {
        gui.update_all_planets(false);
		let center = gui.get_sun_position();
		grid.draw_grid(&mut gui.window, &gui.planets, (5, 5), center);
    }
}








// Adds in the planets away from the main method to make it look better
fn add_in_planets(gui: &mut GUI) {
	// Adding in the sun
	gui.add_planet(1.989e30, 695700000.0).set_color(Vector3::<f32>::new(253.0/255.0, 194.0/255.0, 29.0/255.0));

	// Adding in earth 
	gui.add_planet(5.972e24, 6378100.0*30.0).set_color(Vector3::<f32>::new(79.0,76.0,176.0)/255.0).move_to(Vector3::<f32>::new(1.0, 0.0, 0.0) * 1.496e11/Constants::VISUAL_SCALE_FACTOR/5.0).add_velocity(Vector3::<f32>::new(0.0,15.0,0.0));

	// Another earth for testing
	gui.add_planet(5.972e27, 6378100.0*30.0).set_color(Vector3::<f32>::new(79.0,76.0,176.0)/255.0).move_to(Vector3::<f32>::new(1.0, 0.0, 0.0) * 1.496e11/Constants::VISUAL_SCALE_FACTOR/3.50).add_velocity(Vector3::<f32>::new(0.0,15.0,0.0));
	gui.add_planet(5.972e28, 6378100.0*30.0).set_color(Vector3::<f32>::new(79.0,76.0,176.0)/255.0).move_to(Vector3::<f32>::new(1.0, 0.0, 0.0) * 1.496e11/Constants::VISUAL_SCALE_FACTOR/4.50).add_velocity(Vector3::<f32>::new(0.0,15.0,0.0));
	gui.add_planet(5.972e24, 6378100.0*30.0).set_color(Vector3::<f32>::new(79.0,76.0,176.0)/255.0).move_to(Vector3::<f32>::new(1.0, 0.0, 0.0) * 1.496e11/Constants::VISUAL_SCALE_FACTOR/7.50).add_velocity(Vector3::<f32>::new(0.0,15.0,0.0));
	gui.add_planet(5.972e24, 6378100.0*30.0).set_color(Vector3::<f32>::new(79.0,76.0,176.0)/255.0).move_to(Vector3::<f32>::new(1.0, 0.0, 0.0) * 1.496e11/Constants::VISUAL_SCALE_FACTOR/6.50).add_velocity(Vector3::<f32>::new(0.0,15.0,0.0));
	gui.add_planet(1.989e29, 6378100.0*30.0).set_color(Vector3::<f32>::new(79.0,76.0,176.0)/255.0).move_to(Vector3::<f32>::new(1.0, 0.0, 0.0) * 1.496e11/Constants::VISUAL_SCALE_FACTOR/2.50).add_velocity(Vector3::<f32>::new(0.0,15.0,0.0));
}
