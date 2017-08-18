use graphics::math::{Matrix2d, Vec2d};
use graphics::math;
use graphics::Transformed;
use opengl_graphics::GlGraphics;

pub const DEF_STEERING: SteeringCfg =
    SteeringCfg {
        gravity: 9.81,
        mass: 1200.0,
        inertia_scale: 1.0,
        half_width: 0.8,
        cg_to_front: 2.0,
        cg_to_rear: 2.0,
        cg_to_front_axle: 1.25,
        cg_to_rear_axle: 1.25,
        cg_height: 0.55,
        wheel_radius: 0.3,
        wheel_width: 0.2,
        tire_grip: 2.0,
        lock_grip: 0.7,
        engine_force: 8000.0,
        brake_force: 12000.0,
        ebrake_force: 4800.0,
        weight_transfer: 0.2,
        max_steer: 0.6,
        corner_stiffness_front: 5.0,
        corner_stiffness_rear: 5.2,
        air_resist: 2.5,
        roll_resist: 8.0
    };

#[derive(Copy, Clone, Default)]
pub struct SteeringCfg {
    pub gravity: f64, // m/s^2
    pub mass: f64, // kg
    pub inertia_scale: f64, // Multiply by mass for inertia
    pub half_width: f64, // Centre to side of chassis (metres)
    pub cg_to_front: f64, // Centre of gravity to front of chassis (metres)
    pub cg_to_rear: f64, // Centre of gravity to rear of chassis
    pub cg_to_front_axle: f64, // Centre gravity to front axle
    pub cg_to_rear_axle: f64, // Centre gravity to rear axle
    pub cg_height: f64, // Centre gravity height
    pub wheel_radius: f64, // Includes tire (also represents height of axle)
    pub wheel_width: f64, // Used for render only
    pub tire_grip: f64, // How much grip tires have
    pub lock_grip: f64, // % of grip available when wheel is locked
    pub engine_force: f64,
    pub brake_force: f64,
    pub ebrake_force: f64,
    pub weight_transfer: f64, // How much weight is transferred during acceleration/braking
    pub max_steer: f64, // Maximum steering angle in radians
    pub corner_stiffness_front: f64,
    pub corner_stiffness_rear: f64,
    pub air_resist: f64, // air resistance (* vel)
    pub roll_resist: f64 // rolling resistance force (* vel)
}

#[derive(Default)]
pub struct CarInputState {
    left: f64,
    right: f64,
    throttle: f64,
    brake: f64,
    ebrake: f64,
}

#[derive(Default)]
struct CarConfigComputed {
    inertia: f64,  // will be = mass
    wheel_base: f64, // set from axle to CG lengths
    axle_weight_ratio_front: f64, // % car weight on the front axle
    axle_weight_ratio_rear: f64 // % car weight on the rear axle
}

impl CarConfigComputed {
    fn from_config(cfg: &SteeringCfg) -> Self {
        let wheel_base = cfg.cg_to_front_axle + cfg.cg_to_rear_axle;
        Self {
            inertia: cfg.mass * cfg.inertia_scale,
            wheel_base,
            axle_weight_ratio_front: cfg.cg_to_rear_axle / wheel_base, // % car weight on the front axle
            axle_weight_ratio_rear: cfg.cg_to_front_axle / wheel_base // % car weight on the rear axle
        }
    }
}

#[derive(Default)]
pub struct CarSteering {
    cfg: SteeringCfg,
    input: CarInputState,
    comp: CarConfigComputed,

    heading: f64, // angle car is pointed at (radians)
    position: Vec2d, // metres in world coords
    velocity: Vec2d, // m/s in world coords
    velocity_c: Vec2d, // m/s in local car coords (x is forward y is sideways)
    accel: Vec2d, // acceleration in world coords
    accel_c: Vec2d, // accleration in local car coords
    abs_vel: f64, // absolute velocity m/s
    yaw_rate: f64, // angular velocity in radians
    steer: f64, // amount of steering input (-1.0..1.0)
    steer_angle: f64 // actual front wheel steer angle (-maxSteer..maxSteer)
}

