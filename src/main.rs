extern crate kiss3d;
extern crate nalgebra as na;

use na::{Point3, Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;
//use std::collections::LinkedList;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);
    let framerate = 60.0;
    window.set_framerate_limit(Some(framerate as u64));
    // window.set_background_color(1.0, 1.0, 1.0);
    window.set_background_color(0.1, 0.1, 0.1);

    let red = Point3::new(1.0, 0.0, 0.0);
    let green = Point3::new(0.0, 1.0, 0.0);
    let blue = Point3::new(0.0, 0.0, 1.0);

    let origin = Point3::new(0.0, 0.0, 0.0);
    let i = Point3::new(100.0, 0.0, 0.0);
    let j = Point3::new(0.0, 100.0, 0.0);
    let k = Point3::new(0.0, 0.0, 100.0);

    // starting conditions
    let mut x = 1.0;
    let mut y = 0.0;
    let mut z = 0.0;

    // σ, ρ, and β (sigma, rho, beta)
    let a = 10.0;
    let b = 28.0;
    let c = 2.66667;

    // number of points that will be recorded
    const HISTORY_LEN: usize = 1000;
    let mut history: [Option<Point3<f32>>; HISTORY_LEN] = [None; HISTORY_LEN];
    let mut index = 0;
    let start = Point3::new(x,y,z);
    history[index] = Some(start);

    while window.render() {

        // axes
        window.draw_line(&origin, &i, &red);
        window.draw_line(&origin, &j, &green);
        window.draw_line(&origin, &k, &blue);

        // index marks the front of the "queue"
        index = (index+1)%HISTORY_LEN;

        // lorenz system
        let dx = a*(y-x)/framerate;
        let dy = (x*(b-z)-y)/framerate;
        let dz = (x*y-c*z)/framerate;

        x = x + dx;
        y = y + dy;
        z = z + dz;

        let particle = Point3::new(x as f32, y as f32, z as f32);
        history[index] = Some(particle);

        // connect all the points in history
        for i in index+1..HISTORY_LEN+index {

            let first_index = i%HISTORY_LEN;
            let next_index = (i+1)%HISTORY_LEN;
            
            if !history[next_index].is_none() && !history[first_index].is_none() {

                let progress = (i-index) as f32/(HISTORY_LEN) as f32;
                let linecolor = Point3::new(1.0*progress, 0.0, 1.0*(1.0-progress));
                
                window.draw_line(&history[first_index].unwrap(), &history[next_index].unwrap(), &linecolor);
            }
            
        }
    }
}