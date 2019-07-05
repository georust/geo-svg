use crate::viewbox::ViewBox;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Svg {
    pub elements: Vec<Svg>,
    pub view_box: ViewBox,
}

impl Svg {
    pub fn and(mut self, other: &impl ToSvg) -> Self {
        self.elements.push(other.to_svg());
        self.view_box = self.view_box.add(&other.view_box());
        self
    }

    pub fn with_margin(mut self, margin: f32) -> Self {
        self.view_box = self.view_box.with_margin(margin);
        self
    }
}

impl std::fmt::Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        let mut buffer = String::new();
        let view_box = self.view_box;
        write!(
            buffer,
            r#"<svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid meet" viewBox="{x} {y} {w} {h}">"#,
            x = view_box.min_x(),
            y = view_box.min_y(),
            w = view_box.width(),
            h = view_box.height(),
        )?;
        write!(
            buffer,
            "{}",
            std::iter::once(self)
                .chain(self.elements.iter())
                .map(ToSvg::to_svg_str)
                .collect::<String>()
        )?;
        write!(buffer, "</svg>")?;
        write!(f, "{}", buffer)
    }
}

pub trait ToSvg {
    fn to_svg(&self) -> Svg {
        Svg {
            elements: Vec::new(),
            view_box: self.view_box(),
        }
    }

    fn to_svg_str(&self) -> String;
    fn view_box(&self) -> ViewBox;
}

impl ToSvg for Svg {
    fn to_svg_str(&self) -> String {
        self.elements.iter().cloned().collect()
    }

    fn view_box(&self) -> ViewBox {
        self.view_box
    }
}

impl<T: ToSvg> ToSvg for &[T] {
    fn to_svg_str(&self) -> String {
        self.iter().map(ToSvg::to_svg_str).collect()
    }

    fn view_box(&self) -> ViewBox {
        self.iter().fold(ViewBox::default(), |view_box, svg| {
            view_box.add(&svg.view_box())
        })
    }
}