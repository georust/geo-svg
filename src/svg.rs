use crate::{Color, LineCap, LineJoin, Style, ToSvgStr, Unit, ViewBox};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Svg<'a> {
    pub items: Vec<&'a dyn ToSvgStr>,
    pub siblings: Vec<Svg<'a>>,
    pub viewbox: ViewBox,
    pub style: Style,
    pub width: Option<Unit>,
    pub height: Option<Unit>,
}

impl<'a> Svg<'a> {
    pub fn and(mut self, sibling: Svg<'a>) -> Self {
        self.siblings.push(sibling);
        self
    }

    pub fn with_style(mut self, style: &Style) -> Self {
        self.style = style.clone();
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_style(style);
        }
        self
    }

    pub fn with_margin(mut self, margin: f32) -> Self {
        self.viewbox = self.viewbox.with_margin(margin);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.style.fill = Some(color);
        self.style.stroke_color = Some(color);
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_color(color);
        }
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = Some(opacity);
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_opacity(opacity);
        }
        self
    }

    pub fn with_fill_color(mut self, fill: Color) -> Self {
        self.style.fill = Some(fill);
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_fill_color(fill);
        }
        self
    }

    pub fn with_fill_opacity(mut self, fill_opacity: f32) -> Self {
        self.style.fill_opacity = Some(fill_opacity);
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_fill_opacity(fill_opacity);
        }
        self
    }

    pub fn with_stroke_width(mut self, stroke_width: f32) -> Self {
        self.style.stroke_width = Some(stroke_width);
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_stroke_width(stroke_width);
        }
        self
    }

    pub fn with_stroke_opacity(mut self, stroke_opacity: f32) -> Self {
        self.style.stroke_opacity = Some(stroke_opacity);
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_stroke_opacity(stroke_opacity);
        }
        self
    }

    pub fn with_stroke_color(mut self, stroke_color: Color) -> Self {
        self.style.stroke_color = Some(stroke_color);
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_stroke_color(stroke_color);
        }
        self
    }

    pub fn with_stroke_dasharray(mut self, dasharray: Vec<f32>) -> Self {
        self.style.stroke_dasharray = Some(dasharray.clone());
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_stroke_dasharray(dasharray.clone());
        }
        self
    }

    pub fn with_stroke_linecap(mut self, linecap: LineCap) -> Self {
        self.style.stroke_linecap = Some(linecap.clone());
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_stroke_linecap(linecap.clone());
        }
        self
    }

    pub fn with_stroke_linejoin(mut self, linejoin: LineJoin) -> Self {
        self.style.stroke_linejoin = Some(linejoin.clone());
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_stroke_linejoin(linejoin.clone());
        }
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.style.radius = radius;
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_radius(radius);
        }
        self
    }

    pub fn svg_str(&self) -> String {
        self.items
            .iter()
            .map(|item| item.to_svg_str(&self.style))
            .chain(self.siblings.iter().map(Svg::svg_str))
            .collect()
    }

    pub fn viewbox(&self) -> ViewBox {
        self.items
            .iter()
            .map(|item| item.viewbox(&self.style))
            .chain(self.siblings.iter().map(Svg::viewbox))
            .fold(self.viewbox, |viewbox, other_viewbox| {
                viewbox.add(&other_viewbox)
            })
    }

    /// Typically only `set_width` or `set_height is necessary. The other quantity is computed to match the aspect ratio of the view box.
    pub fn set_width(&mut self, width: Unit) {
        self.width = Some(width);
    }

    /// Typically only `set_width` or `set_height is necessary. The other quantity is computed to match the aspect ratio of the view box.
    pub fn set_height(&mut self, height: Unit) {
        self.height = Some(height);
    }
}

impl<'a> Display for Svg<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let viewbox = self.viewbox();
        let w = viewbox.width();
        let h = viewbox.height();
        write!(
            fmt,
            r#"<svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid meet" viewBox="{x} {y} {w} {h}""#,
            x = viewbox.min_x(),
            y = viewbox.min_y(),
        ).and_then(|_| if self.width.is_some() || self.height.is_some() {
                write!(
                    fmt,
                    r#" width="{width}" height="{height}""#,
                    width = self.width.unwrap_or_else(|| self.height.unwrap().scale(w / h)),
                    height = self.height.unwrap_or_else(|| self.width.unwrap().scale(h / w)),
                )
            } else {
                Ok(())
            }.and_then(|_| write!(fmt, r#">{content}</svg>"#, content = self.svg_str()))
        )
    }
}
