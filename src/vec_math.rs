use graphics::math::*;
use graphics::Transformed;

trait TransformedVec {
    fn trans_vec(self, Vec2d) -> Self;
}

impl<T> TransformedVec for T where T: Transformed {
    fn trans_vec(self, vec: Vec2d) -> Self {
        self.trans(vec[0], vec[1])
    }
}
