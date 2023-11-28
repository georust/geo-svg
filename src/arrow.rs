use geo::{Coord, GeoFloat, Line, Point, Rotate, Vector2DOps};

use crate::{Style, ToSvgStr};

#[derive(Debug, Clone, Copy)]
pub struct Arrow<T: GeoFloat> {
    line: Line<T>,
}

impl<T: GeoFloat> Arrow<T> {
    pub fn new(line: Line<T>) -> Self {
        Self { line }
    }
}

impl<T: GeoFloat> ToSvgStr for Arrow<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        let Some(arrow_dir) = (self.line.end - self.line.start)
            .try_normalize()
            .map(Point::from)
        else {
            return String::default();
        };

        let arrow_head_length = T::from(style.radius).unwrap_or(T::one());

        let [left_arrow_head, right_arrow_head] = [1.0, -1.0]
            .map(|sign| sign * 135.0)
            .map(T::from)
            .map(Option::unwrap)
            .map(|angle| arrow_dir.rotate_around_point(angle, Point::from(Coord::zero())))
            .map(|arrow_head_dir| arrow_head_dir * arrow_head_length)
            .map(|arrow_head_dir| Line::new(self.line.end, self.line.end + arrow_head_dir.0));

        [self.line, left_arrow_head, right_arrow_head]
            .map(|line| {
                format!(
                    r#"<path d="M {x1:?} {y1:?} L {x2:?} {y2:?}"{style}/>"#,
                    x1 = line.start.x,
                    y1 = line.start.y,
                    x2 = line.end.x,
                    y2 = line.end.y,
                    style = style,
                )
            })
            .concat()
    }

    fn viewbox(&self, style: &crate::Style) -> crate::ViewBox {
        let style = Style {
            radius: 0.0,
            ..style.clone()
        };
        self.line
            .start
            .viewbox(&style)
            .add(&self.line.end.viewbox(&style))
    }
}

#[cfg(test)]
mod tests {
    use crate::ToSvg;

    use super::*;

    #[test]
    fn test_arrow() {
        println!(
            "{}",
            Arrow::new(Line::new(Point::new(0.0, 0.0).0, Point::new(10.0, 10.0).0))
                .to_svg()
                .with_stroke_color(crate::Color::Named("black"))
                .with_stroke_width(0.01)
                .with_radius(0.5)
        )
    }
}
