use crate::{Color, Stylable};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Style {
    pub(crate) opacity: Option<f32>,
    pub(crate) fill_color: Option<Color>,
    pub(crate) fill_opacity: Option<f32>,
    pub(crate) stroke_color: Option<Color>,
    pub(crate) stroke_width: Option<f32>,
    pub(crate) stroke_opacity: Option<f32>,
    pub(crate) radius: Option<f32>,
}

impl Stylable for Style {
    fn get_style_mut(&mut self) -> &mut Style {
        self
    }
}

impl Display for Style {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        if let Some(opacity) = self.opacity {
            write!(fmt, r#" opacity="{}""#, opacity)?;
        }
        if let Some(fill) = self.fill_color {
            write!(fmt, r#" fill="{}""#, fill)?;
        }
        if let Some(fill_opacity) = self.fill_opacity {
            write!(fmt, r#" fill-opacity="{}""#, fill_opacity)?;
        }
        if let Some(stroke_color) = self.stroke_color {
            write!(fmt, r#" stroke="{}""#, stroke_color)?;
        }
        if let Some(stroke_width) = self.stroke_width {
            write!(fmt, r#" stroke-width="{}""#, stroke_width)?;
        }
        if let Some(stroke_opacity) = self.stroke_opacity {
            write!(fmt, r#" stroke-opacity="{}""#, stroke_opacity)?;
        }
        Ok(())
    }
}
