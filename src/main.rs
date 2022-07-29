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
use piston::Key::{W,A,S,D};
mod triangulation;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs, pos:[i32; 2], rot:i32, mode:&str) {

        self.gl.draw(args.viewport(), |ctx, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let red = [1.0, 0.0, 0.0, 1.0];
            let shapes = get_shapes(args.window_size[1] / 2.0, pos, rot);

            //render shapes from get_shapes
            for shape in shapes {
                let s:Rectangle = math::margin_rectangle(shape, 5.0);
                rectangle(red, s, ctx.transform, gl);
            }

            //x, y, width, height
            //let rect = math::margin_rectangle([100.0, 10.0, 15.0, 50.0], 5.0);

        });

    }
}


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
    let mut pos = [0,0];
    let mut rot = 0;
    let mut mode = "None";

    //event loop
    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.press_args(){
            if      args == Keyboard(W) {pos[0] += 1; pressing = [0,1]}      //frwd
            else if args == Keyboard(S) {pos[0] -= 1; pressing = [0,0]}      //back
            else if args == Keyboard(A) {pos[1] += 1; pressing = [1,1]}      //left
            else if args == Keyboard(D) {pos[1] -= 1; pressing = [1,0]}      //right
        }

        if let Some(args) = e.release_args(){
            let is_key = args == Keyboard(W) || args == Keyboard(S) || args == Keyboard(A) || args == Keyboard(D);
            if is_key == true { pressing = [9,9]; }
        }

        if pressing != [9,9] { //yes ik theres better ways to do this but whatever
            if      pressing == [0,1] {pos[0] += 1}
            else if pressing == [0,0] {pos[0] -= 1}
            else if pressing == [1,1] {pos[1] += 1}
            else if pressing == [1,0] {pos[1] -= 1}
        }

        if let Some(args) = e.render_args() {
            app.render(&args, pos, rot, &mode);
        }
    }
}



fn get_shapes(y:f64, pos:[i32; 2], rot:i32) -> Vec<[f64;4]> {

    println!("{:?}", pos);

    //rendering stuff
    let mut shapes:Vec<[f64;4]> = Vec::new();
    let line_width = 15.0;
    let baseline = y;

    //raycaster stuff
    let render_cutoff = 100.0;

    let line_list = [
        [[0,3],[5,3]],
        //[[5,5],[4,2]]
    ];

    //render line list
    for line in line_list {

        //parse line
        let parsed_line = triangulation::convert_line(&pos, &render_cutoff, &line);

        //render it
        for cord in parsed_line {
            shapes.push([cord[0], baseline-((cord[1]*10.0)/2.0), line_width, cord[1]*10.0],)
        }
    }

    return shapes;
}
