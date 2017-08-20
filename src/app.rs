use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use graphics::*;
use ncollide::world::{CollisionWorld2, GeometricQueryType, CollisionGroups};
use ncollide::shape::{ShapeHandle2, Cuboid};
use na;
use na::Vector2;
use na::geometry::Isometry2;

use input_state::InputState;
use car_steering::*;

use track::*;

const C_BACKGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct App {
    gl: GlGraphics,
    car: CarSteering,
    world: CollisionWorld2<f64, ()>,
    car_collides: bool,
}

impl App {
    pub fn new (gl: GlGraphics) -> Self {
        let mut world = CollisionWorld2::new(0.02, true);

        let groups = CollisionGroups::new();
        let query = GeometricQueryType::Contacts(0.0);

        let shape = get_border_shape();
        let shape_handle = ShapeHandle2::new(shape);

        let shape_car = ShapeHandle2::new(
            Cuboid::new(Vector2::new(
                (DEF_STEERING.cg_to_front + DEF_STEERING.cg_to_rear) / 2.0,
                DEF_STEERING.half_width
            ))
        );

        let x = -40.0;
        let y = 40.0;
        let r = -3.14159 / 2.0;

        world.deferred_add(0, na::one(), shape_handle, groups, query, ());
        world.deferred_add(1, na::one(), shape_car, groups, query, ());

        Self {
            gl,
            car: CarSteering::new(DEF_STEERING, [x, y], r),
            world,
            car_collides: false
        }
    }
    
    pub fn render(&mut self, args: &RenderArgs) {

        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let car = &self.car;
        let car_collides = self.car_collides;
        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(C_BACKGROUND, gl);

            let transform = c.transform
                .trans(x, y).zoom(x.min(y) / 50.0);

            draw_border(transform, gl);
            car.draw(transform, gl, car_collides);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs, input: &InputState) {

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
        let car_pos = self.car.get_position();
        let (x, y) = (car_pos[0], car_pos[1]);

        self.world.deferred_set_position(1, iso_xyr(x, y, self.car.get_rotation()));
        self.world.update();

        self.car_collides = self.world.contacts().next().is_some();
    }
}

fn iso_xyr(x: f64, y: f64, r: f64) -> Isometry2<f64> {
    Isometry2::new(Vector2::new(x, y), r)
}