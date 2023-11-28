use crate::{Style, ToSvgStr, ViewBox};
use geo::{
    Coord, CoordNum, Geometry, GeometryCollection, Line, LineString, MultiLineString, MultiPoint,
    MultiPolygon, Point, Polygon, Rect, Triangle,
};
use num_traits::NumCast;

impl<T: CoordNum> ToSvgStr for Coord<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        Point::from(*self).to_svg_str(style)
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        Point::from(*self).viewbox(style)
    }
}

impl<T: CoordNum> ToSvgStr for Point<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        format!(
            r#"<circle cx="{x:?}" cy="{y:?}" r="{radius}"{style}/>"#,
            x = self.x(),
            y = self.y(),
            radius = style.radius.unwrap_or(1.0),
            style = style,
        )
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        let radius = style.radius.unwrap_or(1.0) + style.stroke_width.unwrap_or(1.0);
        ViewBox::new(
            NumCast::from(self.x()).unwrap_or(0f32) - radius,
            NumCast::from(self.y()).unwrap_or(0f32) - radius,
            NumCast::from(self.x()).unwrap_or(0f32) + radius,
            NumCast::from(self.y()).unwrap_or(0f32) + radius,
        )
    }
}

impl<T: CoordNum> ToSvgStr for MultiPoint<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        self.0.iter().map(|point| point.to_svg_str(style)).collect()
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        self.0.iter().fold(ViewBox::default(), |view_box, point| {
            view_box.and(&point.viewbox(style))
        })
    }
}

impl<T: CoordNum> ToSvgStr for Line<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        format!(
            r#"<path d="M {x1:?} {y1:?} L {x2:?} {y2:?}"{style}/>"#,
            x1 = self.start.x,
            y1 = self.start.y,
            x2 = self.end.x,
            y2 = self.end.y,
            style = style,
        )
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        let style = Style {
            radius: Some(0.0),
            ..style.clone()
        };
        self.start.viewbox(&style).and(&self.end.viewbox(&style))
    }
}

impl<T: CoordNum> ToSvgStr for LineString<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        self.lines().map(|line| line.to_svg_str(style)).collect()
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        self.lines().fold(ViewBox::default(), |view_box, line| {
            view_box.and(&line.viewbox(style))
        })
    }
}

impl<T: CoordNum> ToSvgStr for MultiLineString<T> {
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
                view_box.and(&line_string.viewbox(style))
            })
    }
}

impl<T: CoordNum> ToSvgStr for Polygon<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        use std::fmt::Write;
        let mut path = String::new();
        for contour in std::iter::once(self.exterior()).chain(self.interiors().iter()) {
            let mut points = contour.points();
            if let Some(first_point) = points.next() {
                write!(path, "M {:?} {:?}", first_point.x(), first_point.y()).unwrap()
            }
            for point in points {
                write!(path, " L {:?} {:?}", point.x(), point.y()).unwrap();
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
                view_box.and(&line_string.viewbox(style))
            })
    }
}

impl<T: CoordNum> ToSvgStr for Rect<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        Polygon::from(*self).to_svg_str(style)
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        Polygon::from(*self).viewbox(style)
    }
}

impl<T: CoordNum> ToSvgStr for Triangle<T> {
    fn to_svg_str(&self, style: &Style) -> String {
        Polygon::new(self.to_array().iter().cloned().collect(), vec![]).to_svg_str(style)
    }

    fn viewbox(&self, style: &Style) -> ViewBox {
        Polygon::new(self.to_array().iter().cloned().collect(), vec![]).viewbox(style)
    }
}

impl<T: CoordNum> ToSvgStr for MultiPolygon<T> {
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
                view_box.and(&polygons.viewbox(style))
            })
    }
}

impl<T: CoordNum> ToSvgStr for Geometry<T> {
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

impl<T: CoordNum> ToSvgStr for GeometryCollection<T> {
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
                view_box.and(&geometry.viewbox(style))
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
            view_box.and(&item.viewbox(style))
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
            view_box.and(&item.viewbox(style))
        })
    }
}
