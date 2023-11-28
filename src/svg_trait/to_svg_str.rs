use crate::{Style, ViewBox};

pub trait ToSvgStr {
    fn to_svg_str(&self, style: &Style) -> String;
    fn viewbox(&self, style: &Style) -> ViewBox;
}
