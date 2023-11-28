use geo_types::{CoordNum, Line};

use crate::{Style, ToSvgStr};

#[derive(Debug, Clone, Copy)]
pub struct Arrow<T: CoordNum> {
    line: Line<T>,
    arrow_head_length: f32,
}

impl<T: CoordNum> Arrow<T> {
    pub fn new(line: Line<T>) -> Self {
        Self {
            line,
            arrow_head_length: 1.0,
        }
    }

    pub fn with_arrow_head_length(mut self, length: f32) -> Self {
        self.arrow_head_length = length;
        self
    }
}

impl<T: CoordNum> ToSvgStr for Arrow<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        let dir = self.line.end - self.line.start;
        todo!("draw arrow head");

        format!(
            r#"<path d="M {x1:?} {y1:?} L {x2:?} {y2:?}"{style}/>"#,
            x1 = self.line.start.x,
            y1 = self.line.start.y,
            x2 = self.line.end.x,
            y2 = self.line.end.y,
            style = style,
        )
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
