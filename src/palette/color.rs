use core::fmt;

use essay_graphics::api::{color::{Hsv, Hsva}, Color};

///
/// Hue, saturation, value.
/// 
/// Note: hue is degree based
/// 
pub struct Hsl(pub f32, pub f32, pub f32);

impl Hsl {
    #[inline]
    pub fn h(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn s(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn v(&self) -> f32 {
        self.2
    }
}

impl fmt::Debug for Hsl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Hsl").field(&self.0).field(&self.1).field(&self.2).finish()
    }
}

impl From<Hsl> for Color {
    #[inline]
    fn from(value: Hsl) -> Color {
        let Hsl(h, s, v) = value;

        Color::from(Hsva(h, s, v, 1.0))
    }
}

impl From<Hsl> for Hsv {
    #[inline]
    fn from(value: Hsl) -> Hsv {
        let Hsl(h, s, v) = value;

        Hsv(h / 360., s, v)
    }
}

#[cfg(test)]
mod test {
    use essay_graphics::api::{color::Hsva, Color};

    #[test]
    fn hsva_to_color() {
        assert_eq!(Color::from(Hsva(0., 1., 1., 1.,)), Color(0xff00_00ff));
        assert_eq!(Color::from(Hsva(1./6., 1., 1., 1.,)), Color(0xffff_00ff));
        assert_eq!(Color::from(Hsva(2./6., 1., 1., 1.,)), Color(0x00ff_00ff));
        assert_eq!(Color::from(Hsva(3./6., 1., 1., 1.,)), Color(0x00ff_ffff));
        assert_eq!(Color::from(Hsva(4./6., 1., 1., 1.,)), Color(0x0000_ffff));
        assert_eq!(Color::from(Hsva(5./6., 1., 1., 1.,)), Color(0xff00_ffff));

        assert_eq!(Color::from(Hsva(0./6., 0.5, 1., 1.,)), Color(0xff7f_7fff));

        assert_eq!(Color::from(Hsva(0./6., 1., 0.5, 1.,)), Color(0x7f00_00ff));
    }

    #[test]
    fn color_to_hsva() {
        assert_eq!(Hsva::from(Color(0xff00_00ff)), Hsva(0., 1., 1., 1.));
        assert_eq!(Hsva::from(Color(0xffff_00ff)), Hsva(0.16666663, 1., 1., 1.));
        assert_eq!(Hsva::from(Color(0x00ff_00ff)), Hsva(0.33333337, 1., 1., 1.));
        assert_eq!(Hsva::from(Color(0x00ff_ffff)), Hsva(0.5, 1., 1., 1.));
        assert_eq!(Hsva::from(Color(0x0000_ffff)), Hsva(0.66666675, 1., 1., 1.));
        assert_eq!(Hsva::from(Color(0xff00_ffff)), Hsva(5./6., 1., 1., 1.));

        assert_eq!(Hsva::from(Color(0x8000_00ff)), Hsva(0., 1., 0.5019608, 1.));
        assert_eq!(Hsva::from(Color(0xff7f_7fff)), Hsva(0., 0.50196075, 1., 1.));
    }
}