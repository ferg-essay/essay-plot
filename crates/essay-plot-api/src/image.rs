use std::sync::Arc;

#[derive(Clone)]
pub struct Image(Arc<ImageId>);

impl Image {
    pub fn new(id: usize) -> Image {
        Image(Arc::new(ImageId(id)))
    }

    #[inline]
    pub fn is_live(&self) -> bool {
        Arc::strong_count(&self.0) > 1
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.0.index()
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.0.index() == other.0.index()
    }
}

pub struct ImageId(usize);

impl ImageId {
    #[inline]
    pub fn index(&self) -> usize {
        self.0
    }
}

