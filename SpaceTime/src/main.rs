/*
 *
 * File Name: 		main.rs
 * Brief:			Creates a simple visualization that shows planets and the bending of spacetime
 * 
 * Progress:		In Progress
 * 
*/

use kiss3d::window::Window;
use kiss3d::light::Light;
use nalgebra::{Point3, Vector3};
use kiss3d::nalgebra::{Translation3};

fn main() {
    let mut window = Window::new("Planets");

    // Add a cube of size 1.0
    let mut cube = window.add_sphere(1.0);
    cube.set_color(0.2, 0.7, 1.0);
    cube.set_local_translation(Translation3::new(0.0, 0.0, 0.0));

    window.set_light(Light::StickToCamera);

    while window.render() {
        // You can rotate, animate, etc., here if you like
    }
}


