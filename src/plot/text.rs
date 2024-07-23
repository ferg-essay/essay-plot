use essay_graphics::api::Point;

use crate::{artist::{TextOpt, Text}, chart::Chart};

pub fn text(
    graph: &mut Chart, 
    pos: impl Into<Point>, 
    text: impl AsRef<str>, 
) -> TextOpt {
    let text = Text::new(pos, text);

    graph.artist(text)
}
