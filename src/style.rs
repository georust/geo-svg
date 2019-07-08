use crate::Color;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub opacity: Option<f32>,
    pub fill: Option<Color>,
    pub fill_opacity: Option<f32>,
    pub stroke_color: Option<Color>,
    pub stroke_width: Option<f32>,
    pub stroke_opacity: Option<f32>,
    pub radius: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            opacity: None,
            fill: None,
            fill_opacity: None,
            stroke_color: None,
            stroke_width: None,
            stroke_opacity: None,
            radius: 1.0,
        }
    }
}

impl Display for Style {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        if let Some(opacity) = self.opacity {
            write!(fmt, r#" opacity="{}""#, opacity)?;
        }
        if let Some(fill) = self.fill {
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
