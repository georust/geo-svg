use crate::{Svg, ToSvg};

/// This trait let's you combine multiple things that can be converted to a SVG into one big
/// compound SVG
pub trait CombineToSVG {
    fn combine_to_svg(&self) -> Option<Svg>;
}

impl<S: ToSvg> CombineToSVG for &[S] {
    fn combine_to_svg(&self) -> Option<Svg> {
        self.iter().map(|s| s.to_svg()).reduce(|a, b| a.and(b))
    }
}

impl<S: ToSvg> CombineToSVG for Vec<S> {
    fn combine_to_svg(&self) -> Option<Svg> {
        self.iter().map(|s| s.to_svg()).reduce(|a, b| a.and(b))
    }
}
