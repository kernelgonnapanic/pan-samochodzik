use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use graphics::*;

use input_state::InputState;
use car_steering::*;

const C_BACKGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct App {
    gl: GlGraphics,
    car: CarSteering,
}

impl App {
    pub fn new (gl: GlGraphics) -> Self {
        Self {
            gl,
            car: CarSteering::new(DEF_STEERING, [0.0,0.0])
        }
    }
    
    pub fn render(&mut self, args: &RenderArgs) {

        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let car = &self.car;
        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(C_BACKGROUND, gl);

            let transform = c.transform
                .trans(x, y);

            car.draw(transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs, input: &InputState) {
        // Rotate 2 radians per second.

        if input.right {
            self.car.steer_right(args.dt);
        }

        if input.left {
            self.car.steer_left(args.dt);
        }

        if input.forward {
            self.car.accelerate(args.dt);
        }

        if input.back {
            self.car.brake(args.dt);
        }

        if input.space {
            self.car.ebrake(args.dt);
        }

        self.car.update(args.dt);
    }
}