impl CarSteering {
    pub fn new(cfg: SteeringCfg, position: Vec2d) -> Self {

        Self {
            comp: CarConfigComputed::from_config(&cfg),
            cfg,
            position,
            ..Self::default()
        }
    }

    fn apply_smooth_steer(&self, steer_input: f64, dt: f64) -> f64 {
        if steer_input.abs() > 0.001 {
            (self.steer + steer_input * dt * 2.0).max(-1.0).min(1.0)
        } else if self.steer > 0.0 {
            (self.steer - dt * 1.0).max(0.0)
        } else if self.steer < 0.0 {
            (self.steer + dt * 1.0).min(0.0)
        } else {
            0.0
        }
    }

    fn apply_safe_steer(&self, steer_input: f64) -> f64 {
        let avel = self.abs_vel.min(250.0); // m/s
        steer_input * (1.0 - (avel / 250.0))
    }

    pub fn update(&mut self, dt: f64) {
        let cfg = &self.cfg;

        let steer_input = self.input.right - self.input.left;
        self.steer = self.apply_smooth_steer(steer_input, dt);
        self.steer = self.apply_safe_steer(self.steer);
        self.steer_angle = cfg.max_steer * self.steer;

        // do physics

        // Pre-calc heading vector
        let sn = self.heading.sin();
        let cs = self.heading.cos();

        // Get velocity in local car coordinates
        self.velocity_c[0] = cs * self.velocity[0] + sn * self.velocity[1];
        self.velocity_c[1] = cs * self.velocity[1] - sn * self.velocity[0];
    
        // Weight on axles based on centre of gravity and weight shift due to forward/reverse acceleration
        let axle_weight_front = cfg.mass * (self.comp.axle_weight_ratio_front * cfg.gravity - cfg.weight_transfer * self.accel_c[0] * cfg.cg_height / self.comp.wheel_base);
        let axle_weight_rear = cfg.mass * (self.comp.axle_weight_ratio_rear * cfg.gravity + cfg.weight_transfer * self.accel_c[0] * cfg.cg_height / self.comp.wheel_base);

        // Resulting velocity of the wheels as result of the yaw rate of the car body.
        // v = yawrate * r where r is distance from axle to CG and yawRate (angular velocity) in rad/s.
        let yaw_speed_front = cfg.cg_to_front_axle * self.yaw_rate;
        let yaw_speed_rear = -cfg.cg_to_rear_axle * self.yaw_rate;

        // Calculate slip angles for front and rear wheels (a.k.a. alpha)
        let slip_angle_front = (self.velocity_c[1] + yaw_speed_front).atan2(self.velocity_c[0].abs()) - self.velocity_c[0].signum() * self.steer_angle;
        let slip_angle_rear = (self.velocity_c[1] + yaw_speed_rear).atan2(self.velocity_c[0].abs());

        let tire_grip_front = cfg.tire_grip;
        let tire_grip_rear = cfg.tire_grip * (1.0 - self.input.ebrake * (1.0 - cfg.lock_grip)); // reduce rear grip when ebrake is on

        let friction_force_front_cy = (-cfg.corner_stiffness_front * slip_angle_front).max(-tire_grip_front).min(tire_grip_front) * axle_weight_front;
        let friction_force_rear_cy = (-cfg.corner_stiffness_rear * slip_angle_rear).max(-tire_grip_rear).min(tire_grip_rear) * axle_weight_rear;

        // Get amount of brake/throttle from our inputs
        let brake = (self.input.brake * cfg.brake_force + self.input.ebrake * cfg.ebrake_force).min(cfg.brake_force);
        let throttle = self.input.throttle * cfg.engine_force;

        // Resulting force in local car coordinates.
        // This is implemented as a RWD car only.
        let traction_force_cx = throttle - brake * self.velocity_c[0].signum();
        let traction_force_cy = 0.0;

        let drag_force_cx = -cfg.roll_resist * self.velocity_c[0] - cfg.air_resist * self.velocity_c[0] * self.velocity_c[0].abs();
        let drag_force_cy = -cfg.roll_resist * self.velocity_c[1] - cfg.air_resist * self.velocity_c[1] * self.velocity_c[1].abs();

        // total force in car coordinates
        let total_force_cx = drag_force_cx + traction_force_cx;
        let total_force_cy = drag_force_cy + traction_force_cy + self.steer_angle.cos() * friction_force_front_cy + friction_force_rear_cy;

        // acceleration along car axes
        self.accel_c[0] = total_force_cx / cfg.mass; // forward/reverse accel
        self.accel_c[1] = total_force_cy / cfg.mass; // sideways accel

        // acceleration in world coordinates
        self.accel[0] = cs * self.accel_c[0] - sn * self.accel_c[1];
        self.accel[1] = sn * self.accel_c[0] + cs * self.accel_c[1];

        // update velocity
        self.velocity[0] += self.accel[0] * dt;
        self.velocity[1] += self.accel[1] * dt;

        self.abs_vel = (self.velocity[0] * self.velocity[0] + self.velocity[1] * self.velocity[1]).sqrt();

        // calculate rotational forces
        let mut angular_torque = (friction_force_front_cy + traction_force_cy) * cfg.cg_to_front_axle - friction_force_rear_cy * cfg.cg_to_rear_axle;

        // Sim gets unstable at very slow speeds, so just stop the car
        if self.abs_vel < 0.5 && throttle == 0.0 {
            self.velocity[0] = 0.0;
            self.velocity[1] = 0.0;
            self.abs_vel = 0.0;
            angular_torque = 0.0;
            self.yaw_rate = 0.0;
        }

        let angular_accel = angular_torque / self.comp.inertia;

        self.yaw_rate += angular_accel * dt;
        self.heading += self.yaw_rate * dt;

        // finally we can update position
        self.position[0] += self.velocity[0] * dt;
        self.position[1] += self.velocity[1] * dt;

        self.input = CarInputState::default()
    }

