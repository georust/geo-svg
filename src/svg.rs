use crate::{Color, Style, ToSvgStr, ViewBox};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Svg<'a> {
    pub items: Vec<&'a ToSvgStr>,
    pub siblings: Vec<Svg<'a>>,
    pub viewbox: ViewBox,
    pub style: Style,
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

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = Some(opacity);
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_opacity(opacity);
        }
        self
    }

    pub fn fill(mut self, fill: Color) -> Self {
        self.style.fill = Some(fill);
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().fill(fill);
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

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.style.radius = radius;
        for sibling in &mut self.siblings {
            *sibling = sibling.clone().with_radius(radius);
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
}

impl<'a> Display for Svg<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let viewbox = self.viewbox();
        write!(
            fmt,
            r#"<svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid meet" viewBox="{x} {y} {w} {h}">{content}</svg>"#,
            x = viewbox.min_x(),
            y = viewbox.min_y(),
            w = viewbox.width(),
            h = viewbox.height(),
            content = self.items
                .iter()
                .map(|item| item.to_svg_str(&self.style))
                .chain(self.siblings.iter().map(Svg::svg_str))
                .collect::<String>()
        )
    }
}
