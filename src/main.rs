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
use piston::Key::{W,A,S,D,P,O,I};
mod triangulation;




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
    let mut rot = 0;
    let mut mode = "point";

    //event loop
    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.press_args(){
            if      args == Keyboard(W) {pressing = [0,1]}      //frwd
            else if args == Keyboard(S) {pressing = [0,0]}      //back
            else if args == Keyboard(A) {pressing = [1,1]}      //left
            else if args == Keyboard(D) {pressing = [1,0]}      //right
            else if args == Keyboard(P) {mode = "point"}
            else if args == Keyboard(O) {mode = "object"}
        }

        if let Some(args) = e.release_args(){
            let is_key = args == Keyboard(W) || args == Keyboard(S) || args == Keyboard(A) || args == Keyboard(D);
            if is_key == true { pressing = [9,9]; }
        }

        if pressing != [9,9] { //yes ik theres better ways to do this but whatever
            let speed = 0.25;

            if      pressing == [0,1] {pos[0] += speed}
            else if pressing == [0,0] {pos[0] -= speed}
            else if pressing == [1,1] {pos[1] += speed}
            else if pressing == [1,0] {pos[1] -= speed}
        }

        if let Some(args) = e.render_args() {
            app.render(&args, pos, rot, &mode);
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs, pos:[f64; 2], rot:i32, mode:&str) {

        self.gl.draw(args.viewport(), |ctx, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let red = [1.0, 0.0, 0.0, 1.0];
            if mode == "point" {
                let shapes = prerender_points(args.window_size[1] / 2.0, pos, rot);

                //render shapes from get_shapes
                for shape in shapes {
                    let s:Rectangle = math::margin_rectangle(shape, 5.0);
                    rectangle(red, s, ctx.transform, gl);
                }
            }

            else if mode == "object" {
                let poly_data = prerender_lines(args.window_size[1] / 2.0, pos, rot);

                for poly in poly_data {
                    polygon(red, &poly, ctx.transform, gl);
                }


            }

            //x, y, width, height
            //let rect = math::margin_rectangle([100.0, 10.0, 15.0, 50.0], 5.0);

        });

    }
}

fn prerender_lines(y:f64, pos:[f64; 2], rot:i32) -> Vec<[[f64;2];3]> {

    let baseline = y;
    let render_cutoff = 100.0;


    let line_list = [
        [[5,5],[20,20]],
        [[20,20],[50,50]]
    ];

    let mut polys:Vec<[[f64;2];3]> = Vec::new();

    for line in line_list {

        //parse line
        let parsed_line = triangulation::convert_line(&pos, &render_cutoff, &line);

        let p1 = parsed_line[0];
        let p2 = parsed_line[1];

        let a =     [p1[0], p1[1] + baseline];
        let b =     [p2[0], p1[1] + baseline];

        let c =     [p1[0], baseline - p1[1]];
        let d =     [p2[0], baseline - p2[1]];

        let polydata1 = [a,b,d];
        let polydata2 = [a,c,d];

        polys.push(polydata1);
        polys.push(polydata2);
    }
    return polys;
}

fn prerender_points(y:f64, pos:[f64; 2], rot:i32) -> Vec<[f64;4]> {

    println!("{:?}", pos);

    //rendering stuff
    let mut shapes:Vec<[f64;4]> = Vec::new();
    let line_width = 15.0;
    let baseline = y;

    //raycaster stuff
    let render_cutoff = 100.0;

    let line_list = [
        [[5,5],[20,20]],
        [[20,20],[50,50]]
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
