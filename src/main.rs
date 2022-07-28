extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::types::Rectangle;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;

use graphics::*;

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

    //event loop?
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            let test_string = "hello world";

            app.render(&args, test_string.to_string());
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs, test: String) {

        self.gl.draw(args.viewport(), |ctx, gl| {

            let red = [1.0, 0.0, 0.0, 1.0];
            let shapes = get_shapes(args.window_size[1] / 2.0, test);

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

fn get_shapes(y:f64, test: String) -> Vec<[f64;4]> {

    //rendering stuff
    let mut shapes:Vec<[f64;4]> = Vec::new();
    let line_width = 15.0;
    let baseline = y;

    //raycaster stuff
    let cam_pos  = [0,0];
    let test_line1 = [[0,3],[5,3]];
    let parsed_line1 = triangulation::convert_line(&cam_pos, &test_line1);

    let coords: [[f64;2];2] = [
        parsed_line1[0],
        parsed_line1[1]
    ];

    for cord in coords {
        shapes.push([cord[0], baseline-((cord[1]*10.0)/2.0), line_width, cord[1]*10.0],)
    }

    return shapes;
}
