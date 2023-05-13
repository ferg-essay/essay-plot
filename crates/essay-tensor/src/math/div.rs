use crate::ops::BinaryKernel;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Div;

impl BinaryKernel for Div {
    #[inline]
    fn f(&self, x: f32, y: f32) -> f32 {
        x / y
    }

    #[inline]
    fn df_dx(&self, _x: f32, y: f32) -> f32 {
        y.recip()
    }

    #[inline]
    fn df_dy(&self, x: f32, y: f32) -> f32 {
        - x / y.powi(2)
    }
}
