extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::env::Args;
use glutin_window::GlutinWindow as Window;
use graphics::types::Rectangle;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;

use graphics::*;
use piston::{Button, ButtonEvent, PressEvent, ReleaseEvent};

use piston::Button::Keyboard;
use piston::ButtonState::Press;
use piston::Key::{W,A,S,D,P,O,I,Q,E};

mod pre_renders;



fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("raycaster-testing", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    //events????
    let mut events = Events::new(EventSettings::new());

    let mut pressing= [9,9];
    let mut pos = [0.0,0.0];
    let mut rot = 0.0;
    let mut mode = "point";

    //convert map to vector
    let real_map = [
        [[10,10],[10,50]],
        [[10,50],[50,50]],
        [[50,50],[50,10]],
        [[50,10],[10,10]],
    ];

    let mut map:Vec<[[i32;2];2]> = Vec::new();

    for i in real_map {
        map.push(i);
    }

    //event loop
    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.press_args(){
            if      args == Keyboard(W) {pressing = [0,1]}      //frwd
            else if args == Keyboard(S) {pressing = [0,0]}      //back
            else if args == Keyboard(A) {pressing = [1,1]}      //left
            else if args == Keyboard(D) {pressing = [1,0]}      //right
            else if args == Keyboard(Q) {pressing = [5,0]}
            else if args == Keyboard(E) {pressing = [5,1]}
            else if args == Keyboard(P) {mode = "point"}
            else if args == Keyboard(O) {mode = "object"}
            else if args == Keyboard(I) {mode = "top-down"}
        }

        if let Some(args) = e.release_args(){

            let key_list = [
                Keyboard(W),
                Keyboard(A),
                Keyboard(S),
                Keyboard(D),
                Keyboard(Q),
                Keyboard(E),
            ];

            for key in key_list {
                if args == key {pressing = [9,9]}

            }

            let is_key = args == Keyboard(W) || args == Keyboard(S) || args == Keyboard(A) || args == Keyboard(D);
            if is_key == true { pressing = [9,9]; }
        }

        if pressing != [9,9] { //yes ik theres better ways to do this but whatever
            let speed = 0.25;
            let rot_speed = 15.0;

            if      pressing == [0,1] {pos[0] += speed}  //movement
            else if pressing == [0,0] {pos[0] -= speed}
            else if pressing == [1,1] {pos[1] += speed}
            else if pressing == [1,0] {pos[1] -= speed}
            else if pressing == [5,0] {rot -= rot_speed} //rotation
            else if pressing == [5,1] {rot += rot_speed}
        }

        //render screen on evrey tick??? event??? idk
        if let Some(args) = e.render_args() {
            app.render(&args, pos, rot, &mode, &map);
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs, pos:[f64; 2], rot:f64, mode:&str, map:&Vec<[[i32;2];2]>) {

        self.gl.draw(args.viewport(), |ctx, gl| {

            //clear the screen
            clear([0.0, 0.0, 0.0, 1.0], gl);

            //define some colors
            let red = [1.0, 0.0, 0.0, 1.0];
            let green = [0.0, 1.0, 0.0, 1.0];
            let blue= [0.0, 0.0, 1.0, 1.0];

            //render with selected mode
            if mode == "point" {
                let points = pre_renders::prerender_points(args.window_size[1] / 2.0, pos, rot, &map);

                //render shapes from get_shapes
                for point in points {
                    let s:Rectangle = math::margin_rectangle(point, 5.0);
                    rectangle(red, s, ctx.transform, gl);
                }
                //cut off for angles (goes up to 360)
                //rectangle(red, [360.0,360.0,15.0,1000.0], ctx.transform, gl);
            }

            else if mode == "object" {
                let poly_data = pre_renders::prerender_objects(args.window_size[1] / 2.0, pos, rot, &map);

                for poly in poly_data {
                    polygon(red, &poly, ctx.transform, gl);
                }
            }

            else if mode == "top-down" {
                let spot_size = 5.0;

                //draw camera pos
                rectangle(green, [pos[0], pos[1], spot_size, spot_size], ctx.transform, gl);

                //draw points in non-camera view
                for l in map {
                    rectangle(red,   [l[0][0] as f64, l[0][1] as f64, spot_size, spot_size], ctx.transform, gl);
                    rectangle(green, [l[1][0] as f64, l[1][1] as f64, spot_size, spot_size], ctx.transform, gl);
                }
            }
        });
    }
}