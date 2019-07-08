use crate::{Style, Svg, ToSvgStr, ViewBox};

pub trait ToSvg {
    fn to_svg(&self) -> Svg;
}

impl<T: ToSvgStr> ToSvg for T {
    fn to_svg(&self) -> Svg {
        Svg {
            items: vec![self],
            siblings: vec![],
            viewbox: ViewBox::default(),
            style: Style::default(),
        }
    }
}
