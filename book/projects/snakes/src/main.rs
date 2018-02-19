extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    x: f64,
    y: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 0.75];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let x = self.x - 50.0;
        let y = self.y;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear screen
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.x = (self.x + 1.0) % 300.0;
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [200, 200]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Things went terribly wrong");

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        x: 100.0,
        y: 100.0
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }


    println!("Hello piston!");
}