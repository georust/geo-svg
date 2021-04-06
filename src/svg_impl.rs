use crate::{Style, ToSvgStr, ViewBox};
use geo_types::{
    Coordinate, Geometry, GeometryCollection, Line, LineString, MultiLineString, MultiPoint,
    MultiPolygon, Point, Polygon, Rect, Triangle,
};
use num_traits::{Num, NumCast};
use std::fmt::{Debug, Display};

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for Coordinate<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        Point::from(*self).to_svg_str(style)
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        Point::from(*self).viewbox(style)
    }
}

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for Point<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        format!(
            r#"<circle cx="{x}" cy="{y}" r="{radius}"{style}/>"#,
            x = self.x(),
            y = self.y(),
            radius = style.radius,
            style = style,
        )
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        let radius = style.radius + style.stroke_width.unwrap_or(1.0);
        ViewBox::new(
            NumCast::from(self.x()).unwrap_or(0f32) - radius,
            NumCast::from(self.y()).unwrap_or(0f32) - radius,
            NumCast::from(self.x()).unwrap_or(0f32) + radius,
            NumCast::from(self.y()).unwrap_or(0f32) + radius,
        )
    }
}

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for MultiPoint<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        self.0.iter().map(|point| point.to_svg_str(style)).collect()
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        self.0.iter().fold(ViewBox::default(), |view_box, point| {
            view_box.add(&point.viewbox(style))
        })
    }
}

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for Line<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        format!(
            r#"<path d="M {x1} {y1} L {x2} {y2}"{style}/>"#,
            x1 = self.start.x,
            y1 = self.start.y,
            x2 = self.end.x,
            y2 = self.end.y,
            style = style,
        )
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        let style = Style {
            radius: 0.0,
            ..style.clone()
        };
        self.start.viewbox(&style).add(&self.end.viewbox(&style))
    }
}

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for LineString<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        self.lines().map(|line| line.to_svg_str(style)).collect()
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        self.lines().fold(ViewBox::default(), |view_box, line| {
            view_box.add(&line.viewbox(style))
        })
    }
}

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for MultiLineString<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        self.0
            .iter()
            .map(|line_string| line_string.to_svg_str(style))
            .collect()
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        self.0
            .iter()
            .fold(ViewBox::default(), |view_box, line_string| {
                view_box.add(&line_string.viewbox(style))
            })
    }
}

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for Polygon<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        use std::fmt::Write;
        let mut path = String::new();
        for contour in std::iter::once(self.exterior()).chain(self.interiors().iter()) {
            let mut points = contour.points_iter();
            if let Some(first_point) = points.next() {
                write!(path, "M {} {}", first_point.x(), first_point.y()).unwrap()
            }
            for point in points {
                write!(path, " L {} {}", point.x(), point.y()).unwrap();
            }
            write!(path, " Z ").unwrap();
        }

        format!(
            r#"<path fill-rule="evenodd" d="{path}"{style}/>"#,
            path = path,
            style = style,
        )
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        self.exterior()
            .lines()
            .chain(
                self.interiors()
                    .iter()
                    .flat_map(|interior| interior.lines()),
            )
            .fold(ViewBox::default(), |view_box, line_string| {
                view_box.add(&line_string.viewbox(style))
            })
    }
}

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for Rect<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        Polygon::from(*self).to_svg_str(style)
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        Polygon::from(*self).viewbox(style)
    }
}

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for Triangle<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        Polygon::new(self.to_array().iter().cloned().collect(), vec![]).to_svg_str(style)
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        Polygon::new(self.to_array().iter().cloned().collect(), vec![]).viewbox(style)
    }
}

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for MultiPolygon<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        self.0
            .iter()
            .map(|polygons| polygons.to_svg_str(style))
            .collect()
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        self.0
            .iter()
            .fold(ViewBox::default(), |view_box, polygons| {
                view_box.add(&polygons.viewbox(style))
            })
    }
}

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for Geometry<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        use Geometry::*;
        match self {
            Point(point) => point.to_svg_str(style),
            Line(line) => line.to_svg_str(style),
            LineString(line_tring) => line_tring.to_svg_str(style),
            Triangle(triangle) => triangle.to_polygon().to_svg_str(style),
            Rect(rect) => rect.to_polygon().to_svg_str(style),
            Polygon(polygon) => polygon.to_svg_str(style),
            MultiPoint(multi_point) => multi_point.to_svg_str(style),
            MultiLineString(multi_line_string) => multi_line_string.to_svg_str(style),
            MultiPolygon(multi_polygon) => multi_polygon.to_svg_str(style),
            GeometryCollection(geometry_collection) => geometry_collection.to_svg_str(style),
        }
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        use Geometry::*;
        match self {
            Point(point) => point.viewbox(style),
            Line(line) => line.viewbox(style),
            LineString(line_tring) => line_tring.viewbox(style),
            Triangle(triangle) => triangle.to_polygon().viewbox(style),
            Rect(rect) => rect.to_polygon().viewbox(style),
            Polygon(polygon) => polygon.viewbox(style),
            MultiPoint(multi_point) => multi_point.viewbox(style),
            MultiLineString(multi_line_string) => multi_line_string.viewbox(style),
            MultiPolygon(multi_polygon) => multi_polygon.viewbox(style),
            GeometryCollection(geometry_collection) => geometry_collection.viewbox(style),
        }
    }
}

impl<T: Num + NumCast + Copy + PartialOrd + Debug + Display> ToSvgStr for GeometryCollection<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        self.0
            .iter()
            .map(|geometry| geometry.to_svg_str(style))
            .collect()
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        self.0
            .iter()
            .fold(ViewBox::default(), |view_box, geometry| {
                view_box.add(&geometry.viewbox(style))
            })
    }
}

impl<'a, T: ToSvgStr> ToSvgStr for &'a [T] {
    fn to_svg_str(&self, style: &Style) -> String {
        self.iter()
            .map(|geometry| geometry.to_svg_str(style))
            .collect()
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        self.iter().fold(ViewBox::default(), |view_box, item| {
            view_box.add(&item.viewbox(style))
        })
    }
}

impl<T: ToSvgStr> ToSvgStr for Vec<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        self.iter()
            .map(|geometry| geometry.to_svg_str(style))
            .collect()
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        self.iter().fold(ViewBox::default(), |view_box, item| {
            view_box.add(&item.viewbox(style))
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, ToSvg};
    use geo_types::{LineString, Point, Polygon};

    #[test]
    fn test_point() {
        println!(
            "{}",
            Point::new(0.0, 0.0)
                .to_svg()
                .with_fill_color(Color::Named("red"))
                .with_radius(10.0)
                .with_stroke_color(Color::Named("black"))
                .and(
                    Point::new(50.0, 0.0)
                        .to_svg()
                        .with_radius(5.0)
                        .with_stroke_color(Color::Named("blue"))
                )
                .with_stroke_width(1.0)
                .with_opacity(0.5)
                .with_fill_opacity(0.5)
                .with_fill_color(Color::Named("green"))
        );
    }

    #[test]
    fn test_polygon() {
        println!(
            "{}",
            Polygon::new(
                LineString(vec![
                    (210.0, 0.0).into(),
                    (300.0, 0.0).into(),
                    (300.0, 90.0).into(),
                    (210.0, 90.0).into()
                ]),
                vec![LineString(vec![
                    (230.0, 20.0).into(),
                    (280.0, 20.0).into(),
                    (280.0, 70.0).into(),
                    (230.0, 70.0).into()
                ])]
            )
            .to_svg()
            .with_fill_color(Color::Named("black"))
            .with_stroke_color(Color::Named("red"))
        );
    }
}
