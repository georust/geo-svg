use crate::{SVGDocument, SVGShape, Style, ToSvgStr, ViewBox};

#[derive(Debug, Clone, Default)]
pub struct SVGDocumentPart {
    pub(crate) style: Style,
    pub(crate) viewbox: ViewBox,
    pub(crate) shapes: Vec<SVGShape>,
}

impl SVGDocumentPart {
    pub fn add_shape(mut self, shapeable: &impl ToSvgStr) -> Self {
        self.viewbox = self.viewbox.and(&shapeable.viewbox(&self.style));

        let content = shapeable.to_svg_str(&self.style);
        self.shapes.push(SVGShape { content });

        self
    }

    pub fn add_shapes<'a, S: ToSvgStr + 'a>(
        self,
        shapeables: impl IntoIterator<Item = &'a S>,
    ) -> Self {
        shapeables
            .into_iter()
            .fold(self, |res, shape| res.add_shape(shape))
    }

    pub fn finish_shapes(self) -> SVGDocument {
        SVGDocument {
            style: Default::default(),
            viewbox: self.viewbox,
            parts: vec![self],
        }
    }

    pub(crate) fn render(&self) -> String {
        self.shapes
            .iter()
            .map(|shape| shape.content.clone())
            .reduce(|a, b| a + &b)
            .unwrap_or_default()
    }
}
