use std::fmt::Display;

use crate::{SVGDocumentBuilder, SVGDocumentPart, Stylable, Style, ViewBox};

#[derive(Debug, Clone)]
pub struct SVGDocument {
    pub(crate) style: Style,
    pub(crate) viewbox: ViewBox,
    pub(crate) parts: Vec<SVGDocumentPart>,
}

impl SVGDocument {
    pub fn style_builder() -> SVGDocumentBuilder {
        SVGDocumentBuilder::default()
    }

    pub fn and(mut self, other: Self) -> Self {
        self.viewbox = self.viewbox.and(&other.viewbox);
        self.style = self.style.or_style(&other.style);
        self.parts.extend(other.parts);
        self
    }

    pub fn render(&self) -> String {
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid meet" viewBox="{x} {y} {w} {h}" {style}>{content}</svg>"#,
            x = self.viewbox.min_x(),
            y = self.viewbox.min_y(),
            w = self.viewbox.width(),
            h = self.viewbox.height(),
            style = self.style,
            content = self
                .parts
                .iter()
                .map(|part| part.render())
                .reduce(|a, b| a + &b)
                .unwrap_or_default()
        )
    }
}

impl Stylable for SVGDocument {
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl Display for SVGDocument {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.render())
    }
}
