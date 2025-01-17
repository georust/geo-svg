use crate::Color;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl Default for LineCap {
    fn default() -> Self {
        LineCap::Butt
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub opacity: Option<f32>,
    pub fill: Option<Color>,
    pub fill_opacity: Option<f32>,
    pub stroke_color: Option<Color>,
    pub stroke_width: Option<f32>,
    pub stroke_opacity: Option<f32>,
    pub stroke_dasharray: Option<Vec<f32>>,
    pub stroke_linecap: Option<LineCap>,
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
            stroke_dasharray: None,
            stroke_linecap: None,
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
        if let Some(stroke_dasharray) = &self.stroke_dasharray {
            write!(
                fmt,
                r#" stroke-dasharray="{}""#,
                stroke_dasharray
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )?;
        }
        if let Some(stroke_linecap) = &self.stroke_linecap {
            write!(
                fmt,
                r#" stroke-linecap="{}""#,
                match stroke_linecap {
                    LineCap::Butt => "butt",
                    LineCap::Round => "round",
                    LineCap::Square => "square",
                }
            )?;
        }
        Ok(())
    }
}
