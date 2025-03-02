use std::fmt::{Display, Formatter, Result};

/// Represents a quantity in absolute CSS units.
///
/// Relative units `%`, `em`, `rem`, `vh` and `vw` are not supported.
#[derive(Clone, Copy, Debug)]
pub enum Unit {
    /// Centimeter (cm), 37.8px
    Centimeter(f32),
    /// Inch (in), 2.54cm, exactly 96px, 72pt, 6pc
    Inch(f32),
    /// No units, defaults as appropriate based on context
    None(f32),
    /// Millimeter (mm), 0.1cm
    Millimeter(f32),
    /// Pica (pc), 1/6 in, 12pt, 16px
    Pica(f32),
    /// Pixel (px), 1/96 in, 2/3 pt, 1/16 pc
    Pixel(f32),
    /// Point (pt), 1/72 in or 4/3 px, 1/12 pc
    Point(f32),
    /// Quarter-millimeter (Q), 1/4 mm, or 1/40 cm
    QuarterMillimeter(f32),
}

impl Unit {
    /// raw value with no units
    pub fn value(&self) -> f32 {
        match self {
            Self::Centimeter(value) => *value,
            Self::Inch(value) => *value,
            Self::None(value) => *value,
            Self::Millimeter(value) => *value,
            Self::Pica(value) => *value,
            Self::Pixel(value) => *value,
            Self::Point(value) => *value,
            Self::QuarterMillimeter(value) => *value,
        }
    }

    /// abbreviated symbol of the unit
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Centimeter(_) => "cm",
            Self::Inch(_) => "in",
            Self::None(_) => "",
            Self::Millimeter(_) => "mm",
            Self::Pica(_) => "pc",
            Self::Pixel(_) => "px",
            Self::Point(_) => "pt",
            Self::QuarterMillimeter(_) => "Q",
        }
    }

    pub fn scale(&self, factor: f32) -> Self {
        match self {
            Self::Centimeter(value) => Self::Centimeter(value * factor),
            Self::Inch(value) => Self::Inch(value * factor),
            Self::None(value) => Self::None(value * factor),
            Self::Millimeter(value) => Self::Millimeter(value * factor),
            Self::Pica(value) => Self::Pica(value * factor),
            Self::Pixel(value) => Self::Pixel(value * factor),
            Self::Point(value) => Self::Point(value * factor),
            Self::QuarterMillimeter(value) => Self::QuarterMillimeter(value * factor),
        }
    }
}

impl Display for Unit {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "{value}{symbol}",
            value = self.value(),
            symbol = self.symbol()
        )
    }
}
