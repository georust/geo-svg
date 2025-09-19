use crate::{Style, Svg, ToSvgStr, ViewBox};

pub trait ToSvg {
    fn to_svg(&self) -> Svg<'_>;
}

impl<T: ToSvgStr> ToSvg for T {
    fn to_svg(&self) -> Svg<'_> {
        Svg {
            items: vec![self],
            siblings: vec![],
            viewbox: ViewBox::default(),
            style: Style::default(),
            width: None,
            height: None,
        }
    }
}