    pub fn accelerate(&mut self, _dt: f64) {
        self.input.throttle = 1.0;
    }

    pub fn brake(&mut self, _dt: f64) {
        self.input.brake = 1.0;
    }

    pub fn ebrake(&mut self, _dt: f64) {
        self.input.ebrake = 1.0;
    }

    pub fn steer_left(&mut self, _dt: f64) {
        self.input.left = 1.0;
    }

    pub fn steer_right(&mut self, _dt: f64) {
        self.input.right = 1.0;
    }

    pub fn get_transform(&self) -> Matrix2d {
        math::translate(self.position).rot_rad(self.heading)
    }

    pub fn draw (&self, transform: Matrix2d, gl: &mut GlGraphics) {
        use graphics::*;

        let cfg = self.cfg;

        let mat = transform.zoom(10.0).append_transform(self.get_transform());

        rectangle(C_RED, [-cfg.cg_to_rear, -cfg.half_width, cfg.cg_to_front + cfg.cg_to_rear, cfg.half_width * 2.0], mat, gl);

        let mat_rear = mat.trans(-cfg.cg_to_rear_axle, 0.0);

        rectangle(C_BLUE, [-cfg.wheel_radius, -cfg.wheel_width / 2.0, cfg.wheel_radius * 2.0, cfg.wheel_width], mat_rear, gl);

        let mat_front = mat.trans(cfg.cg_to_front_axle, 0.0).rot_rad(self.steer_angle);

        rectangle(C_BLUE, [-cfg.wheel_radius, -cfg.wheel_width / 2.0, cfg.wheel_radius * 2.0, cfg.wheel_width], mat_front, gl);

        // polygon(C_BLUE, &[[15.0, -10.0], [15.0, 10.0], [25.0, 6.0], [25.0, -6.0]], mat, gl)

    }
}

const C_RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const C_BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
