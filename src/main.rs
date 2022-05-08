extern crate kiss3d;
extern crate nalgebra as na;

use na::{Point3, Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;
//use std::collections::LinkedList;

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    window.set_light(Light::StickToCamera);
    let framerate = 75.0;
    window.set_framerate_limit(Some(framerate as u64));
    //window.set_background_color(0.255, 0.486, 0.620);

    let red = Point3::new(1.0, 0.0, 0.0);
    let green = Point3::new(0.0, 1.0, 0.0);
    let blue = Point3::new(0.0, 0.0, 1.0);

    let origin = Point3::new(0.0, 0.0, 0.0);
    let i10 = Point3::new(10.0, 0.0, 0.0);
    let j10 = Point3::new(0.0, 10.0, 0.0);
    let k10 = Point3::new(0.0, 0.0, 10.0);
    

    let mut cube = window.add_cube(1.0, 1.0, 1.0);
    cube.set_color(1.0, 0.0, 0.0);
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    let mut x = 1.0;
    let mut y = -1.0;
    let mut z = 1.0;

    let a = 10.0;
    let b = 28.0;
    let c = 2.6667;

    let start = Point3::new(x,y,z);
    const HISTORY_LEN: usize = 500; // will keep 500 points
    let mut history: [Option<Point3<f32>>; HISTORY_LEN] = [None; 500];
    let mut index = 0;
    history[index] = Some(start);

    while window.render() {
        cube.prepend_to_local_rotation(&rot);

        // 10*unit vectors
        window.draw_line(&origin, &i10, &red);
        window.draw_line(&origin, &j10, &green);
        window.draw_line(&origin, &k10, &blue);

        index = index+1%HISTORY_LEN;

        let dx = a*(y-x)/framerate;
        let dy = (x*(b-z)-y)/framerate;
        let dz = (x*y-c*z)/framerate;

        x = x + dx;
        y = y + dy;
        z = z + dz;

        //println!("{:?}, {:?}, {:?}", x,y,z);
        let particle = Point3::new(x as f32, y as f32, z as f32);
        history[index] = Some(particle);

        for point in history {
            match point {
                Some(p) => window.draw_point(&point.unwrap(), &red),
                _ => {},
            }
            
        }
        //window.draw_point(&particle, &red);
    }
}