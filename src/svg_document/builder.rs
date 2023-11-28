use crate::{SVGDocumentPart, Stylable, Style};

#[derive(Debug, Clone, Default)]
pub struct SVGDocumentBuilder {
    style: Style,
}

impl SVGDocumentBuilder {
    pub fn finish_style(self) -> SVGDocumentPart {
        let Self { style } = self;
        SVGDocumentPart {
            style,
            shapes: Default::default(),
            viewbox: Default::default(),
        }
    }
}

impl Stylable for SVGDocumentBuilder {
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}
