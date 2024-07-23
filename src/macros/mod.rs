#[macro_export]
macro_rules! frame_option_struct {
    ($name: ident, $ty: ident, $getter: ident) => {
        pub struct $name {
            view: essay_graphics::layout::View<$crate::chart::Frame>,
            artist: $crate::chart::FrameArtist,
        }
        
        impl $name {
            pub(crate) fn new(
                view: &essay_graphics::layout::View<$crate::chart::Frame>, 
                artist: $crate::chart::FrameArtist,
            ) -> Self {
                Self {
                    view: view.clone(),
                    artist,
                }
            }
        
            fn write(&mut self, fun: impl FnOnce(&mut $ty)) {
                self.view.write(|frame| fun(frame.$getter(self.artist)));
            }
        }
    }
}

#[macro_export]
macro_rules! data_artist_option_struct {
    ($name: ident, $ty: ty) => {
        pub struct $name {
            artist: $crate::artist::ArtistHandle<$ty>,
        }
        
        impl $name {
            pub(crate) fn new(
                artist: $crate::artist::ArtistHandle<$ty>,
            ) -> Self {
                Self {
                    artist: artist,
                }
            }
        
            #[inline]
            #[allow(unused)]
            fn write<R>(&mut self, fun: impl FnOnce(&mut $ty) -> R) -> R {
                self.artist.write(|artist| fun(artist))
            }
        }
    }
}

#[macro_export]
macro_rules! path_style_options {
    ($field: ident) => {

        pub fn color(&mut self, color: impl Into<essay_graphics::api::Color>) -> &mut Self {
            self.write(|ticks| { ticks.$field.color(color); });
            self
        }

        pub fn face_color(&mut self, color: impl Into<essay_graphics::api::Color>) -> &mut Self {
            self.write(|ticks| { ticks.$field.face_color(color); });
            self
        }

        pub fn edge_color(&mut self, color: impl Into<essay_graphics::api::Color>) -> &mut Self {
            self.write(|ticks| { ticks.$field.edge_color(color); });
            self
        }
    
        pub fn line_width(&mut self, width: f32) -> &mut Self {
            self.write(|ticks| { ticks.$field.line_width(width); });
            self
        }
    
        pub fn line_style(&mut self, style: impl Into<essay_graphics::api::LineStyle>) -> &mut Self {
            self.write(|ticks| { ticks.$field.line_style(style); });
            self
        }
    
        pub fn join_style(&mut self, style: impl Into<essay_graphics::api::JoinStyle>) -> &mut Self {
            self.write(|ticks| { ticks.$field.join_style(style); });
            self
        }
    
        pub fn cap_style(&mut self, style: impl Into<essay_graphics::api::CapStyle>) -> &mut Self {
            self.write(|ticks| { ticks.$field.cap_style(style); });
            self
        }
    
        pub fn hatch(&mut self, hatch: impl Into<essay_graphics::api::Hatch>) -> &mut Self {
            self.write(|ticks| { ticks.$field.hatch(hatch); });
            self
        }
    }
}


#[macro_export]
macro_rules! transform_options {
    ($field: ident) => {

    pub fn rotate(&mut self, angle: impl Into<Angle>) -> &mut Self {
        self.write(|artist| {
            artist.$field = artist.$field.rotate(angle.into().to_radians());
            artist.stale();
        });
        self
    }

    pub fn scale(&mut self, scale: f32) -> &mut Self {
        self.write(|artist| {
            artist.$field = artist.$field.scale(scale, scale);
            artist.stale();
        });
        self
    }

    pub fn scale_xy(&mut self, sx: f32, sy: f32) -> &mut Self {
        self.write(|artist| {
            artist.$field = artist.$field.scale(sx, sy);
            artist.stale();
        });
        self
    }

    pub fn translate(&mut self, dx: f32, dy: f32) -> &mut Self {
        self.write(|artist| {
            artist.$field = artist.$field.translate(dx, dy);
            artist.stale();
        });
        self
    }

    pub fn transform(&mut self, transform: impl Into<Affine2d>) -> &mut Self {
        self.write(|artist| {
            artist.$field = transform.into();
            artist.stale();
        });
        self
    }
}}

