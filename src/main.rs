#![feature(iterator_step_by)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate ncollide;
extern crate nalgebra as na;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::{RenderEvent, UpdateEvent};
use piston::input::keyboard::Key;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod app;
mod track;
mod input_state;
mod car_steering;
mod vec_math;
use app::App;
use input_state::InputState;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [600, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut input_state = InputState::default();
    let mut app = App::new(GlGraphics::new(opengl));
    let mut events = Events::new(
        EventSettings::new()
            .ups(120)
    );
    
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        input_state.handle_event(&e);

        if let Some(u) = e.update_args() {
            app.update(&u, &input_state);
        }
    }
}