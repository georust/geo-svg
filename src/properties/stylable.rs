use crate::{Color, Style};

pub trait Stylable
where
    Self: Sized,
{
    fn get_style_mut(&mut self) -> &mut Style;

    fn use_style(mut self, other: &Style) -> Self {
        *self.get_style_mut() = other.clone();
        self
    }

    /// defaults to the settings of the left hand side and falls back to settings from the right
    /// hand side
    ///
    /// There's no guarantee that any of the settings are set after this operation
    fn or_style(mut self, other: &Style) -> Self {
        let style = self.get_style_mut();
        style.opacity = style.opacity.or(other.opacity);
        style.fill_color = style.fill_color.or(other.fill_color);
        style.fill_opacity = style.fill_opacity.or(other.fill_opacity);
        style.stroke_color = style.stroke_color.or(other.stroke_color);
        style.stroke_width = style.stroke_width.or(other.stroke_width);
        style.stroke_opacity = style.stroke_opacity.or(other.stroke_opacity);
        style.radius = style.radius.or(other.radius);
        self
    }

    fn with_opacity(mut self, opacity: f32) -> Self {
        self.get_style_mut().opacity.replace(opacity);
        self
    }

    fn with_fill_color(mut self, color: Color) -> Self {
        self.get_style_mut().fill_color.replace(color);
        self
    }

    fn with_fill_opacity(mut self, opacity: f32) -> Self {
        self.get_style_mut().fill_opacity.replace(opacity);
        self
    }

    fn with_stroke_color(mut self, color: Color) -> Self {
        self.get_style_mut().stroke_color.replace(color);
        self
    }

    fn with_stroke_width(mut self, width: f32) -> Self {
        self.get_style_mut().stroke_width.replace(width);
        self
    }

    fn with_stroke_opacity(mut self, opacity: f32) -> Self {
        self.get_style_mut().stroke_opacity.replace(opacity);
        self
    }

    fn with_radius(mut self, radius: f32) -> Self {
        self.get_style_mut().radius.replace(radius);
        self
    }
}
