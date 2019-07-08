use crate::{Style, Svg, ViewBox};

pub trait ToSvgStr {
    fn to_svg_str(&self, style: &Style) -> String;
    fn viewbox(&self, style: &Style) -> ViewBox;
}

impl<'a> ToSvgStr for Svg<'a> {
    fn to_svg_str(&self, style: &Style) -> String {
        self.clone().with_style(style).to_string()
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        self.clone().with_style(style).viewbox
    }
}
