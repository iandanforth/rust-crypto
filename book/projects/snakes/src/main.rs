extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use rand::Rng;

fn get_window(opengl: OpenGL, width: u32, height: u32) -> Window {
    let window: Window = WindowSettings::new(
        "Chart!",
        [width, height]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Things went terribly wrong");

    return window;
}

pub struct Graph {
    gl: GlGraphics,
    data: Vec<f64>,
    win_height: u32,
    win_width: u32
}

impl Graph {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 0.75];


        let data = &self.data;
        let base = self.win_height as f64;
        let spacing = 10.0;
        let left_margin = 0.0;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear screen
            clear(WHITE, gl);
            let mut offset = 0.0;
            let w = 10.0;
            for val in data {
                let h = val.clone();
                offset += 1.0;
                let x = left_margin + (offset * (2.0 * w)) + (offset * spacing);
                let y = base - h;
                let rect = rectangle::centered([x, y, w, h]);
                rectangle(RED, rect, c.transform, gl);
            }
        });
    }
}



fn main() {
    let opengl = OpenGL::V3_2;

    let win_height: u32 = 400;
    let win_width: u32 = 800;

    let mut window = get_window(opengl, win_width, win_height);

    let mut rand_generator = rand::thread_rng(); // If this isn't mutable it throws an error. No idea why.
    let mut data: Vec<f64> = Vec::new();
    for _ in 0..25 {
        let num = rand_generator.gen_range(1, 101) as f64;
        data.push(num);
    }

    // let data = vec![20.0, 30.0, 40.0, 50.0, 60.0, 0.0, 1.0];

    let mut graph = Graph {
        gl: GlGraphics::new(opengl),
        data: data,
        win_height: win_height,
        win_width: win_width
    };

    let mut events = Events::new(EventSettings::new());

    // Display the window and spin
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            graph.render(&r);
        }
    }
}