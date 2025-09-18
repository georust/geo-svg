use crate::Color;
use std::fmt::{Display, Formatter, Result};

/// LineCap is used to define the shape to be used at the end of strokes.
///
/// Example:
/// ```
/// use geo_types::Point;
/// use geo_svg::{Color, LineCap, ToSvg};
///
/// let point = geo_types::Point::new(0.0, 0.0);
/// let svg_point = point
///     .to_svg()
///     .with_radius(20.0)
///     .with_fill_opacity(0.0)
///     .with_stroke_color(Color::Named("blue"))
///     .with_stroke_width(1.0)
///     .with_stroke_linecap(LineCap::Round)
///     .with_stroke_dasharray(vec![2.0, 1.0, 3.0, 1.0]);
/// ```
///
/// # Visual Examples
///
/// ### Butt
///
#[doc = include_str!("../images/linecap_butt.svg")]
///
/// ### Round
///
#[doc = include_str!("../images/linecap_round.svg")]
///
/// ### Square
///
#[doc = include_str!("../images/linecap_square.svg")]
///
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LineCap {
    #[default]
    Butt,
    Round,
    Square,
}

/// LineJoin is used to define the shape to be used at the corners of strokes.
///
///
/// Example:
/// ```
/// use geo_types::{Triangle};
/// use geo_svg::{Color, LineJoin, ToSvg};
///
/// let triangle = Triangle::new(
///     geo_types::Coord { x: 0.0, y: 0.0 },
///     geo_types::Coord { x: 10.0, y: 0.0 },
///     geo_types::Coord { x: 5.0, y: 10.0 },
/// );
/// let svg_triangle = triangle
///     .to_svg()
///     .with_fill_opacity(0.5)
///     .with_color(Color::Named("red"))
///     .with_stroke_width(2.0)
///     .with_stroke_linejoin(LineJoin::Bevel);
/// ```
///
/// # Visual Examples
///
/// ### Miter
///
#[doc = include_str!("../images/linejoin_miter.svg")]
///
/// ### Round
///
#[doc = include_str!("../images/linejoin_round.svg")]
///
/// ### Bevel
///
#[doc = include_str!("../images/linejoin_bevel.svg")]
///
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum LineJoin {
    #[default]
    Miter,
    Round,
    Bevel,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Style<'a> {
    pub opacity: Option<f32>,
    pub fill: Option<Color<'a>>,
    pub fill_opacity: Option<f32>,
    pub stroke_color: Option<Color<'a>>,
    pub stroke_width: Option<f32>,
    pub stroke_opacity: Option<f32>,
    pub stroke_dasharray: Option<Vec<f32>>,
    pub stroke_linecap: Option<LineCap>,
    pub stroke_linejoin: Option<LineJoin>,
    pub radius: f32,
}

impl Default for Style<'_> {
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
            stroke_linejoin: None,
            radius: 1.0,
        }
    }
}

impl Display for Style<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        if let Some(opacity) = self.opacity {
            write!(fmt, r#" opacity="{opacity}""#)?;
        }
        if let Some(fill) = self.fill {
            write!(fmt, r#" fill="{fill}""#)?;
        }
        if let Some(fill_opacity) = self.fill_opacity {
            write!(fmt, r#" fill-opacity="{fill_opacity}""#)?;
        }
        if let Some(stroke_color) = self.stroke_color {
            write!(fmt, r#" stroke="{stroke_color}""#)?;
        }
        if let Some(stroke_width) = self.stroke_width {
            write!(fmt, r#" stroke-width="{stroke_width}""#)?;
        }
        if let Some(stroke_opacity) = self.stroke_opacity {
            write!(fmt, r#" stroke-opacity="{stroke_opacity}""#)?;
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
        if let Some(stroke_linejoin) = &self.stroke_linejoin {
            write!(
                fmt,
                r#" stroke-linejoin="{}""#,
                match stroke_linejoin {
                    LineJoin::Miter => "miter",
                    LineJoin::Round => "round",
                    LineJoin::Bevel => "bevel",
                }
            )?;
        }
        Ok(())
    }
}
