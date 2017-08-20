use graphics::math::{Matrix2d};
use opengl_graphics::GlGraphics;
use graphics::*;
use ncollide::shape::Polyline2;
use na::Point2;
use std::sync::Arc;

pub const BORDER: &'static [f64] = &[
  -39.0, -42.0,
  -17.0, -43.0,
  5.0, -41.0,
  26.0, -34.0,
  33.0, -25.0,
  37.0, -13.0,
  36.0, -4.0,
  29.0, -2.0,
  21.0, -4.0,
  16.0, -8.0,
  9.0, -11.0,
  -4.0, -12.0,
  -7.0, -6.0,
  -2.0, 7.0,
  18.0, 13.0,
  34.0, 18.0,
  41.0, 31.0,
  36.0, 43.0,
  23.0, 45.0,
  0.0, 46.0,
  -12.0, 39.0,
  -20.0, 22.0,
  -27.0, 12.0,
  -36.0, 17.0,
  -36.0, 31.0,
  -35.0, 46.0,
  -46.0, 46.0,
  -47.0, 34.0,
  -47.0, 16.0,
  -43.0, 6.0,
  -33.0, 2.0,
  -24.0, 1.0,
  -20.0, 6.0,
  -9.0, 20.0,
  -4.0, 28.0,
  6.0, 36.0,
  21.0, 38.0,
  27.0, 35.0,
  27.0, 30.0,
  23.0, 27.0,
  12.0, 24.0,
  1.0, 20.0,
  -9.0, 16.0,
  -16.0, 6.0,
  -21.0, -2.0,
  -21.0, -10.0,
  -19.0, -18.0,
  -10.0, -22.0,
  2.0, -22.0,
  10.0, -22.0,
  17.0, -19.0,
  23.0, -16.0,
  24.0, -21.0,
  22.0, -23.0,
  12.0, -27.0,
  -1.0, -30.0,
  -19.0, -31.0,
  -28.0, -29.0,
  -39.0, -28.0,
];

const C_LINE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub fn draw_border(transform: Matrix2d, gl: &mut GlGraphics) {
    let line_width = 0.1;

    for wnd in BORDER.windows(4).step_by(2) {
        let (x1, y1, x2, y2) = (wnd[0], wnd[1], wnd[2], wnd[3]);
        line(C_LINE, line_width, [x1, y1, x2, y2], transform, gl);
    }
}

pub fn get_border_shape() -> Polyline2<f64> {
    let vec : Vec<Point2<f64>> = BORDER
        .windows(2)
        .step_by(2)
        .map(|wnd| { Point2::new(wnd[0], wnd[1]) })
        .collect();

    let indices : Vec<Point2<usize>> = (0usize..(BORDER.len() / 2 - 1))
        .into_iter()
        .map(|i| { Point2::new(i, i + 1)})
        .collect();

    Polyline2::new(Arc::new(vec), Arc::new(indices), None, None)
}

