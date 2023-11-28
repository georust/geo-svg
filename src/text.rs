use std::fmt::Display;

use geo::{Coord, CoordNum};

use crate::{Style, ToSvgStr, ViewBox};

// 🚧🚧 TODO 🚧🚧 : Implement this to find visual center of polygons to place text automatically
//
// https://blog.mapbox.com/a-new-algorithm-for-finding-a-visual-center-of-a-polygon-7c77e6492fbc

/// Simple Text element for SVGs. This comes in handy if you want to enumerate some sort of
/// geometry for any purposes
pub struct Text<S, C>
where
    S: Display,
    C: CoordNum,
{
    /// anything that is display-able as a string
    text: S,
    /// the rough coordinates of the text
    position: Coord<C>,
    /// the size of the font of the text
    font_size: f32,
}

impl<S, C> Text<S, C>
where
    S: Display,
    C: CoordNum,
{
    /// create new Text object and it's position
    pub fn new(text: S, position: Coord<C>) -> Self {
        Self {
            text,
            position,
            font_size: 10.0,
        }
    }

    /// overwrite the existing font size
    pub fn with_font_size(self, font_size: f32) -> Self {
        Self { font_size, ..self }
    }
}

impl<S, C> ToSvgStr for Text<S, C>
where
    S: Display,
    C: CoordNum + std::fmt::Display,
{
    fn to_svg_str(&self, _style: &Style) -> String {
        let Text {
            text,
            position: Coord { x, y },
            font_size,
        } = self;
        format!(r#"<text font-size="{font_size}" x="{x}" y="{y}">{text}</text>"#)
    }

    // we can probably do better here by calculating a viewbox based on font and font size
    // something along the lines of
    //
    // https://stackoverflow.com/questions/71283347/difference-in-length-calculation-for-svg-text-element
    fn viewbox(&self, _style: &Style) -> ViewBox {
        ViewBox {
            min_x: None,
            min_y: None,
            max_x: None,
            max_y: None,
        }
    }
}
