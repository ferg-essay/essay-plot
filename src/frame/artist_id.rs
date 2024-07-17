#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArtistId {
    artist: ArtistEnum,
    index: usize,
}

impl ArtistId {
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    // TODO: eliminate need for this function
    pub(crate) fn new_data(index: usize) -> ArtistId {
        ArtistId {
            artist: ArtistEnum::Data,
            index,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArtistEnum {
    None,

    Frame, // frame itself

    LeftFrame,
    RightFrame,
    TopFrame,
    BottomFrame,

    Data, // databox artists
}
