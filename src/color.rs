use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Named(&'static str),
    Rgb(u8, u8, u8),
    Hex(u32),
    Hsl(u16, u8, u8),
}

impl Display for Color {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            Color::Named(name) => write!(fmt, "{}", name),
            Color::Rgb(r, g, b) => write!(fmt, "rgb({},{},{})", r, g, b),
            Color::Hex(hex) => write!(fmt, "#{:06X}", hex),
            Color::Hsl(h, s, l) => {
                write!(fmt, "hsl({},{}%,{}%)", h % 360, s.min(&100), l.min(&100))
            }
        }
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_named()
    {
        assert_eq!(format!("{}", Color::Named("red")), "red");
    }

    #[test]
    fn test_rgb()
    {
        assert_eq!(format!("{}", Color::Rgb(255, 0, 0)), "rgb(255,0,0)");
    }

    #[test]
    fn test_hex()
    {
        assert_eq!(format!("{}", Color::Hex(0xFF0000)), "#FF0000");
        assert_eq!(format!("{}", Color::Hex(0xFF)), "#0000FF");
    }

    #[test]
    fn test_hsl()
    {
        assert_eq!(format!("{}", Color::Hsl(0, 100, 50)), "hsl(0,100%,50%)");
    }
}
