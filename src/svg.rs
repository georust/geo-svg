#[derive(Debug, Default, Clone, PartialEq)]
pub struct Svg {
    pub elements: Vec<String>,
    pub view_box: ViewBox,
}

impl Svg {
    pub fn and(mut self, other: &impl ToSvg) -> Self {
        self.elements.push(other.to_svg_str());
        self.view_box = self.view_box.add(&other.view_box());
        self
    }

    pub fn with_margin(mut self, margin: f32) -> Self {
        self.view_box.min_x = self.view_box.min_x.map(|x| x - margin);
        self.view_box.min_y = self.view_box.min_y.map(|y| y - margin);
        self.view_box.max_x = self.view_box.max_x.map(|x| x + margin);
        self.view_box.max_y = self.view_box.max_y.map(|y| y + margin);
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
            self.elements.iter().cloned().collect::<String>()
        )?;
        write!(buffer, "</svg>")?;
        write!(f, "{}", buffer)
    }
}

pub trait ToSvg {
    fn to_svg(&self) -> Svg {
        Svg {
            elements: vec![self.to_svg_str()],
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

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ViewBox {
    min_x: Option<f32>,
    min_y: Option<f32>,
    max_x: Option<f32>,
    max_y: Option<f32>,
}

impl ViewBox {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min_x: Some(min_x),
            min_y: Some(min_y),
            max_x: Some(max_x),
            max_y: Some(max_y),
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            min_x: Self::min_option(self.min_x, other.min_x),
            min_y: Self::min_option(self.min_y, other.min_y),
            max_x: Self::max_option(self.max_x, other.max_x),
            max_y: Self::max_option(self.max_y, other.max_y),
        }
    }

    pub fn min_x(&self) -> f32 {
        self.min_x.unwrap_or_default()
    }

    pub fn min_y(&self) -> f32 {
        self.min_y.unwrap_or_default()
    }

    pub fn max_x(&self) -> f32 {
        self.max_x.unwrap_or_default()
    }

    pub fn max_y(&self) -> f32 {
        self.max_y.unwrap_or_default()
    }

    pub fn width(&self) -> f32 {
        (self.min_x() - self.max_x()).abs()
    }

    pub fn height(&self) -> f32 {
        (self.min_y() - self.max_y()).abs()
    }

    fn min_option(a: Option<f32>, b: Option<f32>) -> Option<f32> {
        match (a, b) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    fn max_option(a: Option<f32>, b: Option<f32>) -> Option<f32> {
        match (a, b) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
}
