use graphics::math::{Matrix2d, Vec2d};
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use graphics::*;

use input_state::InputState;

const C_BACKGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const C_RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const C_BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64, // Rotation for the square.
    position: Vec2d
}

fn car(transform: Matrix2d, gl: &mut GlGraphics) {
    rectangle(C_RED, [-25.0, -10.0, 40.0, 20.0], transform, gl);
    polygon(C_BLUE, &[[15.0, -10.0], [15.0, 10.0], [25.0, 6.0], [25.0, -6.0]], transform, gl)
}

impl App {
    pub fn new (gl: GlGraphics) -> Self {
        Self {
            gl,
            rotation: 0.0,
            position: Default::default(),
        }
    }
    pub fn render(&mut self, args: &RenderArgs) {

        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let rot = self.rotation;
        let pos = self.position;
        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(C_BACKGROUND, gl);

            let transform = c.transform
                .trans(x, y)
                .trans_vec(pos)
                .rot_rad(rot);

            // Draw a box rotating around the middle of the screen.
            car(transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs, input: &InputState) {
        // Rotate 2 radians per second.

        if input.forward && input.right || input.back && input.left {
            self.rotation += 2.0 * args.dt;
        }

        if input.forward && input.left || input.back && input.right {
            self.rotation -= 2.0 * args.dt;
        }

        if input.forward {
            self.position[0] += 100.0 * args.dt * self.rotation.cos();
            self.position[1] += 100.0 * args.dt * self.rotation.sin();
        }

        if input.back {
            self.position[0] -= 100.0 * args.dt * self.rotation.cos();
            self.position[1] -= 100.0 * args.dt * self.rotation.sin();
        }
    }
}

trait TransformedVec {
    fn trans_vec(self, Vec2d) -> Self;
}

impl TransformedVec for Matrix2d {
    fn trans_vec (self, vec: Vec2d) -> Self {
        self.trans(vec[0], vec[1])
    }
}